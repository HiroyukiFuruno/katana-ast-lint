## Context

KAL v0.1.0は、KatanA本体からAST lint ruleを切り出し、各repositoryがtest/CIからlibrary APIを呼べる状態にした。現状のKALはkatanaシリーズ共通ルールの共有境界であり、Rust汎用lint基盤ではない。

v0.2.0では、repositoryごとの違いをコードではなく設定で表現する必要がある。ただし設定ファイルを単なる除外リストにすると、品質ゲートの意味が弱くなる。

## Goals

- `kal.json` でrepository差分を表現する。
- v0.1.0利用repositoryを壊さない。
- 重要度とJSON出力を共通違反形式として扱う。
- 外部ライブラリーに任せるべき解析/実行責務を実装前に判断する。
- KALをkatanaシリーズ共通ルールの境界に留める。

## Non-Goals

- CLIを提供すること。
- Clippy、Dylint、ast-grep、Semgrepを無条件に置き換えること。
- repository固有の例外を無制限に設定で黙らせること。
- KME、preview、editor、export、widgetの内部型をKALへ持ち込むこと。

## Decisions

### Configuration File

`kal.json` はrepository rootに置く。consumer repositoryのtest/CI runnerは、明示されたpathまたはworkspace rootから `kal.json` を読み込む。

設定対象は次に限定する。

- source roots
- rule thresholds
- scoped allow lists with reason
- domain rule input paths
- reporter mode

rule全体の無効化や広いglob除外は標準仕様にしない。必要な場合は、対象rule、理由、代替設計、見直し条件を設定またはOpenSpecへ明記する。

### Backward Compatibility

`kal.json` が存在しない場合はv0.1.0互換の既定値で動作する。既存のrunnerは段階的に `kal.json` へ移行できる。

### External Library Evaluation

実装前に次を比較する。

- Dylint: Rust専用の独自lintをClippy風に配布/実行できるか
- ast-grep: 構文パターンと設定駆動ruleを扱いやすいか
- Semgrep: 汎用パターン検査として過剰でないか
- 現KAL内部実装: katanaシリーズ固有ruleを維持しやすいか

採用判断は「堅牢性が上がる責務だけを外部ライブラリーへ委譲する」ことを基準にする。KALは最終的な共通ルール契約、adapter境界、violation contractを保持する。

## Risks

- 設定ファイルが除外の逃げ道になる。
- 外部ライブラリーを無条件採用すると、library-only APIやconsumer test/CI入口が崩れる。
- KAL内部実装を増やしすぎると、Rust汎用lint基盤の再実装になる。
- JSON出力を先に固定しすぎると、consumer repositoryの実運用に合わない。
