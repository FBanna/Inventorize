<template>

    <tr class="row" :class="{selected: props.state.selected.selected.find((a: any) => a === props.rowData.component_id)}">

        <!-- <td>IMAGE</td> -->

        <td>{{ props.rowData.name }}</td>

        
        <td>

          <div class="field-container">

            <template v-for="field in props.rowData.fields">

              <div class="field">
                {{ field.name }} - {{ field.unit }} - {{ field.object_type }}
              </div>

            </template>

          </div>

          
          
        </td>

        

        <!-- <td v-for="field in props.fields.core">{{props.rowData[field]}}</td>

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


const props = defineProps(["rowData", "state"])

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


.row {
  min-height: 100px;
  box-sizing: border-box;
}
</style>