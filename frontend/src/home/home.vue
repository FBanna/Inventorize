<script setup>
  import { ref } from 'vue';
  import { useRouter } from 'vue-router'

  
  // let test = JSON.parse(
  //   '[{"name": "Error","size": "Error","value": "Error OHM","info": null,"stock": 0,"origin": null,"url": null}]'
  // )
  const router = useRouter()
  let components = ref()
  let prompts = ref()

  let prompt_search = ref(["","","","","",""])
  let prompt_selected = ref([[],[],[],[],[],[]])

  //let search_fields = []

  let search_names = ["name", "size", "value", "info", "stock", "origin", "label"]

  function navigate_to_component(c){
    console.log("HELLO")
    let route = router.resolve({ path: "/component/" + c.id })
    window.open(route.href)
  }
  
  
  async function search_components(){

    console.log(JSON.stringify(prompt_selected.value))

    const requestOptions = {
      method: "POST",
      headers: {"Content-Type": "application/json"},
      body: JSON.stringify(prompt_selected.value)
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

  async function get_all_prompt() {
    let response = await fetch(import.meta.env.VITE_API_URL + "/get_all_prompt")
    let json = await response.json()

    prompts.value = json

  }

  get_all_components()
  get_all_prompt()

  
</script>

<template>

  <span class="search-tools">
    <button class="button search-button" @click="search_components">Search</button>
  </span>
  
  <span class="search-container">

    <span class="search-field" v-for="(prompt, index) of prompts">

      {{ prompt.name }}

      <br>
      <input type="text" v-model="prompt_search[index]" placeholder="Search" class="search">
      <br>

      <select v-model="prompt_selected[index]" multiple="multiple" class="results">
        <option class="result" v-for="result in prompt.prompts" v-show="(result.toLowerCase()).includes(prompt_search[index].toLowerCase())">
          {{ result }}
        </option>
      </select>

      

    </span>


      <!-- <div class="search-field">
        
        name
        <br>
        <input type="text" @change="search_components()" placeholder="Search" v-model="c.name"  class="search">
        <div class="results">

          <div class="result" v-for="results of (prompts[1].prompts)">
            {{ results }} HELLO
          </div>
        
        </div>
      
      </div> -->

      <!-- <div class="search-field">
        
        label
        <br>
        <input type="text" @change="search_components()" placeholder="Search" v-model="c.label"  class="search">
        <div class="results">

        </div>
      
      </div> -->



  </span>

  <div class="search-results-container">

      <table>
        <thead>
          <tr>
            <th v-for="name in search_names" table-heading>
              {{ name }}
            </th>
          </tr>
        </thead>


        <tbody v-for="c in components">
          
          <tr @click="navigate_to_component(c)">
              <td>{{ c.name }}</td>
              <td>{{ c.size }}</td>
              <td>{{ c.value }}</td>
              <td>{{ c.info }}</td>
              <td>{{ c.stock }}</td>
              <td>{{ c.origin }}</td>
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



.search-results-container{
  width: 100%;
  background-color: import.$white;
  box-sizing: border-box;
  padding: 5px;
}

.search-container{

  height: 200px;
  background-color: import.$secondary;
  box-sizing: border-box;
  display: block;
  padding: 5px;

  white-space: nowrap;
  overflow-y: hidden;
  overflow-x: auto;
}

.search-tools {
  float: left;
  height: 200px;
  box-sizing: border-box;
  //display: inline-block;
  padding: 5px;
  width: 200px;
  background-color: import.$white;
  margin: 0;


  font-weight: bolder;
  
}

.search-button {
  width: 100%;
  height: 30px;

}


.search-field {

  height: 100%;
  box-sizing: border-box;
  display: inline-block;
  padding: 5px;
  width: 200px;
  background-color: white;
  margin-right: 5px;

  font-weight: bolder;
  
}

.search{
  box-sizing: border-box;
  width: 100%;
  border-radius: 4px;
  border-width: 1px;
  border-style: solid;
}

.results {

  margin-top: 3px;
  margin-bottom: 3px;
  width: 100%;
  height: calc(100% - 45px);
  box-sizing: border-box;
  border: none;
  outline: none;

  white-space: nowrap;
  overflow-y: auto;
  overflow-x: hidden;
  
  
}

.result {
  font-weight: normal;
  height: 15pt;

}

.link {
  text-decoration: none;
}

</style> 