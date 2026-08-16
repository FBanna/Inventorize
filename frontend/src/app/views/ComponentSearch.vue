<!-- 
<template>


  <span class="search-tools">

    <button class="button search-button" @click="search_components">Search</button>

    <button v-if="selecting" class="button search-button" @click="build_label_zip">BUILD</button>

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



            <th table-heading>image</th>

            
            <th v-for="name in search_names" table-heading>
              {{ name }}
            </th>
          </tr>
        </thead>


        <tbody v-for="c in components">
          
          <tr @click="row_click(c)" @mouseenter="row_enter(c)" v-bind:style="[selected.includes(c.id) ? {'background-color': 'rgba(0, 110, 255, 0.445)'} : {}]">



              <td><img v-if="c.image" class="thumbnail" :src=get_image_src(c)></td>
            
              <td style="width: 80px;">{{ c.name }}</td>
              <td style="width: 50px;">{{ c.size }}</td>
              <td style="width: 80px;">{{ c.value }}</td>
              <td style="width: 80px;">{{ c.info }}</td>
              <td style="width: 50px;">{{ c.stock }}</td>
              <td style="width: 80px;">{{ c.manufacturer }}</td>
              <td style="width: 50px;">{{ c.label }}</td>
          </tr>
          
          
        </tbody>
      </table>

  </div>


</template>

 -->








<template>

  <div class="window">

        

    <div class="search-container">



      <span class="search-tools">

        <button class="button search-button" @click="search_components">Search</button>

        <!-- <button v-if="selecting" class="button search-button" @click="build_label_zip">BUILD</button>

        <button v-if="selecting" class="button search-button" @click="remove_component">DELETE</button>

        

        <input @click="selected = []" class="selector" type="checkbox" v-model="selecting" id="select_check">
        <label for="select_check"></label> -->
          
      </span>

      <span class="facet-container">

        

        <span class="facet-unit" v-for="unit of prompts">

          {{ unit.name }}

          <div class="facets">
            
            <span class="facet" v-for="(facets, key) in unit.facets">

              {{ key }}

              <!-- <input type="text"> -->

            <select v-model=search[unit.class_instance_id].facets[key] @change="get_facets" multiple class="facet-selector">
                  <option class="result" :value="facet.value" v-for="facet in facets">
                    {{ facet.value }} - {{ facet.count }}
                  </option>
                </select>

              <br>
            </span>
          </div>
        </span>


      </span>


    </div>




    <!-- <div class="search-results-container">

      <table>


        <thead>
          <tr>

            <th table-heading>image</th>

            
            <th v-for="name in search_names" table-heading>
              {{ name }}
            </th>
          </tr>
        </thead>

        <tbody v-for="c in components">
          
          <tr @click="row_click(c)" @mouseenter="row_enter(c)" v-bind:style="[selected.includes(c.id) ? {'background-color': 'rgba(0, 110, 255, 0.445)'} : {}]">



              <td><img v-if="c.image" class="thumbnail" :src=get_image_src(c)></td>
            
              <td style="width: 80px;">{{ c.name }}</td>
              <td style="width: 50px;">{{ c.size }}</td>
              <td style="width: 80px;">{{ c.value }}</td>
              <td style="width: 80px;">{{ c.info }}</td>
              <td style="width: 50px;">{{ c.stock }}</td>
              <td style="width: 80px;">{{ c.manufacturer }}</td>
              <td style="width: 50px;">{{ c.label }}</td>
          </tr>
          
          
        </tbody>
      </table>

    </div> -->



  </div>

</template>




<script setup lang="ts">
import { post_class_instance_id_get_class } from '@/api/class';
import { post_search_get_component_with_attributes, post_search_get_facets } from '@/api/search';
import { pushAppError } from '@/error/error_state';
import { ref } from 'vue';

    const props = defineProps(["uuid"])

    const results = ref()
    const prompts = ref()
    const search = ref<any>({})
    const class_ = ref()



    async function search_components() {

        try {
            let res = await post_search_get_component_with_attributes(
                props.uuid,
                Object.values(search.value)
            )

            results.value = res

        } catch(e) {
            pushAppError(e)
        }
    }

    async function get_facets() {

      try {

          let res = await post_search_get_facets(
              props.uuid,
              Object.values(search.value)
          )

          prompts.value = res
      } catch(e) {
          pushAppError(e)
      }

    }

    async function get_class() {

      try {
        let res = await post_class_instance_id_get_class(
          props.uuid
        )

        class_.value = res
      } catch(e) {
          pushAppError(e)
      }
    }



    
    async function setup() {
        await search_components()
        await get_facets()
        initialiseSearch()
        
        await get_class()
        
    }

    function initialiseSearch() {
      for (const unit of prompts.value) {

        search.value[unit.class_instance_id] = {
          class_instance_id: unit.class_instance_id,
          facets: {}
        }

        for (const key of Object.keys(unit.facets)) {
          search.value[unit.class_instance_id].facets[key] = []
        }
      }
    }


    setup()

</script>

<style lang="scss" scoped>

    @use "@/style/import";

    .window {
        background-color: rgba(255, 0, 0, 0.068);
        width: 100%;
        height: 100%;
        min-width: 0;
    }

    .search-container {
      width: 100%;
      height: 200px;
      display: flex;
      flex-direction: row;
      flex-shrink: 0;

      background-color: import.$white;

    }

    .facet-container {

      height: 100%;
      flex: 1;
      min-width: 0;

      //background-color: import.$secondary;
      box-sizing: border-box;
      //display: block;
      padding: 5px;

      white-space: nowrap;
      overflow-y: hidden;
      overflow-x: auto;
    }

    .facet-unit {

      background-color: import.$secondary;

      height: 100%;
      box-sizing: border-box;
      display: inline-block;
      padding: 5px;
      //width: 200px;
      width: fit-content;
      //background-color: white;
      margin-right: 5px;

      font-weight: bolder;

      border: 1px black solid;



    }

    // .facets {
    //   border: 1px red solid;
    //   // height: 100vh;
    // }

    .facet {



      height: 100%;
      box-sizing: border-box;
      display: inline-block;
      padding: 5px;
      width: 200px;
      //background-color: white;
      margin: 0 2px 0 2px;
      

      font-weight: 500;

      text-align: center;


      border: 1px black solid;
      border-radius: 3px;


    }

    .facet-selector {



      margin-top: 3px;
      margin-bottom: 3px;
      width: 100%;
      height: calc(100% - 45px);
      box-sizing: border-box;
      display: block;
      border: none;
      outline: none;

      white-space: nowrap;
      overflow-y: auto;
      overflow-x: hidden;
      
      
    }


    .search-tools {
      //position: relative;
      //float: left;



      box-sizing: border-box;
      //display: inline-block;
      padding: 5px;
      width: 200px;
      height: 100%;
      background-color: import.$white;
      margin: 0;

      font-weight: bolder;
      
    }

    

</style>