<template>

    <tr :class="{selected: props.state.select.selected.find((a: any) => a === props.get_id(props.row_data))}">

      <td v-for="col in props.transform_row_data(props.row_data)">{{ col }}</td>

        <!-- <td>IMAGE</td>

        <td v-for="field in props.fields.core">{{props.rowData[field]}}</td>

        <template v-for="class_ in props.fields.attributes">

          <td v-for="field in class_.fields">


            {{ props.rowData.attributes.find(
                (a: any) => a.class_instance_id === class_.class_instance_id
            ).attributes[field.name] }} 
          </td>

        </template> -->

    </tr>

</template>


<script setup lang="ts">
import type { active } from '@/app/popup/popup_state';
import type { IDGetterFunction, TableState, TransformRowDataFunction } from './Table.vue';

const props = defineProps<{
  row_data: any,
  state: TableState,
  get_id: IDGetterFunction,
  transform_row_data: TransformRowDataFunction,
}>()
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