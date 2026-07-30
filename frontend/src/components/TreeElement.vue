
<template>

    <li class="node" :style="{'paddingLeft': get_depth()}">
        <img v-if="props.node.children.length != 0" class="drop_down" :isdropped="dropped" @click="drop()" src="/public/drop_down.svg">
        <div class="text" @click="drop()">
            {{props.node.name}}
        </div>
        <img class="add add_hide" @click="add" src="/public/add.svg">
    </li>

    <tree-element ref="children" v-if="dropped" v-for="child_node in props.node.children" :node="child_node" :depth="props.depth+1" :onSuccess="props.onSuccess" />

</template>

<script setup lang="ts">
import Add_class_instance from '@/app/popup/add_class_instance.vue';
import { Popups, setActivePopup } from '@/app/popup/popup_state';
import { ref } from 'vue';

    const props = defineProps(["node", "depth", "onSuccess"])
    const dropped = ref<boolean>(true)
    const children = ref();

    function get_depth() {
        return ((props.depth * 20).toString() + "px")
    }

    function drop() {
        dropped.value = !dropped.value
    }

    function should_drop() {
        return dropped.value && (props.node.children.length != 0)
    }

    function add() {
        
        let opts = {
            class_instance_id: props.node.class_instance_id,
            class_name: props.node.name
        }

        setActivePopup(
            Popups.AddClassInstance,
            opts,
            async () => { // never do this again! NEVER
                await props.onSuccess?.()
            }
        )

    }

    function collapse(value: boolean) {
        
        dropped.value = value

        if (children.value == null) {
            return
        }

        for (var child of children.value) {
            child.collapse(value)
        }
    }

    defineExpose({
        collapse
    })



</script>

<style lang="scss" scoped>

@use "/public/import";


.text {
    vertical-align: middle;
    height: 100%;
    user-select: none;
    padding: 0;
    margin: 0;
}

.drop_down {
    height: 100%;
    vertical-align: middle;
}

.drop_down[isdropped='true'] {
    rotate: 90deg;
}

.node {
    box-sizing: border-box;
    height: 20px;
    padding: 0;
    margin: 0;
    width: 100%;
    display: flex;
    flex-direction: row;
    justify-content: flex-start;
}

.add_hide {
    visibility: hidden;
}

.node:hover .add_hide {
    visibility: visible;
}

.add {
    
    height: 90%;

    padding: 0;
    margin: auto;

    margin-right: 5px;

}





// .drop_down {
    
// }


</style>