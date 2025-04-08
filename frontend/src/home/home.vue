<script setup>
  import { ref } from 'vue';

  
  // let test = JSON.parse(
  //   '[{"name": "Error","size": "Error","value": "Error OHM","info": null,"stock": 0,"origin": null,"url": null}]'
  // )

  let components = ref()

  //let search_fields = []

  let search_names = ["name", "size", "value", "info", "stock", "origin", "url", "label"]

  let c = ref({
    name: "",
    size: "",
    value: "",
    info: "",
    stock: 0,
    origin: "",
    url: "",
    label: ""
  })
  
  async function search_components(){

    console.log(search_values.value[0], search_values.value[1], search_values.value[2])

    const requestOptions = {
      method: "POST",
      headers: {"Content-Type": "application/json"},
      body: JSON.stringify(c.value)
    }

    let response = await fetch(import.meta.env.VITE_API_URL + "/post_search_component", requestOptions)
    let json = await response.json()
    components.value = json

  }

  async function get_all_components(){

    let response = await fetch(import.meta.env.VITE_API_URL + "/get_all_component")
    let json = await response.json()
    components.value = json

  }

  get_all_components()

  
</script>

<template>
  
  <div class="search-container">
    <div class="search-scroll">

      <div v-for="n in search_names.length" class="search-field">
        {{ search_names[n] }}

        {{ search_values[n] }}

        <br>
        <input @change="search_components()" placeholder="Search" v-model="search_values[n]"  class="search">
        

        <div class="results"></div>
      
      </div>

    </div>
  </div>

  <div class="search-results-container">

    <table>
      <thead>
        <tr>
          <th v-for="name in search_names" table-heading>
            {{ name }}
          </th>
          <!-- <th table-heading>name</th>
          <th table-heading>size</th>
          <th table-heading>value</th>
          <th table-heading>info</th>
          <th table-heading>stock</th>
          <th table-heading>origin</th>
          <th table-heading>url</th>
          <th table-heading>label</th> -->
        </tr>
      </thead>


      <tbody v-for="c in components">
        <tr onclick="window.location='google.com';">
          <td>{{ c.name }}</td>
          <td>{{ c.size }}</td>
          <td>{{ c.value }}</td>
          <td>{{ c.info }}</td>
          <td>{{ c.stock }}</td>
          <td>{{ c.origin }}</td>
          <td>{{ c.url }}</td>
          <td>{{ c.label }}</td>
        </tr>
        
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