
<template>

    <PresentError ref="error"/>

    <div class="sidebar">
        <ol>
            <TreeElement ref="children" v-for="root in tree" :node="root" :depth=0 />
        </ol>

        <div class="controls">
            
            <button class="button" @click="collapse">Toggle collapsed</button>
        </div>
    </div>

    


</template>

<script setup lang="ts">
    import { get_class_instance_descendants } from '@/api/class_instance';
    import TreeElement from './TreeElement.vue';
    import PresentError from './PresentError.vue';
    import { ref } from 'vue';

    const error = ref();
    const tree = ref();
    const collapsed = ref(true);

    const children = ref();

    


    async function setup() {

        try {
            let result = await get_class_instance_descendants(null)
            tree.value = result;
            
        } catch (e) {
            error.value.showError(e)

        }
        
    }

    function collapse() {
        collapsed.value = !collapsed.value;

        if (children.value == null) {
            return;
        }


        for (var branch of children.value) {
            branch.collapse(collapsed.value)
        }
    }


    setup()

    

    

    


</script>

<style lang="scss" scoped>

@use "/public/import";

.sidebar {
    background-color: import.$white;
    position: fixed;
    top: 50px;
    bottom: 0px;
    left: 0px;
    width: 150px;
    display: flex;
    flex-direction: column;

    
    
    //margin-bottom: 100px;

    // overflow-clip-margin: 10px;
}

ol {
    display: flex;
    flex: 1;
    flex-direction: column;
    margin: 0px;
    list-style-type: none;
    padding-left: 0px;
    width: 100%;
    //height: 100%;
    overflow-y: auto;    
    min-height: 0;

}

.controls {
    //background-color: import.$primary;
    display: flex;
    flex-direction: column;

    border-top: 1px solid import.$grey;
    
    width: 100%;
    height: 100px;
    //flex: 0 0 100px;
}



</style>