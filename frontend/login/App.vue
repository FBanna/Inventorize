<script setup>
import { ref } from 'vue';

const login_api = ref(import.meta.env.VITE_API_URL + "/login")

// async function login() {

//   const data = { username: username.value, password: password.value }
//   fetch(import.meta.env.VITE_API_URL + "/login", {
//     method: "POST",
//     body: JSON.stringify(
//       data
//     ),
//     headers: {
//       "Content-type": "application/json; charset=utf-8"
//     }
//   })
// }



</script>

<template>
  LOGIN PAGE???

  <br><br>

  <form :action=login_api  method="POST">
    <input type="text" name="username" required />
    <input type="password" name="password" required />
    <button type="submit">Login</button>
  </form>


</template>

<style>

</style>
