<script setup>
import { ref } from 'vue';
import { errorMessages } from 'vue/compiler-sfc';

const login_api = ref("http://localhost:3030/login_api")


const username = ref("")
const password = ref("")

async function login() {


  const requestOptions = {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({
      username: username.value,
      password: password.value
    })
  };


  //const response = await fetch(import.meta.env.VITE_API_URL+"/post_id_get_component", requestOptions)
  fetch("http://localhost:3030/login_api", requestOptions)
    .then(response => {


      if(response.redirected) {
        window.location.href = response.url;
      } else if (response.status == 401) {

        console.log("INCORRECT BITCH")
      }

    })



    // }).catch(error => {
    //   console.log("what" + error)
    // }) 
      
    

}

</script>

<template>



    <div class="content page">

      <div>
        <img class="image" src="../public/logo.svg">
      </div>
      
      
      
      Inventorize
      

      <br>

      <div style="width: 350px;">

        <!-- :action=login_api  method="POST" -->


        <input class="input" v-model="username" type="text" placeholder="Username" required />

        <br>

        <input class="input" v-model="password" type="password" placeholder="Password" required />

        <br>


        <button class="button submit" @click="login">Login</button>
        
        
      </div>
      

      
      
    </div>
    ERROR


  



</template>

<style lang="scss">

@use "../public/import.scss";

.submit {
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
  vertical-align: top;

  font-weight: bolder;
  font-size: 30px !important;
  

}

// .error {
//   //background-color: red;

//   position: absolute; 
//   top: 0; 
//   left: 0; 
//   height: 100%; 
//   width: 100%;


// }

.content{
  display: grid;
  place-content: center;

  text-align: center;
  //outline: 1px solid black;
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
