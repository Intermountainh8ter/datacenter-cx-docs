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
| Typst (repo topic only) | none yet | No Typst sources exist. If PDF output is added later it must be registered here. |
