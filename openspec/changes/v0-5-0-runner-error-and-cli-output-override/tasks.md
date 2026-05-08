# Tasks: v0-5-0-runner-error-and-cli-output-override

## 1. Structured Runner Error

### Definition of Ready

- [ ] v0.4.0のCLI runnerがmergeされている
- [ ] v0.4.0のparity testがCIで安定している

### Tasks

- [ ] 1.1 `KalRunError` enumを公開クレートに追加する（`Violations` / `Configuration` / `System`、`#[non_exhaustive]`）
- [ ] 1.2 `Display` 実装で既存の `String` 互換メッセージを返す
- [ ] 1.3 `try_from_workspace` の戻り値を `Result<Self, KalRunError>` に変更する
- [ ] 1.4 `try_assert_clean` の戻り値を `Result<(), KalRunError>` に変更する
- [ ] 1.5 `try_load_config` の戻り値を `Result<KalConfig, KalRunError>` に変更し、`Configuration` variantで包む
- [ ] 1.6 `from_workspace` / `assert_clean` panic APIを `try_*` の薄いラッパに退避し、互換挙動を保つ
- [ ] 1.7 既存の panic API testが通ることを確認する

### Definition of Done

- [ ] runnerが「違反」「設定エラー」「systemエラー」を構造的に返せる
- [ ] panic APIのv0.4.0互換性が保たれている

## 2. CLI Exit Code Mapping

### Definition of Ready

- [ ] `KalRunError` の3 variantが安定している

### Tasks

- [ ] 2.1 CLIで `Violations` → exit 1、`Configuration` / `System` → exit 2 にmapする
- [ ] 2.2 設定エラーケースのCLI testが exit 2 のままであることを確認する
- [ ] 2.3 reporter内部でsystem errorが発生したときに exit 2 を返すユニットテストを追加する（mockable な経路で）
- [ ] 2.4 既存の `cli_check_clean_project_exits_0` / `cli_check_violation_exits_1` / `cli_check_invalid_config_exits_2` がそのまま通ることを確認する

### Definition of Done

- [ ] CLIが3種のexit codeを実装と spec に基づいて厳密に区別する

## 3. CLI Reporter Mode Override

### Definition of Ready

- [ ] `kal check` の最小実装が v0.4.0 でmergeされている

### Tasks

- [ ] 3.1 `kal check --json` / `--text` を引数parserに追加する
- [ ] 3.2 override対象を `KalConfig.reporter.mode` のみに限定する（他フィールドはCLIから変更不可）
- [ ] 3.3 `--json` と `--text` の同時指定を usage error (exit 2) として扱う
- [ ] 3.4 `kal.json` 不在時でもoverrideが適用できることを確認する
- [ ] 3.5 helpメッセージに `--json` / `--text` を追記する
- [ ] 3.6 `--json` overrideのparity testを `tests/cli_parity.rs` に追加する（API側は `kal.json` で `mode: json` 指定）

### Definition of Done

- [ ] `kal check` がreporter modeのread-only overrideをCLI optionで受け付ける
- [ ] override対象が reporter mode のみに限定されている

## 4. Documentation

### Definition of Ready

- [ ] CLI optionの仕様が確定している

### Tasks

- [ ] 4.1 README CLI usage節に `--json` / `--text` の例を追加する
- [ ] 4.2 `docs/quality-gates.md` に「CIだけJSON出力したいケース」での `kal check --json` 例を追加する
- [ ] 4.3 CHANGELOG.mdに v0.5.0 エントリを追加し、`try_*` 戻り値型の breaking change を明記する
- [ ] 4.4 `KalRunError` のrustdocを書く

### Definition of Done

- [ ] consumer repositoryが新CLI optionとerror型を docs から把握できる
- [ ] breaking change が CHANGELOG に明記されている

## 5. Verification

- [ ] 5.1 `scripts/openspec validate "v0-5-0-runner-error-and-cli-output-override" --strict` を実行する
- [ ] 5.2 `cargo check` を実行する
- [ ] 5.3 `cargo test --workspace` を実行する
- [ ] 5.4 `cargo publish --dry-run --locked` を実行する
- [ ] 5.5 `cargo install --path .` 後に `kal check --json` / `--text` を実行できることを確認する
