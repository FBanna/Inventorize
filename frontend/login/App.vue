<script setup>
import { ref } from 'vue';


let username = ref("")
let password = ref("")
//let output = ref("hi")


async function login() {

  const data = { username: "user", password: "password" }
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
  <input v-bind="username">

  password:
  <input v-bind="password">

  <button @click="login">login</button>


</template>

<style>

</style>
