<template>

    <div class="window">

        <div class="controls">

            <button class="button" @click="add_class">Add Class</button>


        </div>

        <div class="table">

            <ClassTable ref="table" />

        </div>

    </div>

    
    
</template>

<script setup lang="ts">
import { post_class } from '@/api/class.ts';
import ClassTable from '../components/classTable/ClassTable.vue';
import { Popups, setActivePopup } from '../popup/popup_state.ts';
import { useTemplateRef } from 'vue';

const table = useTemplateRef("table")

function add_class() {
    setActivePopup(Popups.AddClass, null, () => {
        table.value?.reset()
    })
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