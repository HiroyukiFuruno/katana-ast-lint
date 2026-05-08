## Why

v0.3.0では、consumer repositoryが1行のRust testでKAL標準ルールセットを実行できるようにする。一方で、repositoryによっては「依存追加後にコマンドで実行できる」形の方が導入しやすい。

特に既存の `just` やCIがlint専用targetを持っている場合、Rust testを追加するより、既存targetから `kal` コマンドを呼ぶ方が差分を小さくできる可能性がある。

v0.4.0では、v0.3.0のrunner APIを再利用する薄いCLIを追加し、API利用とCLI利用のどちらでも同じrule catalog、`kal.json`、reporter contractを使えるようにする。

## What Changes

- `kal` CLI binaryを追加する
- CLIはv0.3.0のrunner APIを呼び出す薄い入口にする
- `kal check` で標準ルールセットを実行する
- `kal.json` の読み込み、全ルール実行、scoped allowances、reporter modeはlibrary側の実装を共有する
- CLI専用のrule実装や設定解釈を作らない
- READMEとdocsにAPI利用とCLI利用の選び方を明記する

## Capabilities

### New Capabilities

- `cli-runner`: consumer repositoryが `kal check` でKAL標準ルールセットを実行できる
- `shared-runner-backend`: API runnerとCLI runnerが同じ実行基盤を共有できる

## Impact

- `[[bin]]` target
- CLI command contract
- release artifact policy
- install guidance
- CI / just integration sample
- library-only non-goalの更新
