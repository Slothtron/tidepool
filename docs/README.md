# Tidepool 项目文档目录

本目录包含 Tidepool 项目的所有技术文档，按功能模块分类组织。

## 📚 文档索引

### 🏠 核心文档
- **[项目主文档](../README.md)** - 项目概览、快速开始和功能介绍

### 🛠️ 开发文档
- **[CLI 工具文档](cli-tools.md)** - GVM 等命令行工具的使用说明
- **[版本管理器核心库文档](tidepool-version-manager.md)** - 核心库的 API 使用和架构设计
- **[测试组织文档](test-organization.md)** - 测试开发规范和最佳实践

### 🎯 功能文档
- **[环境配置功能文档](environment-setup-feature.md)** - 环境变量配置说明功能
- **[项目重组说明](project-reorganization.md)** - 项目结构演变历史

### 🔧 开发规范
- **[统一开发规范](../.github/instructions/unified.instructions.md)** - Copilot 统一指令和开发约束

## 📖 文档说明

### 文档组织原则
- **统一位置**: 所有项目文档统一保存在此 `docs/` 目录
- **模块化**: 按功能模块分类，避免重复内容
- **引用管理**: 文档间通过相对路径相互引用
- **及时更新**: 功能变更时同步更新相关文档

### 文档分类说明

#### 核心文档
包含项目概览、快速开始指南等基础信息，是用户和开发者的入口文档。

#### 开发文档
面向开发者的技术文档，包含 API 使用、架构设计、开发规范等。

#### 功能文档
具体功能的详细说明，包含使用方法、配置选项、实现细节等。

### 文档维护
- 新功能文档直接在此目录创建
- 功能变更时及时更新相关文档
- 定期检查文档链接和内容的准确性
- 避免在子目录创建分散的 README 文件

## 🔗 快速导航

### 用户指南
- [快速开始](../README.md#快速开始) - 安装和基本使用
- [GVM 使用指南](cli-tools.md#gvm---go-版本管理器) - Go 版本管理器完整使用说明
- [环境配置说明](environment-setup-feature.md) - 环境变量配置指导

### 开发者指南
- [核心库 API](tidepool-version-manager.md#使用方法) - 版本管理器库的编程接口
- [测试开发规范](test-organization.md) - 如何编写和组织测试
- [项目架构](tidepool-version-manager.md#架构设计) - 系统设计和模块组织

### 贡献指南
- [开发规范](../.github/instructions/unified.instructions.md) - 代码质量和开发流程要求
- [测试要求](test-organization.md#测试质量要求) - 测试编写和质量标准
- [提交规范](../.github/instructions/unified.instructions.md#提交规范) - Git 提交信息格式

---

**注意**: 本项目的所有技术文档都集中在此 `docs/` 目录下，不再在各子目录维护分散的 README 文件。
