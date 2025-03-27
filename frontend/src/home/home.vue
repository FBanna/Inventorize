<script setup>
  import { ref } from 'vue';

  let output = ref("hi")

  async function get_message(){
    let response = await fetch(import.meta.env.VITE_API_URL)
    let json = await response.json()
    output.value = json.message

  }

</script>

<template>
  
  <div class="search-container">
    <div class="search-scroll">

      <div class="search-field">
        hello
        <br>
        <input placeholder="Search" class="search">

        <div class="results"></div>
      
      </div>

    </div>
  </div>


</template>

<style lang="scss">

@use "../../public/import";


.search-container{
  width: 100%;
  height: 200px;
  background-color: import.$secondary;
  margin-top: 0px;
  box-sizing: border-box;
  padding: 5px;
}

.search-scroll {
  white-space: nowrap;
  
  height: 100%;
  overflow-y: hidden;
  overflow-x: scroll;
}

.search-field {
  height: calc(100% - 5px);
  box-sizing: border-box;
  padding: 5px;
  width: 200px;
  background-color: white;
  margin-right: 5px;

  display: inline-block;
  font-weight: bolder;
  
}

.search{
  box-sizing: border-box;
  width: 100%;
  border-radius: 4px;
  border-width: 1px;
  border-style: solid;
}


</style> 