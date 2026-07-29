
<template>

    <PresentError ref="error"/>

    <div class="sidebar">
        <ol>
            <TreeElement v-for="root in tree" :node="root" :depth=0 />
        </ol>
    </div>

    


</template>

<script setup lang="ts">
    import { get_class_instance_descendants } from '@/api/class_instance';
    import TreeElement from './TreeElement.vue';
    import PresentError from './PresentError.vue';
    import { ref } from 'vue';

    const error = ref();
    const tree = ref();

    


    async function setup() {

        
        try {
            let result = await get_class_instance_descendants(null)
            tree.value = result;
            
        } catch (e) {
            error.value.showError(e)

        }
        
    }


    setup()

    

    

    


</script>

<style lang="scss" scoped>

@use "/public/import";

.sidebar {
    background-color: import.$white;
    position: relative;
    height: 100%;
    width: 150px;

    overflow-y: auto;
    overflow-clip-margin: -10px;
    
    
    //margin-bottom: 100px;

    // overflow-clip-margin: 10px;
}

ol {
    margin: 0px;
    margin-left: 0px;
    margin-right: 0px;
    list-style-type: none;
    padding-left: 0px;
    width: 100%;
    height: 100%;

}



</style>