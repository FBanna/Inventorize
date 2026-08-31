<template>

    <div class="window">

        <div class="controls">

            <button class="button" @click="add_origin">Add Origin</button>


        </div>

        <div class="table">

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
import { get_all_labels } from '@/api/label.ts';
import { get_all_manufacturers } from '@/api/manufacturer.ts';
import { CellTypes, type CellData, type TableState } from '../components/table/TableTypes.ts';
import Table from '../components/table/Table.vue';
import { get_all_origins } from '@/api/origin.ts';

const table = useTemplateRef("table")

function add_origin() {
    setActivePopup(Popups.AddOrigin, null, () => {
        table.value?.reset()
    })
}


// Table Functionality

let raw_field_data = null

async function search_function(state: TableState): Promise<Array<any>> {

    let res: any = await get_all_origins()

    return res

}

function transform_function(row: any): CellData[] {

    return [{
        type: CellTypes.String,
        value: row.name
    }, {
        type: CellTypes.String,
        value: row.url
    }]
}

async function column_function(): Promise<Array<any>> {
    return [["Name", "Url"]]
}

function get_id(row: any): any {
    return row.origin_id
}

function row_click(row: any) {
    console.log("origin clicked!")
}

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