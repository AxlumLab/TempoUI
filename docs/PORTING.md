# FinUI → Tempo UI 对照

按 Flutter 版 `lib/fin_ui.dart` 的 **29 个公开导出文件**逐项对照。下面每项已有可用对应，合并模块不重复计组件。迁移的是 Slint/Rust 的外观和功能，不兼容 Dart Widget、Flutter 路由或插件 API。

| FinUI 导出文件 | Tempo UI 对应实现 |
| --- | --- |
| `app_bar` | `TempoAppBar` |
| `app_bottom_tab_bar` | `TempoNav` 的底栏模式 |
| `app_button_group_tab_bar` | `TempoButtonGroupTabBar`，含图标模式和按选项宽度滑动的选中块 |
| `app_button` | `TempoButton`，含变体、禁用态和加载态 |
| `app_colors` | `TempoTheme` 的浅色、深色和强调色 |
| `app_combo_box` | `TempoComboBox`，窄屏底部选择面板、宽屏锚定下拉，支持禁用项和键盘选择 |
| `app_context` | `TempoTheme` 全局语义色，替代 Flutter 主题上下文扩展 |
| `app_dialog` | `TempoDialog`，宽窄屏居中，支持滚动内容、确认/取消、遮罩和 Esc |
| `app_empty_state` | `TempoEmptyState` |
| `app_floating_action_button` | `TempoFloatingActionButton`，常规和紧凑大小 |
| `app_icon_button` | `TempoIconButton`，含变体、加载和提示 |
| `app_icons` | `TempoIcons`，60 个别名对应 58 个官方 SVG |
| `app_page` | `TempoPage`，含滚动、边距和安全区 |
| `app_progress_bar` | `TempoProgressBar`，确定与不确定进度模式 |
| `app_refresh` | `TempoRefresh`、`TempoRefreshText`，刷新、加载、结束状态和原版五语种文案 |
| `app_section` | `TempoSection` + `TempoDivider`。Slint 子节点间显式放分隔线 |
| `app_segmented_control` | `TempoSegmentedControl` |
| `app_selection_tile` | `TempoSelectionTile`、`TempoRadioTile` |
| `app_slider` | `TempoSlider`，范围、步进、禁用、键盘与指针操作 |
| `app_surface` | `TempoSurface`、`TempoClickableSurface` |
| `app_tag` | `TempoTag`，五种变体和可选图标 |
| `app_text_field` | `TempoTextField`、`TempoSearch`，普通/密码/数字/搜索输入和编辑/提交事件 |
| `app_tile` | `TempoTile`，图标前缀、自定义尾部内容和尾部图标 |
| `app_toast` | `TempoToastHost`，底部胶囊、四种状态、超时、手动关闭和替换 |
| `app_tokens` | `TempoTokens` |
| `fui_adaptive_split` | `TempoAdaptiveSplit`，紧凑页/底部面板/弹窗、双栏和三栏 |
| `fui_navigation` | `TempoNavigationShell`、`TempoNavigationContent`、`TempoNav` + Rust `RouteStacks` |
| `fui_page_head` | `TempoPageHead` |
| `fui_theme` | `TempoTheme` + 宿主字体回退，支持宿主明确设置或读取 Slint 提供的配色 |

## 五轮交付记录

1. 补下拉框、对话框、Toast、浮动按钮、刷新和独立按钮组标签栏。
2. 补页面栏、滚动页、页面标题、自适应分栏和导航历史。完善通用输入、进度/步进、单选、主题和图标。
3. 用临时 Flutter 3.44.0 环境跑原库 15 个测试，另加同尺寸截图夹具。对照修复了初始图标标签选中块、Toast 位置、窄屏详情遮盖底栏和桌面侧栏视觉差异。
4. 用 Slint 的原生指针和键盘事件验证了标签点击、输入提交、禁用下拉项、弹窗 Esc、受控刷新、分页结束和模拟安全区。验证窄屏、普通桌面、超宽分栏及两个渲染器。截图不是像素级金标。
5. 整理公共导出、可重复检查脚本、截图和使用说明。发布构建内嵌 SVG 等资源，不依赖原 Flutter 工作区。

## 保留的实现差异和平台验收边界

- Flutter 的 Widget 插槽、路由转场、Hero 和 GetX root overlay 不直接移植。Slint 由宿主提供导航内容和返回事件，Toast 使用每窗口的 Host。当前不实现 Flutter 的 Android predictive-back 转场和 Cupertino 交互式返回动画。
- 安全区接的是 Slint Window 输入，并测了合成 inset。系统状态栏样式、触屏惯性与输入法、系统自动填充、TalkBack/VoiceOver 和原生返回手势仍需实际移动端宿主验收。Windows、Android 和 iOS 没有本轮真机验证。
- 平台强调色和字体以宿主能提供的信息为准。当前支持手动语义色、Slint 平台配色入口及已安装字体回退。不同系统和渲染器的字形、行高、阴影和动效不承诺像素相同。
- Flutter 的 `EasyRefresh` 自定义物理、控制器插件和横向滚动模式不作为 Slint API。当前纵向刷新提供下拉、分页、结果与可定制阈值/文案。
- 音乐库横幅、曲目行和迷你播放器仍是 FMusic 演示，不是原 FinUI 控件或正式客户端。长列表、真实音频和自建服务端没有加入这次 UI 移植。

## 组件截图

- [桌面组件展板](images/catalog-desktop.png)
- [窄屏深色展板](images/catalog-compact.png)
- [窄屏列表和排序](images/flutter-replacements-compact.png)

Flutter 原始对照和截图夹具位于移植时使用的 FMusic 工作区。
