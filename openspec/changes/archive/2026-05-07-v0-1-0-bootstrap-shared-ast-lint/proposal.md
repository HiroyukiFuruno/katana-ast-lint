## Why

KatanA ecosystemでは、KME、preview、editor、export、widgetが別repositoryへ分離される。各repositoryでAST lintを独自に持つと、ルール、違反形式、実行入口がずれて品質統制が効かなくなる。

`katana-ast-lint` はP0として先に共通化し、分離後repositoryの品質ゲートを揃える。

## What Changes

- 共通AST lintのrepository baselineを作る
- 共通rule、repository adapter、reporter、test/CI実行入口を分ける
- KME以降のrepositoryが参照できる契約を定義する
- lint除外で失敗を隠さない運用を明文化する
- CLIは提供しないlibrary-only crateとして移植する
- kml相当の品質ゲート、lefthook、CI/CD、release骨格を用意する
- crates.io token登録はユーザー作業として後続扱いにする

## Capabilities

### New Capabilities

- `shared-ast-lint`: KatanA ecosystem共通のAST lint ruleと違反形式を提供する

## Impact

- 新規crate構成
- v0違反形式
- repository adapter interface
- CI / 手元実行入口
- release workflow
