# Pi Agent 对接与使用指南

本文档指导如何将 `mao_agent` 作为辩证工程参谋与文献外脑，无缝接入到 **Pi Coding Agent** (`@earendil-works/pi-coding-agent`)。

---

## 核心认知：无感协议解耦

`mao_agent` 的后端内核（Rust）遵循标准 **JSON-RPC 2.0 / MCP (2024-11-05)** 规范，**本身完全不依赖 DSH**：
- 导出两项标准 MCP 工具：
  1. `query_dialectical_principles`：检索辩证原则三元组（正题、反题、合题、主要矛盾、张力点）。
  2. `verify_historical_citation`：原典文献引用一致性核查与置信度评分。
- 支持 **Stdio 本地子进程** 与 **Streamable HTTP 常驻服务** 双通道。

---

## 接入方式一：通过 `pi-mcp-adapter` (推荐)

Pi 官方提倡极简内核，推荐通过社区适配器 `pi-mcp-adapter` 挂载 MCP 服务，该适配器通过单代理工具避免初始 Token 膨胀：

### 1. 安装适配器
```bash
pi install npm:pi-mcp-adapter
```

### 2. 配置 MCP 服务器
将 `docs/pi/mcp.json` 的内容复制或合并到你的全局配置 `~/.pi/agent/mcp.json`（或项目根目录 `.pi/mcp.json`）：

#### 选项 A：Stdio 本地进程通道 (单机开发推荐)
```json
{
  "mcpServers": {
    "mao": {
      "command": "cargo",
      "args": ["run", "--release", "--", "mcp", "--offline"],
      "cwd": "D:\\rust\\mao_agent",
      "env": {
        "RUST_LOG": "warn"
      }
    }
  }
}
```

#### 选项 B：Streamable HTTP 通道 (常驻服务推荐)
1. 启动 `mao_agent` HTTP 服务：
   ```bash
   cargo run --release -- serve --bind 127.0.0.1:3210
   ```
2. 在 `mcp.json` 中配置：
   ```json
   {
     "mcpServers": {
       "mao": {
         "url": "http://127.0.0.1:3210/api/v1/mcp"
       }
     }
   }
   ```

---

## 接入方式二：Pi 原生 Skill 发现

若希望保持 Pi 的纯粹极简，不引入常驻 MCP 适配器，可以直接使用 Pi 的原生 Skill 机制：

1. 将 `docs/pi/skills/dialectical-counselor` 复制到全局或项目 Skill 目录：
   - 全局路径：`~/.pi/agent/skills/dialectical-counselor/SKILL.md`
   - 项目路径：`.pi/skills/dialectical-counselor/SKILL.md`
2. Pi 在启动时会自动扫描该技能。当用户遇到重大分歧或复杂任务时，Pi 会自主遵循辩证参谋的四步方法论展开行动。

---

## 接入方式三：系统指令 (System Persona)

将 `docs/pi/dialectical_counselor_persona.md` 中的精简宪章追加到：
- `~/.pi/agent/SYSTEM.md`（全局）或
- 当前项目工作区的 `AGENTS.md`（项目级）

使 Pi Agent 在调用其原生 4 核心工具 (`read`, `write`, `edit`, `bash`) 时，自带“调查研究、抓主要矛盾、集中兵力打歼灭战、实践检验”的工程直觉。
