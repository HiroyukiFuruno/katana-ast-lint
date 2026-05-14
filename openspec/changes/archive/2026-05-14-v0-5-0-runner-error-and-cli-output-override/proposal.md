## Why

v0.4.0で `kal` CLIを追加し、API runnerとCLI runnerが同じbackendを共有するようになった。一方で、v0.4.0実装には次の2点の積み残しがある。

1. **runnerのエラー区別が文字列依存**。`KatanaAstLint::try_assert_clean` および `try_from_workspace` は `Result<_, String>` を返す。CLIは「lint違反」か「設定/実行エラー」かを文字列の中身でなく `try_from_workspace` の成否で推定しているため、reporter内部で発生したsystem errorを `exit 1` と誤判定する余地がある。spec `Requirement: CLI runner uses stable exit codes` の `0/1/2` を厳密に区別するには、runnerが構造化されたエラー型を返す必要がある。

2. **CLIが reporter modeを一時切替できない**。現状 `kal.json` の `reporter.mode` を恒久的に書き換えるしか `text → json` を切り替える手段がない。CIの一部のジョブだけ機械可読出力が欲しいというユースケースで、`kal.json` を環境ごとに分岐させる必要が出る。spec `Requirement: CLI options do not bypass configuration safety` を守る範囲（rule無効化やglob除外は不可）で、reporter modeのみread-only overrideを許す`--json` / `--text` optionを追加する余地がある。

これらはどちらもv0.4.0スコープ内の改善ではなく、公開API変更（1）またはCLIオプション追加（2）を伴うため、v0.5.0で扱う。

## What Changes

- `KatanaAstLint::try_from_workspace` / `try_assert_clean` の戻り値を `Result<_, KalRunError>` に変更する。`KalRunError` は `Violations` / `Configuration` / `System` のvariantを持つ。
- 既存の `from_workspace` / `assert_clean` panic APIは互換のため残す。
- CLIは `KalRunError::Violations` を `exit 1`、それ以外を `exit 2` にmapする。
- `kal check --json` / `kal check --text` で `kal.json` の `reporter.mode` を一時的に上書きできるようにする（rule無効化やpath除外は引き続き不可）。
- `kal check` のhelp出力に `--json` / `--text` を追記する。
- README / docs / CHANGELOG に v0.5.0 変更点を反映する。

## Capabilities

### Modified Capabilities

- `cli-runner`: `kal check` がreporter modeのread-only overrideをCLI optionで受け付ける。
- `shared-runner-backend`: runnerが構造化エラー型を返し、CLIが exit codeを厳密に区別できる。

## Impact

- 公開API（`try_*` 系の戻り値型）
- CLI optionの追加
- exit codeの厳密化に伴うテスト
- 新エラー型の公開と documentation
