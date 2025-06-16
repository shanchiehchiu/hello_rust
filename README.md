# Hello Rust

一個簡單的 Rust 專案範例，用於學習和示範 Rust 程式語言的基本功能。

## 專案說明

這個專案是一個基本的 Rust 應用程式，展示 Rust 的基礎語法和功能。

## 系統需求

- Rust 1.70.0 或更新版本
- Cargo (Rust 的套件管理工具)

## 安裝

1. 確保已安裝 Rust 和 Cargo。如果尚未安裝，請參考 [Rust 官方安裝指南](https://www.rust-lang.org/tools/install)

2. 克隆此儲存庫：
   ```bash
   git clone https://github.com/yourusername/hello_rust.git
   cd hello_rust
   ```

## 建置與執行

### 除錯模式

```bash
cargo run
```

### 發行模式

```bash
cargo run --release
```

## 建置

```bash
cargo build
```

## 執行測試

```bash
cargo test
```

## 專案結構

```
hello_rust/
├── Cargo.toml    # 專案設定與依賴項
├── Cargo.lock    # 確切依賴版本鎖定
└── src/
    └── main.rs  # 主要程式進入點
```

## 貢獻

歡迎提交 Pull Request 或回報問題。請確保您的程式碼符合 Rust 的風格指南。

## 授權

[MIT License](LICENSE)
