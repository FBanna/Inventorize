<template>

    <div class="window">

        <div class="controls">

            <button class="button" @click="add_class">Add Class</button>


        </div>

        <div class="table">

            <!-- <ClassTable ref="table" /> -->

            <Table 
            :transform_row_data="transform_function"
            :row_click="row_click" 
            :get_id="get_id"
            :get_column_groups="column_function"
            :get_search="search_function"
            :slots="[]"
            ref="table"/>

        </div>

    </div>

    
    
</template>

<script setup lang="ts">
import { get_all_classes, post_class } from '@/api/class.ts';
import { Popups, setActivePopup } from '../popup/popup_state.ts';
import { useTemplateRef } from 'vue';
import type { TableState } from '../components/table/TableTypes.ts';
import Table from '../components/table/Table.vue';

const table = useTemplateRef("table")

function add_class() {
    setActivePopup(Popups.AddClass, null, () => {
        table.value?.reset()
    })
}


// Table Functionality

let raw_field_data = null

async function search_function(state: TableState): Promise<Array<any>> {

    let res: any = await get_all_classes()

    return res

}

function transform_function(row: any): Array<any> {

    let attributes = ""

    let fields: Array<any> = row['fields']
    

    for( var field of fields) {
        attributes = attributes + field.name + " "
    }

    return [row.name, attributes]
}

async function column_function(): Promise<Array<any>> {
    return [["name", "Attributes"]]
}

function get_id(row: any): any {
    return row.class_id
}

function row_click(row:any) {}

</script>


<style lang="scss" scoped>

    @use "@/style/import";


.controls {
    width: 100%;
    height: 150px;
    background-color: import.$light_grey;
}

.window {
    height: 100%;
    width: 100%;
    display: flex;
    flex-direction: column;

}

.table {
    height: 100%;
    width: 100%;
    overflow-y: scroll;
    overflow-x: hidden;
    flex: 1;
}


</style>