# Flutter 平替复核

范围是 FinUI 的 Flutter 基元，以及 FMusic 曲库、歌单、队列页直接依赖的列表和菜单。这里区分现成控件、需要在 Tempo UI 保留的产品行为，以及平台宿主职责。

| Flutter / FinUI 的用法 | Slint 的可用基元 | Tempo UI 的选择 |
| --- | --- | --- |
| Row / Column / Stack / Align / Padding / LayoutBuilder | HorizontalLayout、VerticalLayout、Rectangle、宽高绑定 | 直接使用。响应式栏宽和安全区集中在 `TempoNavigationShell`、`TempoAdaptiveSplit` |
| Wrap | FlexboxLayout | 直接用它。对话框动作栏已使用自动换行，不在应用中重复计算宽度 |
| ThemeData / ThemeExtension / MediaQuery | Palette、Window 的宽高与 safe-area-insets | `TempoTheme`、`TempoInsets`。动态系统强调色取决于后端 |
| ListView.builder / SliverList.builder | ListView 的直接 model repeater | `TempoVirtualList`，直接重导出原生 ListView，保留编译器的行虚拟化。VirtualList 展板实际挂了 5,000 行 |
| CustomScrollView / SliverPersistentHeader | ScrollView、ListView，没有一套等价的 Flutter sliver 协议 | 当前建议展板结构，即固定页面栏 + 一个虚拟列表，分组标题可作为数据行；未封装可折叠/吸顶 sliver 协议 |
| ReorderableListView.builder | 交互基元与 ListView，没有现成可排序列表 | `TempoReorderList`，固定行高、拖拽把手、边缘滚动、上/下移按钮。应用负责更新模型、权限和持久化 |
| PopupMenuButton | ContextMenuArea、PopupWindow；前者适合右键/长按上下文菜单 | `TempoMenuButton` 保留按钮锚定、禁用项和键盘操作。上下文菜单直接用原生 ContextMenuArea |
| IndexedStack | 常驻节点 + visible，没有专用 IndexedStack | `TempoIndexedStack`、`TempoStackPage`。调用者绑定索引，不用条件实例化反复卸载当前页 |
| Navigator / MaterialPageRoute / showModalBottomSheet | PopupWindow、FocusScope、Window，不提供完整路径栈 | `RouteStacks` + `TempoRouteMotion`、`TempoDialog`、`TempoSheet`。交互式系统返回仍需要宿主适配 |
| Dialog content / arbitrary actions、TextField suffix | @children 单插槽；可用模型描述动作 | Dialog 支持任意内容 + 自定义动作模型，动作栏换行。输入框后缀是原生插槽。不能传递任意 Flutter Widget |
| ListTile leading / trailing | Slint 子项、图标、布局 | `TempoTile` 已有图标前缀和任意尾部插槽。非图标前缀用自定义行，暂无通用双命名插槽 |
| RefreshIndicator / EasyRefresh / OverlayEntry | SwipeGestureHandler、PopupWindow、Timer | `TempoRefresh`、`TempoToastHost`。不替换整个 Flutter 滚动物理或 GetX 约定 |
| TextField / FocusNode / Tooltip / Material 按键反馈 | TextInput、FocusScope、Tooltip、触摸和键盘事件 | Tempo 控件封装外观、编辑/提交、焦点、禁用与动效。自动填充和移动端软键盘动作标签走平台适配，不冒充 autofill |
| Checkbox / Radio / Slider / ProgressIndicator / IconButton | CheckBox、RadioGroup、Slider、ProgressIndicator、Button、Spinner | 通用原生控件直接可用。FinUI 外观继续使用 Tempo 封装 |

## 本次动效

- `TempoPresence` 在关闭时保留视觉节点直到退出完成，避免直接靠 `if visible` 切掉动画。Toast 沿用 FinUI 的 180 ms 入场、120 ms 退场和 26 px 垂直偏移。
- Dialog 在窗口中居中。Sheet 从底部进入。ComboBox 在窄屏使用底部面板，宽屏在控件旁展开，下面空间不足时优先向上展开。按钮菜单和 ComboBox 都有入场动画。
- `TempoRouteMotion` 提供程序级详情推进/回退动效。窄屏页从侧面过渡，底部详情从下方进入；它不是 Flutter 的手势驱动转场。
- 保留并补齐标签指示条、分段选择、复选状态、焦点、刷新指示条和图标按钮反馈。可通过 `TempoMotionSettings.reduced` 关闭这些动效。Slint 也会在支持的后端自动遵循系统减少动态效果设置。
- 运行 `cargo run --release --locked -p tempo-ui-showcase -- --motion-smoke` 检查真实中间帧、退出后释放弹层和减少动态效果，不以静态截图作为动效验收。

不属于本次 UI 封装范围的还有真实音频、文件选择实现、系统凭据，以及 Windows/Android/iOS 原生宿主验收。
