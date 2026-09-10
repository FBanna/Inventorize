
<template>

<ComponentSearch v-if="component != null" :uuid="component.class_instance_id" />

</template>

<script setup lang="ts">
import { get_component } from '@/api/component';
import { pushAppError } from '@/error/error_state';
import ComponentSearch from './ComponentSearch.vue';
import { ref, type Ref } from 'vue';
import { Popups, setActivePopup } from '../popup/popup_state.ts';
import router from '../router/index.ts';
import { post_class_instance_id_get_class } from '@/api/class.ts';



const props = defineProps(["uuid"])
let component: Ref<any> = ref(null)


async function setup() {

    try {
        component.value = await get_component(props.uuid)
        let class_ = await post_class_instance_id_get_class(component.value.class_instance_id)

        let opts = {
            component: component.value,
            class_: class_
        }

        setActivePopup(
            Popups.Component,
            opts,
            null,
            () => {
                router.push("/" + component.value.class_instance_id)
            }
        )
    } catch (e: any) {
        pushAppError(e)
    }
    
}

setup()

</script>


<style lang="scss" scoped>

</style>