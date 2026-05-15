# katana-ast-lint OpenSpec

## Project

`katana-ast-lint` は、KatanA ecosystem の分離repositoryで共通利用する抽象構文木検査（AST lint）を担う。

## Design Principles

- P0として、KMMやUI widgetより先に分離する。
- 共通rule本体にrepository固有pathを持たせない。
- repository固有のfile探索、fixture、許可対象はadapterへ閉じる。
- CLIは提供しない。各repositoryはlibrary APIをtest/CIから呼び出す。
- 違反形式を共通化し、各repositoryのCIや手元検証で同じ結果を読めるようにする。
- lint除外で品質ゲートを抜ける設計にしない。

## Consumers

- KatanA
- katana-markdown-model
- katana-document-preview
- katana-language-editor
- katana-diagram-renderer
- katana-canvas-forge
- katana-ui-widget
