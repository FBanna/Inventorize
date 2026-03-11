#import "lib.typ": *
#import "@preview/fletcher:0.5.8": diagram, node, edge

#set page(height: 1000pt, width: 1000pt)



#place(center + horizon, dy: -100pt)[



#diagram(
  debug: 1,
  spacing: 80pt,
  edge-stroke: 0.75pt+black,
  node-outset: -5pt,

  node((1,0.75), name: <component>, 
    erd_table(
      name: "Components",
      P_key: ("ID","integer"),
      rows: (
        ("name", "string"),
        ("price", "integer"),
        ("stock", "integer"),
      )

    )
  ),
  node((1,1.75), name: <component_type>, 
    erd_table(
      name: "Component Types",
      P_key: ("C_ID","integer"),
      rows: (
        ("type", "integer")
      )

    )
  ),
  node((0,0), name: <type>,
    erd_table(
      name: "Types",
      P_key: ("ID","integer"),
      rows: (
        ("name", "string"),
        ("inherits", "integer"),
        ("has_attributes", "bool")
      )
    )
  ),
  node((1,0), name: <origin>,
    erd_table(
      name: "Origin",
      P_key: ("C_ID","integer"),
      rows: (
        ("origin", "string"),
        ("part_number", "string")
      )
    )
  ),
  node((0,1), name: <type_attributes>,
    erd_table(
      name: "Type Attributes",
      P_key: ("TYPE_ID","integer"),
      rows: (
        ("attributes", "json"),
        ("component_schema", "json")
      )
    )
  ),
  // node((2,0.5), name: <types>,
    // diagram(
    //   debug: 0,
    //   spacing: 30pt,
    //   edge-stroke: 0.75pt+black,
    //   node-outset: -5pt,

      node((2,0), name: <smd>,

        erd_table(
          name: "SMD",
          P_key: ("C_ID", "integer"),
          rows: (
            ("footprint", "string"),
          )
        )
      
      ),
      node((2,1), name: <resistor>,

        erd_table(
          name: "Resistors",
          P_key: ("C_ID", "integer"),
          rows: (
            ("resistance", "integer"),
            ("accuracy", "integer")
          )
        )
      
      ),
      node((2,2), name: <capacitor>,

        erd_table(
          name: "Capacitors",
          P_key: ("C_ID", "integer"),
          rows: (
            ("capacitance", "integer"),
            ("voltage", "integer")
          )
        )
      
      
  ),
  edge(<component>, <origin>, "-n!"),
  edge(<component>, (1.5,0.75),(1.5,0), <smd>, "-1?"),
  edge(<component>, (1.5,0.75),(1.5,1), <resistor>, "-1?"),
  edge(<component>, (1.5,0.75),(1.5,2), <capacitor>, "-1?"),
  edge(<type>, <type_attributes>, "-1?"),
  edge(<component>, <component_type>, "-n!")

  //fletcher.edge(<type.south>, (0,0.5), <component.west>, "-n")

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

