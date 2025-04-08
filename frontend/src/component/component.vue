<script setup>
import { ref } from 'vue';

let output = ref(1)

async function get_message() {
  console.log(output.value)


  const requestOptions = {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({
      i: output.value
    })
  };
  const response = await fetch(import.meta.env.VITE_API_URL+"/post_build", requestOptions);
  //const data = await response.json();
  //this.postId = data.id;

}

</script>

<template>
  what the what the hell???!!

  {{ output }}

  <input type="number" v-model="output">

  <button @click="get_message">click!</button>


</template>

<style>

</style> 