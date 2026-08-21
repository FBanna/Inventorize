
<template>
  
  <table class="table">

      <thead>

        <tr>
          <th class="head">Name</th>
          <th class="head">Attributes</th>
        </tr>


      </thead>
      
      <tbody>
        <ClassTableRow @click="select_or_deselect(class_.class_id)" :row-data="class_" :state="state" v-for="class_ in data"/>

      </tbody>

      
  </table>
</template>

<script setup lang="ts">
import { ref, type Ref } from 'vue'

import { post_search_get_component_with_attributes } from '@/api/search.ts';
import { pushAppError } from '@/error/error_state.ts';
import { get_fields_from_class_instance } from '@/api/class_instance.ts';
import ClassTableRow from './ClassTableRow.vue';
import { get_all_classes } from '@/api/class';



const data: any = ref()
const fields: any = ref({})
//const props = defineProps(["search", "uuid"])
defineExpose({
  reset
})

type TableState = {
  pageNum: Number,
  pageSize: Number,
  selected: Selected

}

type Selected = {
  selected: Array<any>,
  inverted: Boolean
}

const defaultState: TableState = {
    pageNum: 0,
    pageSize: 5,
    selected: {
      inverted: false,
      selected: []
    }
  }

const state: Ref<TableState> = ref(defaultState)


function select_or_deselect(uuid: any) {

  let index = state.value.selected.selected.findIndex(a => a === uuid)

  if (index == -1) {
    state.value.selected.selected.push(uuid)
  } else {
    state.value.selected.selected.splice(index, 1)
  }

}


async function search() {

    try {
        let res = await get_all_classes()
        data.value = res

    } catch(e) {
        pushAppError(e)
    }
}

async function reset() {

  //await get_fields()
  await search()
  state.value = defaultState
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