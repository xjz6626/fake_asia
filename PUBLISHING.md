# 发布指南 / Publishing Guide

## 📦 发布到 crates.io

### 前置准备

1. **注册 crates.io 账号**
   - 访问 https://crates.io/
   - 使用 GitHub 账号登录
   - 获取 API Token: https://crates.io/settings/tokens

2. **配置 API Token**
```bash
cargo login <your-api-token>
```

### 发布前检查

```bash
# 1. 运行所有测试
cargo test

# 2. 检查包内容
cargo package --list

# 3. 本地构建测试（模拟发布）
cargo package

# 4. 检查文档
cargo doc --open
```

### 发布到 crates.io

```bash
# 发布包
cargo publish
```

如果遇到错误，根据提示修正后重新发布。

**注意事项：**
- 一旦发布，版本号不能重复使用
- 发布前确保 README.md、LICENSE 等文件齐全
- Cargo.toml 中的 repository 链接应指向真实的 GitHub 仓库

---

## 🐙 上传到 GitHub

### 1. 初始化 Git 仓库

如果还没有初始化：

```bash
cd /home/xjz/workplace/fake_asia
git init
git add .
git commit -m "Initial commit: fake_asia v0.1.0"
```

### 2. 创建 GitHub 仓库

1. 访问 https://github.com/new
2. 创建新仓库（建议命名为 `fake_asia`）
3. **不要**初始化 README、.gitignore 或 LICENSE（我们已经有了）

### 3. 关联远程仓库并推送

```bash
# 添加远程仓库（替换为你的 GitHub 用户名）
git remote add origin https://github.com/你的用户名/fake_asia.git

# 推送到 GitHub
git branch -M main
git push -u origin main
```

### 4. 创建标签（可选但推荐）

```bash
# 创建版本标签
git tag -a v0.1.0 -m "Release v0.1.0"
git push origin v0.1.0
```

### 5. 更新 Cargo.toml

发布到 GitHub 后，更新 `Cargo.toml` 中的仓库链接：

```toml
[package]
repository = "https://github.com/你的用户名/fake_asia"
```

---

## 🔄 发布流程（推荐顺序）

### 第一次发布

1. **先发布到 GitHub**
   ```bash
   git init
   git add .
   git commit -m "Initial commit"
   git remote add origin https://github.com/你的用户名/fake_asia.git
   git push -u origin main
   ```

2. **更新 Cargo.toml 中的 repository 链接**

3. **提交更改**
   ```bash
   git add Cargo.toml
   git commit -m "Update repository link"
   git push
   ```

4. **发布到 crates.io**
   ```bash
   cargo publish
   ```

5. **创建 GitHub Release（可选）**
   - 在 GitHub 仓库页面点击 "Releases"
   - 点击 "Create a new release"
   - 标签选择 `v0.1.0`
   - 填写发布说明

### 后续版本更新

1. **修改代码**
2. **更新版本号** (Cargo.toml 中的 version)
3. **运行测试** `cargo test`
4. **提交更改**
   ```bash
   git add .
   git commit -m "Release v0.x.x: 更新说明"
   git push
   ```
5. **创建标签**
   ```bash
   git tag -a v0.x.x -m "Release v0.x.x"
   git push origin v0.x.x
   ```
6. **发布到 crates.io**
   ```bash
   cargo publish
   ```

---

## 📝 .gitignore 文件

确保项目根目录有 `.gitignore` 文件：

```gitignore
# Rust
/target/
Cargo.lock

# IDE
.vscode/
.idea/
*.swp
*.swo
*~

# OS
.DS_Store
Thumbs.db
```

---

## ✅ 发布前检查清单

- [ ] 所有测试通过 (`cargo test`)
- [ ] 代码已格式化 (`cargo fmt`)
- [ ] 没有 clippy 警告 (`cargo clippy`)
- [ ] README.md 完整（中英文）
- [ ] LICENSE 文件存在
- [ ] Cargo.toml 信息完整
  - [ ] description
  - [ ] repository (GitHub 链接)
  - [ ] license
  - [ ] keywords
  - [ ] categories
- [ ] 文档完整 (`cargo doc --open` 检查)
- [ ] 示例代码可运行 (`cargo run --example basic_usage`)

---

## 🔗 有用的链接

- **crates.io**: https://crates.io/
- **Cargo Book**: https://doc.rust-lang.org/cargo/
- **发布指南**: https://doc.rust-lang.org/cargo/reference/publishing.html
- **GitHub Docs**: https://docs.github.com/

---

## 📊 发布后推广

1. **在 Reddit 分享**
   - https://www.reddit.com/r/rust/

2. **在 This Week in Rust 提交**
   - https://this-week-in-rust.org/

3. **在社交媒体分享**
   - Twitter/X (使用 #rustlang 标签)
   - LinkedIn

4. **添加徽章到 README**
   ```markdown
   [![Crates.io](https://img.shields.io/crates/v/fake_asia.svg)](https://crates.io/crates/fake_asia)
   [![Downloads](https://img.shields.io/crates/d/fake_asia.svg)](https://crates.io/crates/fake_asia)
   [![Documentation](https://docs.rs/fake_asia/badge.svg)](https://docs.rs/fake_asia)
   ```

---

Good luck with your publication! 🚀

