## Context

KatanA本体には、通常の静的検査だけでは拾えない構造違反を検知するAST lint相当の考え方がある。これを各repositoryへ複製すると、分離が進むほど検査の意味がずれる。

## Goals

- 共通rule、repository adapter、reporterを分ける。
- 各repositoryが同じ違反形式を使えるようにする。
- KMM、kdp、kle、kcf、kuwの品質ゲートとして使える入口を持つ。
- ルール違反を除外設定で隠さない。
- CLIなしのlibrary-only crateとして公開できる。
- kml相当の品質ゲート、lefthook、CI/CD、release骨格を持つ。

## Non-Goals

- Rust compiler、clippy、formatの代替を作ること。
- KMM文書モデルやpreview内部型をAST lint側へ持ち込むこと。
- 最初からすべてのKatanA固有ruleを共通化すること。
- CLIやユーザー向けコマンドを提供すること。

## Decisions

### Layering

AST lintは次の層に分ける。

- common rule: repository非依存の検査
- repository adapter: 対象file探索、fixture location、許可対象の注入
- reporter: 共通違反形式の出力
- runner: 各repositoryのtest/CIからの実行入口

### Violation Contract

v0ではlocation単位の `Violation` が対象file、line、column、messageを持つ。rule idと修正方針はtest runner / reporterの引数として束ねる。重要度とJSON出力は後続拡張にし、CLIを前提にしない。

### v0.2.0 Configuration

v0.2.0では、現在rule実装や各repository runnerに散らばる対象directory、許可値、閾値、domain ruleの入力pathを `kal.json` に移す。`kal.json` はrepository rootに置き、consumer repositoryのtest/CI runnerが読み込む。

`kal.json` はrepository差分を表現するための設定であり、lint失敗を隠すための無制限除外やrule無効化を標準機能にしない。例外が必要な場合は、ruleの意図、準拠できない理由、代替設計をOpenSpecまたはrepository文書へ明記してから扱う。

v0.1.0利用repositoryを壊さないため、設定未指定時はv0.1.0互換の既定値で動く。重要度とJSON出力は同じv0.2.0 changeで扱い、設定ファイルのrule定義と違反出力の意味を分離しない。

KALはRust汎用lint基盤を再発明するためのrepositoryではない。主目的はkatanaシリーズ共通ルールの共有境界である。v0.2.0では、Dylint、ast-grep、Semgrepなどの外部ライブラリーを使う方が堅牢になる部分と、KAL内部に残すべきKatanA固有ruleの境界を評価してから実装へ進む。

### Release Boundary

release workflowはGitHub Releaseと任意のcrates.io publishを扱う。`CARGO_REGISTRY_TOKEN` はユーザーが登録するため、migrationとGitHub Release準備の完了条件には含めない。

### P0 Gate

P1 `katana-markdown-model` は、実装開始前にこの共通AST lint方針を着手条件へ含める。P2 `katana-ui-widget` とP3の各repositoryも同じ入口を参照する。

## Risks

- adapter層を省くとKatanA固有pathがruleへ混ざる。
- ruleを急いで増やしすぎると、分離前にlint運用だけが重くなる。
- `kal.json` を単なる除外ファイルにすると、共通AST lintの品質統制が形だけになる。
- 外部ライブラリー評価を飛ばすと、KALが汎用lint基盤の再実装へ広がる。
