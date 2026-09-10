
<template>

<ComponentSearch :uuid="component.class_instance_id" />

</template>

<script setup lang="ts">
import { get_component } from '@/api/component';
import { pushAppError } from '@/error/error_state';
import ComponentSearch from './ComponentSearch.vue';
import { ref, type Ref } from 'vue';
import { Popups, setActivePopup } from '../popup/popup_state.ts';



const props = defineProps(["uuid"])
let component: Ref<any> = ref()


async function setup() {

    try {
        component.value = await get_component(props.uuid)

        let opts = {
            component: component.value,
        }

        setActivePopup(
            Popups.Component,
            opts,
            null
        )
    } catch (e: any) {
        pushAppError(e)
    }
    
}

setup()

</script>


<style lang="scss" scoped>

</style>