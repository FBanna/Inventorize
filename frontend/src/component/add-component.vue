<script setup>
import { ref } from 'vue';

let output = ref("hi")

async function get_message(){
  let response = await fetch(import.meta.env.VITE_API_URL)
  let json = await response.json()
  output.value = json.message

}

</script>

<template>
  what the hell???!!

  {{ output }}

  <button @click="get_message">click!</button>


</template>

<style>

</style> 