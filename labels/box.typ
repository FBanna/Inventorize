
#let height = 40mm
#let width = 60mm
#let background = rgb("#c1fdff");

//#import "@preview/tiaoma:0.3.0"
#import "@preview/tiaoma:0.3.0"

#set page(
  width: width,
  height: height,
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


      #align(bottom + right)[
        #tiaoma.qrcode(
          data.at("url"),
          width: 2cm,
        )
      ]

      
    ]

    

  ]

  
  
  


}


#for label in sys.inputs.at("labels", default: ((name: "name", size: "size", value: "value", info: "info", url: "https://typst.app"),)){
  template(data: label)

}
