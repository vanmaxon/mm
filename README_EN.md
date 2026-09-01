![MicroBin screenshot](.github/index.png)

# MicroBin

**English** | [简体中文](README.md)

MicroBin is a lightweight, configurable, single-binary, self-hosted pastebin and URL shortener. It can store text, share files, create URL redirects, and provide expiration, burn-after-reading, editing, syntax highlighting, and QR-code features.

> **Project origin:** This project is derived from [MicroBin v1.2.1](https://github.com/szabodanika/microbin/tree/v1.2.1), created by **Dániel Szabó**, and is maintained and adapted on top of that release. The original project remains copyrighted by Dániel Szabó and its contributors.

## Features

- Text pastes, file uploads, and URL shortening/redirection
- Readable, automatically generated animal-name identifiers
- Optional custom keys using `a-z`, `0-9`, `-`, and `_`, with a length of 3–64 characters
- Raw text at `/raw/{key}` and file downloads at `/file/{key}/{filename}`
- Editable, final, private, public, and burn-after-reading pastes
- Configurable expiration and expired-data cleanup
- Syntax highlighting and QR codes
- Paste listing and manual deletion through `/pastalist`
- HTTP Basic Auth, read-only mode, and optional listing removal
- Automatic dark mode, custom CSS, and pure HTML mode
- English and Simplified Chinese UI; the initial language follows the browser, and a `microbin_lang` cookie remembers manual selection
- Portable JSON and local-file storage for straightforward backup and migration

## Quick start

### Run from source

A Rust toolchain is required. From the repository directory, run:

```bash
cargo run --release -- --editable --highlightsyntax
```

The server listens on `0.0.0.0:8080` by default. Open <http://localhost:8080>.

For development, use:

```bash
cargo run -- --editable --highlightsyntax
```

### Run with Docker

```bash
docker build -t ghcr.io/vanmaxon/mm:latest .
docker run -d \
  --name microbin \
  -p 8080:8080 \
  -v microbin-data:/app/pasta_data \
  ghcr.io/vanmaxon/mm:latest --editable --highlightsyntax
```

The current CI image name is `ghcr.io/vanmaxon/mm`. The default branch uses the `latest` tag and also publishes a Taipei-date version tag in `YYYYMMDD` format, such as `20260901`. To use the published image directly, run `docker pull ghcr.io/vanmaxon/mm:latest` first. The `microbin-data` volume persists application data. Open <http://localhost:8080> after the container starts.

## Configuration

Every command-line option can also be configured with its corresponding `MICROBIN_*` environment variable. Run the following command for the complete list:

```bash
cargo run -- --help
```

Common options include:

| CLI option | Environment variable | Default | Description |
| --- | --- | --- | --- |
| `--port` | `MICROBIN_PORT` | `8080` | HTTP listen port |
| `--bind` | `MICROBIN_BIND` | `0.0.0.0` | Listen address |
| `--public-path` | `MICROBIN_PUBLIC_PATH` | Empty | Public base URL; recommended behind a reverse proxy |
| `--threads` | `MICROBIN_THREADS` | `1` | Number of web workers |
| `--editable` | `MICROBIN_EDITABLE` | Off | Allow editable pastes |
| `--highlightsyntax` | `MICROBIN_HIGHLIGHTSYNTAX` | Off | Enable syntax highlighting |
| `--qr` | `MICROBIN_QR` | Off | Enable QR codes |
| `--private` | `MICROBIN_PRIVATE` | Off | Make new pastes private by default |
| `--readonly` | `MICROBIN_READONLY` | Off | Disable creation through the web UI |
| `--no-listing` | `MICROBIN_NO_LISTING` | Off | Hide the paste-list page |
| `--auth-username` | `MICROBIN_AUTH_USERNAME` | Empty | Basic Auth username |
| `--auth-password` | `MICROBIN_AUTH_PASSWORD` | Empty | Basic Auth password |
| `--gc-days` | `MICROBIN_GC_DAYS` | `90` | Remove pastes after this many days without a read; use `0` to disable this cleanup |
| `--custom-css` | `MICROBIN_CUSTOM_CSS` | Empty | URL for a custom stylesheet |

Environment-variable example:

```bash
MICROBIN_PORT=8081 \
MICROBIN_PUBLIC_PATH=https://paste.example.com \
MICROBIN_EDITABLE=true \
MICROBIN_HIGHLIGHTSYNTAX=true \
cargo run --release
```

> In PowerShell, set variables first—for example, `$env:MICROBIN_PORT = "8081"`—and then run the application.

## Data and backups

Runtime data is stored under `pasta_data/`:

- `pasta_data/database.json` stores paste metadata.
- `pasta_data/public/` stores uploaded files.

Back up the complete `pasta_data/` directory so metadata and uploaded files remain consistent. A backup is also recommended before upgrades or redeployment. This directory contains user data and should not be committed to Git.

## Development checks

```bash
cargo build
cargo test
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
```

## Security notes

- For public deployments, use an HTTPS reverse proxy and configure `MICROBIN_PUBLIC_PATH` correctly.
- If pastes must not be publicly browsable, enable Basic Auth or set `MICROBIN_NO_LISTING=true`.
- Uploaded content is stored on local disk; monitor available space and keep regular backups.
- Report security issues according to [SECURITY.md](SECURITY.md).

## License and attribution

This project is derived from Dániel Szabó's MicroBin v1.2.1 and is distributed under the [BSD 3-Clause License](LICENSE).

Copyright © 2022 Dániel Szabó. All rights reserved.
