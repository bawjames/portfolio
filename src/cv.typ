// Settings
#set text(size: 13pt)
#set page(margin: 4em)
#set terms(tight: false, indent: 1em, hanging-indent: 1em)
#set stack(spacing: 1em)
#set par(first-line-indent: (amount: 1em, all: true), hanging-indent: 1em)

// Show rules
#show heading: smallcaps
#show heading.where(level: 1): align.with(center)
#show title: smallcaps
#show strong: smallcaps
#show link: underline
#show link: text.with(fill: maroon.darken(50%))

// Let bindings
#let data = yaml("cv.yml")

#let contact = [
  / Email: #link("mailto:" + data.contact.email, data.contact.email)
  / Phone: #data.contact.phone
  / Github: #link(data.contact.github)
  / LinkedIn:
  / Updated: #datetime.today().display("[day] [month repr:short] [year]")
]

#context {
  stack(
    spacing: 1fr,
    dir: ltr,
    title(data.author) + [=== Curriculum Vitae],
    contact,
  )
}

#line(length: 100%)

#let employment = data.experience.map(item => {
  let end = if "end" in item {
    item.end
  } else [Present]

  heading(level: 2, item.where)
  [/ #item.what: #item.start - #end]
})

#let projects = data.projects.map(item => {
  heading(level: 2, item.what)
  item.desc
  linebreak()
  [/ More at: #link(item.link)]
})

#let interests = data.interests.map(item => {
  heading(level: 2, item.what)
  item.desc
})

#columns(2, gutter: 2em)[
  // Experience
  = Experience

  #box(align(center, smallcaps(
    data.languages.join(h(.5em))
  )))

  #stack(..employment)

  = Projects
  #stack(..projects)

  #colbreak()
  = Interests
  #stack(..interests)
]
