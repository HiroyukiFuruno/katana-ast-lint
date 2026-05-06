## Context

KatanA本体には、通常の静的検査だけでは拾えない構造違反を検知するAST lint相当の考え方がある。これを各repositoryへ複製すると、分離が進むほど検査の意味がずれる。

## Goals

- 共通rule、repository adapter、reporterを分ける。
- 各repositoryが同じ違反形式を使えるようにする。
- KME、kdp、kle、kcf、kuwの品質ゲートとして使える入口を持つ。
- ルール違反を除外設定で隠さない。
- CLIなしのlibrary-only crateとして公開できる。
- kml相当の品質ゲート、lefthook、CI/CD、release骨格を持つ。

## Non-Goals

- Rust compiler、clippy、formatの代替を作ること。
- KME文書モデルやpreview内部型をAST lint側へ持ち込むこと。
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

### Release Boundary

release workflowはGitHub Releaseと任意のcrates.io publishを扱う。`CARGO_REGISTRY_TOKEN` はユーザーが登録するため、migrationとGitHub Release準備の完了条件には含めない。

### P0 Gate

P1 `katana-markdown-engine` は、実装開始前にこの共通AST lint方針を着手条件へ含める。P2 `katana-ui-widget` とP3の各repositoryも同じ入口を参照する。

## Risks

- adapter層を省くとKatanA固有pathがruleへ混ざる。
- ruleを急いで増やしすぎると、分離前にlint運用だけが重くなる。
