# TODO: 优化下载进度显示UI

## 任务描述

优化下载进度显示UI，使用分片和 tokio 并发下载go压缩包

## 需求详细

### 当前状态
- 下载进度显示基本但单一
- 下载过程无并发优化
- 用户体验可以改善

### 改进目标
1. **分片下载**
   - 支持将大文件分成多个片段并发下载
   - 提升下载速度，特别是在网络条件良好的情况下
   - 支持断点续传

2. **tokio 并发下载**
   - 使用 tokio 的异步并发能力
   - 多个下载任务并行执行
   - 合理控制并发数量以避免过度占用带宽

3. **UI 增强**
   - 显示实时下载速度
   - 显示分片进度
   - 显示剩余时间估算
   - 支持暂停/恢复下载

## 技术实现建议

### 1. 分片下载模块
```rust
// 建议的模块结构
pub struct ChunkedDownloader {
    url: String,
    total_size: u64,
    chunk_size: usize,
    concurrent_chunks: usize,
    output_path: PathBuf,
}

impl ChunkedDownloader {
    pub async fn download_with_progress(&self, progress_handle: &TaskProgressHandle) -> Result<()>;
    pub async fn resume_download(&self) -> Result<()>;
    pub fn pause_download(&self) -> Result<()>;
}
```

### 2. UI 改进
- 使用 `indicatif` 库增强进度条显示
- 添加多行进度显示支持
- 实时速度计算和显示

### 3. 集成点
- 在 `src/downloader.rs` 中集成分片下载
- 更新 `src/progress/` 模块以支持分片进度
- 在 `enhanced_install.rs` 中使用新的下载器

## 相关文件

- `src/downloader.rs` - 主要修改目标
- `src/progress/enhanced_install.rs` - 集成点
- `src/progress/manager.rs` - 进度管理
- `src/progress/types.rs` - 可能需要新的进度类型

## 优先级

**中等** - 这是用户体验改进，不是功能性修复

## 预估工作量

**1-2周** - 包括设计、实现、测试和文档更新

## 依赖

- `tokio` - 已有
- `reqwest` - 已有  
- `indicatif` - 可能需要添加
- `futures` - 已有

## 验收标准

1. 支持大文件的分片并发下载
2. 下载速度相比单线程有明显提升
3. UI 显示丰富的进度信息（速度、ETA、分片状态）
4. 支持下载的暂停和恢复
5. 错误处理和重试机制完善
6. 与现有进度系统无缝集成

## 备注

- 需要考虑服务器是否支持 Range 请求
- 需要合理设置并发数量以避免被服务器限制
- 考虑用户可配置的下载选项

---

*创建日期: 2025年8月14日*
*状态: 待开发*
*分配给: 待分配*
