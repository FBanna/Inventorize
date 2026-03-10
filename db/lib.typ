



#let erd_table(

  name: text,
  P_key: (),
  rows: ()
  
  // (
  //   // {(name: text, 
  //   // type: text)}
  // )
  // 
  




) = {

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
  

  let P_key_mapped = (emoji.key + "" + P_key.at(0), P_key.at(1))
  
  // P_key.map(t => {
  //   (emoji.key + "" + t.at(0), t.at(1))
  // })

  // let F_key_mapped = F_key.map(t => {
  //   (emoji.key + "" + t.at(0), t.at(1))
  // })

  box(
    width: 200pt,
    stroke: 2pt+black,
    outset: 0pt,
    inset: 0pt,



    table(
      columns: (1fr,1fr),

      table.header(
        table.cell(colspan: 2)[
          #name
        ],
      ),
      
      ..P_key_mapped,
      ..rows.flatten()


    )



  )
  
  
  

  

}