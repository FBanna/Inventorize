<template>
  
  <table class="table">

    <thead>
    
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

    </thead>


    <TableRow @click="row_click(entry)" :transform_row_data="props.transform_row_data" :row_data="entry" :get_id="props.get_id" :state="state" v-for="entry in rows"/>

    
  </table>

  <div v-if="props.limited_pages" class="page-controls-bottom">
      <div class="page-info">
        Page size of {{ state.page.page_size }}
      </div>

      <div class="page-movements">

        <button class="page-button" @click="page_start"><<</button>
        <button class="page-button" @click="page_minus"><</button>

        <button class="page-button" v-for="p_num in get_page_buttons()">{{ p_num + 1}}</button>

        <button class="page-button" @click="page_plus">></button>
        <button class="page-button" @click="page_end">>></button>

      </div>
    </div>
</template>

<script setup lang="ts">
import { ref, type Ref } from 'vue'
//import ComponentTableRow from './ComponentTableRow.vue';
import { post_search_get_component_with_attributes } from '@/api/search.ts';
import { pushAppError } from '@/error/error_state.ts';
import { get_fields_from_class_instance } from '@/api/class_instance.ts';
import TableRow from './TableRow.vue';


export type TableState = {
  page: TablePageQuery,
  page_count: number,
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
  row_click: RowClickFunction,
  limited_pages?: Boolean
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
    page: {
      
      page_pos: 0,
      page_size: 50
    },
    page_count: 2,
    select: {
      inverted: false,
      selected: [],
      selecting: false
    }
  }

const state: Ref<TableState> = ref(defaultState)


// Page functions


async function page_start() {

  state.value.page.page_pos = 0
  await search()
  

}

async function page_minus() {

  

}

async function page_set(num: any) {

}

async function page_plus() {

}

async function page_end() {

}

async function get_page_buttons(): Array<number> {


  let WINDOW_RADIUS = 2;



  let current = state.value.page.page_pos;

  let out: Array<number> = [];

  
    // If too small return
  if (state.value.page_count < (WINDOW_RADIUS * 2 + 1)) {

    for (let i = 0; i < state.value.page_count; i ++) {
      out.push(i)
    }

    return out

  }


  if (current - WINDOW_RADIUS < 0) {
    current = 0
  } else {
    current = current - WINDOW_RADIUS;
  }

  
  

  for (let i = 0; i < ((WINDOW_RADIUS * 2) + 1); i++) {
    

    if ( current == (state.value.page_count - 1)) {
      break;
    }

    out.push(current)
  
    current = current + 1;

  } 

  console.log(out)



  return out
  


}

// Click Function


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
    } catch(e: any) {
        pushAppError(e)
    }

}

async function get_columns_from_function() {

  try {
    let res = await props.get_column_groups()
    column_groups.value = res

  } catch(e: any) {
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
  width: 100%;
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


.page-controls-bottom {
  width: 100%;
  height: 30px;
}

.page-info {
  float: left;
  height: 100%;
}

.page-movements {
  float: right;
  height: 100%;
}

.page-button {
  border: 0;
  margin: 1px;
  height: 30px;
  width: 30px;
}

</style>