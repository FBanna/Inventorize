#let diameter = 10mm
#let height = 30mm
#let width = 25mm
#let background = rgb("#fa5353");

#set page(
  width: width,
  height: height,
  margin: 0mm,
  fill: background
)

#align(top+right)[
  #rect(
    outset: -0.5mm,
    stroke: (thickness: 0.1mm, dash: "densely-dash-dotted"),
    inset: 1mm
  )[
    #circle(
      radius: diameter/2,
      stroke: 0.1mm + black
    )
  ]
]

