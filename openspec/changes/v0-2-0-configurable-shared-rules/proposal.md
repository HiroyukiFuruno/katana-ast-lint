## Why

v0.1.0の `katana-ast-lint` は、katanaシリーズ共通ルールをlibrary APIとして共有できる状態にした。一方で、対象directory、閾値、許可値、domain ruleの入力pathはまだコードやrepository runnerに残りやすい。

このまま各repositoryが個別にrunnerやruleを調整すると、共通ルールのつもりがrepositoryごとにずれる。また、KAL自身がRust汎用lint基盤を再実装する方向へ広がる危険もある。

v0.2.0では `kal.json` でrepository差分を設定化し、同時にDylint、ast-grep、Semgrepなどの外部ライブラリーを使うべき責務を評価する。

## What Changes

- repository rootの `kal.json` を設定ファイルとして定義する
- 対象directory、ruleごとの閾値、明示的な許可値、domain ruleの入力pathを設定で表現する
- 設定未指定時はv0.1.0互換の既定値で動作する
- `kal.json` を無制限除外やrule無効化の逃げ道にしない
- 重要度とJSON出力をrule設定と同じ契約で定義する
- Dylint、ast-grep、Semgrepなどの外部ライブラリー採用可否を実装前に比較する

## Capabilities

### New Capabilities

- `configurable-shared-rules`: katanaシリーズ共通ルールをrepositoryごとに安全に設定できる
- `external-lint-foundation-evaluation`: 外部ライブラリーへ委譲できる責務を判断できる

## Impact

- `kal.json` schema
- 設定読み込みAPI
- 既定値と後方互換性
- violation重要度
- JSON出力形式
- 外部ライブラリー評価結果
