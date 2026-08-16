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
    0,0), 
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
      key: (
        ("component_id","uuid"),
        ("class_instance_id", "uuid")
      
      ),
      rows: (
        ("name", "text"),
        ("stock", "int"),
        ("manufacturer*", "text"),
        ("label", "uuid")
        
      )

    )
  ),
  node((2,0.1), name: <component_origin>,
    erd_table(
      name: "component_origin",
      key: (
        ("origin_id", "uuid"),
        ("component_id","uuid"),
        ),
      rows: (
        ("part_number*", "text"),
        ("price*", "numeric")
        
      )
    )
  ),

  node((1,0.099999), name: <origin>,
    erd_table(
      name: "Origin",
      key: (
        ("origin_id", "uuid"),
      ),
      rows: (
        ("name", "text"),
        ("url", "text"),
        ("hurl", "path")
      )
    )
  
  ),



  node((1,1), name: <component_class>, 
    erd_table(
      name: "component_class",
      key: (
        ("component_id","uuid"),
        ("class_instance_id", "uuid")
      ),
      rows: (
        ("attributes", "json"),
      )

    )
  ), 

  node((1,1.7), name: <file>,

    erd_table(
      name: "component_file",
      key: (
        ("file_id", "uuid"),
        ("component_id", "uuid")
      ),
      rows: (
        ("name", "text"),
        ("mime", "text"),
        
      )
    )
  
  ),
  node((1,2.4), name: <image>,

    erd_table(
      name: "component_image",
      key: (
        ("component_id", "uuid"),
      ),
      rows: (
        ("full", "bytea"),
        ("thumb", "bytea")
        
      )
    )
  
  ),
  node((2,2), name: <label>,

    erd_table(
      name: "label",
      key: (
        ("label_id", "uuid"),
      ),
      rows: (
        ("name", "text"),
        ("path", "path"),
      )
    )
  
  ),


  // node((1,0.5), name: <prompt>, 
  //   erd_table(
  //     name: "prompt",
  //     key: (
  //       //("component_id","int"),
  //       ("class_id", "int"),
  //     ),
  //     rows: (
  //       ("attribute", "text"),
  //       ("value", "text"),
  //       ("count", "int")
  //     )

  //   )
  // ), 
  // 
  
  // node((0,1), name: <class>,

  //   erd_table(
  //     name: "class",
  //     key:(("class_id", "int"),),
  //     rows: (
        
  //     )
  //   )
  
  // ),


  node((0,1), name: <class_instance>,
    erd_table(
      name: "class_instance",
      key: (
        ("class_instance_id","uuid"),
        ("class_id", "uuid"),
        ("parent**", "uuid")
      ),
      rows: (
        
      )
    )
  ),
  node((0,2), name: <class>,
    erd_table(
      name: "class",
      key: (("class_id","uuid"),),
      rows: (
        ("name", "text"),
        ("fields", "json"),
        ("schema", "json"),
        //("prompts", "json")
      )
    )
  ),

  node((0,3), name: <error>,
    erd_table(
      name: "error",
      key: (("component_id", "uuid"),),
      rows: (
        ("message ??", "text"),
      ),
      colour: rgb("#eaa65c")
    )

  ),
  // node((2,0.5), name: <classs>,
    // diagram(
    //   debug: 0,
    //   spacing: 30pt,
    //   edge-stroke: 0.75pt+black,
    //   node-outset: -5pt,
    // 
  

  // node((2,0.5), name: <smd>,

  //   erd_table(
  //     name: "SMD",
  //     P_key: ("C_ID", "int"),
  //     rows: (
  //       ("footprint", "text"),
  //     )
  //   )
  
  // ),
  // node((2,1), name: <resistor>,

  //   erd_table(
  //     name: "Resistors",
  //     P_key: ("C_ID", "int"),
  //     rows: (
  //       ("resistance", "int"),
  //       ("accuracy", "int")
  //     )
  //   )
  
  // ),
  // node((2,1.6), name: <capacitor>,

  //   erd_table(
  //     name: "Capacitors",
  //     P_key: ("C_ID", "int"),
  //     rows: (
  //       ("capacitance", "int"),
  //       ("voltage", "int")
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
  //edge(<class>, (0.5,0), (0.5, 0.5), <prompt>, "-n!"),
  edge(<class_instance>, <component_class>, "-n!"),
  edge(<class>, <class_instance>, "-n?"),
  edge(<component>, <component_origin>, "-n?"),
  edge(<component>, (1.5,1), (1.5,1.7), <file>, "-n?"), 
  edge(<component>, (1.53,1), (1.53,2.4), <image>, "-1?"),
  edge(<origin>, (1,0.1), <component_origin>, "-n?"),
  // edge(<component>, (1.5,0.75),(1.5,0), <smd>, "-1?"),
  // edge(<component>, (1.5,0.75),(1.5,1), <resistor>, "-1?"),
  //edge(<component>, (1.45,0.75),(1.45,0.9), <attribute_example>, "-n?"),
  //edge(<class>, <class_attributes>, "-1?"),
  edge(<component>, <component_class>, "-n!"),
  edge(<label>, <component>, "-n?")

  //fletcher.edge(<class.south>, (0,0.5), <component.west>, "-n")

)
]

// #place(
//   (center + horizon),
//   dy: -60pt,
//   erd_table(
//     name: "Components",
//     P_key: ("ID","int"),
//     rows: (
//       ("name", "text"),
//       ("ho", "hsdf")
//     )

//   )
// )


// #place(
//   (center + horizon),
//   dy: -60pt,
//   dx: -300pt,
//   erd_table(
//     name: "classs",
//     P_key: ("ID","int"),
//     rows: (
//       ("name", "text"),
//       ("ho", "hsdf")
//     )

//   )
// )

