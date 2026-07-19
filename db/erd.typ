#import "lib.typ": *
#import "@preview/fletcher:0.5.8": diagram, node, edge

#set page(height: 600pt, width: 1200pt)

#set block(spacing: 0pt)

#place(center + horizon)[



#diagram(
  debug: 0,
  spacing: 40pt,
  edge-stroke: 1pt+black,
  node-outset: -5pt,


  node((
    1,0), 
    stroke: rgb("#396bac"),
    fill: rgb("#f6f8ff"), {

      set text(
        size: 25pt
      )
      [*Inventorize ERD*]
    }
  ),

  node((2,1), name: <component>, 
    erd_table(
      name: "component",
      key: (("component_id","integer"),),
      rows: (
        ("name", "string"),
        ("stock", "integer"),
        ("manufacturer", "string"),
        ("label", "string"),
        // ("image", "bool"),
        // ("datasheet", "bool")
        
      )

    )
  ),
  node((2,0), name: <origin>,
    erd_table(
      name: "origin",
      key: (("component_id","integer"),),
      rows: (
        ("origin", "string"),
        ("part_number", "string"),
        ("price", "integer"),
        
      )
    )
  ),



  node((1,1), name: <component_type>, 
    erd_table(
      name: "component_type",
      key: (
        ("component_id","integer"),
        ("type_instance_id", "integer")
      ),
      rows: (
        ("attributes", "json"),
      )

    )
  ), 

  node((1,2), name: <file>,

    erd_table(
      name: "component_file",
      key: (
        ("file_id", "UUID"),
        ("component_id", "integer")
      ),
      rows: (
        ("name", "string"),
        ("mime", "string"),
        
      )
    )
  
  ),
  node((2,2.4), name: <image>,

    erd_table(
      name: "component_image",
      key: (
        ("component_id", "integer"),
      ),
      rows: (
        ("full", "bytea"),
        ("thumb", "bytea")
        
      )
    )
  
  ),


  // node((1,0.5), name: <prompt>, 
  //   erd_table(
  //     name: "prompt",
  //     key: (
  //       //("component_id","integer"),
  //       ("type_id", "integer"),
  //     ),
  //     rows: (
  //       ("attribute", "string"),
  //       ("value", "string"),
  //       ("count", "integer")
  //     )

  //   )
  // ), 
  // 
  
  node((0,1), name: <type>,

    erd_table(
      name: "type",
      key:(("type_id", "integer"),),
      rows: (
        ("name", "string")
      )
    )
  
  ),


  node((0,0), name: <type_instance>,
    erd_table(
      name: "type_instance",
      key: (
        ("type_instance_id","integer"),
        ("type_id", "integer")  
      ),
      rows: (
        ("path", "ltree"),
      )
    )
  ),
  node((0,2), name: <type_attributes>,
    erd_table(
      name: "type_attribute",
      key: (("type_id","integer"),),
      rows: (
        ("fields", "json"),
        ("schema", "json"),
        //("prompts", "json")
      )
    )
  ),

  node((1,3), name: <error>,
    erd_table(
      name: "error",
      key: (("component_id", "integer"),),
      rows: (
        ("message ??", "string"),
      ),
      colour: rgb("#eaa65c")
    )

  ),
  // node((2,0.5), name: <types>,
    // diagram(
    //   debug: 0,
    //   spacing: 30pt,
    //   edge-stroke: 0.75pt+black,
    //   node-outset: -5pt,
    // 
  

  // node((2,0.5), name: <smd>,

  //   erd_table(
  //     name: "SMD",
  //     P_key: ("C_ID", "integer"),
  //     rows: (
  //       ("footprint", "string"),
  //     )
  //   )
  
  // ),
  // node((2,1), name: <resistor>,

  //   erd_table(
  //     name: "Resistors",
  //     P_key: ("C_ID", "integer"),
  //     rows: (
  //       ("resistance", "integer"),
  //       ("accuracy", "integer")
  //     )
  //   )
  
  // ),
  // node((2,1.6), name: <capacitor>,

  //   erd_table(
  //     name: "Capacitors",
  //     P_key: ("C_ID", "integer"),
  //     rows: (
  //       ("capacitance", "integer"),
  //       ("voltage", "integer")
  //     )
  //   )
  // ),
  // node((2,0.2), name: <attribute_example_label>, [*Example User Defined Attributes*]),
  // node(enclose: (<attribute_example_label>, <smd>, <resistor>, <capacitor>),
  //   outset: 0pt,
  //   stroke: rgb("#396bac"),
  //   fill: rgb("#f6f8ff"),
  //   snap: -1,
  //   name: <attribute_example>,
  // ),
  //edge(<type>, (0.5,0), (0.5, 0.5), <prompt>, "-n!"),
  edge(<type_instance>, (0.5, 0), (0.5, 1), <component_type>, "-n!"),
  edge(<type>, <type_instance>, "-n?"),
  edge(<component>, <origin>, "-n?"),
  edge(<component>, (1.7,1), (1.7,2), <file>, "-n?"), 
  edge(<component>, <image>, "-1?"),
  // edge(<component>, (1.5,0.75),(1.5,0), <smd>, "-1?"),
  // edge(<component>, (1.5,0.75),(1.5,1), <resistor>, "-1?"),
  //edge(<component>, (1.45,0.75),(1.45,0.9), <attribute_example>, "-n?"),
  edge(<type>, <type_attributes>, "-1?"),
  edge(<component>, (1.6,1.2), (1.5,1.2), (1.5,1), <component_type>, "-n!")

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

