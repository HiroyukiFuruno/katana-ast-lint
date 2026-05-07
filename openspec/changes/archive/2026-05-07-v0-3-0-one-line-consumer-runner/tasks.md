# Tasks: v0-3-0-one-line-consumer-runner

## 1. API Contract

### Definition of Ready

- [x] v0.2.0の `kal.json` schemaと既定値方針が確定している
- [x] v0.2.0のviolation severityとreporter modeが確定している

### Tasks

- [x] 1.1 1行runnerのpublic API名を確定する
- [x] 1.2 workspace root解決の責務を定義する
- [x] 1.3 `kal.json` 読み込み失敗時のエラー契約を定義する
- [x] 1.4 `assert_clean()` と違反一覧を返すAPIを分ける
- [x] 1.5 既存 `AstLinterOps::run(...)` の互換性を維持する

### Definition of Done

- [x] consumer repositoryのAST lint testが1行で書ける
- [x] 失敗時の出力がv0.2.0のreporter contractに従う

## 2. Rule Catalog

### Definition of Ready

- [x] v0.2.0のthreshold設定とseverity contractが確定している

### Tasks

- [x] 2.1 全ルールをcatalogへ登録する
- [x] 2.2 rule id、category、severity、remediation guidanceをcatalogで管理する
- [x] 2.3 ruleごとの既定thresholdをcatalogで管理する
- [x] 2.4 source file ruleとdomain ruleのinput kindを分ける
- [x] 2.5 catalog未登録のpublic ruleが残らない検証を追加する

### Definition of Done

- [x] 標準ルールセットがKAL側で一元管理されている
- [x] consumer repositoryがrule名や案内文を手で束ねなくてよい

## 3. `kal.json` Runtime Integration

### Definition of Ready

- [x] v0.2.0の `kal.json` parserが利用できる

### Tasks

- [x] 3.1 source rootsを `kal.json` からrunnerへ渡す
- [x] 3.2 domain rule input pathsを `kal.json` からrunnerへ渡す
- [x] 3.3 thresholdsをcatalog既定値へ上書き適用する
- [x] 3.4 scoped allowancesを違反判定へ適用する
- [x] 3.5 rule全体の無効化や広いglob除外を受け付けない検証を追加する

### Definition of Done

- [x] repository差分をrunnerコードではなく `kal.json` で表現できる
- [x] 全ルール実行の前提が崩れない

## 4. Consumer Integration

### Definition of Ready

- [x] 1行runner APIとrule catalogが実装済みである

### Tasks

- [x] 4.1 READMEの推奨例を1行runnerへ更新する
- [x] 4.2 docsにconsumer repository向け最小test例を追加する
- [x] 4.3 KAL自身の `tests/repository_ast_lint.rs` を1行runnerへ移行する
- [x] 4.4 既存の個別rule API利用例をadvanced useとして残す
- [x] 4.5 実checkoutがあるconsumer repositoryで導入差分を確認する

### Definition of Done

- [x] 新規consumer repositoryが最小1行のtestでKALを導入できる
- [x] 既存consumer repositoryが段階的に移行できる

## 5. Verification

- [x] 5.1 `scripts/openspec validate "v0-3-0-one-line-consumer-runner" --strict` を実行する
- [x] 5.2 `just check` を実行する
- [x] 5.3 `cargo publish --dry-run --locked` を実行する
