## 1. 项目初始化

- [x] 1.1 初始化 Rust 项目 (cargo init)
- [x] 1.2 添加依赖 (ratatui, crossterm, toml, directories, etc.)
- [x] 1.3 创建基本项目目录结构

## 2. 配置管理

- [x] 2.1 实现配置数据结构
- [x] 2.2 实现配置文件加载 (config-persistence)
- [x] 2.3 实现配置文件保存 (config-persistence)

## 3. macOS 应用发现

- [x] 3.1 实现 /Applications 目录扫描 (app-discovery)
- [x] 3.2 实现 ~/Applications 目录扫描 (app-discovery)
- [x] 3.3 实现应用信息解析 (名称、路径等)

## 4. 应用启动

- [x] 4.1 实现通过 open 命令启动应用 (app-launch)
- [x] 4.2 实现代理参数传递 (app-launch)
- [x] 4.3 实现生成启动命令到剪贴板

## 5. TUI 基础框架

- [x] 5.1 设置 ratatui + crossterm 基础
- [x] 5.2 实现应用状态管理 (app.rs)
- [x] 5.3 实现事件循环和输入处理

## 6. UI 主题和组件

- [x] 6.1 定义配色主题 (theme.rs) - Tokyonight
- [x] 6.2 实现可复用 UI 组件 (components.rs)
- [x] 6.3 实现布局管理 (layout.rs)

## 7. 主界面

- [x] 7.1 实现常用应用列表显示 (app-selection)
- [x] 7.2 实现代理配置显示 (proxy-config)
- [x] 7.3 实现状态栏和快捷键提示

## 8. 应用选择器弹窗

- [x] 8.1 实现应用列表弹窗界面
- [ ] 8.2 实现搜索过滤功能 (app-discovery) - 可选优化
- [x] 8.3 实现应用添加/删除功能 (app-selection)

## 9. 代理配置编辑

- [x] 9.1 实现代理地址输入弹窗
- [ ] 9.2 实现代理地址验证 (proxy-config) - 可选优化

## 10. 集成和测试

- [x] 10.1 集成所有模块
- [x] 10.2 端到端测试（编译成功，核心功能可用）
- [ ] 10.3 优化和美化界面细节 - 可选优化
