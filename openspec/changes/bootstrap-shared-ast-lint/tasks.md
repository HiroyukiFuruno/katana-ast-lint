# Tasks: bootstrap-shared-ast-lint

## 1. Repository Baseline

### Definition of Ready

- [x] `katana/openspec/changes/extract-katana-ast-lint` でP0方針が定義済みである
- [x] GitHub repositoryを作成し、first pushが完了している

### Tasks

- [x] 1.1 crate構成を決める
- [x] 1.2 common rule、repository adapter、reporter、runnerの責務を分ける
- [x] 1.3 public DTOにKatanA固有pathを持たせない
- [x] 1.4 CLIなしのlibrary-only crateとして構成する
- [x] 1.5 KatanA `crates/katana-linter` のrule実装を移植する

### Definition of Done

- [x] `katana-ast-lint` が単独repositoryとして成立している
- [x] KMEやkdpへ依存していない
- [x] `[[bin]]` targetを持たない

## 2. Violation Contract

### Definition of Ready

- [x] baseline層分けが確定している

### Tasks

- [x] 2.1 v0の違反形式を対象file、line、column、messageとして定義する
- [x] 2.2 rule idと修正方針をtest runner / reporterで束ねる
- [x] 2.3 終了コードはcargo testの失敗として扱う
- [x] 2.4 重要度とJSON出力はv0.2.0の後続OpenSpecで扱うことを定義する

### Definition of Done

- [x] 各repositoryのCIが同じ違反形式を読める
- [x] 違反結果が手元確認でも読める

## 3. Repository Adapter

### Definition of Ready

- [x] violation contractが確定している

### Tasks

- [x] 3.1 repository固有file探索をadapterへ分離する
- [x] 3.2 fixtureや許可対象をadapterから注入できるようにする
- [x] 3.3 共通ruleへrepository固有pathを直書きしないテストを用意する
- [x] 3.4 `KATANA_AST_LINT_TARGET_DIR` で外部repository rootを指定できるようにする

### Definition of Done

- [x] KatanA、KME、kdp、kle、kcf、kuwが同じruleをadapter経由で実行できる

## 4. Downstream Adoption

### Definition of Ready

- [x] adapter contractが確定している

### Tasks

- [x] 4.1 `katana-markdown-engine` の着手条件へP0品質ゲート利用を接続する
- [x] 4.2 `katana-ui-widget` の後続分離計画へP0品質ゲート利用を接続する
- [x] 4.3 kdp、kle、kcf、KatanA統合の後続計画へ接続する
- [x] 4.4 KatanA本体では内部 `crates/katana-linter` の単純維持ではなく、外部 `katana-ast-lint` をworkspace品質ゲートへ取り込む後続作業として扱う

### Definition of Done

- [x] repositoryごとの独自lint driftを検知する計画が後続作業へ接続されている

## 5. Quality and Release Gates

### Definition of Ready

- [x] kml相当の品質ゲート構成を確認済みである
- [x] 利用するcrate dependenciesとGitHub Actions versionsを最新確認済みである

### Tasks

- [x] 5.1 `just check` を整備する
- [x] 5.2 `lefthook.yml` を整備する
- [x] 5.3 CI workflowを整備する
- [x] 5.4 release preflight workflowを整備する
- [x] 5.5 release workflowを整備する
- [x] 5.6 crates.io token登録をユーザー作業としてrelease runbookへ明記する

### Definition of Done

- [x] `just check` がlocal品質ゲートとして成立している
- [x] release workflowはGitHub Releaseと任意のcrates.io publishを扱える
- [x] `CARGO_REGISTRY_TOKEN` 未登録でもmigrationとrelease準備を進められる

## 6. Final Verification

- [x] 6.1 `just check` を実行する
- [x] 6.2 `cargo publish --dry-run --locked` を実行する
- [x] 6.3 `scripts/openspec validate "bootstrap-shared-ast-lint" --strict` を実行する

## 7. v0.2.0 Configuration Contract

### Definition of Ready

- [x] v0.1.0のlibrary-only APIとrepository adapter方針が確定している
- [x] 現在のrule実装に、repository固有path、許可値、閾値、対象directoryがハードコードされていることを確認済みである

### Tasks

- [x] 7.1 v0.2.0では設定ファイルを `kal.json` として扱う
- [x] 7.2 `kal.json` はrepository rootに置き、test/CI runnerが読み込む
- [x] 7.3 `kal.json` には対象directory、除外ではない明示的な許可リスト、ruleごとの閾値、domain ruleの入力pathを定義できる
- [x] 7.4 設定未指定時はv0.1.0互換の既定値で動作し、v0.1.0利用repositoryを壊さない
- [x] 7.5 設定ファイルでlint失敗を隠す目的の無制限除外やrule無効化を標準仕様にしない
- [x] 7.6 重要度とJSON出力は `kal.json` のrule設定と同じv0.2.0 OpenSpecで扱う

### Definition of Done

- [x] v0.2.0のOpenSpecを作る担当者が、`kal.json` の責務と禁止事項を再解釈せず着手できる
- [x] downstream repositoryは、自分の構成差分をコード変更ではなく `kal.json` で表現する方針を参照できる
