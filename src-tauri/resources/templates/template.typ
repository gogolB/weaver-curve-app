#import sys: inputs

#set page(
  paper: "a4",
  margin: 2cm
)

#set text(
  font: "Tex Gyre Cursor",
  size: 11pt
)

#let content = inputs.v

#align(center, text(17pt)[
  *Weaver Curve*
])

= Demographics
- *Gender:* #content.gender
- *Age:* #content.age months
#if content.premature_weeks != none [
  - *Premature Conception:* #content.premature_weeks weeks and #content.premature_days days
  - *Corrected Age:* #content.corrected_age months
] else if content.premature_days != none [
  - *Premature Conception:* #content.premature_days days
  - *Corrected Age:* #content.corrected_age months
]

= Clinical
- *Child Head Circumference:* #content.head_circumference cm
- *Mother Head Circumference:* #content.mother_head_circumference cm
- *Father Head Circumference:* #content.father_head_circumference cm

== Scores
#if content.premature_weeks != none [
  #table(
    columns: (1fr, auto, auto),
    inset: 10pt,
    align: horizon,
    table.header(
      [*Child Score*], [*Child Score (Corrected)*], [*Mother Score*], [*Father Score*], [*Parental Average*],
    ),
    content.child_score, content.corrected_child_score, content.mother_score, content.father_score, content.parental_average
  )
] else if content.premature_days != none [
  #table(
    columns: (1fr, auto, auto),
    inset: 10pt,
    align: horizon,
    table.header(
      [*Child Score*], [*Child Score (Corrected)*], [*Mother Score*], [*Father Score*], [*Parental Average*],
    ),
    content.child_score, content.corrected_child_score, content.mother_score, content.father_score, content.parental_average
  )
] else [
  #table(
    columns: (1fr, auto, auto),
    inset: 10pt,
    align: horizon,
    table.header(
      [*Child Score*], [*Mother Score*], [*Father Score*], [*Parental Average*],
    ),
    content.child_score, content.mother_score, content.father_score, content.parental_average
    
  )
]


== Weaver Diagram
#align(center, [
  #image(
    "./images/graph.png",
    width: 20cm
  )
])