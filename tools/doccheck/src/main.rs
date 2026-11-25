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
//! 6. Every alias (doc_type alias) names its source in `alias_of:` (a path
//!    relative to docs/), the source exists and is not itself an alias, both
//!    carry the same status, and the alias body links to the source. Aliases
//!    keep published links alive when a document lives at a different path.

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

/// Value of a frontmatter field, trimmed.
fn field<'a>(text: &'a str, key: &str) -> Option<&'a str> {
    text.lines()
        .skip(1)
        .take_while(|l| l.trim() != "---")
        .find_map(|l| l.strip_prefix(key)?.strip_prefix(':'))
        .map(str::trim)
}

fn doc_type(text: &str) -> Option<&str> {
    field(text, "doc_type")
}

/// Link from the document at `from` to the document at `to`, both relative
/// to the docs root with forward slashes.
fn relative_link(from: &str, to: &str) -> String {
    let depth = from.matches('/').count();
    format!("{}{to}", "../".repeat(depth))
}

/// Problems with aliases. `docs` holds (relative path, file name, text).
fn alias_problems(docs: &[(String, String, String)]) -> Vec<String> {
    let find = |rel: &str| docs.iter().find(|(r, _, _)| r == rel);
    let mut out = Vec::new();
    for (rel, _, text) in docs {
        let target = field(text, "alias_of");
        if doc_type(text) != Some("alias") {
            if target.is_some() {
                out.push(format!("{rel}: has alias_of but doc_type is not alias"));
            }
            continue;
        }
        let Some(target) = target.filter(|t| !t.is_empty()) else {
            out.push(format!("{rel}: alias has no alias_of"));
            continue;
        };
        let Some((_, _, source)) = find(target) else {
            out.push(format!("{rel}: alias_of names a missing document: {target}"));
            continue;
        };
        if doc_type(source) == Some("alias") {
            out.push(format!("{rel}: alias_of names another alias: {target}"));
        }
        if field(source, "status") != field(text, "status") {
            out.push(format!("{rel}: status differs from its source {target}"));
        }
        let link = relative_link(rel, target);
        if !text.contains(&format!("]({link})")) {
            out.push(format!("{rel}: alias body does not link to ({link})"));
        }
    }
    out
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

    for p in alias_problems(&docs) {
        fail(p);
    }

    println!("checked {} files, {errors} problem(s)", docs.len());
    if errors == 0 {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DRAFT: &str = "status: \"DRAFT — author review pending\"";

    fn doc(rel: &str, front: &str, body: &str) -> (String, String, String) {
        let name = rel.rsplit('/').next().unwrap_or(rel).to_string();
        (rel.into(), name, format!("---\n{front}\n---\n\n{body}\n"))
    }

    fn source() -> (String, String, String) {
        doc("ist/readiness.md", &format!("title: R\ndoc_type: checklist\n{DRAFT}"), "# R")
    }

    fn alias(front_extra: &str, body: &str) -> (String, String, String) {
        doc(
            "checklists/readiness.md",
            &format!("title: R\ndoc_type: alias\n{DRAFT}\n{front_extra}"),
            body,
        )
    }

    #[test]
    fn relative_link_climbs_one_level_per_folder() {
        assert_eq!(relative_link("a.md", "x/y.md"), "x/y.md");
        assert_eq!(relative_link("a/b.md", "x/y.md"), "../x/y.md");
        assert_eq!(relative_link("a/b/c.md", "x/y.md"), "../../x/y.md");
    }

    #[test]
    fn field_matches_whole_key_only() {
        let t = "---\ndoc_type_x: no\ndoc_type: alias\n---\nalias_of: body\n";
        assert_eq!(field(t, "doc_type"), Some("alias"));
        assert_eq!(field(t, "alias_of"), None, "body lines are not frontmatter");
    }

    #[test]
    fn valid_alias_passes() {
        let docs = vec![
            source(),
            alias("alias_of: ist/readiness.md", "This document lives at [R](../ist/readiness.md)."),
        ];
        assert!(alias_problems(&docs).is_empty(), "{:?}", alias_problems(&docs));
    }

    #[test]
    fn alias_to_missing_document_fails() {
        let docs = vec![alias("alias_of: ist/gone.md", "[R](../ist/gone.md)")];
        let p = alias_problems(&docs);
        assert!(p[0].contains("missing document"), "{p:?}");
    }

    #[test]
    fn alias_without_target_fails() {
        let p = alias_problems(&[alias("", "nothing")]);
        assert!(p[0].contains("no alias_of"), "{p:?}");
    }

    #[test]
    fn alias_without_link_fails() {
        let docs = vec![source(), alias("alias_of: ist/readiness.md", "See the IST folder.")];
        let p = alias_problems(&docs);
        assert!(p[0].contains("does not link"), "{p:?}");
    }

    #[test]
    fn alias_of_alias_fails() {
        let mut second = alias("alias_of: checklists/readiness.md", "[R](../checklists/readiness.md)");
        second.0 = "templates/readiness.md".into();
        let docs = vec![
            source(),
            alias("alias_of: ist/readiness.md", "[R](../ist/readiness.md)"),
            second,
        ];
        let p = alias_problems(&docs);
        assert!(p.iter().any(|m| m.contains("another alias")), "{p:?}");
    }

    #[test]
    fn status_mismatch_fails() {
        let mut src = source();
        src.2 = src.2.replace("DRAFT — author review pending", "Reviewed");
        let docs = vec![src, alias("alias_of: ist/readiness.md", "[R](../ist/readiness.md)")];
        let p = alias_problems(&docs);
        assert!(p.iter().any(|m| m.contains("status differs")), "{p:?}");
    }

    #[test]
    fn alias_of_on_a_normal_document_fails() {
        let mut src = source();
        src.2 = src.2.replace("doc_type: checklist", "doc_type: checklist\nalias_of: x.md");
        let p = alias_problems(&[src]);
        assert!(p[0].contains("doc_type is not alias"), "{p:?}");
    }
}
