<template>
  
  <table class="table">
    
      <tr>

        <template v-for="group in column_groups">

            <th class="head" v-for="col in group">{{ col }}</th>

        </template>

        <!-- <th class="head" v-></th>

        <th class="head">image</th>

        <th class="head" v-for="attr in fields.core">
          {{ attr }}
        </th>

        <template v-for="class_ in fields.attributes">

          
          
          <th class="head" v-for="attr in class_.fields">
            {{ attr.name}}
          </th>

        </template> -->

      </tr>


      <TableRow @click="row_click(entry)" :transform_row_data="props.transform_row_data" :row_data="entry" :get_id="props.get_id" :state="state" v-for="entry in rows"/>
  </table>
</template>

<script setup lang="ts">
import { ref, type Ref } from 'vue'
//import ComponentTableRow from './ComponentTableRow.vue';
import { post_search_get_component_with_attributes } from '@/api/search.ts';
import { pushAppError } from '@/error/error_state.ts';
import { get_fields_from_class_instance } from '@/api/class_instance.ts';
import TableRow from './TableRow.vue';


export type TableState = {
  pageNum: Number,
  pageSize: Number,
  select: Select

}

type Select = {
  selected: Array<any>,
  inverted: Boolean,
  selecting: Boolean
}



type SearchFunction = (
  state: TableState
) => Promise<Array<any>>

export type TransformRowDataFunction = (row: any) => Array<any>

type ColumnFunction = () => Promise<Array<Array<String>>>

export type IDGetterFunction = (row: any) => any    

type RowClickFunction = (row: any) => void


const column_groups: any = ref([])
const rows: any = ref([])



const props = defineProps<{
  get_search: SearchFunction,
  transform_row_data: TransformRowDataFunction,
  get_column_groups: ColumnFunction,
  get_id: IDGetterFunction,
  row_click: RowClickFunction
}>()

// const props = defineProps({
//     get_search: Search,
//     get_fields: String
// })


defineExpose({
  reset,
  search
})


const defaultState: TableState = {
    pageNum: 0,
    pageSize: 5,
    select: {
      inverted: false,
      selected: [],
      selecting: false
    }
  }

const state: Ref<TableState> = ref(defaultState)


function row_click(row: any) {


    if (state.value.select.selecting) {

        let id = props.get_id(row)

        let index = state.value.select.selected.findIndex(a => a === id)

        if (index == -1) {
            state.value.select.selected.push(id)
        } else {
            state.value.select.selected.splice(index, 1)
        }

    } else {

        props.row_click(row)

    }  

}


async function search() {
    
    try {
        let res = await props.get_search(state.value)

        rows.value = res
    } catch(e) {
        pushAppError(e)
    }

    // try {
    //     let res = await post_search_get_component_with_attributes(
    //         props.uuid,
    //         Object.values(props.search)
    //     )
    //     data.value = res

    // } catch(e) {
    //     pushAppError(e)
    // }
}

async function get_columns_from_function() {

  try {
    let res = await props.get_column_groups()
    column_groups.value = res

  } catch(e) {
      pushAppError(e)
  }
}

async function reset() {

  // data.value = []
  // fields.value = {}
  
  await get_columns_from_function()
  await search()
  state.value = structuredClone(defaultState)
}


async function setup() {
  await reset()
}

setup()



</script>

<style lang="scss" scoped>

@use "@/style/import";

.table{
  border-collapse: collapse;
  margin: 5px;
}

.head{
  //background-color: import.$text;
  color: import.$text !important;
  
  border-bottom: 1px import.$grey solid;
  //margin: 50px;
  //text-align: center;
  min-width: 50px;
  max-width: 150px;
  padding-left: 5px;
  padding-right: 50px;
  height: 35px;
  box-sizing: border-box;
  //padding: 50px;


  
}



// div[loop]{
//   border: 0px;
//   padding: 0px;
//   margin: 0px;
//   height: 35px;
//   background-color: antiquewhite;

// }

</style>