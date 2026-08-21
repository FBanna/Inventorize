<template>

    <div class="screen" v-if="active != null" @click="clear">

         <div v-on:click.stop> <!-- click stopper -->

            <Add_class_instance v-if="active == Popups.AddClassInstance" />
            <Add_class v-if="active == Popups.AddClass" />

        </div>

        

    </div>
    

</template>


<script lang="ts" setup>
    import Add_class_instance from './add_class_instance.vue';
    import { ref } from "vue";
    import {active, Popups, opts, clearActivePopup} from "@/app/popup/popup_state.ts";
import Add_class from './add_class.vue';

    function clear() {
        clearActivePopup()
    }


</script>


<style lang="scss" scoped>

    .screen {
        width: 100%;
        height: 100%;
        top: 0;
        position: absolute;
        background-color: rgba(175, 175, 175, 0.301);

        display: flex;
        justify-content: center;
        align-items: center;
    }

</style>