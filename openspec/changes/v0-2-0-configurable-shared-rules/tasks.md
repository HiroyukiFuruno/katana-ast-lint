# Tasks: v0-2-0-configurable-shared-rules

## 1. External Foundation Review

### Definition of Ready

- [ ] v0.1.0の公開APIとconsumer runnerの使い方が確認できている
- [ ] KatanA、KME、kdp、kle、kcf、kuwで必要なrule差分が棚卸しされている

### Tasks

- [ ] 1.1 Dylintで置き換えられるRust専用rule実行責務を確認する
- [ ] 1.2 ast-grepで表現できる構文パターンruleと設定形式を確認する
- [ ] 1.3 SemgrepがKALの用途に対して過剰か、採用価値があるか確認する
- [ ] 1.4 外部ライブラリーへ委譲する責務と、KAL内部に残す責務を表にする
- [ ] 1.5 採用しない候補は理由を残し、後続で同じ比較を繰り返さないようにする

### Definition of Done

- [ ] KALがRust汎用lint基盤を再実装しない判断材料が揃っている
- [ ] katanaシリーズ共通ルールとしてKALに残す範囲が明確である

## 2. `kal.json` Contract

### Definition of Ready

- [ ] external foundation reviewの判断が完了している

### Tasks

- [ ] 2.1 `kal.json` のschemaを定義する
- [ ] 2.2 source rootsを設定できるようにする
- [ ] 2.3 rule thresholdsを設定できるようにする
- [ ] 2.4 scoped allow listsに理由と見直し条件を必須化する
- [ ] 2.5 domain rule input pathsを設定できるようにする
- [ ] 2.6 rule全体の無制限無効化や広いglob除外を標準仕様にしない

### Definition of Done

- [ ] repository固有差分をコード変更ではなく `kal.json` で表現できる
- [ ] 設定がlint失敗の隠蔽に使われない

## 3. Runtime and API

### Definition of Ready

- [ ] `kal.json` contractが確定している

### Tasks

- [ ] 3.1 設定読み込みAPIを追加する
- [ ] 3.2 `kal.json` 未指定時はv0.1.0互換の既定値で動作させる
- [ ] 3.3 test/CI runnerから設定を渡せるようにする
- [ ] 3.4 既存rule APIを壊さず、段階移行できる入口を残す

### Definition of Done

- [ ] v0.1.0利用repositoryが設定未導入でも壊れない
- [ ] consumer repositoryが設定導入後もlibrary-only APIを使い続けられる

## 4. Violation Output

### Definition of Ready

- [ ] runtime/APIの設定入口が確定している

### Tasks

- [ ] 4.1 violation重要度を定義する
- [ ] 4.2 JSON出力形式を定義する
- [ ] 4.3 text reporterとJSON reporterの責務を分ける
- [ ] 4.4 consumer repositoryのCIで読みやすい失敗出力を確認する

### Definition of Done

- [ ] 人間向け出力と機械向け出力の両方が共通契約で扱える

## 5. Verification

- [ ] 5.1 `scripts/openspec validate "v0-2-0-configurable-shared-rules" --strict` を実行する
- [ ] 5.2 `just check` を実行する
- [ ] 5.3 KatanA、KME、kdp、kle、kcf、kuwのうち、実checkoutがあるrepositoryで取り込み方針を確認する
