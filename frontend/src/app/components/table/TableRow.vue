<template>

    <tr :class="{selected: props.state.select.selected.find((a: any) => a === props.get_id(props.row_data))}">

      <td v-for="col in props.transform_row_data(props.row_data)">

        <template v-if="col.type == CellTypes.Slot">
          <slot :name="col.value" :row="props.row_data">EMPTY</slot>
        </template>

        <template class="normal-cell" v-if="col.type == CellTypes.String">
          {{ col.value }}
        </template>
        
      </td>

    </tr>

</template>


<script setup lang="ts">
import type { active } from '@/app/popup/popup_state';
import { CellTypes, type CellData, type IDGetterFunction, type TableState, type TransformRowDataFunction } from './TableTypes.ts';


const props = defineProps<{
  row_data: any,
  state: TableState,
  get_id: IDGetterFunction,
  transform_row_data: TransformRowDataFunction,
  slots: string[]
}>()
</script>


<style lang="scss" scoped>

@use "@/style/import";

td {
  //text-align: center;
  //padding-right: 15px;
  border-bottom: 1px solid rgb(145, 145, 145);
  height: 40px;
}

tr:nth-child(even) {background-color: #f2f2f2;}

.selected {
  background-color: import.$secondary !important;
}

.normal-cell {
  padding: 5px;
}


</style>