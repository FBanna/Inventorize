<script setup>
import { ref } from 'vue';

let output = ref("hi")

async function get_message(){


  const requestOptions = {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(1)
  };
  const response = await fetch(import.meta.env.VITE_API_URL+"/post_build", requestOptions);
  const data = await response.json();
  this.postId = data.id;

}

</script>

<template>
  what the hell???!!

  {{ output }}

  <button @click="get_message">click!</button>


</template>

<style>

</style> 