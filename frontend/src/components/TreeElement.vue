
<template>

    <li :style="{'textIndent': get_depth()}"> {{ props.node.name}}</li>

    <tree-element v-for="child_node in props.node.children" :node="child_node" :depth="props.depth+1" />

</template>

<script setup lang="ts">

    const props = defineProps(["node", "depth"])


    function get_depth() {
        return ((props.depth * 20).toString() + "px")
    }

</script>

<style lang="sass" scoped>



</style>