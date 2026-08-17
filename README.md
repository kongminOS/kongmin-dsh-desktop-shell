# Kongmin Rein

A thin desktop shell that wraps the **official DeepSeek Harness web UI** into a native desktop app.

> 纯客户端壳：把官方 DeepSeek Harness Web 版封装为桌面应用。不含任何治理/插件/服务代码——它只是官方 Web UI 的窗口。

## Why

DeepSeek Harness 的官方 Web 界面（`dsh web`，默认 `http://127.0.0.1:3080`）已经很好用。这个壳只是给你一个原生桌面窗口，而不是浏览器标签页。

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

## License

MIT — see [LICENSE](LICENSE).

---

*Kongmin Rein is an independent third-party wrapper. It is not affiliated with or endorsed by DeepSeek.*
