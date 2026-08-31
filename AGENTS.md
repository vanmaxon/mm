# Repository Guidelines

## Project Structure & Module Organization

MicroBin is a single-crate Rust 2021 web application. `src/main.rs` configures the Actix Web server and shared state, while `src/args.rs` defines CLI flags and `MICROBIN_*` environment variables. Request handlers live in `src/endpoints/`; shared persistence, authentication, ID, QR, and syntax-highlighting helpers belong in `src/util/`. The core paste model is in `src/pasta.rs`. Askama HTML templates and embedded assets are under `templates/` and `templates/assets/`. Runtime paste data is written to `pasta_data/` and must not be committed.

## Build, Test, and Development Commands

- `cargo run -- --editable --highlightsyntax` starts a development server on port 8080.
- `cargo build` compiles a debug binary; `cargo build --release` matches the Docker and Render production build.
- `cargo test` runs all unit and integration tests.
- `cargo fmt --all -- --check` verifies standard Rust formatting.
- `cargo clippy --all-targets --all-features -- -D warnings` catches common mistakes and treats warnings as failures.
- `docker build -t microbin .` builds the production container.

Use `cargo run -- --help` for the complete configuration list. Prefer environment variables such as `MICROBIN_PORT=8081` when testing deployment-style configuration.

## Coding Style & Naming Conventions

Use rustfmt defaults (four-space indentation) and keep code Clippy-clean. Follow Rust naming conventions: `snake_case` for modules, functions, and variables; `PascalCase` for structs and enums; `SCREAMING_SNAKE_CASE` for constants. Keep endpoint modules focused on HTTP concerns and move reusable logic into `src/util/`. Match existing Askama template names to their routes, such as `pastalist.rs` and `pastalist.html`.

## Testing Guidelines

The current snapshot has no committed tests, so add focused `#[cfg(test)]` unit modules beside pure logic and integration tests under `tests/` for request flows. Name tests by observable behavior, for example `rejects_unsafe_upload_filename`. Cover success, invalid input, and persistence behavior. Run `cargo test`, formatting, and Clippy before submitting.

## Commit & Pull Request Guidelines

Git history is not included in this source snapshot, so no repository-specific commit convention can be inferred. Use short, imperative subjects such as `Validate public URL configuration`, and keep unrelated changes separate. Pull requests should explain the user-visible effect, list verification commands, link relevant issues, and include screenshots for template or CSS changes. Highlight configuration, storage-format, or security implications explicitly; report vulnerabilities through the contact in `SECURITY.md` rather than a public issue.
