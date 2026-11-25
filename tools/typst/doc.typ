// Renders one Markdown document from docs/ as a PDF.
//
//   typst compile --root . --input path=docs/mop/mop-template.md tools/typst/doc.typ out.pdf
//
// The Markdown stays the single source. This file only adds page layout,
// the DRAFT status header, and the disclaimer footer.

#import "@preview/cmarker:0.1.6"

#let path = sys.inputs.at("path", default: "docs/DISCLAIMER.md")
#let source = read("/" + path).replace("\r\n", "\n")

// Split the frontmatter from the body.
#let m = source.match(regex("(?s)^---\n(.*?)\n---\n(.*)$"))
#let front = if m == none { "" } else { m.captures.at(0) }
#let body = if m == none { source } else { m.captures.at(1) }

#let field(name) = {
  let f = front.match(regex("(?m)^" + name + ":\s*\"?([^\"\n]*)\"?\s*$"))
  if f == none { "" } else { f.captures.at(0) }
}
#let title = field("title")
#let status = field("status")

#set document(title: title)
#set page(
  paper: "us-letter",
  margin: (x: 0.7in, top: 1in, bottom: 1in),
  header: text(size: 8pt, fill: luma(90))[
    #title #h(1fr) #strong(status)
  ],
  footer: text(size: 7.5pt, fill: luma(90))[
    Template only. Not for use without site-specific review by qualified persons.
    No warranty. See DISCLAIMER in the repository.
    #h(1fr) #context counter(page).display("1 / 1", both: true)
  ],
)
#set text(font: ("Libertinus Serif", "DejaVu Sans Mono"), size: 10pt)
#set par(justify: false)
#show heading.where(level: 1): set text(size: 16pt)
#show heading.where(level: 2): set text(size: 12pt)
#show heading: set block(above: 1.2em, below: 0.7em)
#show link: it => it.body
#show quote: it => block(
  fill: luma(240), inset: 8pt, radius: 2pt, width: 100%, it.body,
)

// Tables carry sign-off columns, so keep them compact and ruled.
#show table: set text(size: 8pt)
#set table(stroke: 0.4pt + luma(120), inset: 4pt)
#show table.cell.where(y: 0): set text(weight: "bold")

// Checklist items print as empty boxes to tick by hand.
#let prepared = body.replace("- [ ] ", "- ☐ ")

#cmarker.render(prepared)
