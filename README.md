# Texere - 快速草稿工具

<p align="center">
  <img src="./src-tauri/icons/128x128.png" alt="Texere Logo" width="128" height="128">
</p>

<p align="center">
  你是否也遇到过这些尴尬时刻？
- 在微信或 Slack 刚打了一长段话，手滑按了个回车就发出去了，只能尴尬撤回。
- 网页表单输入框小得可怜，稍微写多一点就得反复滚动，还没法存草稿。
- 想给内容做个简单的 AI 润色或排版，还得打开浏览器、登录 AI 网站、粘贴、复制、再切回来。

Texere 是一款为「深度输入者」设计的快速草稿工具。它平时安静地躲在菜单栏，当你需要输入一段复杂内容时，一个快捷键唤出一个置顶、透明、支持 Vim 的编辑器，让你在任何地方都能享受极致的输入体验。

</p>

<p align="center">
  <img src="https://img.shields.io/badge/Tauri-v2-FFC131?logo=tauri&logoColor=white" alt="Tauri v2">
  <img src="https://img.shields.io/badge/Svelte-5-FF3E00?logo=svelte&logoColor=white" alt="Svelte 5">
  <img src="https://img.shields.io/badge/Rust-2021-000000?logo=rust&logoColor=white" alt="Rust">
  <img src="https://img.shields.io/badge/TypeScript-5.9-3178C6?logo=typescript&logoColor=white" alt="TypeScript">
</p>

---

## ❓ 解决了什么问题？

| 场景 | 痛点 | Texere 方案 |
| :--- | :--- | :--- |
| 在 Slack/微信/钉钉输入长文本 | 输入框太小，看不清全文 | ⌥+Space 唤出大屏编辑器 |
| 在网页表单填写复杂内容 | 误触回车直接提交，内容丢失 | 独立窗口编辑，确认后再粘贴 |
| 需要格式化或润色文字 | 复制到多个工具处理，流程繁琐 | 内置 AI 一键排版 |
| 想语音输入长段内容 | 聊天软件语音不支持编辑 | 草稿窗口支持语音输入 + 编辑 |
| 习惯 Vim 操作 | 浏览器/聊天软件不支持 | 完整 Vim 键位绑定 |

## 💡 使用场景

- **场景一：长消息编辑**
  在微信或钉钉群发长通知前，先在 Texere 中仔细推敲排版，避免手滑发送。
- **场景二：复杂表单填写**
  面对网页上的多行文本框，先在 Texere 中完成草稿，确保逻辑清晰后再一键粘贴。
- **场景三：多轮对话准备**
  在进行关键客户沟通前，利用 Texere 的置顶特性参考资料并整理话术。
- **场景四：代码片段分享**
  利用内置的语法高亮和 Vim 模式，快速整理并格式化代码片段。

## 🔄 无缝粘贴工作流

1. **唤出**：按下 `⌥ + Space`，Texere 窗口立即出现在光标位置。
2. **编辑**：在 CodeMirror 驱动的编辑器中利用 Vim 键位高效输入。
3. **润色**：使用内置 AI 功能一键转换语气或排版。
4. **粘贴**：按下 `⌘ + Enter`（或设置自动粘贴），内容自动同步至原应用并隐藏窗口。

## ✨ 功能特性

- 🚀 **极速唤出** - 窗口池技术实现 <50ms 响应，比新建窗口快 10 倍
- 📝 **专业编辑器** - 基于 CodeMirror 6，支持 Vim 键位绑定
- 🤖 **AI 格式化** - 集成 OpenAI API，一键转换文本风格（正式/非正式/技术）
- 🧠 **AI 高级配置** - 支持自定义 Base URL、模型与 System Prompt
- 📋 **智能剪贴板** - 自动复制并粘贴回上一个应用，保持工作流连贯
- 📑 **模板管理** - 自定义文本模板，快速插入常用内容
- 🪟 **自定义窗口 Chrome** - 支持拖拽移动、原生风格控制区与可调整窗口大小
- 🎨 **外观主题系统** - 深色/浅色/自动模式 + Tokyo Night/Catppuccin/Nord/Rose Pine
- ⌨️ **全局快捷键** - 任意界面一键唤出/隐藏
- 🎯 **光标跟随** - 窗口自动出现在鼠标位置附近

## 📸 界面预览

*截图占位符 - 建议添加应用实际界面截图*

## 🛠️ 技术栈

| 层级 | 技术 | 用途 |
|------|------|------|
| **前端** | Svelte 5 | 响应式 UI 框架 |
| **构建** | Vite | 快速开发构建 |
| **后端** | Rust + Tauri v2 | 原生桌面应用 |
| **编辑器** | CodeMirror 6 | 文本编辑核心 |
| **API** | OpenAI | AI 文本处理 |

## 🖥️ 平台支持

| 平台 | 状态 | 说明 |
|------|------|------|
| macOS 12+ | ✅ 主力支持 | 使用 macOS 私有 API，功能最完整 |
| Windows 10/11 | ⚠️ 实验性支持 | 已接入支持代码，但尚未完整验证 |

## 🚀 快速开始

### 环境要求

- macOS 12+（推荐，功能最完整）或 Windows 10/11（实验性）
- [Bun](https://bun.sh/) 或 Node.js 18+
- [Rust](https://www.rust-lang.org/tools/install) 1.77.2+

### 安装步骤

```bash
# 克隆仓库
git clone <repository-url>
cd texere

# 安装前端依赖
bun install

# 开发模式（同时启动前端和 Tauri）
bun run tauri dev

# 基础生产构建（ad-hoc 签名）
make build

# 查看可用签名证书
make signing-identities

# 构建已签名的生产版本（避免 ad-hoc 签名导致权限失效）
make build-signed APPLE_SIGNING_IDENTITY='Apple Development: Your Name (TEAMID)'
```

## 📖 使用指南

### 快捷键

| 快捷键 | 功能 |
|--------|------|
| `⌥ + Space` | 唤出/隐藏 Texere 窗口 |
| `Esc` | 关闭窗口（不复制） |
### Vim 键位支持

Texere 内置了完整的 Vim 模式，支持以下常用操作：

| 键位 | 功能 |
| :--- | :--- |
| `i` / `a` | 进入插入模式 |
| `Esc` / `Ctrl + [` | 返回普通模式 |
| `h` `j` `k` `l` | 方向导航 |
| `w` `b` `e` | 按单词移动 |
| `dd` / `cc` | 删除/修改整行 |
| `yy` / `p` | 复制/粘贴 |
| `/` | 搜索内容 |


### AI 格式化

1. 在编辑器中输入或粘贴文本
2. 点击状态栏的 "AI" 按钮或选择格式化风格
3. 选择目标风格：正式 / 非正式 / 技术
4. 等待 AI 处理，文本自动替换

### 模板使用

1. 点击状态栏模板按钮打开模板管理器
2. 添加常用文本片段
3. 在编辑器中通过模板选择器快速插入

### 设置说明

点击状态栏设置图标可配置：

- **OpenAI API Key** - 用于 AI 格式化功能
- **OpenAI Base URL** - 支持官方端点与兼容端点（例如代理网关）
- **AI 模型** - 可自定义任意可用模型
- **System Prompt** - 定义 AI 改写风格和边界
- **Vim 模式** - 开启/关闭 Vim 键位绑定
- **自动粘贴** - 关闭窗口时是否自动粘贴到前一应用
- **外观模式与风格** - 选择深色/浅色/自动和主题风格
- **快捷键录制** - 在设置页直接录制唤出/复制关闭快捷键
- **语音输入（规划中）** - 支持通过麦克风录入并实时转文字进行编辑

## 🏗️ 项目结构

```
texere/
├── src/                          # 前端源码
│   ├── lib/
│   │   ├── components/           # Svelte 组件
│   │   │   ├── Editor.svelte     # 编辑器组件
│   │   │   ├── StatusBar.svelte  # 状态栏
│   │   │   ├── AIFormatButton.svelte
│   │   │   ├── TemplateManager.svelte
│   │   │   └── ...
│   │   ├── editor/               # CodeMirror 配置
│   │   │   └── createEditor.ts   # 编辑器初始化
│   │   ├── ai.ts                 # AI 格式化接口
│   │   ├── settings.ts           # 设置管理
│   │   ├── templates.ts          # 模板 CRUD
│   │   └── types.ts              # 类型定义
│   ├── App.svelte                # 主应用组件
│   └── main.ts                   # 前端入口
├── src-tauri/                    # Rust 后端
│   └── src/
│       ├── lib.rs                # 应用入口
│       ├── ai.rs                 # OpenAI 集成
│       ├── window.rs             # 窗口管理
│       ├── clipboard.rs          # 剪贴板操作
│       ├── tray.rs               # 菜单栏托盘
│       ├── settings.rs           # 设置持久化
│       ├── templates.rs          # 模板存储
│       └── types.rs              # 共享类型
├── package.json                  # 前端依赖
└── Cargo.toml                    # Rust 依赖
```
## 🗺️ 路线图

- [ ] **多端同步**：通过 iCloud 同步草稿内容
- [ ] **本地 AI 驱动**：支持 Ollama 等本地大模型，保护隐私
- [ ] **语音实时编辑**：深度集成 macOS 语音识别
- [ ] **多窗口管理**：支持保存多个草稿标签页


## 🧪 开发命令

```bash
# 仅启动前端开发服务器
bun run dev

# 类型检查
bun run check

# 构建前端
bun run build

# 运行 Rust 测试
cd src-tauri && cargo test

# 构建 Tauri 应用（ad-hoc）
make build

# 查看可用签名证书
make signing-identities

# 构建并校验签名（会打印 Identifier / TeamIdentifier / Signature）
make build-signed APPLE_SIGNING_IDENTITY='Apple Development: Your Name (TEAMID)'
```

## ⚙️ 配置说明

### Tauri 配置

`src-tauri/tauri.conf.json`：

- **窗口大小**：480x360 像素
- **始终置顶**：`alwaysOnTop: true`
- **透明背景**：`transparent: true`
- **无边框**：`decorations: false`
- **macOS 私有 API**：用于 macOS 上的高级窗口控制（Windows 走标准能力）

### 权限配置

`src-tauri/capabilities/default.json`：

- 全局快捷键
- 剪贴板管理
- 本地存储
- 窗口控制

## 🤝 贡献指南

欢迎提交 Issue 和 PR！请遵循以下规范：

1. **代码风格**
   - Rust：`snake_case.rs`
   - TypeScript：`camelCase.ts`
   - Svelte：`PascalCase.svelte`

2. **类型安全**
   - 新增类型需在 `src/lib/types.ts` 和 `src-tauri/src/types.rs` 中同步定义
   - 添加序列化测试确保 TS/Rust 类型兼容

3. **提交规范**
   - 使用清晰的提交信息
   - 一个 PR 只做一件事

## 📝 注意事项

⚠️ **重要**：

- macOS 为主力支持平台（依赖私有 API），Windows 为实验性支持（尚未完整验证）
- 首次运行需要授予**辅助功能**权限以实现自动粘贴
- Windows 使用前建议先在本机进行完整冒烟测试

## 📄 许可证

[MIT License](LICENSE)

## 🙏 致谢

- [Tauri](https://tauri.app/) - 强大的 Rust 桌面应用框架
- [Svelte](https://svelte.dev/) - 革命性的前端框架
- [CodeMirror](https://codemirror.net/) - 专业级文本编辑器
- [replit/codemirror-vim](https://github.com/replit/codemirror-vim) - Vim 键位支持

---

<p align="center">
  Made with ❤️ for desktop power users
</p>
