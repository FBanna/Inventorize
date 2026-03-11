#import "lib.typ": *
#import "@preview/fletcher:0.5.8": diagram, node, edge

#set page(height: 500pt, width: 900pt)



#place(center + horizon)[



#diagram(
  debug: 0,
  spacing: 80pt,
  edge-stroke: 0.75pt+black,
  node-outset: -5pt,


  node((1,-0.5), stroke: rgb("#396bac"),
    fill: rgb("#f6f8ff"), [*Inventorize ERD*]),
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
      )
    )
  ),
  node((1,0), name: <origin>,
    erd_table(
      name: "Origin",
      P_key: ("C_ID","integer"),
      rows: (
        ("origin", "string"),
        ("part_number", "string"),
        
      )
    )
  ),
  node((0,1), name: <type_attributes>,
    erd_table(
      name: "Type Attributes",
      P_key: ("TYPE_ID","integer"),
      rows: (
        ("attributes", "json"),
        ("component_schema", "json"),
        ("prompts", "json")
      )
    )
  ),
  // node((2,0.5), name: <types>,
    // diagram(
    //   debug: 0,
    //   spacing: 30pt,
    //   edge-stroke: 0.75pt+black,
    //   node-outset: -5pt,
    // 
  

  node((2,0.5), name: <smd>,

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
  node((2,1.6), name: <capacitor>,

    erd_table(
      name: "Capacitors",
      P_key: ("C_ID", "integer"),
      rows: (
        ("capacitance", "integer"),
        ("voltage", "integer")
      )
    )
  ),
  node((2,0.2), name: <attribute_example_label>, [*Example User Defined Attributes*]),
  node(enclose: (<attribute_example_label>, <smd>, <resistor>, <capacitor>),
    outset: 0pt,
    stroke: rgb("#396bac"),
    fill: rgb("#f6f8ff"),
    snap: -1,
    name: <attribute_example>,
  ),
  edge(<component>, <origin>, "-n!"),
  // edge(<component>, (1.5,0.75),(1.5,0), <smd>, "-1?"),
  // edge(<component>, (1.5,0.75),(1.5,1), <resistor>, "-1?"),
  edge(<component>, (1.45,0.75),(1.45,0.9), <attribute_example>, "-1?"),
  edge(<type>, <type_attributes>, "-1?"),
  edge(<component>,<component_type>, "-n!")

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

