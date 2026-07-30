<template>

    <div class="container">
        <PresentError v-for="error in errors" :error="error"/>
    </div>
    

</template>


<script lang="ts" setup>
    import { ref } from "vue";
    import PresentError from "./PresentError.vue";
    import {errors} from "@/error/error_state.ts"

</script>


<style lang="scss" scoped>


    .container {
        position: fixed;
        right: 0;
        bottom: 0;

        display: flex;
        flex-direction: column;

        gap: 0.5rem;

        z-index: 3;
    }

</style>