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
// Wide forms (a dozen columns on a portrait page) leave columns narrower
// than some single words; hyphenate rather than let a word run into the
// next cell.
#show table.cell: set text(hyphenate: true)

// Fill-in forms have empty cells, so their columns would shrink to the width
// of the headings and leave no room to write. Any table narrower than the
// page is stretched to full width, with each column's share based on its
// longest text. Tables that already fill the page are left as they are.
#let plain-length(c) = {
  if type(c) == str { c.len() }
  else if type(c) != content { 0 }
  // The writing-space box added below counts as content, so a cell that
  // has already been given one is not treated as empty again.
  else if c.func() == box { 1 }
  else if c.has("text") and type(c.text) == str { c.text.len() }
  else if c.has("children") { c.children.map(plain-length).sum(default: 0) }
  else if c.has("body") { plain-length(c.body) }
  else if c.has("child") { plain-length(c.child) }
  else { 0 }
}
// An empty cell is one to write in: give it a minimum height.
#let writing-space = box(width: 0pt, height: 13pt)
#let is-group(c) = c.func() == table.header or c.func() == table.footer
#show table: it => {
  let n = it.columns.len()
  let cells = it.children.map(c => if is-group(c) { c.children } else { (c,) }).flatten()
  let has-empty = cells.any(c => plain-length(c) == 0)
  let all-auto = it.columns.all(c => c == auto)
  // Nothing to do. This is also what stops the rule from applying forever to
  // the table it produces: that table has no empty cells, and if it was
  // stretched its columns are no longer automatic.
  if not has-empty and not all-auto { return it }

  let column = i => cells.enumerate()
    .filter(p => calc.rem(p.at(0), n) == i)
    .map(p => plain-length(p.at(1)))
  let shares = range(n).map(i => calc.clamp(column(i).fold(0, calc.max), 10, 40))
  // A column with nothing below its heading is the column to write in
  // (for example "Entry" beside a list of fields). Sizing it by its text
  // would make it the narrowest, so it gets the widest column's share.
  let widest = shares.fold(0, calc.max)
  let weights = range(n).map(i =>
    if column(i).slice(1).all(l => l == 0) { widest } else { shares.at(i) }
  )
  let filled = it.children.map(c =>
    if not is-group(c) and plain-length(c) == 0 { writing-space } else { c }
  )
  layout(size => {
    let stretch = all-auto and measure(it).width < size.width - 1pt
    if not stretch and not has-empty { it }
    else {
      table(columns: if stretch { weights.map(w => w * 1fr) } else { it.columns }, ..filled)
    }
  })
}

// Checklist items print as empty boxes to tick by hand.
#let prepared = body.replace("- [ ] ", "- ☐ ")

#cmarker.render(prepared)
