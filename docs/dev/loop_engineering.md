# Loop Engineering L1 治理层开发者文档

> 适用仓库：`mao_agent`（单 crate：向量数据库 + 检索 Agent 引擎）。
> 本文档只描述已落地的 L1 治理层现状：`gate.yaml`、`scripts/gate_check.py`、`scripts/hook_guard.py`、`scripts/loop_l1_inspect.py`、`loop-budget.md`、`loop-constraints.md`、`STATE.md`。
> 运行手册中的对应章节为 `docs/ops/runbook.md` §7，政策原文以仓库根目录文件为准。

## 1. L1 是什么：设计意图与边界

L1（Level-1 loop）是一组**面向机器自主循环的门禁与巡检脚本**，目标是让无人值守的 Agent 循环：

- 写不坏东西（不碰密钥、不碰索引产物、不碰 `corpus/**/raw/**` 原始资产、不删改测试）；
- 停得下来（预算熔断、`MAO_LOOP_PAUSE` kill switch）；
- 留下可审计的状态（唯一允许的循环写操作：原子替换根目录 `STATE.md`）。

硬性边界（见 `loop-constraints.md` 与 `docs/ops/runbook.md` §7）：

- L1 是 **report-only**：可以读、可以跑检查、可以用 `--full` 重写 `STATE.md`，**必须不改写任何应用源码**（Rust/Python）。
- 索引产物对循环只读：`data/*.bin`、`data/tantivy_index/**`、`*.embedcache`、`.fastembed_cache/**` 只能由人工批准的 `cargo run -- ingest` 重建，不允许手工 patch。
- 每个问题最多 **3 次**自动化修复尝试，之后必须升级给人工（对应 gate checker 退出码 `2`）。
- 测试红线：不允许删除、跳过、`#[ignore]` 测试，不允许放宽断言（Recall 门限、引用拒识套件、hook/gate 自检）。

## 2. 文件一览

| 文件 | 角色 |
|------|------|
| `gate.yaml` | 机器可读的路径政策：`denylist`、`maxFiles`、`autoMergeAllowlist`。被 `gate_check.py` 与 `hook_guard.py` 共同执行。需与 `loop-constraints.md` 保持同步。 |
| `scripts/gate_check.py` | 政策检查器（仅 Python 3 标准库）。CLI：`check --action <commit\|tool\|merge\|auto-merge> --paths p1,p2`。自检：`--self-test`。 |
| `scripts/hook_guard.py` | 生命周期 Hook 守卫：`pre-tool`（写前拦截）与 `stop`（停止前 `cargo check`）。自检：无参运行即 `self-check`。复用 `gate_check.check_gate`。 |
| `scripts/loop_l1_inspect.py` | 只读巡检流水线：`--check-integrity`（门 1–3）或 `--full`（门 1–6，含离线评测与 `STATE.md` 原子写）。 |
| `loop-budget.md` | 日配额政策与降级协议（政策上限，非实测消费）。 |
| `loop-constraints.md` | 对自主 Agent 的绑定规则（测试红线、产物不变式、尝试预算、路径接线）。 |
| `STATE.md` | 运行状态脊柱：Snapshot、Retrieval Gate、High Priority、Watch List、Recent Noise 共 5 节。L1 唯一可写的生产文件。 |

## 3. L1 六道门禁流水线（`loop_l1_inspect.py`）

两种运行模式：

- `--check-integrity`：只跑门 1–3（编译 + 语料 + 索引存在性），不跑评测，不写 `STATE.md`。
- `--full`：跑门 1–6（再加离线 Recall@5、hard-negative 测试、`STATE.md` 原子替换）。

不带任何 flag 直接运行会打印用法并以退出码 `1` 结束。`MAO_LOOP_PAUSE` 置位时直接退出码 `2`（见第 6 节），不跑任何门、不写 `STATE.md`。

最终退出码：全部 `ok` 则 `0`，任一门失败则 `1`，熔断则 `2`。

| 门 | 名称（脚本内） | 检查内容（以源码为准） | 失败判据 |
|----|---------------|----------------------|----------|
| 1 | `1 compile` | 运行 `cargo check --no-default-features`，超时 180s。`cargo` 不存在或超时也算失败。 | 非零退出、`cargo` 缺失、超时。 |
| 2 | `2 corpus` | `corpus/` 必须存在；递归枚举 markdown，把任一路径分量（大小写不敏感）含 `raw` 的文件划为 raw 资产并隔离；其余 `.md`/`.markdown` 为 active；文件名（不分大小写）为 `readme.md`/`index.md`/`changelog.md` 的记为 nav 并跳过 frontmatter 检查；其余 formal 文档必须以 `---` 开头且首个 frontmatter 块内有 `title:` 行（正则 `(?m)^title\s*:`）。 | `corpus/` 缺失、raw 混入 active 集、任一 formal 文档缺 `title:`。详情附带 `active`/`raw`/`formal`/`nav` 计数。 |
| 3 | `3 indexes` | 对 `data/vector_store.bin` + `data/tantivy_index/` 与 `data/offline_run2/vector_store.bin` + `data/offline_run2/tantivy_index/` 逐一判定 `_index_ok`：bin 文件存在且非空，tantivy 目录存在且含 `meta.json`。两者任一可用即通过。`data/graph_store.bin` 仅记录存在与否（缺失记为 `graph_store.bin absent (no-op)`，不判失败）。 | 两处快照(select)均不可用。 |
| 4 | `4 recall`（仅 `--full`） | 要求 `evals/retrieval/queries.jsonl` 存在；按顺序试 `data/`、`data/offline_run2/`、`data/offline_run/` 三处快照，对第一处 `_index_ok` 的快照运行 `cargo run --no-default-features --quiet -- eval-retrieval --k 5 --mode hybrid --no-rerank --offline --json --queries-file <queries> --index-file <bin> --tantivy-dir <tv>`（超时 600s）。stdout 逐行解析 `{"type":"summary"}` 与 `{"type":"query"}`；门限 `RECALL_GATE = 0.99`（`recall >= 0.99` 通过）。若某快照因维度不匹配失败（stdout+stderr 含 `向量模型不匹配`、`IdentityMismatch`、`DimensionMismatch` 任一）则换下一个候选。 | queries 文件缺失、`cargo` 缺失、评测超时、评测进程非零退出、无 `summary` JSON、`Recall@5 < 0.99`。详情形如 `Recall@5=1.000 MRR@5=0.989 NDCG@5=0.992 n=105 index=data/vector_store.bin`。 |
| 5 | `5 hard-neg`（仅 `--full`） | 运行 `cargo test --no-default-features --test retrieval_hard_eval_test -- --test-threads 1`（超时 300s）。 | 非零退出、`cargo` 缺失、超时。成功备注为 `retrieval_hard_eval_test ok (8-gram hard gold)`。 |
| 6 | `6 STATE.md`（仅 `--full`） | 用 `tempfile.mkstemp` + `os.replace` 原子替换根目录 `STATE.md`，写回后校验文件含 `Last run:` 与首节标题，否则判失败。内容见第 7 节。 | 原子写抛 `OSError`、写回文件缺标题/被截断。 |

## 4. `gate.yaml` 语义与 `gate_check.py` 退出码

当前 `gate.yaml`（`version: 1`）：

```yaml
denylist:
  - "config.toml"
  - ".env*"
  - "data/*.bin"
  - "data/tantivy_index/**"
  - "corpus/**/raw/**"
  - "raw/**"
  - "*.embedcache"
  - ".fastembed_cache/**"
maxFiles: 8
autoMergeAllowlist:
  - "docs/**"
  - "**/*.md"
  - "AGENTS.md"
```

### 4.1 判定顺序（`evaluate()` 原样）

1. `--action` 不在 `commit`、`tool`、`merge`、`auto-merge` 四者之中 → `error`，退出码 `1`。
2. 路径列表为空 → `error`，退出码 `1`。
3. 任一路径命中 `denylist` 任一模式 → `escalate`，退出码 `2`（附 `matched_pattern`）。
4. 路径数量超过 `maxFiles` → `escalate`，退出码 `2`。语义是**严格大于**：`maxFiles: 8` 时 8 个文件放行、9 个文件升级（自检用 9 个 `src/f{i}.rs` 覆盖此分支）。
5. 仅当 `--action auto-merge`：每个路径必须命中 `autoMergeAllowlist` 任一模式，否则 `escalate`，退出码 `2`。注意 `commit`/`tool`/`merge` 不检查 allowlist。
6. 以上全过 → `allow`，退出码 `0`。

### 4.2 退出码含义

| 退出码 | 含义 | 常量 | 输出去向 |
|--------|------|------|----------|
| `0` | allow（放行） | `EXIT_ALLOW` | 原因写 stdout |
| `2` | escalate（升级给人工 / 拒绝自动执行） | `EXIT_ESCALATE` | 原因写 stderr |
| `1` | error（用法或政策文件错误，非政策拒绝） | `EXIT_ERROR` | 原因写 stderr |

`error` 的触发条件（源码 `check_gate` / `run_cli`）：非法 `--action`、未给路径、缺 `check` 子命令、`gate.yaml` 不存在或读失败、YAML 模式错误（`version` 缺失、版本非 `1`、`denylist`/`autoMergeAllowlist` 非字符串列表、`maxFiles` 非整数、顶层缩进异常、无 key 的列表项等）。`argparse` 自身的 exit 2 会被 `run_cli` 归一化为 `1`。

### 4.3 路径匹配细节（与直觉可能不同的地方）

- 归一化：反斜杠转正斜杠、去首尾空白、去 `./` 前缀、去前导 `/`；绝对路径尝试相对仓库根换算，换算失败则去前导 `/` 后按字面匹配。
- glob 是自研的 minimatch 风格 globstar（`_glob_to_regex`）：`**/` 匹配零或多级目录，`**` 匹配任意字符（含 `/`），`*` 不跨 `/`，`?` 匹配单个非 `/` 字符。
- 不含 `/` 的模式（如 `config.toml`、`.env*`、`*.embedcache`）会**额外匹配 basename**：即 `a/b/config.toml` 同样命中。
- `pat` 以 `/**` 结尾时，若去掉 `/**` 后能匹配则也算命中。
- `--paths` 是逗号分隔，进函数前按逗号切分并 strip，空项丢弃。
- `--gate-file` 缺省时先看 `./gate.yaml`，不存在则回退到脚本所在仓库根的 `gate.yaml`。
- `--json` 输出 `{"decision","exit","reason","matched_pattern","action","paths"}`，供机器消费。

## 5. PreToolUse hook 接线（`hook_guard.py`）

`hook_guard.py` 有三种模式，由 `sys.argv[1]`（大小写不敏感，`_` 视为 `-`）决定，无参默认 `self-check`：

| 调用 | 作用 |
|------|------|
| `python scripts/hook_guard.py pre-tool`（别名 `pre-tool-use`、`pretool`） | PreToolUse：从 stdin 读 JSON payload，返回一行 JSON `{"decision": "allow" \| "deny"}`。 |
| `python scripts/hook_guard.py stop` | Stop 阶段：运行 `cargo check --no-default-features --quiet`（超时 120s），编译失败/超时/执行异常返回 `{"decision": "continue", "reason": ...}`（要求继续处理），成功返回 `{"decision": "allow"}`。 |
| `python scripts/hook_guard.py`（无参） | 自检：直调 `handle_pre_tool` 与一次 stdin 子进程往返，验证 `config.toml` 拒绝、`src/lib.rs` 放行、`corpus/**/raw/**` 拒绝、`data/vector_store.bin` 经 stdin 拒绝。 |

### 5.1 `pre-tool` 的三层检查（`handle_pre_tool` 原样）

1. **门禁路径拦截**：当工具名属于 `WRITE_TOOLS`（`write_to_file`、`replace_file_content`、`Write`、`write`、`search_replace`、`StrReplace`）时，从 `TargetFile`、`target_file`、`file_path`、`path`、`old_path`、`new_path` 六个键收集路径，以 `--action tool` 调 `check_gate`。非 `allow`（即 `escalate` **或** `error`）一律 `deny` ——这是 fail-closed：检查器出错也拒绝写。拒绝理由会引用 `gate.yaml` 并提示用 `cargo run -- ingest` 重建索引。
2. **`config.toml` 活密钥拦截**：目标文件归一化后以 `config.toml` 结尾或包含 `/config.toml` 时，扫描 `CodeContent`、`ReplacementContent`、`TargetContent`、`contents`、`new_string` 五个内容键；`CommandLine` 参数中若同时出现 `config.toml` 与疑似密钥也拦截。判定函数 `contains_live_secret`：匹配 `api_key`/`cohere_key`/`secret`/`token` 的 `key = "value"` 赋值或 `--api-key`/`--embed-api-key` 传参形态，且值不在占位符表（`your_api_key`、`placeholder`、`xxx`、`todo`、`none`、`<api_key>` 等）且长度 ≥ 8；或 32–64 位口令字符且上下文含 `api`/`key`/`cohere`/`token`/`secret`/`config`。命中则 `deny` 并提示使用占位符或 `COHERE_API_KEY`。
3. **索引破坏命令拦截**：对 `CommandLine` 参数，若非 `cargo` 开头，且匹配 `> ... data/vector_store.bin|tantivy_index` 重定向或 `rm|del|erase|remove-item ... data/vector_store.bin|tantivy_index` 删除形态，则 `deny` 并提示用 `cargo run -- ingest` 重建。

payload 形状兼容两种键风格：`toolCall`/`tool_call` 下的 `name` + `args`/`input`，或顶层 `tool` + `args`。`args` 非 dict 时按空 dict 处理。

### 5.2 输出契约

- `pre-tool` 只返回 `allow` 或 `deny`，附 `reason`。
- `stop` 只返回 `allow` 或 `continue`（注意不是 `deny`）：编译失败时要求 agent 继续修复，不允许带着编译错误停机。
- 未知模式返回 `{"decision": "allow"}`。

## 6. `MAO_LOOP_PAUSE` 熔断开关

- 语义：置位即停。`loop_l1_inspect.py` 的 `pause_requested()` 在解析任何门之前检查环境变量，取值 strip 后为 `1`、`true`、`TRUE`、`yes` 任一即视为请求暂停；此时向 stderr 写 `MAO_LOOP_PAUSE is set; L1 inspect escalating` 并以退出码 `2` 立即返回，**不跑评测、不写 `STATE.md`**。
- `loop-constraints.md` 补充：置位时“stop immediately”。
- 恢复：unset 该变量，并在 `STATE.md` 的 `Last run` 中记录恢复（`loop-budget.md` Kill switch 节）。

```bash
MAO_LOOP_PAUSE=1 python scripts/loop_l1_inspect.py --full; echo "exit=$?"
# expect: stderr 熔断提示，exit=2
```

## 7. `STATE.md`：L1 唯一可写文件

`gate_state()` 在 `--full` 下用原子写（`tempfile.mkstemp` + `os.replace`，失败清理临时文件）重建 `STATE.md`，固定 5 节标题（`HEADINGS`）：

1. `Corpus & Index State Snapshot` ——由门 2 的计数与门 3 的详情渲染成表格（active markdown 数、`title:` formal 数、nav 数、raw 资产数、索引字节数）。
2. `Retrieval Baseline Gate` ——门 4 的 `summary`（Recall@5 / MRR@5 / NDCG@5 / `n_queries` / `k` / `mode` / `index` / flags），门限标注 `Recall@5 ≥ 0.99`。无 summary 时记 `FAIL / skipped`。
3. `High Priority` ——合并逻辑：保留旧文件中**非** `- [L1]` 开头的非空行，丢弃占位行 `- (empty — L1 ...)`，再把本次新的 `- [L1] ...` 行**前置**。新行来源：每个失败的门一行（`[L1] <gate> failed: <首行详情>`），每个 Recall miss 的 query 一行（含 `recall=` 与截断 80 字的 query）。无人为行且无失败时为 `- (empty)`。
4. `Watch List` ——原样保留旧内容，空则 `- (empty)`。
5. `Recent Noise` ——原样保留旧内容，空则 `- (empty)`。

文件头固定两行：`Last run: <UTC ISO Z> (<mode>)` 与 `Mode: report-only L1`。

## 8. 预算与降级协议（`loop-budget.md`）

配额是**政策上限**，不是实测账单。调用远端 embed/chat/rerank 前必须先读该文件。

| Provider | CLI 触点 | 日调用上限 | 日 token 上限 | ≥80% 时降级 |
|----------|----------|-----------|--------------|-------------|
| SiliconFlow embed（`BAAI/bge-m3`，1024-dim） | `ingest` / `search` / `ask` `--embed-provider siliconflow` | 1800 | 400000 | `--offline`（DeterministicEmbedder 512-dim；若维度变化需重建索引） |
| Gemini embed（`gemini-embedding-2`，768-dim） | `--embed-provider gemini` | 400 | 160000 | 还有 key 则用 SiliconFlow，否则 `--offline` |
| Cohere chat（`command-r7b-12-2024`） | `ask` / `serve` | 80 | 160000 | `generate_offline_dialectical_answer`（不走网络） |
| Cohere rerank（`rerank-v3.5`） | `search` / `ask` / `serve` | 80 | n/a（文档进、分数出） | `--no-rerank` |
| L1 inspect | `scripts/loop_l1_inspect.py` | 4 | 0 remote | 恒为 `--offline --no-rerank`；绝不调用 SiliconFlow/Cohere/Gemini |

降级协议（达任一 cap 的 80% 即执行）：

1. 后续 retrieval/ask 统一加 `--offline --no-rerank`。
2. 不得在 L1 循环内用 SiliconFlow/Gemini 重建索引（维度会分叉）。
3. 在 `STATE.md` High Priority 记录一行 `- [L1] budget 80% — offline`。
4. 人工清除该 High Priority 行之前不得恢复远端 provider。

补充：CLI 侧已有免费档 pacing（`--batch-size` 16–32，远端 batch 间隔 100ms，约 2000 RPM / 500k TPM），循环上限设得更低，防止 sweeper 烧光账户。L1 `--check-integrity` 是纯本地（`cargo check` + 文件系统）；`--full` 增加离线 `eval-retrieval` 与 `retrieval_hard_eval_test`，只要保持 `--offline --no-rerank` 就零计费 token。

## 9. 常见使用示例

```bash
# 门禁检查：允许 vs 升级（注意退出码）
python scripts/gate_check.py check --action tool --paths "src/lib.rs"          # expect exit 0
python scripts/gate_check.py check --action tool --paths "config.toml"         # expect exit 2
python scripts/gate_check.py check --action tool --paths "corpus/papers_we_love/raw/test.pdf"  # expect exit 2
python scripts/gate_check.py check --action commit --paths "data/vector_store.bin"             # expect exit 2
python scripts/gate_check.py check --action auto-merge --paths "docs/ops/runbook.md"           # expect exit 0
python scripts/gate_check.py check --action auto-merge --paths "src/lib.rs"                    # expect exit 2
python scripts/gate_check.py check --action tool --paths "src/lib.rs" --json   # 机器可读决策

# 自检（gate + hook）
python scripts/gate_check.py --self-test
python scripts/hook_guard.py

# PreToolUse 手动探查（stdin 输入 JSON payload）
echo '{"toolCall":{"name":"write_to_file","args":{"TargetFile":"data/vector_store.bin","CodeContent":"x"}}}' \
  | python scripts/hook_guard.py pre-tool
# expect: {"decision": "deny", ...}

# L1 巡检
python scripts/loop_l1_inspect.py --check-integrity   # 门 1–3：编译 + 语料/raw + 索引
python scripts/loop_l1_inspect.py --full              # 门 1–6：再加离线 Recall@5、hard-negative、STATE.md 原子写

# 熔断验证
MAO_LOOP_PAUSE=1 python scripts/loop_l1_inspect.py --full; echo "exit=$?"   # expect exit=2
```

## 10. 已知局限

如实记录当前实现与理想治理之间的差距，改动前请先读对应源码行：

- **Hook 对畸形 stdin 是 fail-open**：`hook_guard.py` 的 `main()` 中 `sys.stdin.read()` + `json.loads` 包在宽泛 `try/except` 内，解析失败或空输入时 `payload = {}`，进而 `handle_pre_tool({})` 返回 `allow`。即攻击者/故障调用方只要送畸形 JSON 就能绕过 PreToolUse。`self_check` 只覆盖合法 JSON 路径，未覆盖此分支。
- **写拦截只认识固定工具名与固定路径键**：路径检查仅对 `WRITE_TOOLS` 六个工具名与 `PATH_ARG_KEYS` 六个键生效；换用其他工具名/参数名（如把路径放在自定义键或嵌套结构里）即绕过门禁。`deny_if_gated` 在 `paths` 为空时直接返回放行。
- **Shell 破坏拦截覆盖面窄**：`CommandLine` 正则只拦截重定向/删除 `data/vector_store.bin|tantivy_index` 的两种写法，不覆盖 `denylist` 其余条目（如 `.env*`、`*.embedcache`、`.fastembed_cache/**`、tantivy 内单文件删除、跨盘符/大小写变体之外的写法），且 `cargo` 开头的命令一律跳过该层检查。
- **索引新鲜度未校验**：门 3 只检查 bin 非空 + tantivy 目录含 `meta.json`，不校验向量维度、不比较索引 mtime 与语料 mtime、不核对 chunk 数量。维度错配要到门 4 靠匹配错误字符串（`向量模型不匹配`、`IdentityMismatch`、`DimensionMismatch`）才被动发现，且只对三个硬编码候选路径有效。
- **评测候选路径硬编码**：门 4 只试 `data/`、`data/offline_run2/`、`data/offline_run/` 三处；自定义 `--index-file` 布局的快照不会被巡检到，可能出现“线上用 A、巡检测 B”的分裂。
- **语料门只认 `title:` 有无**：不校验 YAML 合法性与其他 frontmatter 字段（`author`/`date`/`period`/`volume`/`category`）；nav 豁免只看文件名（`readme/index/changelog.md`），同名正式文档会被误豁免；`raw` 隔离只看路径分量是否等于 `raw`（大小写不敏感），文件名含 `raw` 不算。
- **`STATE.md` 合并可能堆积**：`High Priority` 每次前置新的 `- [L1]` 行但从不自动清理旧 `- [L1]` 行（只保留非 L1 人工行），长期 `--full` 运行会使该节只增不减，需人工清。
- **预算无计量 enforcement**：`loop-budget.md` 的日上限与“4 次 L1/天”靠自觉遵守，脚本侧无计数器、无远端调用拦截；L1“绝不调远端”靠固定传 `--offline --no-rerank` 实现，而非网络 sandbox。
- **`merge` action 存在但文档面窄**：`ALLOWED_ACTIONS` 含 `merge`，但 `AGENTS.md`/runbook 示例只提 `commit|tool|auto-merge`；`merge` 与 `commit` 在判定逻辑上目前完全等价（都不查 allowlist），预留语义尚未用上。
- **自研 YAML/glob 非标准**：`parse_gate_yaml` 只支持注释、标量、标量列表三个子集，`glob_match` 只是 minimatch 近似实现，与 `.gitignore` 语义不完全一致；给 `gate.yaml` 加复杂写法前建议先跑 `--self-test` 验证。
