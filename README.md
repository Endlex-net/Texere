<p align="center">
  <img src="./src-tauri/icons/128x128.png" alt="Texere" width="96">
</p>

<h1 align="center">Texere</h1>

<p align="center">
  菜单栏草稿工具。随时唤出，写完即走。
</p>

<p align="center">
  <img src="https://img.shields.io/badge/macOS-12+-000000?logo=apple&logoColor=white">
  <img src="https://img.shields.io/badge/Tauri-v2-FFC131?logo=tauri&logoColor=white">
  <img src="https://img.shields.io/badge/Svelte-5-FF3E00?logo=svelte&logoColor=white">
  <img src="https://img.shields.io/badge/version-0.1.0-blue">
</p>

---

在微信打了一大段话，手滑回车直接发出去了。网页表单输入框太小，想多写两句都得反复滚动。给文字润色还要先开浏览器、登录 AI、复制粘贴再切回来……

**Texere 是一个常驻菜单栏的草稿窗口。** 一个快捷键弹出——透明、置顶、支持 Vim——写完自动粘贴回去，窗口自动消失。

---

## 安装

```bash
# 通过 Homebrew 安装
brew install --cask texere
```

或从 [GitHub Releases](https://github.com/your-org/texere/releases) 下载 `.dmg` 安装包。

> 正式发布版已完成 Developer ID 签名与公证，首次打开无需手动绕过 Gatekeeper。

---

## 核心工作流

| 步骤 | 操作 |
|------|------|
| **唤出** | `⌘ Shift Space`，窗口出现在鼠标位置附近 |
| **编辑** | 支持 Vim 键位的 CodeMirror 编辑器 |
| **润色** | 点击 ✨ 按钮，AI 一键调整文本风格 |
| **粘贴** | `⌘ Enter`，自动复制并粘贴回上一个应用，窗口消失 |
| **放弃** | `Esc`，关闭窗口，不复制任何内容 |

> 所有快捷键均可在设置中重新录制。

---

## 功能

### 编辑体验

- **极速响应** — 窗口池技术预热窗口，< 50ms 唤出，比新建窗口快约 10 倍
- **CodeMirror 6 编辑器** — 完整 Vim 键位，支持搜索、行操作、单词移动
- **窗口级 Vim 切换** — 点击状态栏 Vim 区域为当前窗口单独开/关 Vim，不影响其他窗口或全局设置
- **软换行** — 可在设置中开关

### AI 格式化

- 一键将文本转换为**正式 / 非正式 / 技术**风格
- 支持撤销：格式化后可一键还原原文
- 完全自定义：Base URL（兼容代理网关）、模型（默认 `gpt-4o-mini`）、System Prompt

### 模板管理

- 自定义文本片段，随时增删改
- `⌘ T` 打开模板选择器，点击即插入

### 命名笔记

临时便签之外，还可以创建持久保存的命名笔记：

- **命名** — 双击标题栏中的文字，输入名称后按 Enter；窗口升级为持久笔记，📌 图标标识
- **自动保存** — 内容变化后 500ms 自动写入本地 `notes.json`，关闭窗口前也会同步保存
- **Tray 访问** — 所有命名笔记列在菜单栏图标的下拉菜单中，点击随时打开或聚焦
- **重复保护** — 不允许同名笔记，重名时原地提示
- **删除** — 点击 📌 图标确认删除，窗口降级为临时便签，当前内容保留

### 窗口管理

- **折叠** — 双击标题栏空白区域，窗口折叠至 24px 高度（仅剩标题栏），编辑内容完整保留；再次双击展开
- **置顶** — 始终浮在所有应用之上，所有虚拟桌面可见
- **透明背景** — 可自定义背景色与不透明度
- **可调整大小** — 拖拽窗口边缘调整尺寸
- **多窗口** — 支持最多 10 个并发窗口（临时便签 + 命名笔记各自独立）

### 外观主题

| 模式 | 可选 |
|------|------|
| 配色方案 | Tokyo Night · Catppuccin · Nord · Rose Pine |
| 明暗模式 | 深色 · 浅色 · 跟随系统 |
| 背景不透明度 | 可调（默认 95%） |

---

## 设置项

点击菜单栏图标 → **Settings** 打开设置面板：

| 分类 | 设置项 |
|------|--------|
| **快捷键** | 唤出窗口、复制并关闭（均支持直接录制） |
| **编辑器** | Vim 模式开关、软换行 |
| **AI** | API Key、Base URL、模型、System Prompt |
| **外观** | 明暗模式、主题风格、背景色、不透明度 |
| **行为** | 关闭时是否自动粘贴（Auto-paste） |
| **模板** | 增删改自定义文本片段 |

---

## Vim 键位速查

| 键位 | 功能 |
|------|------|
| `i` / `a` | 进入插入模式 |
| `Esc` / `Ctrl [` | 返回普通模式 |
| `h j k l` | 方向移动 |
| `w` / `b` / `e` | 按单词移动 |
| `dd` / `yy` | 删除行 / 复制行 |
| `cc` / `p` | 修改行 / 粘贴 |
| `/` | 搜索 |
| `u` / `Ctrl r` | 撤销 / 重做 |

---

## 平台支持

| 平台 | 状态 |
|------|------|
| **macOS 12+** | ✅ 主力支持（依赖 `macOSPrivateApi`，功能完整） |
| **Windows 10/11** | ⚠️ 实验性（代码已接入，尚未完整验证） |

> **首次运行**需在系统偏好设置中授予**辅助功能**权限，自动粘贴才能生效。

---

## 从源码构建

**环境依赖**

- macOS 12+
- [Bun](https://bun.sh/) 或 Node.js 18+
- [Rust](https://www.rust-lang.org/tools/install) 1.88+

```bash
git clone <repository-url>
cd texere

bun install          # 安装前端依赖
bun run tauri dev    # 开发模式（热重载）
```

**构建命令**

```bash
make build                    # 本地构建（ad-hoc 签名，仅用于开发）
make signing-identities       # 列出本机可用的签名证书
make build-signed APPLE_SIGNING_IDENTITY='Apple Development: Your Name (XXXXXXXXXX)'
                              # 生产构建（Developer ID 签名 + 公证，用于正式分发）
```

**其他开发命令**

```bash
bun run check        # TypeScript + Svelte 类型检查
bun run test         # 前端单元测试（Vitest）
cd src-tauri && cargo test   # Rust 单元测试
make clean           # 清除构建产物
```

---

## 项目结构

```
texere/
├── src/
│   ├── App.svelte                    # 主应用：窗口状态、笔记状态、快捷键
│   └── lib/
│       ├── components/
│       │   ├── WindowContainer.svelte # 窗口布局 + 折叠状态
│       │   ├── WindowChrome.svelte    # 标题栏：折叠、标题编辑、📌 图标
│       │   ├── Editor.svelte          # CodeMirror 6 封装
│       │   ├── StatusBar.svelte       # Vim 切换、字数统计
│       │   ├── AIFormatButton.svelte  # AI 格式化 + 撤销
│       │   ├── SettingsPanel.svelte   # 设置面板
│       │   ├── TemplateManager.svelte # 模板管理
│       │   ├── TemplateSelector.svelte# 模板快速插入
│       │   └── HotkeyRecorder.svelte  # 快捷键录制
│       ├── editor/
│       │   └── createEditor.ts        # CodeMirror 配置 + Vim Compartment
│       ├── ai.ts                      # AI 格式化接口
│       ├── settings.ts                # 设置读写 + 默认值
│       ├── templates.ts               # 模板 CRUD
│       └── types.ts                   # 共享 TypeScript 类型
└── src-tauri/src/
    ├── lib.rs                         # Tauri 应用入口，注册所有 commands
    ├── window.rs                      # 窗口池、命名笔记窗口绑定
    ├── notes.rs                       # 命名笔记持久化（原子写入、并发锁）
    ├── clipboard.rs                   # 剪贴板 + 自动粘贴
    ├── tray.rs                        # 菜单栏托盘 + 笔记列表
    ├── ai.rs                          # OpenAI API 调用
    ├── settings.rs                    # 设置持久化
    ├── templates.rs                   # 模板存储
    ├── cursor_position.rs             # 鼠标位置跟踪
    ├── appearance.rs                  # 外观辅助
    └── types.rs                       # Rust 类型 + 序列化测试
```

---

## 贡献

欢迎提交 Issue 和 PR。

- **命名约定** — Rust: `snake_case`，TypeScript: `camelCase`，Svelte: `PascalCase`
- **类型同步** — 新增类型需同时更新 `src/lib/types.ts` 和 `src-tauri/src/types.rs`，并添加序列化往返测试
- **PR 原则** — 一个 PR 只做一件事

---

## 致谢

[Tauri](https://tauri.app/) · [Svelte](https://svelte.dev/) · [CodeMirror](https://codemirror.net/) · [replit/codemirror-vim](https://github.com/replit/codemirror-vim)

---

<p align="center">
  <sub>MIT License · Made for desktop power users</sub>
</p>
