# datacenter-cx-docs

Open, reusable commissioning, startup, and operations documents for data centers: checklists, test plans, templates, and reference one-pagers. Written for owner's reps, GCs, Cx agents, controls integrators, and facilities engineers.

> **Status: DRAFT.** Every document carries `status: "DRAFT — author review pending"` in its frontmatter until the author changes it. Nothing here is a tagged release. Verified releases will be tagged.

> **Disclaimer.** These are generic templates and examples, not procedures for any real site, and they are not engineering or safety advice. Work on critical infrastructure can kill people and destroy equipment. Nothing here may be used on real equipment without site-specific review and approval by qualified persons, against current drawings, the manufacturer's instructions, and the applicable codes and standards. Provided as-is, with no warranty and no liability. **Read the full [DISCLAIMER](docs/DISCLAIMER.md) before using anything.**

## What is here

| Path | Contents |
|---|---|
| `docs/guides/` | Which document to use, procedure best practices, the pre-site plan template, backout planning, commissioning levels, glossary |
| `docs/mop/` | Method of Procedure template and examples (UPS bypass, generator load bank, distribution isolation, cooling unit, battery string, transfer switch, chilled water pump, firmware update, fuel polishing) |
| `docs/sop/` | Standard Operating Procedure template and examples (rounds, generator run, change authorization, shift turnover) |
| `docs/eop/` | Emergency Operating Procedure template and examples (utility loss, UPS failure, cooling loss, water leak, fire, fuel leak, generator failure, loss of monitoring, severe weather) |
| `docs/cx-scripts/` | Functional performance test script template and examples (UPS, generator and transfer, cooling unit, monitoring, transfer switches, chilled water plant, fire interfaces, heat-rise test) |
| `docs/ist/` | Integrated systems test plan, readiness checklist, and a scenario script |
| `docs/spares/` | Critical spare parts program guide, scoring worksheet, inventory template, audit SOP |
| `docs/operations/` | Operations program overview, drill schedule and scorecard, training and qualification matrix, vendor emergency contact sheet, maintenance calendar |
| `docs/DISCLAIMER.md` | The full disclaimer |
| `docs/checklists/` | Pre-functional checklist |
| `docs/templates/` | Pre-task briefing, issues log, post-event review |
| `docs/one-pagers/` | Quick references |
| `docs/VERIFY.md` | Every `[VERIFY: ...]` placeholder awaiting a checked standards reference (generated) |
| `docs/LANGUAGE-REGISTER.md` | Anything in the repo that is not Rust, and why |
| `tools/doccheck/` | Rust linter run in CI |
| `tools/pdfbuild/` and `tools/typst/` | Rust tool and Typst template that build a PDF of every document |

## Standards references

These documents do not reproduce standards text or numeric limits. Wherever a value or clause comes from a standard (ASHRAE, NFPA 70/70E/110, NETA ATS, IEEE), the document holds a `[VERIFY: <standard> <section>]` placeholder. Always use the current edition of the standard and the project's design basis.

## Generic by design

Examples are generic (for example, a 1.5 MW modular hall). No client, site, or project-specific information belongs in this repository.

## Checking the docs

```
cargo run --manifest-path tools/doccheck/Cargo.toml -- docs
```

`doccheck` confirms that:

- each document has the DRAFT frontmatter and a `doc_type`;
- each procedure (`mop`, `sop`, `eop`, `cx-script`, `ist`) has a **Pre-site plan** heading and a **Backout** heading;
- each `[VERIFY: ...]` placeholder is listed in `docs/VERIFY.md` (regenerate it with `-- docs --write-verify`);
- each document is linked from `docs/SUMMARY.md`;
- no term from a local, untracked `.doccheck-banned.txt` appears.

## Every procedure has a pre-site plan and a backout

Each MOP, SOP, EOP, Cx script, and IST script here carries a pre-site plan (what must be true before anyone travels to site or the window opens) and a backout procedure (how to return to the last known good state). For EOPs these are the preparedness and return-to-normal sections. These examples are starting points: a procedure is usable only after site-specific review against current drawings and the manufacturer's instructions.

To browse as a book: `mdbook serve` (uses `book.toml`).

## Printable PDFs

Each document can be built as a PDF for printing and sign-off. Every page carries the DRAFT status in the header and a short disclaimer in the footer.

```
cargo run --manifest-path tools/pdfbuild/Cargo.toml -- docs pdf
```

This needs [Typst](https://typst.app) on your PATH, or the `TYPST` environment variable set to the Typst executable. The first build downloads the `cmarker` Typst package. Output goes to `pdf/`, which is not tracked in git. CI builds the same PDFs on every push and attaches them to the workflow run as the `draft-pdfs` artifact. They are drafts, not a release.

## Licensing

The repository is split-licensed:

- **Code** (everything under `tools/` and `.github/`): MIT, see [LICENSE-MIT](LICENSE-MIT).
- **Documents** (everything under `docs/`): CC BY 4.0, see [LICENSE-CC-BY-4.0](LICENSE-CC-BY-4.0). Reuse with attribution.

## Disclaimer

See the notice at the top of this page and the full [DISCLAIMER](docs/DISCLAIMER.md). In short: templates only, drafts are unverified, not professional advice, the applicable codes, standards, and manufacturer instructions govern, and there is no warranty or liability.
