# Tasks: v0-4-0-cli-runner

## 1. CLI Boundary

### Definition of Ready

- [ ] v0.3.0のone-line runner APIが実装済みである
- [ ] v0.3.0のrule catalogが実装済みである
- [ ] v0.2.0の `kal.json` parserとreporter contractが実装済みである

### Tasks

- [ ] 1.1 CLIを追加する理由とlibrary-only方針の変更点をREADMEとOpenSpecへ反映する
- [ ] 1.2 binary名を `kal` として確定する
- [ ] 1.3 CLIはrunner APIを呼ぶ薄い入口に限定する
- [ ] 1.4 CLI専用のrule catalogや設定解釈を作らない設計にする
- [ ] 1.5 `[[bin]]` target追加に伴うrelease artifact方針を確認する

### Definition of Done

- [ ] CLI追加後もrule実装と設定解釈がlibrary側に一元化されている
- [ ] API方式とCLI方式の責務差分がdocsで説明されている

## 2. Command Contract

### Definition of Ready

- [ ] CLI boundaryが確定している

### Tasks

- [ ] 2.1 `kal check` commandを定義する
- [ ] 2.2 workspace root解決をrunner APIと共有する
- [ ] 2.3 `kal.json` 読み込みをrunner APIと共有する
- [ ] 2.4 text reporterとJSON reporterの出力先を定義する
- [ ] 2.5 exit code `0`, `1`, `2` の意味を固定する
- [ ] 2.6 rule無効化や広いglob除外をCLI optionとして追加しない

### Definition of Done

- [ ] `kal check` が全標準ルールを実行する
- [ ] CLI optionが `kal.json` の安全な設定境界を迂回しない

## 3. API and CLI Parity

### Definition of Ready

- [ ] `kal check` の最小実装がある

### Tasks

- [ ] 3.1 API runnerとCLI runnerで同じfixtureを使うparity testを追加する
- [ ] 3.2 違反なし、違反あり、設定エラーのexit code testを追加する
- [ ] 3.3 text reporterの出力がAPI runnerとCLI runnerで同等であることを確認する
- [ ] 3.4 JSON reporterの出力がAPI runnerとCLI runnerで同等であることを確認する
- [ ] 3.5 CLI追加後も既存API testが通ることを確認する

### Definition of Done

- [ ] API方式とCLI方式で同じ入力に対して同じlint結果になる
- [ ] CLI追加が既存API利用repositoryを壊さない

## 4. Consumer Integration

### Definition of Ready

- [ ] CLI parity testが通っている

### Tasks

- [ ] 4.1 READMEにAPI方式とCLI方式の選び方を追加する
- [ ] 4.2 docsに `just ast-lint` から `kal check` を呼ぶ例を追加する
- [ ] 4.3 docsにCIでversion固定して `kal check` を実行する例を追加する
- [ ] 4.4 実checkoutがあるconsumer repositoryで、API方式とCLI方式の差分量を比較する
- [ ] 4.5 推奨導入順をdocsに明記する

### Definition of Done

- [ ] consumer repositoryが既存の `just` / CI構成に合わせてAPI方式かCLI方式を選べる
- [ ] CLI方式でも設定とrule実行がKAL側に一元化される

## 5. Verification

- [ ] 5.1 `scripts/openspec validate "v0-4-0-cli-runner" --strict` を実行する
- [ ] 5.2 `just check` を実行する
- [ ] 5.3 `cargo publish --dry-run --locked` を実行する
- [ ] 5.4 `cargo install --path .` 後に `kal check` を実行できることを確認する
