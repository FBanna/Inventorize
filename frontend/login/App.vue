<script setup>
import { ref } from 'vue';


let username = ref("")
let password = ref("")
//let output = ref("hi")


async function login() {

  const data = { username: username.value, password: password.value }
  let response = await fetch(import.meta.env.VITE_API_URL + "/login", {
    method: "POST",
    body: JSON.stringify(
      data
    ),
    headers: {
      "Content-type": "application/json; charset=utf-8"
    }
  })

  let json = await response.json()
  output.value = json.result
}

</script>

<template>
  LOGIN PAGE???

  <br><br>


  username:
  <input v-model="username">

  password:
  <input v-model="password">




  {{ password }}

  <button @click="login">login</button>


</template>

<style>

</style>
