<script setup>
import { ref } from 'vue';

const login_api = ref("http://localhost:3030/login_api")



</script>

<template>

  


    <div class="content page">

      <div>
        <img class="image" src="../public/logo.svg">
      </div>
      
      
      
      Inventorize
      

      <br>

      <div style="width: 350px;">

        <form :action=login_api  method="POST">

        <div >
          <input class="input" type="text" name="username" placeholder="Username" required />

          <br>

          <input class="input" type="password" name="password" placeholder="Password" required />

          <br>

          <button class="submit" type="submit">Login</button>
        </div>

        </form>

      </div>
      
    </div>


  



</template>

<style lang="scss">

@use "../public/import.scss";

.submit {
  background-color: import.$accent;
  color: white !important;
  border-style: hidden;
  border-radius: 1px;
  width: calc(90% + 6px);
  height: 34px;
  margin-top: 4px;
}


.page{
  position: absolute; 
  top: 0; 
  left: 0; 
  height: 100%; 
  width: 100%;

  font-weight: bolder;
  font-size: 30px !important;
  

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
  border-color: import.$primary;
  border-style: solid;
  border-width: 1px;
  border-radius: 1px;
  margin: 4px;
  height: 30px;
  width: 90%;
}

</style>
