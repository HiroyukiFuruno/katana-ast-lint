# Tasks: bootstrap-shared-ast-lint

## 1. Repository Baseline

### Definition of Ready

- [ ] `katana/openspec/changes/extract-katana-ast-lint` でP0方針が定義済みである

### Tasks

- [ ] 1.1 crate構成を決める
- [ ] 1.2 common rule、repository adapter、reporter、runnerの責務を分ける
- [ ] 1.3 public DTOにKatanA固有pathを持たせない

### Definition of Done

- [ ] `katana-ast-lint` が単独repositoryとして成立している
- [ ] KMEやkdpへ依存していない

## 2. Violation Contract

### Definition of Ready

- [ ] baseline層分けが確定している

### Tasks

- [ ] 2.1 rule id、重要度、対象file、範囲、message、修正方針を持つ違反DTOを定義する
- [ ] 2.2 JSON出力と人間向け出力を同じDTOから作る
- [ ] 2.3 終了コードの扱いを定義する

### Definition of Done

- [ ] 各repositoryのCIが同じ違反形式を読める
- [ ] 違反結果が手元確認でも読める

## 3. Repository Adapter

### Definition of Ready

- [ ] violation contractが確定している

### Tasks

- [ ] 3.1 repository固有file探索をadapterへ分離する
- [ ] 3.2 fixtureや許可対象をadapterから注入できるようにする
- [ ] 3.3 共通ruleへrepository固有pathを直書きしないテストを用意する

### Definition of Done

- [ ] KatanA、KME、kdp、kle、kcf、kuwが同じruleをadapter経由で実行できる

## 4. Downstream Adoption

### Definition of Ready

- [ ] adapter contractが確定している

### Tasks

- [ ] 4.1 `katana-markdown-engine` の品質ゲートへ接続する
- [ ] 4.2 `katana-ui-widget` の品質ゲートへ接続する
- [ ] 4.3 kdp、kle、kcf、KatanA統合の後続計画へ接続する

### Definition of Done

- [ ] repositoryごとの独自lint driftを検知できる

## 5. Final Verification

- [ ] 5.1 `npx -y @fission-ai/openspec validate "bootstrap-shared-ast-lint" --strict` を実行する
