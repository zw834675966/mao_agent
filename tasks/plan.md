# Implementation Plan: Cycle 15 — Loop Engineering Architecture Alignment

## Overview

基于对标开源业界权威方案 [`cobusgreyling/loop-engineering`](https://github.com/cobusgreyling/loop-engineering)（基于 Addy Osmani 与 Boris Cherny 的 Loop Engineering 理念及 arXiv:2608.21884 学术成果），将 `mao_agent` 从被动式单体检索外脑，升维构建为具备**硬安全围栏（Hard Safety Gate）、运行期状态中枢（Durable State）、模型预算熔断（Token Budgeting）与首个自治巡检外循环（L1 Corpus & Retrieval Health Sweeper）**的闭环自治系统。

严格贯彻 **Google Engineering Practices**（小粒度单责任 CL、密封测试）与 **最小切片法则**，将所有工作项垂直切分为 **3 个阶段、9 个原子任务（XS/S 级，单任务 ≤ 2 个文件）**，每个切片均包含独立可执行的验证命令。

---

## 最小颗粒度核查矩阵 (Granularity Audit Matrix)

| Task ID | 任务名称 | 影响文件数 | 颗粒度评级 | 为什么已达到最小颗粒度（不可再分理由） |
| :--- | :--- | :--- | :--- | :--- |
| **15-1** | 制定机器安全门禁 `gate.yaml` | 1 (`gate.yaml`) | **XS** | 纯声明式策略：仅定义路径黑名单、maxFiles 与白名单，不含执行代码 |
| **15-2** | 实现门禁检查器 `scripts/gate_check.py` | 1 (`scripts/gate_check.py`) | **XS** | 单脚本：实现与 upstream `loop-gate` 协议完全兼容的 CLI 检查器（0/2/1 退出码） |
| **15-3** | 接驳 Hook 拦截器与门禁 | 1 (`scripts/hook_guard.py`) | **XS** | 单脚本手术：仅重构 `PreToolUse` 文件写入拦截，将硬编码收敛至 `gate.yaml` |
| **15-4** | 建立运行期状态中枢 `STATE.md` | 1 (`STATE.md`) | **XS** | 单状态文件：规范语料树哈希、索引元数据、召回基线与待办/观察哨分区 |
| **15-5** | 建立模型预算与熔断 `loop-budget.md` | 1 (`loop-budget.md`) | **XS** | 单规范文档：定义 SiliconFlow、Cohere、Gemini 与本地离线的每日额度与熔断开关 |
| **15-6** | 建立智能体约束 `loop-constraints.md` | 1 (`loop-constraints.md`) | **XS** | 单规范文档：明确测试禁删红线、二进制索引只读红线与 3 次修复重试上限 |
| **15-7** | 实现 L1 语料与索引完整性探测 | 1 (`scripts/loop_l1_inspect.py`) | **XS** | 单脚本骨架：实现 Gate 1-3（构建编译、语料 frontmatter 校验、raw 隔离、索引存在性） |
| **15-8** | 实现 L1 召回基线检测与状态自动回写 | 1 (`scripts/loop_l1_inspect.py`) | **XS** | 单脚本增强：驱动 `eval-retrieval --json`，解析 summary 并原子回写 `STATE.md` |
| **15-9** | 同步运维总规约 Runbook 与 AGENTS.md | 2 (`docs/ops/runbook.md`, `AGENTS.md`) | **S** | 仅同步 Loop Engineering 架构索引与使用指南，保持 SSOT 统一 |

---

## 架构演进与依赖图谱 (Architecture Dependency Graph)

```
[Phase 1: 硬安全门禁与机器防护]
  Task 15-1 (gate.yaml 规范) ──► Task 15-2 (gate_check.py 检查器) ──► Task 15-3 (hook_guard 接驳)
                                                                            │
                                                                  [Checkpoint 15-1]
                                                                            │
[Phase 2: 持久化状态账本与成本熔断]                                           ▼
  Task 15-4 (STATE.md 中枢) ──► Task 15-5 (loop-budget 预算) ──► Task 15-6 (constraints 约束)
                                                                            │
                                                                  [Checkpoint 15-2]
                                                                            │
[Phase 3: 首个自治外循环 L1 与总索引]                                         ▼
  Task 15-7 (L1 语料索引探测) ──► Task 15-8 (L1 召回基线回写) ──► Task 15-9 (Runbook/AGENTS)
                                                                            │
                                                                  [Checkpoint 15-3: 全量交付验收]
```

---

## 任务拆解详情 (Task Breakdown)

### Phase 1: Hard Safety Gate & Machine Enforcement (硬安全门禁与机器防护)

#### Task 15-1: Define Machine-Readable `gate.yaml` Policy
- **Description**: 在项目根目录下创建 `gate.yaml`，严格定义不可篡改的 `denylist`（覆盖 `config.toml`, `.env*`, `data/*.bin`, `data/tantivy_index/**`, `corpus/**/raw/**`, `raw/**`, `*.embedcache`, `.fastembed_cache/**`）、文件上限 `maxFiles: 8`，以及文档白名单 `autoMergeAllowlist`。
- **Acceptance criteria**:
  - 根目录存在 `gate.yaml`，包含 `version: 1`。
  - `denylist` 严格包含上述 8 类敏感与派生路径。
  - `maxFiles` 设为 8。
- **Verification**: `python -c "import yaml; d=yaml.safe_load(open('gate.yaml')); assert d['version']==1 and len(d['denylist'])>=8"`
- **Dependencies**: None
- **Files likely touched**: `gate.yaml`
- **Estimated scope**: XS (1 file)

#### Task 15-2: Implement Standalone `scripts/gate_check.py` CLI Tool
- **Description**: 使用纯 Python 3 标准库（零外部依赖）实现与 upstream `loop-gate` 协议完全兼容的命令行门禁检查器，支持 `--action <commit|tool|auto-merge>`、`--paths`、`--gate-file` 与 `--json`。
- **Acceptance criteria**:
  - 命中 `denylist` 退出码为 `2` (ESCALATE)。
  - 变更文件数超过 `maxFiles` 退出码为 `2` (ESCALATE)。
  - 合法文件放行退出码为 `0` (ALLOWED)。
- **Verification**:
  - `python scripts/gate_check.py check --action tool --paths "config.toml"` 返回 2
  - `python scripts/gate_check.py check --action tool --paths "src/lib.rs"` 返回 0
  - `python scripts/gate_check.py check --action tool --paths "corpus/papers_we_love/raw/doc.pdf"` 返回 2
- **Dependencies**: Task 15-1
- **Files likely touched**: `scripts/gate_check.py`
- **Estimated scope**: XS (1 file)

#### Task 15-3: Connect Gate Check to `scripts/hook_guard.py` PreToolUse
- **Description**: 重构 `scripts/hook_guard.py`，在 `handle_pre_tool` 拦截到 `write_to_file` 与 `replace_file_content` 时，直接提取目标路径并调用 `gate_check` 执行机械拦截，消除硬编码。
- **Acceptance criteria**:
  - `hook_guard.py` 内部引入 `gate_check` 校验模块。
  - 拦截到 `gate.yaml` 黑名单路径时阻断执行并输出友好安全告警。
- **Verification**: `python scripts/hook_guard.py` 运行自检通过
- **Dependencies**: Task 15-2
- **Files likely touched**: `scripts/hook_guard.py`
- **Estimated scope**: XS (1 file)

---

### Phase 2: Durable State Machine & Cost Governance (持久化状态账本与成本熔断)

#### Task 15-4: Establish Operational `STATE.md` Schema and Baseline Ledger
- **Description**: 建立根目录动态记忆中枢 `STATE.md`，记录语料库规范文档规模、当前索引快照、离线检索基线指标（Recall@5/MRR@5/NDCG@5）、High Priority 阻断项、Watch List 与 Recent Noise 分区。
- **Acceptance criteria**:
  - `STATE.md` 包含标准 ISO-8601 `Last run` 字段与触发模式。
  - 包含 `Corpus & Index State Snapshot` 与 `Retrieval Baseline Gate` 结构化表格。
  - 具备 High Priority、Watch List 与 Recent Noise 分区。
- **Verification**: 检查 `STATE.md` 包含所有必需的章节标头。
- **Dependencies**: Checkpoint 15-1
- **Files likely touched**: `STATE.md`
- **Estimated scope**: XS (1 file)

#### Task 15-5: Establish Multi-Model Quotas in `loop-budget.md`
- **Description**: 制定 `loop-budget.md`，为 SiliconFlow、Cohere Chat、Cohere Rerank、Gemini 与本地离线模式设定日请求/Token 硬顶，建立达 80% 配额自动降级为 `--offline --no-rerank` 的协议与 `MAO_LOOP_PAUSE` 熔断总闸。
- **Acceptance criteria**:
  - 文档包含日配额分配表（Max Calls/Day, Max Tokens/Day, Fallback）。
  - 明确声明超出预算时的降级协议与 Kill Switch。
- **Verification**: 人工审查文档结构与参数严谨性。
- **Dependencies**: Task 15-4
- **Files likely touched**: `loop-budget.md`
- **Estimated scope**: XS (1 file)

#### Task 15-6: Establish Agent Constraints in `loop-constraints.md`
- **Description**: 制定 `loop-constraints.md`，确立不可妥协的工程红线：禁止删除/压制测试促成 CI、禁止直接手工修改 `data/*.bin`、单任务最大重试 3 次超限升级人工。
- **Acceptance criteria**:
  - 包含三大不可动摇红线（Testing Redlines, Artifacts Invariants, Max Attempts Cap）。
- **Verification**: 人工审查约束覆盖面。
- **Dependencies**: Task 15-5
- **Files likely touched**: `loop-constraints.md`
- **Estimated scope**: XS (1 file)

---

### Phase 3: First Autonomous L1 Loop (首个 L1 自治巡检外循环)

#### Task 15-7: Implement L1 Corpus & Index Integrity Inspector in `scripts/loop_l1_inspect.py`
- **Description**: 编写纯 Python 3 标准库脚本 `scripts/loop_l1_inspect.py`，实现 Gate 1（Cargo 编译与 clippy 门禁）、Gate 2（扫描 `corpus/` 校验 frontmatter 必填项及断言 `raw/` 零污染）、Gate 3（`data/vector_store.bin` 与 `data/tantivy_index/meta.json` 制品完整性）。
- **Acceptance criteria**:
  - `scripts/loop_l1_inspect.py` 支持 `--check-integrity` 模式。
  - 能准确校验 447 篇正式文档的 YAML frontmatter 并验证排除 `raw/` 下资产。
  - 任何检查不合规输出明确错误摘要并以退出码 1 退出。
- **Verification**: `python scripts/loop_l1_inspect.py --check-integrity`
- **Dependencies**: Checkpoint 15-2
- **Files likely touched**: `scripts/loop_l1_inspect.py`
- **Estimated scope**: XS (1 file)

#### Task 15-8: Implement L1 Baseline Retrieval Gate & `STATE.md` Sync
- **Description**: 在 `scripts/loop_l1_inspect.py` 中扩充 Gate 4-6：执行离线检索评测 `cargo run --no-default-features -- eval-retrieval --k 5 --mode hybrid --no-rerank --offline --json`，解析 summary JSON，断言 `recall_at_k >= 0.99`，并原子写回 `STATE.md`（更新时间戳、最新指标，若有失败用例捕获写入 High Priority）。
- **Acceptance criteria**:
  - `scripts/loop_l1_inspect.py --full` 完整跑通 6 道门禁。
  - 运行后 `STATE.md` 中的 `Last run` 与 `Retrieval Baseline Gate` 获得真实数据原子更新。
  - 整个过程遵循 Report-Only 哲学，不产生代码提交。
- **Verification**: `python scripts/loop_l1_inspect.py --full`
- **Dependencies**: Task 15-7
- **Files likely touched**: `scripts/loop_l1_inspect.py`
- **Estimated scope**: XS (1 file)

#### Task 15-9: Update Runbook and AGENTS.md with Loop Engineering Protocol
- **Description**: 同步项目运维总规约 [`docs/ops/runbook.md`](file:///D:/rust/mao_agent/docs/ops/runbook.md) 与全局准则 [`AGENTS.md`](file:///D:/rust/mao_agent/AGENTS.md)，正式引入 Loop Engineering 规范（`gate.yaml`、`STATE.md`、`loop-budget.md` 与 L1 巡检指令）。
- **Acceptance criteria**:
  - `docs/ops/runbook.md` 增加第 7 节 “Loop Engineering & Automated Maintenance”。
  - `AGENTS.md` 明确声明智能体在写盘前必须遵从 `gate.yaml` 约束并定期同步 `STATE.md`。
- **Verification**: `cargo fmt --check`
- **Dependencies**: Task 15-8
- **Files likely touched**: `docs/ops/runbook.md`, `AGENTS.md`
- **Estimated scope**: S (2 files)

---

## 风险与应对策略 (Risks & Mitigations)

| 风险 | 影响等级 | 应对策略 |
| :--- | :--- | :--- |
| **`eval-retrieval` 耗时过长** | 中 | L1 巡检脚本默认调用 `--k 5 --offline --no-rerank`，耗时约 3~5 秒，绝不在巡检中调用远程 API。 |
| **Windows 下文件锁定** | 低 | 脚本读取二进制与文本文件时统一显式采用只读与上下文管理器，避免持有打开句柄。 |
| **`STATE.md` 被并发修改覆盖** | 低 | 脚本写回采用“生成临时文件 + `os.replace` 原子替换”模式，防止文件截断。 |
