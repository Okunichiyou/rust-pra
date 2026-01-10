# README

rustを練習するためのリポジトリ

## 環境構築

### Rustのインストール

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

プロジェクトで使用するバージョンは`rust-toolchain.toml`で管理されています。
プロジェクトディレクトリで`cargo`コマンドを実行すると、指定バージョンが自動でインストール・適用されます。

### 動作確認

```bash
rustc --version
cargo --version
```
