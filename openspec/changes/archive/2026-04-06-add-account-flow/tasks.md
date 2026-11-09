## 1. 基础设施准备

- [x] 1.1 创建 `src/components/accounts/` 目录结构
- [x] 1.2 创建 `src/services/email/providerPresets.ts` 服务商预设配置
- [x] 1.3 扩展 `src/services/mock/accountService.ts` Mock 服务

## 2. UI 基础组件

- [x] 2.1 实现 `src/components/ui/Modal.vue` 基础弹窗组件
- [x] 2.2 实现 Modal 打开/关闭动画
- [x] 2.3 实现 Modal 点击外部关闭和 ESC 关闭

## 3. AccountSwitcher 组件

- [x] 3.1 实现 `AccountSwitcher.vue` 基础结构（头像 + 名称 + 邮箱）
- [x] 3.2 实现账户下拉菜单（显示账户列表）
- [x] 3.3 实现当前账户高亮和切换功能
- [x] 3.4 实现 "Add account" 按钮和事件触发
- [x] 3.5 实现无账户时的空状态 UI
- [x] 3.6 集成 AccountSwitcher 到 `Sidebar.vue`

## 4. AddAccount 入口 Modal

- [x] 4.1 实现 `AddAccount.vue` Modal 入口组件
- [x] 4.2 实现服务商选择 UI
  - POP3/SMTP 选项（可点击）
  - Gmail OAuth2 选项（显示 coming soon）
- [x] 4.3 实现选择后进入 `AddPop3Account` 流程
- [x] 4.4 在 `MailLayout.vue` 添加 `showAddAccount` 状态管理

## 5. AddPop3Account 多步骤表单 - 基础

- [x] 5.1 实现步骤指示器 UI（Account → Incoming → Test → Done）
- [x] 5.2 实现步骤导航逻辑（上一步/下一步/取消）
- [x] 5.3 实现表单数据状态管理（reactive form state）
- [x] 5.4 实现键盘快捷键（Enter 下一步，Esc 取消）

## 6. AddPop3Account - Step 1: 基本信息

- [x] 6.1 实现邮箱地址输入框
- [x] 6.2 实现显示名称输入框（可选）
- [x] 6.3 实现密码输入框（密码遮罩）
- [x] 6.4 实现邮箱格式验证
- [x] 6.5 实现邮箱域名自动匹配预设功能

## 7. AddPop3Account - Step 2: POP3 服务器配置

- [x] 7.1 实现服务器地址输入框
- [x] 7.2 实现端口输入框
- [x] 7.3 实现 SSL/TLS 开关
- [x] 7.4 实现手动修改配置功能
- [x] 7.5 实现 SSL 开关联动端口（SSL:995, 非SSL:110）

## 8. AddPop3Account - Step 3: 测试连接

- [x] 8.1 实现 Mock 测试连接服务（模拟延迟 ~1.5s）
- [x] 8.2 实现测试按钮和加载状态
- [x] 8.3 实现成功结果显示（绿色勾 + 服务器信息）
- [x] 8.4 实现失败结果显示（红色叉 + 错误信息）
- [x] 8.5 实现密码包含 "wrong" 时模拟失败
- [x] 8.6 实现重新测试按钮

## 9. AddPop3Account - Step 4: 保存完成

- [x] 9.1 实现保存成功提示 UI
- [x] 9.2 实现 "Add Account" 按钮（仅测试成功后可用）
- [x] 9.3 实现保存到 accountStore
- [x] 9.4 实现保存到 localStorage
- [x] 9.5 实现 "Go to Inbox" 和 "Add Another" 按钮

## 10. accountStore 更新

- [x] 10.1 扩展 `Account` 接口（添加 server 配置字段）
- [x] 10.2 实现 `addAccount` action
- [x] 10.3 实现 `switchAccount` action
- [x] 10.4 实现 localStorage 持久化逻辑
- [x] 10.5 实现初始化时从 localStorage 加载账户

## 11. 集成与测试

- [x] 11.1 测试：打开添加账号 Modal（多种入口）
- [x] 11.2 测试：使用预设添加 Gmail 账号
- [x] 11.3 测试：使用预设添加 163 账号
- [x] 11.4 测试：手动配置自定义服务器
- [x] 11.5 测试：连接测试失败场景
- [x] 11.6 测试：账户切换功能
- [x] 11.7 测试：重启应用后账户恢复（localStorage）
- [x] 11.8 测试：表单验证（邮箱格式、必填字段）