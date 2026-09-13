# fivem-doctor

**Static analysis and quality checks for FiveM resources.**

`fivem-doctor` は、FiveM リソースを静的解析し、サーバーで実行する前に問題になりやすい箇所を検出する Rust 製の CLI ツールです。

`fxmanifest.lua` の参照ファイル、Lua コード、ネットワークイベント、ループ処理などを解析し、問題を診断結果として報告します。

> **Status:** Early development — v0.1.0

---

## Features

- `fxmanifest.lua` の存在チェック
- マニフェストから参照されているファイルの存在チェック
- 非推奨となった `lua54 'yes'` の検出
- Lua コードの静的解析
- `Wait(0)` を使用した連続ループの検出
- デバッグ用 `print(...)` の検出
- 潜在的に危険なネットワークイベントの検出
- 権限確認のない可能性がある特権操作の検出
- クライアントからサーバーへ値を渡すネットワークイベントの検出
- Terminal / JSON 出力
- Severity による診断結果のフィルタリング
- CI/CD で利用できる終了コード

---

## Requirements

- Rust stable
- Cargo

Rust のインストール方法については、公式の Rust ドキュメントを参照してください。

---

## Installation

現在の v0.1.0 では、ソースコードからビルドして使用できます。

```bash
git clone https://github.com/r4gi-dev/fivem-doctor.git
cd fivem-doctor
cargo build --release
```

ビルドされたバイナリは以下に生成されます。

```text
target/release/fivem-doctor
```

---

## Usage

FiveM リソースを指定して解析します。

```bash
fivem-doctor ./my-resource
```

例えば、以下のようなリソースを解析できます。

```text
my-resource/
├── fxmanifest.lua
├── client.lua
├── server.lua
└── config.lua
```

### JSON output

機械的に結果を処理したい場合は `--format json` を使用します。

```bash
fivem-doctor ./my-resource --format json
```

JSON は CI/CD や他のツールとの連携を想定しています。

### Severity filter

表示する最低 Severity を指定できます。

```bash
fivem-doctor ./my-resource --severity info
```

```bash
fivem-doctor ./my-resource --severity warning
```

```bash
fivem-doctor ./my-resource --severity error
```

`--severity warning` の場合、`WARNING` と `ERROR` の診断結果が対象になります。

`--severity error` の場合、`ERROR` の診断結果のみが対象になります。

---

## Example

例えば、以下のようなコードがあるとします。

```lua
RegisterNetEvent('bank:withdraw')
AddEventHandler('bank:withdraw', function(amount)
    TriggerEvent('bank:removeMoney', amount)
end)
```

`fivem-doctor` は、ネットワークイベントから受け取った値が適切に検証されていない可能性を診断できます。

また、以下のようなコードも検出対象です。

```lua
CreateThread(function()
    while true do
        Wait(0)

        -- continuous work
    end
end)
```

この場合、フレームごとに処理を実行するループとして警告されます。

---

## Rules

v0.1.0 では以下のルールを実装しています。

| Rule | Severity | Description |
|---|---|---|
| F001 | ERROR | `fxmanifest.lua` が存在しない |
| F002 | ERROR | `fxmanifest.lua` から参照されているファイルが存在しない |
| F003 | INFO | `lua54 'yes'` が指定されている |
| F004 | WARNING | クライアントから呼び出されるネットワークイベントで、受け取った値の検証が不明確 |
| F005 | WARNING | ループ内で `Wait(0)` が使用されている |
| F006 | INFO | `print(...)` などのデバッグ出力が存在する |
| F007 | WARNING | ネットワークイベント内で権限確認が不明確な特権操作が行われている |
| F008 | WARNING | クライアントからサーバーイベントへ値が渡されている |

### F001 — Missing manifest

FiveM リソースに `fxmanifest.lua` が存在しない場合に検出します。

### F002 — Missing referenced file

`fxmanifest.lua` に記述されている `client_script`、`server_script`、`shared_script`、`file` などの参照先が存在しない場合に検出します。

### F003 — Deprecated lua54

`lua54 'yes'` が明示的に指定されている場合に情報として報告します。

FiveM の現在の Lua 環境では Lua 5.4 が標準となっているため、不要になった設定を確認するためのルールです。

### F004 — Unsafe network event

`RegisterNetEvent` などによってクライアントから呼び出される可能性のあるイベントについて、入力値に対する明確な検証が確認できない場合に警告します。

### F005 — Frame loop

ループ内で `Wait(0)` が使用されている場合に警告します。

フレームごとの処理が必要なケースもあるため、必ずしも問題を意味するものではありません。

### F006 — Debug output

`print(...)` などのデバッグ出力を検出します。

開発中には便利ですが、本番環境に不要なログを残していないか確認するためのルールです。

### F007 — Privileged network event

ネットワークイベントから、以下のような特権操作が行われている可能性があり、明確な権限確認が確認できない場合に警告します。

- Money 操作
- `ExecuteCommand`
- `DropPlayer`
- その他の特権的な処理

### F008 — Client-controlled server event

クライアント側から引数付きで `TriggerServerEvent` が呼び出されている場合に検出します。

これは脆弱性そのものを意味するものではありませんが、サーバー側で入力値を適切に検証する必要がある箇所を見つけるために使用します。

---

## Important: Static Analysis Limitations

`fivem-doctor` は **静的解析ツール**です。

特に F004、F007、F008 はコードの構造から問題の可能性を推測するヒューリスティックなチェックです。

そのため、

- 警告がある = 必ず脆弱性がある
- 警告がない = 完全に安全

という意味ではありません。

例えば、独自の権限システムや複雑なバリデーションを使用している場合、`fivem-doctor` がその処理を認識できず警告する可能性があります。

最終的な安全性の判断は、開発者によるコードレビューと実際の動作確認が必要です。

---

## Exit Codes

`fivem-doctor` は CI/CD で利用できるように終了コードを提供します。

| Exit code | Meaning |
|---:|---|
| `0` | 対象 Severity 以上の診断結果なし |
| `1` | 対象 Severity 以上の診断結果あり |
| `2` | CLI または設定エラー |
| `3` | リソースの解析エラー |

例えば、問題が見つかった場合は終了コード `1` になります。

```bash
fivem-doctor ./my-resource
echo $?
```

これにより、GitHub Actions などの CI/CD パイプラインで解析結果に応じて処理を分岐できます。

---

## Output

### Terminal

デフォルトでは Terminal に診断結果を表示します。

```bash
fivem-doctor ./my-resource
```

診断には以下の情報が含まれます。

- Rule ID
- Severity
- File
- Line
- Column
- Message
- Explanation
- Suggestion

### JSON

```bash
fivem-doctor ./my-resource --format json
```

JSON 出力はスクリプトや CI/CD から扱いやすい形式になっています。

---

## Development

開発環境を取得します。

```bash
git clone https://github.com/r4gi-dev/fivem-doctor.git
cd fivem-doctor
```

### Format

```bash
cargo fmt
```

### Check

```bash
cargo check
```

### Clippy

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

### Test

```bash
cargo test
```

Pull Request を作成する前に、これらのチェックがすべて通ることを推奨します。

---

## Project Structure

```text
fivem-doctor/
├── Cargo.toml
├── Cargo.lock
├── README.md
├── SPEC.md
├── LICENSE
├── CONTRIBUTING.md
├── CODE_OF_CONDUCT.md
├── CHANGELOG.md
├── .gitignore
├── .github/
│   ├── workflows/
│   │   ├── ci.yml
│   │   └── release.yml
│   ├── ISSUE_TEMPLATE/
│   │   ├── bug_report.yml
│   │   └── feature_request.yml
│   └── pull_request_template.md
├── src/
│   ├── main.rs
│   ├── diagnostic.rs
│   ├── cli/
│   ├── analyzer/
│   ├── rules/
│   └── report/
└── tests/
    ├── fixtures/
    └── integration.rs
```

---

## Roadmap

### v0.1.0

Core static analysis.

- Resource analysis
- `fxmanifest.lua` analysis
- Lua AST analysis
- F001–F008
- Terminal output
- JSON output
- Severity filtering
- Integration tests

### v0.2

Usability improvements.

- Configuration file
- Improved Lua analysis
- Ignore / suppression support
- More accurate diagnostics

### v0.3

Extended FiveM analysis.

- More security rules
- Event analysis improvements
- Dependency analysis
- Framework-aware analysis
- QBCore-related checks

### v0.4

CI/CD integration.

- GitHub Actions improvements
- SARIF output
- Better CI integration

### v0.5

Framework support.

- ESX-related checks
- ox-related checks
- Additional framework-aware rules

### v1.0

Stable release.

- Stable diagnostic behavior
- Improved rule accuracy
- Documentation
- CI/CD support
- Production-ready CLI

The roadmap is subject to change as the project develops.

---

## Contributing

Bug reports, feature requests, improvements, and Pull Requests are welcome.

Before contributing, please read:

- `CONTRIBUTING.md`
- `CODE_OF_CONDUCT.md`
- `SPEC.md`

When proposing a new rule, please consider:

1. What problem does the rule detect?
2. What are the expected false positives?
3. What are the expected false negatives?
4. Can the rule be implemented using static analysis?
5. Does the rule fit the scope of `fivem-doctor`?

The project aims to keep diagnostics predictable and useful rather than attempting to detect every possible problem.

---

## License

`fivem-doctor` is licensed under the MIT License.

See [`LICENSE`](LICENSE) for details.

---

## Disclaimer

`fivem-doctor` is a development and static analysis tool.

It does not guarantee that a FiveM resource is secure, performant, or free of bugs.

Always review important code manually and test resources in an appropriate development environment before deploying them to a production server.

---

## Links

- Repository: https://github.com/r4gi-dev/fivem-doctor
- Issue Tracker: https://github.com/r4gi-dev/fivem-doctor/issues

---

**Built for FiveM developers who want to find problems before they reach the server.**