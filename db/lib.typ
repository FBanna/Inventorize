



#let erd_table(

  name: text,
  key: (),
  rows: ()
  
  // (
  //   // {(name: text, 
  //   // type: text)}
  // )
  // 
  




) = {


  set text(
    size: 14pt
  )



  set table(
    fill: (x,y) => {
      if calc.rem(y,2) == 0 {
        rgb("#b1d3ff")
      } else {
        rgb("#d9e8fc")
      }
    },

    stroke: (x,y) => {

      if y == 0 {
        (bottom: 1pt + black)
      }

    },
    align: (x,y) => {
      if y == 0 {
        center
      } else {
        left
      }

      
    }
  )

  



  // show table.cell: it => {
  //   if it.y == 1 {
  //     ta
  //   }
  // }
  // 
  

  //let key_mapped = (emoji.key + "" + key.at(0), key.at(1))
  
  // P_key.map(t => {
  //   (emoji.key + "" + t.at(0), t.at(1))
  // })

  // let F_key_mapped = F_key.map(t => {
  //   (emoji.key + "" + t.at(0), t.at(1))
  // })
  // 
  // 
  


 
  block(
    width: 300pt,
    stroke: 2pt+black,
    outset: 0pt,
    inset: 0pt,
    spacing: 0pt,



    table(
      columns: (1fr,1fr),
      

      table.header(
        table.cell(colspan: 2)[
          #name
        ],
      ),
      
      ..key.map(k => (emoji.key + "" + k.at(0), k.at(1))).flatten(),
      ..rows.flatten()

    )
  )
  
  
  

  

}