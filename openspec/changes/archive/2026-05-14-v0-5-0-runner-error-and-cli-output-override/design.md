## Context

v0.4.0で確定したCLI契約は次のとおり。

- exit code: `0` 違反なし / `1` 違反あり / `2` 設定または実行エラー
- CLIはlibrary backend（v0.3.0 runner API）を呼ぶだけで、独自のrule実装や設定解釈を持たない
- `kal.json` の `reporter.mode` でtext / JSONを切り替える

実装上、`KatanaAstLint::try_from_workspace` と `try_assert_clean` は `Result<_, String>` を返す。CLIは

- `try_from_workspace` が `Err` → exit 2
- `try_assert_clean` が `Err` → exit 1

としているため、`try_assert_clean` 実行中に発生したsystem error（例: reporter出力時のIOエラー、catalog参照失敗など将来的なケース）も `exit 1` と扱われてしまう。spec が要求する「設定、入力、実行不能」を `2` に分離するためには、runnerが「違反」と「systemエラー」を構造的に区別して返す必要がある。

加えて、reporter modeをCIで一時的に切り替えたい要望がある。一方で、CLI optionにrule無効化や glob除外を入れると spec `Requirement: CLI options do not bypass configuration safety` に反する。output modeは安全境界を迂回する設定面ではないため、この範囲に限ってread-only overrideを認める。

## Goals

- runnerの戻り値で「違反」「設定エラー」「systemエラー」を区別する。
- CLIが上記をexit code `1`/`2` に厳密にmapする。
- `kal check --json` / `--text` でreporter modeを一時overrideできる。
- 既存の `from_workspace` / `assert_clean` の panic 互換挙動を保つ。
- API runnerとCLI runnerのparityを v0.4.0 と同じ強度で維持する。

## Non-Goals

- rule単位の有効/無効CLI option（spec違反のため引き続き不可）。
- glob除外CLI option。
- `kal.json` schemaの再定義。
- CLI commandの追加（`kal check` 以外は v0.5.0 では追加しない）。
- error型のserde互換（v0.5.0では非serializableで良い）。

## Decisions

### Error Type

公開クレートに `KalRunError` を追加する。

```rust
#[derive(Debug)]
pub enum KalRunError {
    /// rule violations were detected and reported
    Violations(String),
    /// kal.json or workspace configuration is invalid / unreadable
    Configuration(String),
    /// non-violation runtime failure (IO, internal invariants)
    System(String),
}
```

- `try_from_workspace` の `kal.json` parse失敗 → `Configuration`
- `try_assert_clean` の violation検出 → `Violations`
- それ以外（上記2つに当てはまらない実行不能） → `System`
- `Display` 実装で既存の `String` 互換メッセージを返す（panic API 互換のため）。

### Panic API Compatibility

`from_workspace` / `assert_clean` は次のように `try_*` を呼んで `unwrap_or_else(|e| panic!("{e}"))` する。挙動はv0.4.0と等価。

### CLI Mapping

```rust
match KatanaAstLint::try_from_workspace() {
    Ok(linter) => match linter.try_assert_clean() {
        Ok(()) => exit(0),
        Err(KalRunError::Violations(_)) => exit(1),
        Err(KalRunError::Configuration(_) | KalRunError::System(_)) => exit(2),
    },
    Err(KalRunError::Configuration(_) | KalRunError::System(_)) => exit(2),
    Err(KalRunError::Violations(_)) => exit(2), // unreachable; treat as system bug
}
```

### CLI Output Override

```bash
kal check               # use kal.json reporter.mode (default text)
kal check --json        # force JSON mode for this invocation
kal check --text        # force text mode for this invocation
```

- override対象は `KalConfig.reporter.mode` のみ。`kal.json` の他フィールド（rules有効/無効、scoped allowance等）は変更不可。
- `--json` と `--text` を同時指定した場合は exit 2 で usage error。
- `kal.json` が存在しない場合でも override は適用できる。

### Parity Preservation

`tests/cli_parity.rs` のtext / JSON parity testは引き続き等価性を確認する。`--json` overrideのケースを1本追加し、API側は `kal.json` で `mode: json` を指定したときの結果と完全一致することをassertする。

## Risks

- `KalRunError` を公開API化すると、後方互換性のために将来のvariant追加で `#[non_exhaustive]` を付けないと patch bump で SemVer 違反になる。v0.5.0の段階で `#[non_exhaustive]` を付ける。
- `try_*` の戻り値型変更はminor bump相当の breaking change。consumer repositoryのうち `Result<_, String>` を直接 match しているコードがあれば修正が必要。v0.5.0 changelogで明示する。
- CLI optionが増えると、将来的に rule disable optionを追加したい誘惑が出る。本change以降のproposalでも `Requirement: CLI options do not bypass configuration safety` を再確認するルールにする。
