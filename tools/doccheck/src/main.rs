//! doccheck: lint the `docs/` tree.
//!
//! Usage: doccheck [docs-dir] [--write-verify]
//!
//! Checks:
//! 1. Every Markdown file (except SUMMARY.md) has frontmatter with the DRAFT status.
//! 2. Every `[VERIFY: ...]` placeholder is listed in VERIFY.md against its file.
//!    `--write-verify` regenerates VERIFY.md from the documents first.
//! 3. Every procedure (doc_type mop, sop, eop, cx-script, ist) has a
//!    "Pre-site plan" heading and a "Backout" heading.
//! 4. Every document is linked from SUMMARY.md.
//! 5. No term from the local, untracked `.doccheck-banned.txt` appears in docs/.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

const STATUS_LINE: &str = "status: \"DRAFT — author review pending\"";
const PROCEDURE_TYPES: [&str; 5] = ["mop", "sop", "eop", "cx-script", "ist"];
const REQUIRED_HEADINGS: [&str; 2] = ["pre-site plan", "backout"];

fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(rd) = fs::read_dir(dir) else { return };
    for e in rd.flatten() {
        let p = e.path();
        if p.is_dir() {
            walk(&p, out);
        } else if p.extension().is_some_and(|x| x == "md") {
            out.push(p);
        }
    }
}

/// Unique placeholders in order of first appearance.
fn placeholders(text: &str) -> Vec<String> {
    let mut found: Vec<String> = Vec::new();
    let mut rest = text;
    while let Some(i) = rest.find("[VERIFY:") {
        let tail = &rest[i..];
        let Some(j) = tail.find(']') else { break };
        let p = tail[..=j].to_string();
        if !found.contains(&p) {
            found.push(p);
        }
        rest = &tail[j + 1..];
    }
    found
}

fn doc_type(text: &str) -> Option<&str> {
    text.lines()
        .skip(1)
        .take_while(|l| l.trim() != "---")
        .find_map(|l| l.strip_prefix("doc_type:"))
        .map(str::trim)
}

fn has_heading(text: &str, needle: &str) -> bool {
    text.lines()
        .any(|l| l.starts_with('#') && l.to_lowercase().contains(needle))
}

fn verify_row(rel: &str, placeholder: &str) -> String {
    format!("| {rel} | `{placeholder}` |")
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let write_verify = args.iter().any(|a| a == "--write-verify");
    let root = args
        .iter()
        .find(|a| !a.starts_with("--"))
        .cloned()
        .unwrap_or_else(|| "docs".into());
    let root = Path::new(&root);

    let mut files = Vec::new();
    walk(root, &mut files);
    files.sort();

    // (relative path with forward slashes, file name, text)
    let docs: Vec<(String, String, String)> = files
        .iter()
        .map(|f| {
            let rel = f
                .strip_prefix(root)
                .unwrap_or(f)
                .to_string_lossy()
                .replace('\\', "/");
            let name = f
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("")
                .to_string();
            let text = fs::read_to_string(f).unwrap_or_default().replace("\r\n", "\n");
            (rel, name, text)
        })
        .collect();

    let verify_path = root.join("VERIFY.md");
    if write_verify {
        let mut out = String::from(
            "---\ntitle: VERIFY Register\ndoc_type: register\n",
        );
        out.push_str(STATUS_LINE);
        out.push_str(
            "\nlicense: CC BY 4.0\n---\n\n# VERIFY Register\n\n\
Every `[VERIFY: ...]` placeholder in `docs/` is listed here against the file that contains it. \
Replace the placeholder in the source document with the checked reference, then regenerate this \
file with `doccheck docs --write-verify`. CI fails if a placeholder is missing from this list.\n\n\
| File | Placeholder |\n|---|---|\n",
        );
        for (rel, name, text) in &docs {
            if name == "VERIFY.md" {
                continue;
            }
            for p in placeholders(text) {
                out.push_str(&verify_row(rel, &p));
                out.push('\n');
            }
        }
        if let Err(e) = fs::write(&verify_path, out) {
            eprintln!("cannot write {}: {e}", verify_path.display());
            return ExitCode::FAILURE;
        }
    }

    let verify_text = fs::read_to_string(&verify_path)
        .unwrap_or_default()
        .replace("\r\n", "\n");
    let summary_text = fs::read_to_string(root.join("SUMMARY.md")).unwrap_or_default();
    let banned: Vec<String> = fs::read_to_string(".doccheck-banned.txt")
        .unwrap_or_default()
        .lines()
        .map(|l| l.trim().to_lowercase())
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .collect();
    if banned.is_empty() {
        eprintln!("note: no .doccheck-banned.txt found; skipping banned-term check");
    }

    let mut errors = 0;
    let mut fail = |msg: String| {
        eprintln!("{msg}");
        errors += 1;
    };

    for (rel, name, text) in &docs {
        let is_summary = name == "SUMMARY.md";

        if !is_summary {
            if !(text.starts_with("---\n") && text.contains(STATUS_LINE)) {
                fail(format!("{rel}: missing frontmatter or DRAFT status line"));
            }
            if !summary_text.contains(&format!("({rel})")) {
                fail(format!("{rel}: not linked from SUMMARY.md"));
            }
            if let Some(t) = doc_type(text) {
                if PROCEDURE_TYPES.contains(&t) {
                    for h in REQUIRED_HEADINGS {
                        if !has_heading(text, h) {
                            fail(format!("{rel}: {t} document has no \"{h}\" heading"));
                        }
                    }
                }
            } else {
                fail(format!("{rel}: no doc_type in frontmatter"));
            }
        }
        if name != "VERIFY.md" {
            for p in placeholders(text) {
                if !verify_text.contains(&verify_row(rel, &p)) {
                    fail(format!("{rel}: placeholder not in VERIFY.md: {p}"));
                }
            }
        }
        let lower = text.to_lowercase();
        for b in &banned {
            if lower.contains(b.as_str()) {
                fail(format!("{rel}: contains banned term (see local list)"));
            }
        }
    }

    println!("checked {} files, {errors} problem(s)", docs.len());
    if errors == 0 {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
