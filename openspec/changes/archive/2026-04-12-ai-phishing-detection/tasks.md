## 1. AI Provider 基础设施（前置）

- [x] 1.1 添加 openai SDK 依赖到 package.json
- [x] 1.2 创建 src-tauri/src/commands/ai.rs：AI 配置读写命令
- [x] 1.3 实现 get_ai_config_path()：复用 accounts.rs 目录路径逻辑
- [x] 1.4 实现 load_ai_config() 和 save_ai_config() 函数
- [x] 1.5 创建 src/services/ai/ 目录结构
- [x] 1.6 实现 aiConfig.ts：前端配置加载和保存服务
- [x] 1.7 实现 aiService.ts：OpenAI SDK 初始化和调用接口
- [x] 1.8 实现超时处理和错误类型定义
- [x] 1.9 实现 AIService.isAvailable() 可用性检查

## 2. AI Provider UI 配置

- [x] 2.1 创建 AiSettings.vue 组件：baseURL、API Key、model 输入
- [x] 2.2 实现预设 Provider 下拉选项（OpenAI、百炼、DeepSeek、Moonshot、智谱、Ollama）
- [x] 2.3 实现 API Key 输入框
- [x] 2.4 实现保存配置按钮和逻辑
- [x] 2.5 实现测试连接按钮和逻辑（10秒超时）
- [x] 2.6 实现 AI 启用/禁用开关
- [x] 2.7 在 SettingsPage 中集成 AI 设置区域
- [x] 2.8 实现配置保存成功/失败通知

## 3. 数据库与数据结构扩展

- [x] 3.1 扩展 SQLite emails 表：添加 is_phishing, phishing_reasons 字段
- [x] 3.2 创建 is_phishing 字段索引，优化钓鱼邮件查询性能
- [x] 3.3 扩展 AccountConfig 结构：添加 phishing_detection_enabled 字段
- [x] 3.4 更新 accounts.rs 的 AccountConfig 结构定义
- [x] 3.5 编写数据库迁移脚本，确保存量数据兼容

## 4. 钓鱼检测服务

- [x] 4.1 创建 src/services/phishing/ 目录结构
- [x] 4.2 实现 phishingDetectionService.ts 核心检测服务
- [x] 4.3 实现 loadPhishingPrompt()：直接读取 phishing-prompt.md 文件
- [x] 4.4 实现 createDefaultPhishingPrompt()：首次使用时创建默认提示词文件
- [x] 4.5 实现 buildPhishingRequest()：构建 systemPrompt + userContent
- [x] 4.6 实现 parsePhishingResult()：解析 AI 返回的 JSON 结果
- [x] 4.7 实现超时和错误处理：5 秒超时限制，错误不阻塞同步

## 5. 默认提示词资源

- [x] 5.1 创建 src/assets/phishing-prompt-default.md 默认提示词文件
- [x] 5.2 默认文件包含完整的钓鱼检测 System Prompt
- [x] 5.3 默认文件包含 JSON 输出格式要求
- [x] 5.4 默认文件包含用户自定义扩展区域说明

## 6. 邮件同步集成

- [x] 6.1 在邮件同步流程中集成钓鱼检测调用点
- [x] 6.2 实现检测开关判断：根据账户配置决定是否调用检测
- [x] 6.3 实现检测结果存储：同步完成后写入钓鱼字段
- [x] 6.4 实现钓鱼邮件自动标记逻辑

## 7. UI 组件 - 钓鱼检测设置

- [x] 7.1 在账户设置区域添加钓鱼检测开关 Toggle
- [x] 7.2 实现开关状态保存到账户配置
- [x] 7.3 实现 AI Provider 状态检查和提示（未配置时显示警告）
- [x] 7.4 添加提示词文件管理 UI：显示路径、重置按钮
- [x] 7.5 实现提示词重置确认对话框

## 8. UI 组件 - 钓鱼文件夹

- [x] 8.1 在侧边栏添加"钓鱼邮件"分类入口
- [x] 8.2 实现钓鱼邮件列表页面
- [x] 8.3 实现检测原因列表显示
- [x] 8.4 实现钓鱼邮件详情页：显示检测原因、警告 Banner
- [x] 8.5 实现"移回收件箱"操作按钮和逻辑
- [x] 8.6 实现"确认是钓鱼"操作按钮和逻辑
- [x] 8.7 实现空钓鱼文件夹的引导提示

## 9. 集成与测试

- [ ] 9.1 编写 AI Provider 配置单元测试
- [ ] 9.2 编写 AI 调用服务单元测试
- [ ] 9.3 编写钓鱼检测服务单元测试
- [ ] 9.4 编写提示词文件加载单元测试
- [ ] 9.5 编写检测结果解析单元测试
- [ ] 9.6 测试各预设 Provider 连接（至少测试 OpenAI 和百炼）
- [ ] 9.7 测试数据库迁移不影响存量数据
- [ ] 9.8 测试跨平台配置目录路径正确性