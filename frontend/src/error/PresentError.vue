<script setup lang="ts">
    import type { StatusError } from '@/api/util';
    import { ref } from 'vue';
import { errors, type QueuedError } from './error_state';

    //const props = defineProps(["error"])

    const props = defineProps<{
        error: QueuedError
    }>()

</script>

<template>

  <div class="error_box">

    
    <!-- <button class="close" @click="show = false">&#10006</button> -->

    <div class="message">
        {{ props.error.error.message }}
    </div>


    <div class="many" v-if="props.error.count != 1">x{{ props.error.count }}</div>

  </div>


</template>

<style scoped lang="scss">

@use "@/style/import";

.error_box {
    background-color: import.$primary;

    // position: absolute;
    // z-index: 100;

    width: 220px;
    height: 120px;

    border-radius: 10px;

    padding: 5px;

    margin: 5px;

    display: flex;
    flex-direction: column;
    //align-items: flex-end;

    // right: 10px;
    // bottom: 10px;


}

.close {
    position: absolute;
    top: 5px;
    right: 5px;
    

    background-color: rgba(255, 127, 80, 0);
    border: 0px;
}

.message {
    width: 100%;
    flex: 1;
}

.many {
    width: min-content;
    align-self: flex-end;

    margin-right: 0;


}


</style>
