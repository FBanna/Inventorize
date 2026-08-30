<template>

    <div class="screen" v-if="active != null" @click="clear">

         <div v-on:click.stop> <!-- click stopper -->

            <Add_class_instance v-if="active == Popups.AddClassInstance" />
            <Add_class v-if="active == Popups.AddClass" />
            <Add_component v-if="active == Popups.AddComponent" />
            <Add_label v-if="active == Popups.AddLabel" />
            <Add_manufacturer v-if="active == Popups.AddManufacturer" />
            <Add_origin v-if="active == Popups.AddOrigin" />

        </div>

        

    </div>
    

</template>


<script lang="ts" setup>
    import Add_class_instance from './add_class_instance.vue';
    import { ref } from "vue";
    import {active, Popups, opts, clearActivePopup} from "@/app/popup/popup_state.ts";
    import Add_class from './add_class.vue';
    import Add_component from './add_component.vue';
    import Add_label from './add_label.vue';
import Add_manufacturer from './add_manufacturer.vue';
import Add_origin from './add_origin.vue';

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