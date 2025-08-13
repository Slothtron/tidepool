# Progress System Documentation

## 概述

Progress System 是 tidepool-gvm 的核心进度管理框架，提供了全面的任务跟踪、进度显示、验证和错误处理能力。

## 模块结构

```text
src/progress/
├── mod.rs                  # 模块入口和公共导出
├── types.rs               # 核心类型定义
├── manager.rs             # 进度管理器
├── tracker.rs             # 任务跟踪器
├── reporter.rs            # 进度报告器
├── simple_install.rs      # 简化安装系统
├── tasks.rs               # Phase 3: 任务定义系统
├── planner.rs             # Phase 3: 安装规划器
├── executor.rs            # Phase 3: 任务执行引擎
├── validator.rs           # Phase 3: 验证引擎
└── enhanced_install.rs    # Phase 3: 增强安装协调器
```

## 核心类型

### TaskId
任务的唯一标识符，用于跟踪和管理任务状态。

### InstallStep
定义安装过程的各个阶段：

- Validating - 验证阶段
- CheckingCache - 检查缓存
- CreatingDirectories - 创建目录
- Downloading - 下载文件
- Extracting - 解压缩
- Installing - 安装组件
- Configuring - 配置
- Verifying - 验证
- Finalizing - 完成
- Custom - 自定义步骤

### ProgressState
进度状态：

- NotStarted - 未开始
- InProgress - 进行中
- Completed - 已完成
- Failed - 失败
- Cancelled - 已取消

## 主要组件

### EnhancedProgressManager
主要的进度管理器，负责：

- 创建和管理多个并发任务
- 协调整体进度显示
- 提供实时进度更新

核心方法：

- start_task() - 启动新任务
- update_overall_progress() - 更新整体进度

### TaskProgressHandle
任务进度的操作句柄：

- update_progress() - 更新进度
- complete() - 完成任务
- fail() - 标记失败
- update_download_progress() - 更新下载进度
- update_extraction_progress() - 更新解压进度

## Phase 3 高级功能

### InstallTask
详细的任务定义，包含：

- 任务ID和描述
- 预计执行时间
- 前置条件
- 子任务列表
- 验证标准
- 重试策略
- 进度权重
- 验证检查
- 任务操作
- 回滚步骤

### TaskAction
支持的任务操作类型：

- FileDownload - 文件下载
- DirectoryCreate - 目录创建
- ArchiveExtract - 压缩包解压
- FileMove - 文件移动
- SymlinkCreate - 符号链接创建
- PermissionSet - 权限设置
- Verification - 验证检查
- Command - 自定义命令执行
- Cleanup - 清理操作

### InstallPlanner
智能安装规划器功能：

- 依赖分析和任务排序
- 平台检测和适配
- 时间估算
- 回滚策略规划

### TaskExecutor
任务执行引擎特性：

- 并发任务执行
- 自动错误恢复
- 详细执行日志
- 智能回滚机制

### ValidationEngine
全面的验证系统：

验证类型：

- 路径存在性检查
- 文件完整性验证
- 权限检查
- 磁盘空间验证
- Go 版本匹配验证

### EnhancedInstallationCoordinator
完整的安装协调器，执行流程：

1. **预安装验证** - 环境检查和准备
2. **安装规划** - 创建详细执行计划
3. **任务执行** - 按计划执行所有任务
4. **后安装验证** - 完整性和功能验证

## 使用示例

### 基础用法

```rust
use tidepool_gvm::progress::install_with_fallback;
use tidepool_gvm::config::Config;

let config = Config::load()?;
install_with_fallback("1.21.0", &config, false).await?;
```

### 高级用法

```rust
use tidepool_gvm::progress::EnhancedInstallationCoordinator;

let mut coordinator = EnhancedInstallationCoordinator::new();
coordinator.install_enhanced("1.21.0", &config, false).await?;
```

## 错误处理

### RetryPolicy
自动重试策略：

- 最大重试次数
- 基础延迟时间
- 最大延迟时间
- 退避倍数
- 可重试错误类型

### ErrorType
可重试的错误类型：

- NetworkError - 网络错误
- TemporaryFileSystemError - 临时文件系统错误
- ResourceTemporarilyUnavailable - 资源暂时不可用

### RollbackStep
回滚操作：

- RemoveFile - 删除文件
- RemoveDirectory - 删除目录
- RestoreFile - 从备份恢复文件
- RemoveSymlink - 删除符号链接
- CustomCleanup - 自定义清理命令

## 最佳实践

### 进度管理

- 为长时间运行的操作提供详细进度反馈
- 使用权重分配来平衡不同任务的进度贡献
- 及时更新进度状态和描述信息

### 错误处理

- 实现全面的预检查验证
- 为可恢复的错误提供重试机制
- 设计清晰的回滚策略

### 用户体验

- 提供有意义的错误消息和建议
- 显示预计完成时间
- 支持取消长时间运行的操作

### 性能优化

- 使用异步操作避免阻塞
- 合理使用并发执行任务
- 实现智能缓存策略

## 未来扩展

### 计划中的功能

- **网络优化**: 分片下载和断点续传
- **UI 增强**: 更丰富的进度可视化
- **插件系统**: 支持自定义任务类型
- **云端同步**: 跨设备安装状态同步
- **性能分析**: 安装过程性能监控

### 扩展点

- 新的 TaskAction 类型
- 自定义 ValidationCheck 规则
- 插件化的 ProgressReporter
- 可配置的重试策略

## 贡献指南

如需扩展 Progress System：

1. **添加新的任务类型**: 在 TaskAction 枚举中添加新变体
2. **扩展验证规则**: 在 ValidationCheck 中添加新的验证类型
3. **优化用户界面**: 改进 ProgressReporter 的显示效果
4. **增强错误处理**: 扩展 ErrorType 和重试逻辑

所有修改都应该：

- 保持向后兼容性
- 添加相应的测试
- 更新文档
- 遵循现有的代码风格

---

*文档版本: v1.0*
*最后更新: 2025年8月14日*
*适用版本: tidepool-gvm v0.1.5+*
