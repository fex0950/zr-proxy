## Context

构建一个 macOS TUI 工具，用于管理通过代理启动的应用程序。项目使用 Rust 语言，ratatui 作为 TUI 库。

## Goals / Non-Goals

**Goals:**
- 提供简洁美观的 TUI 界面
- 支持快速发现和选择 macOS 应用
- 支持代理配置的持久化
- 提供多种启动方式（直接启动、生成命令等）

**Non-Goals:**
- 不支持 Windows/Linux（当前版本）
- 不实现代理服务器本身
- 不实现应用内代理设置修改

## Decisions

### 1. TUI 库选择: ratatui + crossterm
- **选择**: ratatui（基于 tui-rs 的活跃分支）+ crossterm 终端后端
- **原因**: ratatui 是 Rust 生态最成熟的 TUI 库，有丰富的组件和活跃的社区
- **备选**: iced（GUI，不符合需求）、termion（不如 crossterm 跨平台好）

### 2. 界面设计风格
- **选择**: 现代简约风格，使用渐变色和圆角边框
- **配色方案**: Tokyonight
  - 背景色: 深蓝黑 (#1a1b26)
  - 次级背景: (#16161e)
  - 边框色: (#3b4261)
  - 主色调: 蓝色 (#7aa2f7)、紫色 (#bb9af7)
  - 强调色: 亮粉 (#ff79c6)、橙色 (#ff9e64)
  - 文字色: 亮白 (#c0caf5)、注释色 (#565f89)
- **布局**: 两栏式（主列表 + 详情面板），带标题栏和快捷键状态栏

### 3. 应用发现方式
- **选择**: 结合系统 API 和目录扫描
- 优先使用 LaunchServices API 获取系统注册应用
- 补充扫描 `/Applications` 和 `~/Applications` 目录
- 支持用户手动添加自定义路径

### 4. 配置存储
- **选择**: `~/Library/Application Support/zr-proxy/config.toml`
- 格式: TOML（易读易编辑）
- 内容: 代理地址、已选应用列表、UI 偏好设置

### 5. 项目结构
```
zr-proxy/
├── src/
│   ├── main.rs           # 入口点
│   ├── app.rs            # 应用状态和核心逻辑
│   ├── ui/
│   │   ├── mod.rs        # UI 模块入口
│   │   ├── theme.rs      # 配色和主题
│   │   ├── components.rs # 可复用组件
│   │   └── layout.rs     # 布局管理
│   ├── macos/
│   │   ├── mod.rs        # macOS 模块入口
│   │   ├── apps.rs       # 应用发现
│   │   └── launch.rs     # 应用启动
│   └── config.rs         # 配置管理
├── Cargo.toml
└── README.md
```

## Risks / Trade-offs

| 风险 | 缓解措施 |
|------|----------|
| macOS API 绑定复杂 | 使用 `objc` crate 或调用系统命令作为备选方案 |
| ratatui 学习曲线 | 从简单布局开始，逐步添加高级特性 |
| 性能问题（大量应用） | 实现虚拟滚动和搜索过滤 |
