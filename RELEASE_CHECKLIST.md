# fivem-doctor v0.1.0 リリース準備タスク

## 目的

`fivem-doctor` v0.1.0 を、GitHub上で第三者が取得・ビルド・実行できる最初の公開版としてリリースする。

現状：

- F001〜F008 実装済み
- Integration Test: **12 passed / 0 failed**
- `cargo check`: PASS
- `cargo fmt`: PASS
- Rust CLIとして基本動作確認済み

---

# Phase 1 — コア品質確認

## 1. Exit Code仕様を修正

### 目的

CLI仕様と実装を一致させる。

現在は `ERROR` が存在する場合のみexit code `1` になるため、`--severity warning` 使用時などの挙動を修正する。

### タスク

- [ ] `--severity info` の場合、INFO以上のfindingがあればexit `1`
- [ ] `--severity warning` の場合、WARNING/ERRORがあればexit `1`
- [ ] `--severity error` の場合、ERRORがあればexit `1`
- [ ] 対象findingがなければexit `0`
- [ ] CLI/configエラーはexit `2`
- [ ] analysis errorはexit `3`

### テスト

以下をintegration testに追加する。

- [ ] INFO finding → `--severity info` → exit 1
- [ ] INFO finding → `--severity warning` → exit 0
- [ ] WARNING finding → `--severity warning` → exit 1
- [ ] WARNING finding → `--severity error` → exit 0
- [ ] ERROR finding → `--severity error` → exit 1
- [ ] 問題なし → exit 0

---

# Phase 2 — Reporter品質

## 2. JSON出力を検証

### 目的

CIや外部ツールから利用できるJSON出力をv0.1.0で安定させる。

### タスク

- [ ] `--format json` の実行確認
- [ ] Diagnosticの全フィールドが正しく出力されることを確認
- [ ] severityが `ERROR` / `WARNING` / `INFO` になることを確認
- [ ] line/columnが正しく出力されることを確認
- [ ] JSONとして正常にparseできることを確認

### テスト

- [ ] F004 JSON出力テスト
- [ ] F007 JSON出力テスト
- [ ] F008 JSON出力テスト
- [ ] 問題なしのJSON出力テスト

---

# Phase 3 — CLI動作確認

## 3. CLIオプション確認

### タスク

- [ ] `fivem-doctor --help`
- [ ] `fivem-doctor --version`
- [ ] `fivem-doctor <path>`
- [ ] `--format terminal`
- [ ] `--format json`
- [ ] `--severity info`
- [ ] `--severity warning`
- [ ] `--severity error`

### 異常系

- [ ] 存在しないパス
- [ ] ファイルをresource pathとして指定
- [ ] 壊れたLuaファイル
- [ ] 壊れたfxmanifest.lua
- [ ] 不正なCLIオプション

---

# Phase 4 — ルール品質確認

## 4. F001〜F008の動作確認

### F001

- [ ] fxmanifest.luaが存在しないresource
- [ ] ERRORとして出力される
- [ ] exit codeが正しい

### F002

- [ ] client_script
- [ ] server_script
- [ ] shared_script
- [ ] file
- [ ] files
- [ ] 存在しないファイルを検出

### F003

- [ ] `lua54 'yes'` を検出
- [ ] INFOとして出力
- [ ] 不要な誤検出がない

### F004

- [ ] RegisterNetEvent + AddEventHandlerを検出
- [ ] handler argumentsを検出
- [ ] validationなしを警告
- [ ] HasPermission等で誤検出しない
- [ ] 未登録イベントを不要に警告しない

### F005

- [ ] `while` + `Wait(0)`
- [ ] `repeat` + `Wait(0)`
- [ ] numeric for + `Wait(0)`
- [ ] loop外のWait(0)を警告しない

### F006

- [ ] `print()`を検出
- [ ] INFOとして出力
- [ ] locationを出力

### F007

- [ ] AddMoney
- [ ] RemoveMoney
- [ ] SetMoney
- [ ] ExecuteCommand
- [ ] DropPlayer
- [ ] 権限チェックありの場合は誤検出しない

### F008

- [ ] TriggerServerEvent + 引数を検出
- [ ] 引数なしの場合は検出しない
- [ ] 複数引数を検出
- [ ] client.luaで検出
- [ ] locationを出力

---

# Phase 5 — 誤検知・境界条件

## 5. False Positive確認

### タスク

- [ ] コメント内の文字列で誤検出しない
- [ ] 文字列リテラル内の関数名で誤検出しない
- [ ] 関数名が似ているだけのコードで誤検出しない
- [ ] 通常のserver eventでF007を出さない
- [ ] 通常のTriggerServerEventで不要なF008を出さない
- [ ] 権限チェック済みイベントでF007を出さない

### 注意

v0.1.0では完全なデータフロー解析を実装しない。

ルールが「potentially」「obvious」などのヒューリスティック判定であることを明確にする。

---

# Phase 6 — コード品質

## 6. Rustコード整理

- [ ] `cargo fmt --check`
- [ ] `cargo check`
- [ ] `cargo test`
- [ ] `cargo clippy --all-targets --all-features -- -D warnings`
- [ ] 不要なコメント・デバッグコード削除
- [ ] 不要な依存関係確認
- [ ] compiler warningがないことを確認
- [ ] TODO/FIXMEを確認

### 目標

```text
cargo fmt --check     PASS
cargo check           PASS
cargo test            PASS
cargo clippy          PASS
```

---

# Phase 7 — README完成

## 7. README.md

第三者がREADMEだけを読んで使い始められる状態にする。

### 必須内容

- [ ] プロジェクト概要
- [ ] Features
- [ ] Installation
- [ ] Usage
- [ ] CLI options
- [ ] Rule一覧
- [ ] Diagnostic例
- [ ] Exit codes
- [ ] JSON output
- [ ] Limitations
- [ ] Roadmap
- [ ] Contributing
- [ ] License

### Usage例

```powershell
fivem-doctor ./my-resource
```

```powershell
fivem-doctor ./my-resource --format json
```

```powershell
fivem-doctor ./my-resource --severity warning
```

### READMEに明記

F004/F007/F008などは静的解析によるヒューリスティック判定であり、

> 「安全であること」または「脆弱性が存在すること」を完全に保証するものではない

ことを明記する。

---

# Phase 8 — SPEC / ドキュメント整合性

## 8. SPEC.md

- [ ] 現在の実装とSPECが一致している
- [ ] F001〜F008を記載
- [ ] CLI仕様を更新
- [ ] exit code仕様を更新
- [ ] JSON reporter仕様を確認
- [ ] v0.1で実装しない機能を明記

## 9. CHANGELOG.md

`0.1.0` セクションを追加。

### 記載内容

- Initial public release
- F001〜F008
- Terminal reporter
- JSON reporter
- Lua AST-based analysis
- FiveM resource manifest analysis

---

# Phase 9 — OSS基本ファイル

## 10. LICENSE

- [ ] MIT Licenseを配置
- [ ] copyright holder/yearを確認

## 11. CONTRIBUTING.md

- [ ] 開発環境
- [ ] build方法
- [ ] test方法
- [ ] rule追加方法
- [ ] Pull Request方針

## 12. CODE_OF_CONDUCT.md

- [ ] OSS向けCode of Conductを配置

## 13. GitHub Templates

- [ ] Bug Report
- [ ] Feature Request
- [ ] Pull Request Template

---

# Phase 10 — CI

## 14. GitHub Actions

`.github/workflows/ci.yml`

### 必須チェック

- [ ] Windows
- [ ] Ubuntu
- [ ] Rust stable
- [ ] `cargo fmt --check`
- [ ] `cargo check`
- [ ] `cargo test`
- [ ] `cargo clippy`

可能なら：

- [ ] macOS

### CI確認

- [ ] Pull RequestでCIが動く
- [ ] mainへのpushでCIが動く
- [ ] CI failureが正しく表示される

---

# Phase 11 — 実環境テスト

## 15. 実際のFiveM resourceで検証

テストfixtureだけではなく、実際のFiveM resourceを対象にする。

### 確認項目

- [ ] QBCore resource
- [ ] client/server/shared script
- [ ] 複数Luaファイル
- [ ] 複数のmanifest entry
- [ ] 実際のnetwork event
- [ ] 大きめのresource

### 確認すること

- [ ] クラッシュしない
- [ ] 解析が極端に遅くならない
- [ ] 大量の不要な警告が発生しない
- [ ] JSON出力が正常
- [ ] exit codeが期待通り

---

# Phase 12 — リリース前セキュリティ確認

## 16. セキュリティ

- [ ] 外部コマンドを不用意に実行していない
- [ ] resource内のLuaを実行しない
- [ ] ファイルを書き換えない
- [ ] `fivem-doctor`自身がresourceを変更しない
- [ ] パストラバーサルの問題がないか確認
- [ ] 巨大ファイルによる異常動作を確認
- [ ] 壊れたLua入力でpanicしない

### v0.1の基本方針

```text
READ ONLY
```

resourceを解析するだけで、解析対象を変更しない。

---

# Phase 13 — バージョン確認

## 17. Version

すべてのバージョン表記を確認する。

- [ ] Cargo.toml → `0.1.0`
- [ ] README → `0.1.0`
- [ ] CHANGELOG → `0.1.0`
- [ ] CLI `--version` → `0.1.0`
- [ ] Git tag → `v0.1.0`

---

# Phase 14 — Git最終確認

## 18. Repository clean-up

```powershell
git status
```

### 確認

- [ ] 不要ファイルがない
- [ ] `target/` がcommit対象になっていない
- [ ] デバッグファイルがない
- [ ] 個人情報が含まれていない
- [ ] ローカルパスが不要に含まれていない
- [ ] `.gitignore`が正しい

---

# Phase 15 — 最終テスト

## 19. Release前Full Test

以下を順番に実行。

```powershell
cargo fmt --check
```

```powershell
cargo check
```

```powershell
cargo test
```

```powershell
cargo clippy --all-targets --all-features -- -D warnings
```

```powershell
cargo build --release
```

```powershell
.\target\release\fivem-doctor.exe --version
```

期待：

```text
fivem-doctor 0.1.0
```

---

# Phase 16 — GitHub Release

## 20. Commit

```powershell
git status
git add .
git commit -m "Prepare v0.1.0 release"
```

## 21. Push

```powershell
git push origin main
```

## 22. Tag

```powershell
git tag -a v0.1.0 -m "Release v0.1.0"
```

## 23. Tag push

```powershell
git push origin v0.1.0
```

---

# Phase 17 — GitHub Release

## 24. Release作成

GitHubで `v0.1.0` Releaseを作成する。

### Release title

```text
v0.1.0
```

### Release内容

- Initial public release
- F001〜F008 static analysis rules
- Terminal output
- JSON output
- FiveM manifest analysis
- Lua AST analysis

### 確認

- [ ] Releaseが公開されている
- [ ] tagが`v0.1.0`
- [ ] READMEがGitHub上で正常表示
- [ ] CIがPASS
- [ ] Source codeが取得可能

---

# Phase 18 — リリース後確認

## 25. Clean Environment Test

別ディレクトリでcloneして確認。

```powershell
git clone https://github.com/r4gi-dev/fivem-doctor.git
cd fivem-doctor
cargo build --release
```

### 確認

- [ ] 新規cloneからbuildできる
- [ ] testがPASS
- [ ] CLIが起動する
- [ ] fixtureを解析できる

---

# v0.1.0 Definition of Done

以下をすべて満たしたらv0.1.0リリース可能とする。

- [ ] F001〜F008が動作
- [ ] Integration Test全PASS
- [ ] CLI exit codeが仕様通り
- [ ] JSON reporterが動作
- [ ] `cargo fmt --check` PASS
- [ ] `cargo check` PASS
- [ ] `cargo test` PASS
- [ ] `cargo clippy` PASS
- [ ] README完成
- [ ] SPEC.md整合
- [ ] CHANGELOG完成
- [ ] LICENSE完成
- [ ] CONTRIBUTING.md完成
- [ ] CODE_OF_CONDUCT.md完成
- [ ] GitHub Actions CI PASS
- [ ] 実際のFiveM resourceで動作確認
- [ ] セキュリティ確認完了
- [ ] `Cargo.toml` version = `0.1.0`
- [ ] `v0.1.0` tag作成
- [ ] GitHub Release公開
- [ ] clean cloneからbuild可能

---

# 推奨実施順

```text
1. Exit Code修正
       ↓
2. Exit Code Test追加
       ↓
3. JSON Test追加
       ↓
4. CLI異常系確認
       ↓
5. F001〜F008境界条件確認
       ↓
6. cargo clippy
       ↓
7. README完成
       ↓
8. SPEC / CHANGELOG更新
       ↓
9. OSSファイル確認
       ↓
10. GitHub Actions CI
       ↓
11. 実FiveM resourceテスト
       ↓
12. セキュリティ確認
       ↓
13. Release build
       ↓
14. Git commit / push
       ↓
15. v0.1.0 tag
       ↓
16. GitHub Release
       ↓
17. clean clone確認
       ↓
       v0.1.0 RELEASED
```

# 現在の進捗

```text
F001〜F008          ████████████████████ 100%
Integration Tests   ████████████████████ 100% (12/12)
Core Analyzer       ████████████████████ 100%
CLI                 ███████████████░░░░░  75%
Reporter            ███████████████░░░░░  75%
Documentation       ████████░░░░░░░░░░░░  40%
CI                  █████░░░░░░░░░░░░░░░  25%
Release Preparation ███████░░░░░░░░░░░░░  35%
```

## リリース前の最重要タスク

現状から最優先で進めるのは以下。

1. **Exit code修正**
2. **Exit codeテスト**
3. **JSON reporterテスト**
4. **CLI異常系テスト**
5. **F001〜F008境界条件テスト**
6. **Clippy**
7. **README完成**
8. **SPEC / CHANGELOG更新**
9. **CI完成**
10. **Release build + clean clone確認**