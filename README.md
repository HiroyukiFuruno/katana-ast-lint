# katana-ast-lint

`katana-ast-lint` は、KatanA ecosystem の分離repositoryで共通利用する抽象構文木検査（AST lint）です。

P0として先に分離し、`katana-markdown-engine`、`katana-document-preview`、`katana-language-editor`、`katana-canvas-forge`、`katana-ui-widget` が同じ品質ゲートを使える状態にします。

## 初期方針

- 共通rule本体にKatanA固有pathを直書きしません。
- repository固有のfile探索やfixture指定はadapterで扱います。
- 違反形式はrule id、重要度、対象file、範囲、message、修正方針を持ちます。
- lintを通すためだけの除外設定追加は禁止します。
