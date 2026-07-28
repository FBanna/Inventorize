
<template>

    <li class="node" :style="{'textIndent': get_depth()}">
        <img v-if="props.node.children.length != 0" class="drop_down" :isdropped="dropped" @click="drop()" src="/public/drop_down.svg">
        <div class="text" @click="drop()">
            {{props.node.name}}
        </div>
        <img class="add add_hide" src="/public/add.svg">
    </li>

    <tree-element v-if="dropped" v-for="child_node in props.node.children" :node="child_node" :depth="props.depth+1" />

</template>

<script setup lang="ts">
import { ref } from 'vue';

    const props = defineProps(["node", "depth"])
    const dropped = ref<boolean>(true)

    function get_depth() {
        return ((props.depth * 20).toString() + "px")
    }

    function drop() {
        dropped.value = !dropped.value
    }

    function should_drop() {
        return dropped.value && (props.node.children.length != 0)
    }



</script>

<style lang="scss" scoped>

@use "/public/import";


.text {
    vertical-align: middle;
    height: 100%;
    float: left;
    user-select: none;
}

.drop_down {
    height: 20px;
    vertical-align: middle;
    float: left;
}

.drop_down[isdropped='true'] {
    rotate: 90deg;
}

.node {
    height: 20px;
    width: 100%;
}

.add_hide {
    display: none;
}

.node:hover .add_hide {
    display: block;
}

.add {
    
    height: 20px;
    vertical-align: middle;
    float: right;
    margin-left: 10px;
    //position: fixed;
    
    // float: inline-end;
    //flex-direction: row;
    // float: right;
}





// .drop_down {
    
// }


</style>