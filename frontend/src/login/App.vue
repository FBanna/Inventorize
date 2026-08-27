<script setup lang="ts">
import { login } from '@/api/login';
import { StatusError } from '@/api/util';
import { pushAppError } from '@/error/error_state';

import { ref } from 'vue';



  const username = ref("")
  const password = ref("")


  const urlParams = new URLSearchParams(window.location.search);
  const next = urlParams.get('next');
  
  async function do_login() {

    try {
      await login(
        username.value,
        password.value,
        next
      )
    } catch (e: any) {
      pushAppError(e)
    }
    
  }


</script>

<template>

  


  

  <div class="login_content login_page">

    <div>
      <img class="login_image" src="/images/logo.svg">
    </div>
    
    
    
    Inventorize
    

    <br>

    <div style="width: 350px;"> 

      <!-- :action=login_api  method="POST" -->


      <input class="login_input" v-model="username" type="text" placeholder="Username" required />

      <br>

      <input class="login_input" v-model="password" type="password" placeholder="Password" required />

      <br>


      <button class="button login_submit" @click="do_login">Login</button>

    
      
      
    </div>

      

      

      <!-- <div class="error">
        {{ error }}
      </div> -->
      
      
    </div>

    
    


  



</template>

<style scoped lang="scss">

@use "@/style/import.scss";

.login_submit {
  width: calc(90% + 6px);
  height: 34px;
  margin-top: 4px;
}


.login_page{
  position: absolute; 
  top: 0; 
  left: 0; 
  height: 100%; 
  width: 100%;
  vertical-align: top;

  font-weight: bolder;
  font-size: 30px !important;
  

}

// .error {

//   //background-color: red;

//   margin-left: auto;
//   margin-right: auto;

//   margin-top: 10px;

//   color: red !important;

// }

.login_content{
  display: grid;
  place-content: center;

  text-align: center;
  //outline: 1px solid black;
}

.login_image {
  width: 80px;
  height: 80px;
}

.login_input {
  border-color: import.$primary;
  border-style: solid;
  border-width: 1px;
  border-radius: 1px;
  margin: 4px;
  height: 30px;
  width: 90%;
}

</style>
