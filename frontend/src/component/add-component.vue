<script setup>
import { ref } from 'vue';

const add_component_api = ref(import.meta.env.VITE_API_URL + "/add_component")


</script>

<template>
  

  ADD COMPONENT
  
  <form :action="add_component_api" method="POST">

    <input class="input" type="text" name="name" placeholder="name" required>

    <br>

    
    <input class="input" type="text" name="size" placeholder="size" required>
    
    <br>

    
    <input class="input" type="text" name="value" placeholder="value" required>
    
    <br>

    
    <input class="input" type="text" name="info" placeholder="info" required>
    
    <br>

    
    <input class="input" type="number" name="stock" placeholder="stock" required>
    
    <br>

    
    <input class="input" type="text" name="origin" placeholder="origin" required>
    
    <br>

    
    <input class="input" type="text" name="url" placeholder="url" required>
    
    <br>

    <button class="submit" type="submit">Login</button>

  </form>


</template>

<style>

</style> 