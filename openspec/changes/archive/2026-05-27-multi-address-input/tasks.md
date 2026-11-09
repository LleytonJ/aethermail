## 1. 类型定义

- [x] 1.1 在 `src/services/email/emailService.ts` 中添加 `EmailAddress` 接口定义
- [x] 1.2 添加 `formatAddressForSend` 工具函数，将 `EmailAddress` 格式化为 SMTP 字符串
- [x] 1.3 添加 `parseEmailAddresses` 解析函数的类型声明

## 2. AddressInput 组件改造

- [x] 2.1 修改 `modelValue` prop 类型从 `string[]` 改为 `EmailAddress[]`
- [x] 2.2 实现 `parseEmailAddresses` 函数，支持分隔符分割和显示名称提取
- [x] 2.3 实现 `validateEmail` 函数，验证邮箱格式
- [x] 2.4 添加 `@paste` 事件处理，解析粘贴内容
- [x] 2.5 修改 `handleKeyDown`，在输入逗号/分号时触发解析
- [x] 2.6 修改 `handleKeyDown` 的 Enter/Tab 处理逻辑，调用解析函数
- [x] 2.7 修改 `handleBlur`，调用解析函数处理失焦时的输入
- [x] 2.8 实现错误状态管理，存储无效地址列表
- [x] 2.9 实现错误显示 UI，在输入框下方显示无效地址提示
- [x] 2.10 修改地址标签显示逻辑，区分有无显示名称的格式
- [x] 2.11 更新 `addAddress` 函数，支持批量添加和验证
- [x] 2.12 更新 `selectSuggestion` 函数，适配新的数据结构

## 3. ComposeEmail 组件适配

- [x] 3.1 修改 `form.to` 和 `form.cc` 类型从 `string[]` 改为 `EmailAddress[]`
- [x] 3.2 添加草稿数据兼容转换逻辑（`string[]` → `EmailAddress[]`）
- [x] 3.3 修改 `validateForm` 函数，适配新的数据结构
- [x] 3.4 修改 `handleSend` 函数，调用 `formatAddressForSend` 格式化地址
- [x] 3.5 修改 `handleSaveDraft` 函数，转换地址格式以兼容存储

## 4. 联系人服务适配（如需要）

- [x] 4.1 检查 `Contact` 类型是否需要适配
- [x] 4.2 确保 `selectSuggestion` 返回正确的 `EmailAddress` 格式

## 5. 测试验证

- [x] 5.1 测试单地址输入（纯邮箱格式）
- [x] 5.2 测试单地址输入（带显示名称格式）
- [x] 5.3 测试多地址粘贴（逗号分隔）
- [x] 5.4 测试多地址粘贴（分号分隔）
- [x] 5.5 测试多地址粘贴（换行分隔）
- [x] 5.6 测试混合分隔符输入
- [x] 5.7 测试无效地址错误提示
- [x] 5.8 测试部分无效地址的拒绝策略
- [x] 5.9 测试地址标签显示（有无名称）
- [x] 5.10 测试地址删除功能
- [x] 5.11 测试草稿保存和加载兼容性
- [x] 5.12 测试实际邮件发送
