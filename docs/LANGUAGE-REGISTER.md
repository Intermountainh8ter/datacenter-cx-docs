---
title: Language Register
doc_type: register
status: "DRAFT — author review pending"
license: CC BY 4.0
---

# Language Register

Rust is the preferred language. Anything that is not Rust is recorded here.

| Item | Language / format | Reason |
|---|---|---|
| Documents under `docs/` | Markdown | Documents are prose, not code; Markdown is what mdBook renders. |
| `.github/workflows/ci.yml` | YAML | Required format for GitHub Actions. |
| `book.toml` | TOML | mdBook configuration format (mdBook itself is Rust). |
| `tools/typst/doc.typ` | Typst | Page layout for PDF output. Typst is the typesetting system and has its own markup language; the Typst compiler itself is written in Rust. |
| `cmarker` Typst package (downloaded at build time, not stored in the repo) | Typst and Rust (WebAssembly) | Renders the Markdown documents inside Typst, so Markdown stays the single source and no second copy of each document is kept. |
