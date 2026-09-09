---
name: ai-codeblock-docs
description: 为 src/ 下每个 Rust 代码单元（.rs 文件）生成 1:1 的 AI 阅读 md 文档（docs/ai/ 镜像目录），并维护规范化资料同步索引与 AI 约束文档。当需要"给某个模块补 AI 文档"、"同步 docs/ai"、"生成/校验代码单元文档"时使用。
---

# AI Codeblock Docs Skill

为项目的每一个 Rust 代码单元维护一份供 AI 阅读的结构化 md 文档，保证 AI 在接入本项目时能以固定格式快速获得：该单元是什么、对外暴露什么、依赖什么、被谁依赖、有哪些硬约束与陷阱。

## 目录映射规则

- 源文件 `src/<path>/<name>.rs` → 文档 `docs/ai/src/<path>/<name>.md`（目录结构 1:1 镜像）。
- 根级文件：`src/lib.rs` → `docs/ai/src/lib.md`，`src/main.rs` → `docs/ai/src/main.md`，`src/config.rs` → `docs/ai/src/config.md`，`src/error.rs` → `docs/ai/src/error.md`，`src/model.rs` → `docs/ai/src/model.md`，`src/retry.rs` → `docs/ai/src/retry.md`。
- 全局约束文档：`docs/ai/CONSTRAINTS.md`（跨模块硬约束，禁止在单文件文档中重复大段抄写，只放链接引用）。
- 同步索引：`docs/ai/README.md`（由 `scripts/sync_ai_docs.py` 生成，不手改）。

## 单文档模板（必须严格遵守）

每个文档使用以下固定章节，禁止增删一级章节：

```markdown
# <单元名>（<src 相对路径>）

> AI 阅读文档 | 上游依赖: <…> | 下游消费: <…> | 状态: <stable|wip|deprecated>

## 1. 一句话职责
（≤ 40 字，说明该单元存在的唯一理由）

## 2. 对外接口（AI 调用面）
- `pub fn/type/trait/const` 清单：签名 + 一行语义 + 返回错误类型
- 标注 #[cfg(feature)] 门控的项必须写明 feature 名

## 3. 内部结构
（关键私有类型/函数的职责分块说明，≤ 10 条要点）

## 4. 数据流与调用链
（输入从哪来、输出到哪去；用 `A -> B -> C` 文本链表示）

## 5. 硬约束与陷阱（必须读）
- 离线/网络约束、维度匹配、所有权/生命周期坑、panic 路径、锁与并发
- 与 docs/ai/CONSTRAINTS.md 的条目互相引用（如 `[C-03]`）

## 6. 测试锚点
- 对应的单元测试 / tests/ 集成文件名，以及该单元的验证方式
```

## 生成流程（全流程）

1. **扫描**：`python scripts/sync_ai_docs.py --list` 获取 src 与 docs/ai 的覆盖差异表。
2. **阅读源码**：逐个通读目标 `.rs` 文件（含模块内 `mod.rs` 的 re-exports），文档内容必须来自真实代码，禁止凭模块名臆测。
3. **生成**：按模板写 `docs/ai/src/.../*.md`；先写 `## 5 硬约束与陷阱` 再写其余章节（陷阱优先）。
4. **引用编号**：跨模块约束统一引用 `docs/ai/CONSTRAINTS.md` 中的 `[C-xx]` 编号，不得自行发明编号。
5. **同步**：全部生成后运行 `python scripts/sync_ai_docs.py` 重建 `docs/ai/README.md` 索引（含覆盖矩阵与新鲜度）。
6. **验收**：
   - `sync_ai_docs.py --check` 退出码 0（src 与 docs/ai 1:1 无缺失）。
   - 每份文档含全部 6 个一级章节、`## 5` 非空。
   - 抽查 3 份文档：`## 2` 中每个 pub 项都能在源码中 grep 到。

## 同步规则（资料同步规范化）

- 源文件 mtime 晚于对应文档 → 视为 stale，索引中标 `stale`，需按流程重新生成该单元。
- 删除源文件时必须同步删除对应文档；重命名 = 删 + 建。
- `docs/ai/README.md` 只允许由 `sync_ai_docs.py` 产出（幂等，可重复运行）。
- 单元文档之间不互相内容复制，一律以 `[C-xx]` 或相对链接引用，避免多处失步。

## 禁止事项

- 禁止在单元文档中粘贴大段源码（> 15 行）。
- 禁止记录 API key、token、真实路径下的机密配置值。
- 禁止修改任何 `.rs` 源文件——本技能只读源码、只写 docs/ai。
