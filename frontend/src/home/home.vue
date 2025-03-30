<script setup>
  import { ref } from 'vue';

  
  // let test = JSON.parse(
  //   '[{"name": "Error","size": "Error","value": "Error OHM","info": null,"stock": 0,"origin": null,"url": null}]'
  // )

  let components = ref()

  async function search_components(){

    let response = await fetch(import.meta.env.VITE_API_URL + "/get_all_component")
    //let response = await fetch(import.meta.env.VITE_API_URL)
    let json = await response.json()
    components.value = json

  }
  search_components()

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

  <div class="search-results-container">

    <table>
      <tbody>
        <th table-heading>name</th>
        <th table-heading>size</th>
        <th table-heading>value</th>
        <th table-heading>info</th>
        <th table-heading>stock</th>
        <th table-heading>origin</th>
        <th table-heading>url</th>
      </tbody>

      <tbody v-for="c in components">
        <td>{{ c.name }}</td>
        <td>{{ c.size }}</td>
        <td>{{ c.value }}</td>
        <td>{{ c.info }}</td>
        <td>{{ c.stock }}</td>
        <td>{{ c.origin }}</td>
        <td>{{ c.url }}</td>
      </tbody>
    </table>
  </div>


</template>

<style lang="scss">

@use "../../public/import";

th[table-heading]{
  background-color: import.$text;
  color: import.$white !important;
  height: 35px;
  border: 1px import.$grey solid;
  
}

td{
  border: 1px import.$grey solid;
  height: 25px;
  padding: 3px;
}

table{
  border-collapse: collapse;
}

.search-container{
  width: 100%;
  height: 200px;
  background-color: import.$secondary;
  margin-top: 0px;
  box-sizing: border-box;
  padding: 5px;
}

.search-results-container{
  width: 100%;
  background-color: import.$white;
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