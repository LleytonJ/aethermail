# IMAP Connection Capability

## Summary

建立 IMAP 服务器连接，进行用户认证，为后续邮件操作提供会话。

## Interface

### 输入

```typescript
interface ImapConfig {
  host: string;       // IMAP 服务器地址
  port: number;       // IMAP 端口（通常 993 for SSL）
  use_ssl: boolean;   // 是否使用 SSL/TLS
}

interface ImapCredentials {
  username: string;   // 用户名（通常为邮箱地址）
  password: string;   // 密码
}
```

### 输出

```typescript
interface ConnectionTestResult {
  success: boolean;
  message: string;
  server_greeting?: string;  // 服务器欢迎消息
}
```

### 命令

| 命令 | 描述 |
|-----|------|
| `imap_test_connection(config, credentials)` | 测试 IMAP 连接，返回测试结果 |

## Behavior

### 连接流程

```
1. 建立 TCP 连接到 IMAP 服务器
2. 如果 use_ssl=true，升级为 TLS 连接
3. 读取服务器欢迎消息（未标记响应）
4. 发送 LOGIN 命令进行认证
5. 认证成功，返回会话对象
```

### 错误处理

| 错误场景 | 处理方式 |
|---------|---------|
| TCP 连接超时 | 返回 "连接超时" 错误 |
| TLS 握手失败 | 返回 "TLS 配置失败" 错误 |
| 认证失败 | 返回 "认证失败" 错误，包含服务器返回的错误消息 |
| 服务器拒绝 | 返回服务器返回的错误消息 |

## Implementation Notes

- 使用 `async-imap` crate 实现
- 异步操作，使用 tokio runtime
- 连接超时设置为 30 秒
- 认证超时设置为 60 秒

## Security

- SSL/TLS 连接必须支持
- 密码通过 TLS 通道传输，不暴露
- 不支持明文传输（use_ssl=false 仅用于开发测试）

## Dependencies

- async-imap >= 0.9
- tokio-native-tls
- tokio (异步 runtime)