## 1. CSS 样式准备

- [x] 1.1 在 globals.css 中新增 `solid-panel` 类（无 blur，保留边框和阴影）
- [x] 1.2 确保 solid-panel 的背景色与 glass-panel 视觉一致

## 2. 应用样式变更

- [x] 2.1 在 MailLayout.vue 中将 EmailList 区域的 `glass-panel` 替换为 `solid-panel`
- [x] 2.2 保留 Sidebar 和 ReadingPane 的 `glass-panel` 不变

## 3. 验证效果

- [ ] 3.1 运行应用，验证滚动流畅度改善
- [ ] 3.2 检查 EmailList 区域视觉一致性（边框、阴影）