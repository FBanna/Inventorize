#import "lib.typ": *
#import "@preview/fletcher:0.5.8"

#set page(height: 1000pt, width: 1000pt)



#place(center + horizon, dy: -100pt)[



#fletcher.diagram(
  debug: 0,
  spacing: 50pt,
  edge-stroke: 0.75pt+black,
  node-outset: -5pt,

  fletcher.node((1,0.5), name: <component>, 
    erd_table(
      name: "Components",
      P_key: ("ID","integer"),
      rows: (
        ("name", "string"),
        ("ho", "hsdf")
      )

    )
  ),
  fletcher.node((0,0), name: <type>,
    erd_table(
      name: "Types",
      P_key: ("ID","integer"),
      rows: (
        ("name", "string"),
        ("inherits", "integer")
      )
    )
  ),
  fletcher.node((2,0.5), name: <types>,
    fletcher.diagram(
      debug: 0,
      spacing: 30pt,
      edge-stroke: 0.75pt+black,
      node-outset: -5pt,

      fletcher.node((0,0), name: <smd>,

        erd_table(
          name: "SMD",
          P_key: ("ID", "integer"),
          rows: (
            ("footprint", "string"),
          )
        )
      
      ),
      fletcher.node((0,1), name: <resistor>,

        erd_table(
          name: "Resistors",
          P_key: ("ID", "integer"),
          rows: (
            ("resistance", "integer"),
            ("accuracy", "integer")
          )
        )
      
      ),
      fletcher.node((0,2), name: <capacitor>,

        erd_table(
          name: "Capacitors",
          P_key: ("ID", "integer"),
          rows: (
            ("capacitance", "integer"),
            ("voltage", "integer")
          )
        )
      
      )
    )
  ),
  fletcher.edge(<type.south>, (0,0.5), <component.west>, "-n")

)
]

// #place(
//   (center + horizon),
//   dy: -60pt,
//   erd_table(
//     name: "Components",
//     P_key: ("ID","integer"),
//     rows: (
//       ("name", "string"),
//       ("ho", "hsdf")
//     )

//   )
// )


// #place(
//   (center + horizon),
//   dy: -60pt,
//   dx: -300pt,
//   erd_table(
//     name: "Types",
//     P_key: ("ID","integer"),
//     rows: (
//       ("name", "string"),
//       ("ho", "hsdf")
//     )

//   )
// )

