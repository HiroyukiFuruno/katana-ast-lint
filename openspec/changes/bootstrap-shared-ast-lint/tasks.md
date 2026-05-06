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
- [ ] 2.4 重要度とJSON出力は後続拡張として別OpenSpecで扱う

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

- [x] 4.1 `katana-markdown-engine` の品質ゲートへ接続する
- [x] 4.2 `katana-ui-widget` の品質ゲートへ接続する
- [x] 4.3 kdp、kle、kcf、KatanA統合の後続計画へ接続する

### Definition of Done

- [x] repositoryごとの独自lint driftを検知できる

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
