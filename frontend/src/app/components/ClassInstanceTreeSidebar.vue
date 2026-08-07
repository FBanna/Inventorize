
<template>

    <!-- <PresentError ref="error"/> -->

    <div class="sidebar">
        <ol>

            <TreeElement ref="children" v-for="root in tree" :node="root" :depth=0 :onSuccess="setup" />

            <li class="end_add"><img class="add_img" @click="end_add" src="/images/add.svg"></li>

        </ol>

        <div class="controls">
            
            <button class="button" @click="collapse">Toggle collapsed</button>
        </div>
    </div>

    


</template>

<script setup lang="ts">
    import { get_class_instance_descendants } from '@/api/class_instance';
    import TreeElement from './TreeElement.vue';

    import { ref } from 'vue';
    import { pushAppError } from '@/error/error_state.ts';
    import { Popups, setActivePopup } from '@/app/popup/popup_state.ts';

    const tree = ref();
    const collapsed = ref(true);

    const children = ref();

    function end_add() {
        let opts = {
            class_instance_id: null,
            class_name: "Root"
        }

        setActivePopup(
            Popups.AddClassInstance,
            opts,
            async () => { // never do this again! NEVER
                await setup()
            }
        )
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


    async function setup() {

        try {
            let result = await get_class_instance_descendants(null)
            tree.value = result;
            
        } catch (e) {
            pushAppError(e)
            //error.value.showError(e)

        }
        
    }


    setup()



</script>

<style lang="scss" scoped>

@use "@/style/import";

.sidebar {
    background-color: import.$light_grey;

    display: flex;
    flex-shrink: 0;

    flex: 0 0 150px;
    width: 150px;
    height: 100%;

    flex-direction: column;
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

.end_add {
    box-sizing: border-box;
    height: 30px;
    padding: 0;
    margin: 0;
    width: 100%;
    display: flex;
    flex-direction: row;
    justify-content: flex-start;

}


.add_img {
    height: 90%;
    padding: 0;
    margin: auto;
}






</style>