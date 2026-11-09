## 1. 数据库层 - 草稿附件表

- [x] 1.1 创建 draft_attachments 表结构
- [x] 1.2 在 DbAttachment/DraftAttachment 类型中添加相关字段
- [x] 1.3 实现 save_draft_attachment/get_draft_attachments/delete_draft_attachment 数据库操作

## 2. 后端 SMTP 层 - 附件支持

- [x] 2.1 在 EmailMessage 结构体中添加 attachments 字段
- [x] 2.2 在 smtp/types.rs 中定义 AttachmentData 结构（filename, content_type, content/path）
- [x] 2.3 修改 build_message() 支持 multipart/mixed + multipart/alternative 结构
- [x] 2.4 实现附件 MIME part 构建逻辑（Base64 编码、Content-Type、Content-Disposition）
- [ ] 2.5 测试 SMTP 发送带附件的邮件

## 3. 后端 Commands 层 - 附件处理

- [x] 3.1 修改 send_email command 接收 attachments 参数
- [x] 3.2 修改 create_draft command 保存附件信息
- [x] 3.3 修改 update_draft command 更新附件信息
- [x] 3.4 修改 send_draft command 读取并发送草稿附件
- [x] 3.5 添加 read_file_for_attachment command（读取文件内容或返回路径）
- [x] 3.6 添加 validate_attachment_path command（检查文件是否存在）

## 4. 前端类型定义

- [x] 4.1 定义 ComposeAttachment TypeScript 接口（id, filename, contentType, size, path, content）
- [x] 4.2 在 compose store 中添加 attachments 数组状态
- [x] 4.3 添加 addAttachment/removeAttachment actions

## 5. 前端服务层

- [x] 5.1 修改 emailService.sendEmail() 增加 attachments 参数
- [x] 5.2 修改 emailService.createDraft() 增加 attachments 参数
- [x] 5.3 修改 emailService.updateDraft() 增加 attachments 参数
- [x] 5.4 修改 emailService.sendDraft() 处理草稿附件

## 6. 前端 UI - 附件列表组件

- [x] 6.1 创建 ComposeAttachmentList.vue 组件
- [x] 6.2 创建 ComposeAttachmentItem.vue 组件（显示文件名、大小、类型图标、删除按钮）
- [x] 6.3 实现文件类型图标映射（PDF、图片、文档、压缩包等）
- [x] 6.4 实现文件大小格式化显示

## 7. 前端 UI - 拖拽功能

- [x] 7.1 在 ComposeEmail.vue 中添加拖拽事件监听（dragover, dragenter, dragleave, drop）
- [x] 7.2 实现拖拽高亮样式（边框变色、提示文字）
- [x] 7.3 处理 drop 事件获取文件路径列表
- [x] 7.4 实现拖拽添加附件的逻辑流程

## 8. 前端 UI - 添加附件按钮

- [x] 8.1 在 ComposeEmail.vue 底部栏添加 "添加附件" 按钮
- [x] 8.2 使用 Tauri dialog API 打开文件选择对话框
- [x] 8.3 处理多文件选择
- [x] 8.4 实现按钮点击添加附件的逻辑流程

## 9. 集成与测试

- [x] 9.1 在 ComposeEmail.vue 中集成 ComposeAttachmentList 组件
- [x] 9.2 确保发送按钮正确传递附件数据
- [ ] 9.3 测试拖拽添加附件完整流程
- [ ] 9.4 测试按钮添加附件完整流程
- [ ] 9.5 测试删除附件功能
- [ ] 9.6 测试保存草稿带附件
- [ ] 9.7 测试编辑草稿恢复附件
- [ ] 9.8 测试发送草稿带附件