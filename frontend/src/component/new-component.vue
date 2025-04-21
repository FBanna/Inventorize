<script setup>
import { ref } from 'vue';

const add_component_api = ref(import.meta.env.VITE_API_URL + "api/post_component")


</script>

<template>
  
  <div class="window">
    ADD COMPONENT
  
  <form :action="add_component_api" method="POST">

    <input class="input" type="text" name="name" placeholder="name" required>
    <br>
    
    <input class="input" type="text" name="size" placeholder="size">
    <br>
    
    <input class="input" type="text" name="value" placeholder="value">
    <br>

    <input class="input" type="text" name="info" placeholder="info">
    <br>

    <input class="input" type="number" name="stock" placeholder="stock" required>
    <br>

    <input class="input" type="text" name="origin" placeholder="origin">
    <br>

    <input class="input" type="text" name="label" placeholder="label">
    <br>

    <button class="submit" type="submit">Submit</button>

  </form>
  </div>
  


</template>

<style lang="scss">

@use "../../public/import";

.window{
  margin: 5px;
}

.input{
  box-sizing: border-box;
  width: 200px;
  border-radius: 4px;
  border-width: 1px;
  border-style: solid;
  margin-bottom: 5px;
}

.submit {
  background-color: import.$accent;
  color: white !important;
  border-style: hidden;
  border-radius: 1px;
  width: 100px;
  height: 25px;
}


</style> 