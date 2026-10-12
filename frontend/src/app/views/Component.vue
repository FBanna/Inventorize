
<template>

<ComponentSearch v-if="component != null" :uuid="component.class_instance_id" />

</template>

<script setup lang="ts">
import { get_component } from '@/api/component';
import { pushAppError } from '@/error/error_state';
import ComponentSearch from './ComponentSearch.vue';
import { ref, type Ref } from 'vue';
import { opts, Popups, setActivePopup } from '../popup/popup_state.ts';
import router from '../router/index.ts';
import { post_class_instance_id_get_class } from '@/api/class.ts';



const props = defineProps(["uuid"])
let component: Ref<any> = ref()


async function setup() {

    component.value = await get_component(props.uuid)

    setActivePopup(
        Popups.Component,
        {
            component: component.value
        },
        null,
    )

}

setup()

</script>


<style lang="scss" scoped>

</style>