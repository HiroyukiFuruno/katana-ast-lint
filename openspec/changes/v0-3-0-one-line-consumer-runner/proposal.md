## Why

v0.1.0のKALは、共通ルールをlibrary APIとして横展開できる状態にした。しかし利用側repositoryでは、rule名、案内文、対象directory、lint関数を個別に束ねるrunner実装が必要になる。

v0.2.0では `kal.json` によるrepository設定を進めているため、v0.3.0ではその設定を前提に、利用側の実装を1行へ近づける実行入口を整備する。

KALに入っているルールは基本的に堅牢化ルールであり、consumer repositoryは原則として全ルールを実行する。repositoryごとの差分は、実行コードではなく `kal.json` のsource roots、domain input paths、thresholds、scoped allowancesで表現する。

## What Changes

- 標準ルールセットをまとめて実行するpublic runner APIを追加する
- consumer repositoryのAST lint testを1行で書ける入口を提供する
- `kal.json` を読み込んで、全ルール実行時のsource roots、domain input paths、thresholds、scoped allowancesを適用する
- rule名、severity、remediation guidance、reporter設定をKAL側のrule catalogへ集約する
- 個別ruleを手で束ねる既存APIは、移行期間のために残す
- CLIは追加しない

## Capabilities

### New Capabilities

- `one-line-consumer-runner`: consumer repositoryが1行のtest runnerでKAL標準ルールセットを実行できる
- `shared-rule-catalog`: rule id、説明、既定threshold、案内文、既定有効状態をKAL側で管理できる

## Impact

- public runner API
- rule catalog
- `kal.json` runtime integration
- consumer repository integration sample
- README / docs integration guidance
