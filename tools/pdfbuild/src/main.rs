//! pdfbuild: render every Markdown document in `docs/` to a PDF with Typst.
//!
//! Usage (from the repository root): pdfbuild [docs-dir] [out-dir]
//!
//! Defaults: docs-dir = "docs", out-dir = "pdf". The output mirrors the
//! folder layout of docs/. Set the TYPST environment variable to the Typst
//! executable if it is not on PATH. SUMMARY.md (the mdBook index) is skipped,
//! and so are alias pages (doc_type alias), which only point to a document
//! that is printed in its own right.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

const TEMPLATE: &str = "tools/typst/doc.typ";

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

/// True if the frontmatter marks the document as an alias page.
fn is_alias(text: &str) -> bool {
    text.lines()
        .skip(1)
        .take_while(|l| l.trim() != "---")
        .any(|l| l.trim() == "doc_type: alias")
}

fn main() -> ExitCode {
    let mut args = std::env::args().skip(1);
    let docs = PathBuf::from(args.next().unwrap_or_else(|| "docs".into()));
    let out_dir = PathBuf::from(args.next().unwrap_or_else(|| "pdf".into()));
    let typst = std::env::var("TYPST").unwrap_or_else(|_| "typst".into());

    if !Path::new(TEMPLATE).exists() {
        eprintln!("{TEMPLATE} not found: run pdfbuild from the repository root");
        return ExitCode::FAILURE;
    }

    let mut files = Vec::new();
    walk(&docs, &mut files);
    files.sort();

    let (mut built, mut failed) = (0, 0);
    for f in &files {
        if f.file_name().is_some_and(|n| n == "SUMMARY.md") {
            continue;
        }
        if is_alias(&fs::read_to_string(f).unwrap_or_default().replace("\r\n", "\n")) {
            continue;
        }
        let rel = f.strip_prefix(&docs).unwrap_or(f);
        let out = out_dir.join(rel).with_extension("pdf");
        if let Some(parent) = out.parent() {
            if let Err(e) = fs::create_dir_all(parent) {
                eprintln!("cannot create {}: {e}", parent.display());
                return ExitCode::FAILURE;
            }
        }
        // Typst wants a forward-slash path relative to --root.
        let input = f.to_string_lossy().replace('\\', "/");
        let status = Command::new(&typst)
            .args(["compile", "--root", ".", "--input"])
            .arg(format!("path={input}"))
            .arg(TEMPLATE)
            .arg(&out)
            .status();
        match status {
            Ok(s) if s.success() => built += 1,
            Ok(_) => {
                eprintln!("FAILED: {input}");
                failed += 1;
            }
            Err(e) => {
                eprintln!("cannot run `{typst}`: {e}. Install Typst or set TYPST.");
                return ExitCode::FAILURE;
            }
        }
    }

    println!("built {built} PDF(s) in {}, {failed} failed", out_dir.display());
    if failed == 0 {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

#[cfg(test)]
mod tests {
    use super::is_alias;

    #[test]
    fn alias_pages_are_detected_from_frontmatter_only() {
        assert!(is_alias("---\ntitle: X\ndoc_type: alias\n---\n\nbody\n"));
        assert!(!is_alias("---\ntitle: X\ndoc_type: checklist\n---\n\nbody\n"));
        assert!(!is_alias("---\ntitle: X\n---\n\ndoc_type: alias\n"), "body text is not frontmatter");
    }
}
