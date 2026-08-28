export enum CellTypes {
  String = "string", 
  Slot = "slot"
}


export type CellData = {
  type: string,
  value: string
}



export type TableState = {
  page: TablePageQuery,
  has_next: boolean,
  select: Select

}


export type TablePageQuery = {
  page_pos: number,
  page_size: number,
}

type Select = {
  selected: Array<any>,
  inverted: Boolean,
  selecting: Boolean
}



// Function Defs
export type SearchFunction = (
  state: TableState
) => Promise<Array<any>>

export type TransformRowDataFunction = (row: any) => CellData[]

export type ColumnFunction = () => Promise<Array<Array<string>>>

export type IDGetterFunction = (row: any) => any    

export type RowClickFunction = (row: any) => void