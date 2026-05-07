## Context

KALはv0.1.0からlibrary-onlyを前提にしてきた。これはconsumer repositoryのtest/CIへ組み込みやすく、ルール実装を共有するには十分だった。

ただし、利用側の最小差分という観点では、CLIにも価値がある。既存の `just ast-lint` やCI lint stepがあるrepositoryでは、Rust testを新設するより `kal check` を呼ぶ方が自然な場合がある。

v0.4.0ではlibrary APIを捨てず、CLIを薄い実行入口として追加する。

## Goals

- `kal check` で標準ルールセットを実行できる。
- API runnerとCLI runnerで同じrule catalogと `kal.json` 解釈を使う。
- CLI追加でrule実装や設定解釈を二重化しない。
- consumer repositoryがAPI方式とCLI方式を選べる。
- 既存の `just` / CI変更を小さくできる導入例を提供する。

## Non-Goals

- CLIを主要なrule実装場所にすること。
- API runnerを廃止すること。
- `kal.json` schemaをv0.4.0で再定義すること。
- rule単位の自由な無効化CLI optionを提供すること。
- 汎用Rust lint CLIへ拡張すること。

## Decisions

### Command Shape

v0.4.0のCLIは最小のcommand setから始める。

```bash
kal check
```

`kal check` はカレントディレクトリからworkspace rootを解決し、root直下の `kal.json` を読み込む。`kal.json` がない場合は、v0.2.0で定義された既定値を使う。

### Shared Backend

CLIはv0.3.0のrunner APIを呼ぶだけにする。CLI専用のrule catalog、path解決、allowance処理、reporter処理は作らない。

CLIの責務は次に限定する。

- 引数を解釈する
- workspace rootをlibrary runnerへ渡す
- exit codeを返す
- stdout / stderrへreporter結果を出す

### Exit Codes

exit codeは次の範囲に限定する。

- `0`: 違反なし
- `1`: lint違反あり
- `2`: 設定エラー、入力エラー、実行不能

### Installation

consumer repositoryは必要に応じて次のどちらかを選ぶ。

```bash
cargo install katana-ast-lint
kal check
```

または、repository-local toolとして実行する。

```bash
cargo run --package katana-ast-lint --bin kal -- check
```

最終的な推奨形は、release artifactとconsumer repositoryの運用に合わせてtasks内で確定する。

### Relationship with API Runner

API方式は、既存の `cargo test` に自然に乗せたいrepository向けに残す。

CLI方式は、既存の `just ast-lint` やCI stepからlint commandを呼びたいrepository向けに提供する。

どちらを使っても、同じ `kal.json` とrule catalogで同じ結果になる必要がある。

## Risks

- library-only方針を変えるため、README、OpenSpec、release policyの更新漏れが起きやすい。
- CLI optionが増えすぎると、`kal.json` を迂回する別設定面になる。
- API runnerとCLI runnerで結果がずれると、品質ゲートとして信用できなくなる。
- `cargo install` 前提に寄せすぎると、consumer repositoryのCIでversion固定が難しくなる。
