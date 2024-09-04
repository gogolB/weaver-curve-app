#import sys: inputs
#let content = inputs.v
#set page(
  paper: "a4",
  margin: 2cm,
  header: [
    #set text(8pt)
    #smallcaps[Weaver Curve App #content.version]
    #h(1fr) _Date Generated: #datetime.today().display("[weekday repr:long] [month repr:short] [day padding:zero], [year repr:full]")_
  ],
)

#set text(
  font: "Roboto",
  size: 11pt
)

#align(center, text(17pt)[
  *Weaver Curve*
])

= Demographics

#box(height: 38pt,
 columns(2, gutter: 11pt)[
   #set par(justify: true)
  - *Gender:* #content.gender
  - *DOB:* #content.dob
  - *Age:* #content.age months
  #if content.premature_weeks > 0 [
    - *Premature Conception:* #content.premature_weeks weeks and #content.premature_days days
    - *Corrected Age:* #content.corrected_age months
  ] else if content.premature_days > 0 [
    - *Premature Conception:* #content.premature_days days
    - *Corrected Age:* #content.corrected_age months
  ]
 ]
)

= Clinical
- *Child Head Circumference:* #content.head_circumference cm
- *Mother Head Circumference:* #content.mother_head_circumference cm
- *Father Head Circumference:* #content.father_head_circumference cm

== Scores

#let cs = float(content.child_score)
#let pa = float(content.parental_average)


#let cs_fill = if cs < pa - 2 {red.lighten(60%)} else if cs > pa + 2 {red.lighten(60%)} else {green.lighten(60%)}

#if content.premature_weeks > 0 [

  #let ccs = float(content.corrected_child_score)
  #let ccs_fill = if ccs < pa - 2 {red.lighten(60%)} else if ccs > pa + 2 {red.lighten(60%)} else {green.lighten(60%)}

  #table(
    columns: 5,
    inset: 10pt,
    align: horizon,
    table.header(
      [*Child Score*], [*Child Score (Corrected)*], [*Mother Score*], [*Father Score*], [*Parental Average*],
    ),
    table.cell(fill: cs_fill, [#cs]), table.cell(fill: ccs_fill, [#ccs]), content.mother_score, content.father_score, content.parental_average
  )
] else if content.premature_days > 0 [
  #let ccs = float(content.corrected_child_score)
  #let ccs_fill = if ccs < pa - 2 {red.lighten(60%)} else if ccs > pa + 2 {red.lighten(60%)} else {green.lighten(60%)}
  
  #table(
    columns: (1fr, auto, auto),
    inset: 10pt,
    align: horizon,
    table.header(
      [*Child Score*], [*Child Score (Corrected)*], [*Mother Score*], [*Father Score*], [*Parental Average*],
    ),
    table.cell(fill: cs_fill, [#cs]), table.cell(fill: ccs_fill, [#ccs]), content.mother_score, content.father_score, content.parental_average
  )
] else [
  #table(
    columns: (1fr, auto, auto),
    inset: 10pt,
    align: horizon,
    table.header(
      [*Child Score*], [*Mother Score*], [*Father Score*], [*Parental Average*],
    ),
    table.cell(fill: cs_fill, [#cs]),  content.mother_score, content.father_score, content.parental_average
    
  )
]


== Weaver Diagram
#align(center, [
  #image(
    "./images/graph.png",
    width: 20cm
  )
])