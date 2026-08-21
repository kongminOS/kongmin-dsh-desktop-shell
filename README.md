# Kongmin Rein

A thin desktop shell that wraps the **official DeepSeek Harness web UI** into a native desktop app.

> 纯客户端壳：把官方 DeepSeek Harness Web 版封装为桌面应用。不含任何治理/插件/服务代码——它只是官方 Web UI 的窗口。

## Why

DeepSeek Harness 的官方 Web 界面（`dsh web`，默认 `http://127.0.0.1:3080`）已经很好用。这个壳只是给你一个原生桌面窗口，而不是浏览器标签页。


## 国内镜像（Gitee）

- 国内镜像仓库：https://gitee.com/kongminos/kongmin-dsh-desktop-shell （免代理，国内下载/克隆更快）
- GitHub 原仓：https://github.com/kongminOS/kongmin-dsh-desktop-shell
## Requirements

- [DeepSeek Harness](https://github.com/deepseek-ai/dsh) CLI installed: `npm install -g @deepseek-ai/dsh`
- Node.js 20+

## Usage

```bash
# 1. Start the official engine
dsh web --port 3080

# 2. Launch the shell (or build & run)
cargo tauri dev
```

Or build a release installer:

```bash
cargo tauri build
```

## Known Issues & Fixes (DSH Engine 0.1.0-rc.6 / rc.7)

The following two issues belong to the **DeepSeek Harness engine itself** (not this shell). Both exist in DSH 0.1.0-rc.6 and rc.7. Machines patched as described below work normally; official releases may include these fixes later.

**1. Creating a session / selecting a model fails with `agent-presets: refusing to compose an unscoped context`**

- Symptom: In GUI mode, creating or resuming any agent (new session, resume, model switch) fails.
- Root cause: The engine dependency tree and the user config tree (`~/.dsh/profiles/web/node_modules`) each load a copy of `@deepseek-ai/dsh-scope`, where the scope tag `kScope = Symbol("dsh.scope")` is a module-local Symbol. The two instances differ, so `createScope` writes with one instance while `mount` reads with the other and always sees `undefined`.
- Fix: Change `const kScope = Symbol("dsh.scope")` to `const kScope = Symbol.for("dsh.scope")` in both copies (global-registry Symbol, shared across instances).

**2. Chat fails with `DeepSeek API error (HTTP 404)`**

- Symptom: Agents are created fine, but sending a message returns 404 from the LLM call.
- Root cause: If `llm-deepseek.baseURL` in `~/.dsh/settings.yaml` is set to `https://api.deepseek.com/anthropic` (Anthropic-compatible endpoint), it mismatches DSH's OpenAI-format adapter (which calls `/chat/completions`), so requests hit a non-existent path.
- Fix: Set `llm-deepseek.baseURL` to `https://api.deepseek.com` (or delete the key to use the default). settings.yaml hot-reloads.

## License

MIT — see [LICENSE](LICENSE).

---

*Kongmin Rein is an independent third-party wrapper. It is not affiliated with or endorsed by DeepSeek.*

