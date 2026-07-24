<script setup>
  import { ref } from 'vue';

  const show = ref(false)
  const message = ref()

  function showError(error) {
    show.value = true
    message.value = error

    setTimeout(() => {show.value = false}, 5000)
  }

  defineExpose({
    showError
  })


</script>

<template>

    <div class = "error_box" v-if="show">

        <!-- <button class="close" @click="show = false">&#10006</button> -->
        {{ message }}
    </div>


</template>

<style scoped lang="scss">

@use "../../public/import";

.error_box {
    background-color: import.$primary;

    position: absolute;
    z-index: 100;

    width: 220px;
    height: 120px;

    border-radius: 10px;

    padding: 5px;

    right: 10px;
    bottom: 10px;

    font-weight: bolder;


}

.close {
    position: absolute;
    top: 5px;
    right: 5px;

    background-color: rgba(255, 127, 80, 0);
    border: 0px;
}


</style>
