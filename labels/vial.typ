#let diameter = 15mm
#let height = 40mm
#let width = 30mm
#let background = rgb("#ffffff");

#import "@preview/cades:0.3.0": qr-code

#let inputs = sys.inputs





#set page(
  width: width,
  height: height + diameter + 3mm,
  margin: 0mm,
  fill: background,
)





#set block(above: 0mm, below: 0mm)

#set text(size: 3mm, font: "Gabarito")

#set par(leading: 1mm)

#let template(data: ()) = {

  rect(
    stroke: (thickness: 0.5mm, dash: "densely-dash-dotted"),
    inset: 0mm,
    outset: 0mm,
    width: 100%,
    height: 100%
  )[
    #rect(
      width: width,
      height: height,
      outset: 0mm,
      stroke: none,
      //stroke: (thickness: 0.1mm, dash: "densely-dash-dotted"),
      inset: 2mm
    )[

      #set block(above: 1mm, below: 0mm)

      #data.at("name")\
      #data.at("size")\
      #data.at("value")\
      #data.at("info")


      #align(bottom+ right)[
        #qr-code(
          inputs.at("url", default: "https://typst.app"),
          width: 1cm,
          error-correction: "L",
          background: background
        )
      ]

      
    ]

    #line(end: (100%, 0mm))

    #rect(
      stroke: none,
      outset: 0mm,
      inset: 1mm,
      width: 100%,

      
    )[
      #align(horizon + center)[
        #circle(
            radius: diameter/2,
            stroke: 0.5mm + black,
            outset: 0mm,
          )[

            #text(size: 3mm)[
              #data.at("name")\
              #data.at("value")\

              //#sys.inputs.name
            ]
            
            
          ]
      ]
    ]

  ]

  
  
  


}


#for label in sys.inputs.at("labels", default: ((name: "name", size: "size", value: "value", info: "info", url: "https://typst.app"),)){
  template(data: label)

}
