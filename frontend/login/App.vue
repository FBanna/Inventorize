<script setup>
import { ref } from 'vue';

const login_api = ref(import.meta.env.VITE_API_URL + "/login")


function @KeyboardEvent.

</script>

<template>

  


    <div class="content page">

      <div>
        <img class="image" src="../public/logo.svg">
      </div>
      
      
      
      Login to Inventorize
      

      <br>


      <form :action=login_api  method="POST">

        <div>
          <input class="input" type="text" name="username" placeholder="Username" required />

          <br>

          <input class="input" type="password" name="password" placeholder="Password" required />

          <br>

          <button class="submit" type="submit">Login</button>
        </div>
        
      </form>
    </div>


  



</template>

<style lang="scss">

@import "../public/colours.scss";

.submit {
  background-color: $accent;
  color: white;
  border-style: hidden;
  border-radius: 1px;
  width: calc(90% + 6px);
  height: 25px;
  margin-top: 4px;

}


.page{
  position: absolute; 
  top: 0; 
  left: 0; 
  height: 100%; 
  width: 100%;
  font-family: Arial, Helvetica, sans-serif;
  font-weight: bolder;
  font-size: 30px;

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
  border-color: $primary;
  border-style: solid;
  border-width: 1px;
  border-radius: 1px;
  margin: 4px;
  height: 21px;
  width: 90%;
}

</style>
