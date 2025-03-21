<script setup>
import { ref } from 'vue';

const login_api = ref(import.meta.env.VITE_API_URL + "/login")


</script>

<template>

  


    <div class="content page">

      <div>
        <img class="image" src="../public/logo.svg">
      </div>
      
      
      
      Login to Inventorize
      

      <br>

      <form :action=login_api  method="POST">
        <input class="input" type="text" name="username" required />

        <br>

        <input class="input" type="password" name="password" required />

        <br>

        <button class="submit" type="submit">Login</button>
      </form>
    </div>


  



</template>

<style lang="scss">

@import "../public/colours.scss";

.submit {
  background-color: $accent;
  border-style: hidden;
  width: 156px;
  border-radius: 3px;

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

.image {
  width: 80px;
  height: 80px;
}

.input {
  border-radius: 3px;
  border-color: $primary;
  border-style: solid;
  margin: 2px;
  width: 150px;
}


</style>
