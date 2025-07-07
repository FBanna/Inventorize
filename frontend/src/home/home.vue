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

  const selecting = ref(false)
  const selected = ref([])

  //let search_fields = []

  let search_names = ["name", "size", "value", "info", "stock", "origin", "label"]

  function row_click(c){

    if(selecting.value){

      let i = selected.value.indexOf(c.id)

      if (i == -1){
        selected.value.push(c.id)
      } else {
        selected.value.splice(i , 1)
      }

    } else {
      let route = router.resolve({ path: "/component/" + c.id })
      window.open(route.href)
    }

    
  }

  async function remove_component() {
    const requestOptions = {
      method: "POST",
      headers: {"Content-Type": "application/json"},
      body: JSON.stringify({
        i: selected.value
      })
    } 

    let response = await fetch(import.meta.env.VITE_API_URL + "api/post_id_remove_list_component", requestOptions)

    selected.value = []

    await search_components() // MIGHT BE WRONG!

    await get_all_prompt()
    
  }
  
  
  async function search_components(){

    const requestOptions = {
      method: "POST",
      headers: {"Content-Type": "application/json"},
      body: JSON.stringify(prompt_selected.value)
    }

    

    let response = await fetch(import.meta.env.VITE_API_URL + "api/post_search_component", requestOptions)
    let json = await response.json()
    components.value = json

  }

  async function get_all_components(){

    let response = await fetch(import.meta.env.VITE_API_URL + "api/get_all_component")
    let json = await response.json()
    components.value = json

  }

  async function get_all_prompt() {
    let response = await fetch(import.meta.env.VITE_API_URL + "api/get_all_prompt")
    let json = await response.json()

    console.log(json)

    prompts.value = json

  }

  function get_image_src(c) {
    return import.meta.env.VITE_API_URL + "data/" + c.id + "/full.png"
  }

  async function build_label_zip() {

    const requestOptions = {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        list: selected.value.map(function(str) {
          return Number(str)
        })

      })
    };

    await fetch(import.meta.env.VITE_API_URL+"api/post_build_zip", requestOptions)
      .then(async response => {


        if (response.status == 200) {

          

          let data = await response.bytes()

          // CHANGE FOR AN ACTUAL NAME
          const file = new File([data], 'output.zip', {
            type: 'application/zip',
          })

          const link = document.createElement('a')
          const url = URL.createObjectURL(file)

          link.href = url
          link.download = file.name
          document.body.appendChild(link)
          link.click()

          document.body.removeChild(link)
          window.URL.revokeObjectURL(url)

        } else if (response.status == 404) {

          console.log("error making labels!")

        }

      })
    }



  get_all_components()
  get_all_prompt()

  
</script>

<template>

  <span class="search-tools">
    <button class="button search-button" @click="search_components">Search</button>

    <br>

    <button v-if="selecting" class="button search-button" @click="build_label_zip">BUILD</button>
    <br>
    <button v-if="selecting" class="button search-button" @click="remove_component">DELETE</button>

    <input @click="selected = []" class="selector" type="checkbox" v-model="selecting" id="select_check">
    <label for="select_check"></label>

    
  </span>
  
  <span class="search-container">

    <span class="search-field" v-for="(prompt, index) of prompts">

      {{ prompt.name }}

      <br>
      <input type="text" v-model="prompt_search[index]" placeholder="Search" class="search">
      <br>

      <select v-model="prompt_selected[index]" multiple="multiple" class="results">
        <option class="result" v-for="result in prompt.prompts" v-show="(result[0].toLowerCase()).includes(prompt_search[index].toLowerCase())">
          {{ result[0] }}
        </option>
      </select>

      

    </span>

  </span>

  <div class="search-results-container">

      <table>
        <thead>
          <tr>

            <!-- <th table-heading v-if="selecting">select</th> -->


            <th table-heading>image</th>

            
            <th v-for="name in search_names" table-heading>
              {{ name }}
            </th>
          </tr>
        </thead>


        <tbody v-for="c in components">
          
          <tr @click="row_click(c)" v-bind:style="[selected.includes(c.id) ? {'background-color': 'rgba(0, 110, 255, 0.445)'} : {}]">

              <!-- <td v-if="selecting"><input type="checkbox" :value="c.id" v-model="selected"></td> -->

              <td><img v-if="c.image" class="thumbnail" :src=get_image_src(c)></td>
            
              <td style="width: 80px;">{{ c.name }}</td>
              <td style="width: 50px;">{{ c.size }}</td>
              <td style="width: 80px;">{{ c.value }}</td>
              <td style="width: 80px;">{{ c.info }}</td>
              <td style="width: 50px;">{{ c.stock }}</td>
              <td style="width: 80px;">{{ c.origin }}</td>
              <td style="width: 50px;">{{ c.label }}</td>
          </tr>
          
          
        </tbody>
      </table>

  </div>


</template>

<style scoped lang="scss">

@use "../../public/import";

input.selector[type=checkbox] {
  display: none;
}


input.selector[type=checkbox] + label {
  background-image: url("../../public/unselecting.svg");
  background-size: contain;
  height: 30px;
  width: 30px;
  margin: 1px;
  display: inline-block;
  
}

input.selector[type=checkbox]:checked + label{
  background-image: url("../../public/selecting.svg");
  background-size: contain;
  height: 30px;
  width: 30px;
  display: inline-block;
  margin: 1px;
}

th[table-heading]{
  background-color: import.$text;
  color: import.$white !important;
  height: 35px;
  border: 1px import.$grey solid;

  
}


td{
  border: 1px import.$grey solid;
  height: 35px;
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
  position: relative;
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

.search-tools label {
  position: absolute;
  bottom: 2px;
  right: 2px;
}

.search-button {
  width: 100%;
  height: 30px;
  margin-top: 2px;

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

.thumbnail {
  width: 100px;
}

</style> 