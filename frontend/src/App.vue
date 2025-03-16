<script setup>
import { ref } from 'vue';

let output = ref("hi")

async function get_message(){
  let response = await fetch(import.meta.env.VITE_API_URL)
  let json = await response.json()
  output.value = json.message

}

async function login() {
  let response = await fetch(import.meta.env.VITE_API_URL + "/login", {
    method: "POST",
    body: JSON.stringify({
      username: "use2r",
      password: "pass"
    }),
    headers: {
      "Content-type": "application/json; charset=UTF-8"
    }
  })

  let json = await response.json()
  output.value = json.result
}

</script>

<template>
  hello

  {{ output }}

  <button @click="get_message">click!</button>

  <button @click="login">login</button>



</template>

<style>

</style>
