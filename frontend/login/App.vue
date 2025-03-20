<script setup>
import { ref } from 'vue';

const login_api = ref(import.meta.env.VITE_API_URL + "/login")


</script>

<template>

  


    <div class="content page">
      
      <img src="../public/logox128.png">
      
      Login to Inventorize
      

      <br><br>

      <form :action=login_api  method="POST">
        <input type="text" name="username" required />

        <br>

        <input type="password" name="password" required />

        <br>

        <button class="submit" type="submit">Login</button>
      </form>
    </div>


  



</template>

<style lang="scss">

@import "../public/colours.scss";

.submit {
  background-color: $accent;
}


.page{
  position: absolute; 
  top: 0; 
  left: 0; 
  height: 100%; 
  width: 100%;

}

.content{
  display: grid;
  place-content: center;

  text-align: center;
  outline: 1px solid black;
}


</style>
