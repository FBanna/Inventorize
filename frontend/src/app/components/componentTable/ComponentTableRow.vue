<template>

    <tr :class="{selected: props.state.selected.selected.find((a: any) => a === props.rowData.component_id)}">

        <td>IMAGE</td>

        <td v-for="field in props.fields.core">{{props.rowData[field]}}</td>

        <template v-for="class_ in props.fields.attributes">

          <td v-for="field in class_.fields">


            {{ props.rowData.attributes.find(
                (a: any) => a.class_instance_id === class_.class_instance_id
            ).attributes[field.name] }} 
          </td>

        </template>

    </tr>

</template>


<script setup lang="ts">
import type { active } from '@/app/popup/popup_state';


const props = defineProps(["rowData", "fields", "state"])
let uuid = props.rowData.uuid

defineExpose({
    uuid
})

</script>


<style lang="scss" scoped>

@use "@/style/import";

td {
  //text-align: center;
  padding-right: 15px;
  border-bottom: 1px solid rgb(145, 145, 145);
  height: 40px;
}

tr:nth-child(even) {background-color: #f2f2f2;}

.selected {
  background-color: import.$secondary !important;
}

</style>