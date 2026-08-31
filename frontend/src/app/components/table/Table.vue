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


    <TableRow @click="row_click(entry)" :transform_row_data="props.transform_row_data" :row_data="entry" :get_id="props.get_id" :state="state" :slots="props.slots" v-for="entry in rows" >
      
      <template #[slot]="slotted_props" v-for="slot in props.slots">
        <slot :name="slot" :row="slotted_props.row"></slot>
      </template>
      
    </TableRow>

    
  </table>

  <div v-if="props.limited_pages" class="page-controls-bottom">

      <div class="page-info">
        Page size of 
        <select class="page-size-dropdown" v-model.number="state.page.page_size" @change="page_start" >
          <option value=25>25</option>
          <option value=50>50</option>
          <option value=100>100</option>
          <option value=200>200</option>
        </select>
      </div>

      <div class="page-movements">

        <button class="page-button" @click="page_start"><<</button>
        <button class="page-button" @click="page_minus"><</button>

        <button class="page-button" :class="{ 'page-button-selected': p_num == state.page.page_pos }" v-for="p_num in get_page_buttons()" @click="page_set(p_num)">{{ p_num + 1}}</button>

        <button class="page-button" @click="page_plus">></button>

      </div>
    </div>
</template>

<script setup lang="ts">
import { ref, type Ref } from 'vue'
import { post_search_get_component_with_attributes } from '@/api/search.ts';
import { pushAppError } from '@/error/error_state.ts';
import { get_fields_from_class_instance } from '@/api/class_instance.ts';
import TableRow from './TableRow.vue';
import type { ColumnFunction, IDGetterFunction, RowClickFunction, SearchFunction, TableState, TransformRowDataFunction } from './TableTypes';




let WINDOW_RADIUS = 1;





// Data

const defaultState: TableState = {
    page: {
      
      page_pos: 0,
      page_size: 50
    },
    has_next: false,
    select: {
      inverted: false,
      selected: [],
      selecting: false
    }
  }

const state: Ref<TableState> = ref(defaultState)
const column_groups: Ref<any[]> = ref([])
const rows: Ref<any[]> = ref([])



const props = defineProps<{
  get_search: SearchFunction,
  transform_row_data: TransformRowDataFunction,
  get_column_groups: ColumnFunction,
  get_id: IDGetterFunction,
  row_click: RowClickFunction,
  limited_pages?: boolean,
  slots: string[]
}>()


defineExpose({
  reset,
  search
})


// Page functions


async function page_start() {

  state.value.page.page_pos = 0
  await search()
  

}

async function page_minus() {

  if (state.value.page.page_pos != 0) {
    state.value.page.page_pos -= 1
    await search()
  }

}

async function page_set(num: number) {
  state.value.page.page_pos = num
  await search()
}

async function page_plus() {

  if (state.value.has_next) {
    state.value.page.page_pos += 1
    await search()
  }

}

function get_page_buttons(): Array<number> {


  let current = state.value.page.page_pos;

  let out: Array<number> = [];


  if (current - WINDOW_RADIUS < 0) {
    current = 0
  } else {
    current = current - WINDOW_RADIUS
  }

  while (current <= state.value.page.page_pos) {

    out.push(current)
    current += 1

  }

  if (state.value.has_next) {
    out.push(current)
  }

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

.page-button-selected {
  background-color: import.$secondary;
}

.page-button {
  border: 0;
  margin: 1px;
  height: 30px;
  width: 30px;
}



</style>