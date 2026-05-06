## Context

現状のconsumer repositoryは、KALを使うために `AstLinterOps::run(...)` をruleごとに並べる必要がある。これは横展開可能だが、利用側にrunner実装を増やし、repositoryごとのrule driftを生みやすい。

v0.2.0は `kal.json` の設定契約を担当する。v0.3.0は、その設定を使って「基本的に全ルールを実行する」体験を提供する。

## Goals

- consumer repositoryのAST lint testを1行へ近づける。
- KAL標準ルールセットをKAL側で管理する。
- repository差分は `kal.json` に閉じ込める。
- 全ルール実行を既定値にする。
- 既存の個別rule APIを壊さず段階移行できる。

## Non-Goals

- CLIを提供すること。
- v0.2.0の `kal.json` schemaそのものを再定義すること。
- rule全体の無制限無効化を許すこと。
- consumer repository固有runnerをKALへ持ち込むこと。
- KALを汎用Rust lint frameworkへ広げること。

## Decisions

### One-Line Runner API

consumer repositoryの標準形は次のようにする。

```rust
katana_ast_lint::KatanaAstLint::from_workspace().assert_clean();
```

`from_workspace()` はworkspace rootを解決し、root直下の `kal.json` を読み込む。`kal.json` がない場合は、v0.2.0で定義されたv0.1.0互換の既定値を使う。

`assert_clean()` は全ルールを実行し、違反があればtext reporterでtestを失敗させる。JSON出力が設定されている場合は、v0.2.0のviolation contractに従う。

### Rule Catalog

KALは標準ルールセットをrule catalogとして管理する。catalogは次を持つ。

- rule id
- rule category
- default severity
- remediation guidance
- default thresholds
- required input kind
- implementation function

consumer repositoryは原則としてcatalog全体を実行する。repository固有の調整は `kal.json` で扱う。

### Configuration Boundary

`kal.json` は「どのルールを使うか」ではなく、「全ルールを実行するために必要なrepository差分」を表現する。

許可する差分はv0.2.0の範囲に合わせる。

- source roots
- domain rule input paths
- rule thresholds
- scoped allowances with reason and review condition
- reporter mode

rule全体の無効化や広いglob除外は標準仕様にしない。必要な場合は、OpenSpecで別途理由を示す。

### Migration Path

既存の `AstLinterOps::run(...)` と個別rule APIは残す。v0.3.0ではREADMEとdocsの推奨例を1行runnerへ切り替え、個別APIはadvanced useとして扱う。

## Risks

- 全ルール実行を既定にすると、既存consumer repositoryの導入時に違反が多く出る。
- rule catalogと `kal.json` の責務が曖昧だと、設定がrule無効化の抜け道になる。
- domain rule input pathsが不足すると、1行runnerが使いやすくても導入できないrepositoryが残る。
- 既存APIを急に非推奨扱いにすると、v0.1.0利用repositoryの移行負荷が大きい。
