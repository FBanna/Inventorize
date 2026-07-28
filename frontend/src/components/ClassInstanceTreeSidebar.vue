
<template>

    <PresentError ref="error"/>

    <TreeElement v-for="root in tree" :node="root" :depth=0 />


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

<style lang="sass" scoped>



</style>