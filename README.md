# datacenter-cx-docs

Open, reusable commissioning, startup, and operations documents for data centers: checklists, test plans, templates, and reference one-pagers. Written for owner's reps, GCs, Cx agents, controls integrators, and facilities engineers.

> **Status: DRAFT.** Every document carries `status: "DRAFT — author review pending"` in its frontmatter until the author changes it. Nothing here is a tagged release. Verified releases will be tagged.

## What is here

| Path | Contents |
|---|---|
| `docs/guides/` | Which document to use, procedure best practices, the pre-site plan template, backout planning, commissioning levels, glossary |
| `docs/mop/` | Method of Procedure template and examples (UPS bypass, generator load bank, distribution isolation, cooling unit) |
| `docs/sop/` | Standard Operating Procedure template and examples (rounds, generator run, change authorization, shift turnover) |
| `docs/eop/` | Emergency Operating Procedure template and examples (utility loss, UPS failure, cooling loss, water leak) |
| `docs/cx-scripts/` | Functional performance test script template and examples (UPS, generator and transfer, cooling unit, monitoring) |
| `docs/ist/` | Integrated systems test plan, readiness checklist, and a scenario script |
| `docs/spares/` | Critical spare parts program guide, scoring worksheet, inventory template, audit SOP |
| `docs/checklists/` | Pre-functional checklist |
| `docs/templates/` | Pre-task briefing, issues log, post-event review |
| `docs/one-pagers/` | Quick references |
| `docs/VERIFY.md` | Every `[VERIFY: ...]` placeholder awaiting a checked standards reference (generated) |
| `docs/LANGUAGE-REGISTER.md` | Anything in the repo that is not Rust, and why |
| `tools/doccheck/` | Rust linter run in CI |

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

## Licensing

The repository is split-licensed:

- **Code** (everything under `tools/` and `.github/`): MIT, see [LICENSE-MIT](LICENSE-MIT).
- **Documents** (everything under `docs/`): CC BY 4.0, see [LICENSE-CC-BY-4.0](LICENSE-CC-BY-4.0). Reuse with attribution.

## Disclaimer

Drafts are provided as-is, with no warranty. They support, and do not replace, the project's design basis, applicable codes and standards, and qualified professional judgment.
