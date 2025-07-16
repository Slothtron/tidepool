# Tidepool OpenAPI CLI 工具

## 📦 项目简介

Tidepool OpenAPI CLI 是一个用于自动扫描、解析 OpenAPI 标准接口文档（仅支持 JSON/YAML）并输出 Markdown 功能点清单的命令行工具。适用于接口梳理、文档生成和团队协作场景。

---

## 🚀 主要功能

- 📁 递归扫描：自动查找指定目录下的 `.json`、`.yaml`、`.yml` 格式 OpenAPI 文档
- 📖 接口解析：提取所有 API 路径、方法、功能描述、请求参数和响应结构
- 📝 Markdown 输出：生成结构化的接口功能点清单，便于查阅和对接
- 🔍 多文档支持：支持批量处理和多文档合并
- 💡 友好交互：输出清晰、错误提示友好，附带解决建议

---

## 🛠️ 安装与使用

### 1. 安装

```shell
# 使用 Cargo 安装（假设已发布到 crates.io）
cargo install tidepool-openapi
```

### 2. 基本用法

```shell
# 扫描当前目录下所有 OpenAPI 文档并输出 Markdown
tidepool-openapi scan

# 指定目录扫描
tidepool-openapi scan --dir docs/apis

# 只扫描 YAML 文件
tidepool-openapi scan --ext yaml

# 预览指定接口文档的功能点
tidepool-openapi preview docs/apis/user-api.yaml
```

---

## 📋 输出示例

```markdown
## 用户管理 API

### GET /users
- 功能：获取用户列表
- 参数：
  - page (query, integer, 可选)：页码
  - size (query, integer, 可选)：每页数量
- 响应：200 OK，返回用户列表数据

### POST /users
- 功能：创建新用户
- 参数：
  - body (application/json)：用户信息
- 响应：201 Created，返回新建用户详情
```

---

## ⚠️ 常见问题与解决方案

- ❌ 未找到 OpenAPI 文档
  - 💡 请确认目录下存在 `.json`、`.yaml` 或 `.yml` 文件
- ❌ 文档格式错误
  - 💡 请检查文档是否符合 OpenAPI 2.0/3.x 标准

---

## 🧩 贡献指南

1. Fork 本仓库并新建分支
2. 提交前请确保通过所有测试和格式检查
3. 提交信息请遵循 Git 约束规范

---

## 📚 相关文档

- [OpenAPI 官方文档](https://swagger.io/specification/)
- Rust 开发约束规范
- 用户交互规范

---

友好、清晰、有用的 CLI 体验，助力高效 API 管理！
