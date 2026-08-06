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

        <span class="search-tools">

            <button class="button search-button" @click="search_components">Search</button>

            <!-- <button v-if="selecting" class="button search-button" @click="build_label_zip">BUILD</button>

            <button v-if="selecting" class="button search-button" @click="remove_component">DELETE</button>

            

            <input @click="selected = []" class="selector" type="checkbox" v-model="selecting" id="select_check">
            <label for="select_check"></label> -->

            
        </span>

        <span class="facet-container">

        <span class="facet-unit" v-for="unit of prompts">

          <!-- {{ prompt.class_instance_id }} -->

          <span class="facet" v-for="(facets, key) in unit.facets">

            {{ key }}

            <!-- <input type="text"> -->

            <select multiple class="results">
                <option class="result" v-for="facet in facets">
                  {{ facet.value }} - {{ facet.count }}
                </option>
              </select>

            <br>

          </span>

            <!-- {{ prompt.name }}

            <br>
            <input type="text" v-model="prompt_search[index]" placeholder="Search" class="search">
            <br>

            <select v-model="prompt_selected[index]" multiple="multiple" class="results">
                <option class="result" v-for="result in prompt.prompts" v-show="(result[0].toLowerCase()).includes(prompt_search[index].toLowerCase())">
                {{ result[0] }}
                </option>
            </select> -->

            

            </span>

        </span>
        
        


    </div>

</template>




<script setup lang="ts">
import { post_search_get_component_with_attributes, post_search_get_facets } from '@/api/search';
import { pushAppError } from '@/error/error_state';
import { ref } from 'vue';

    const props = defineProps(["uuid"])

    const results = ref()
    const prompts = ref()



    async function search_components() {


        try {
            let res = await post_search_get_component_with_attributes(
                props.uuid,
                []
            )

            let rest = await post_search_get_facets(
                props.uuid,
                []
            )

            results.value = res
            prompts.value = rest
        } catch(e) {
            pushAppError(e)
        }
    }


    async function setup() {
        await search_components()
    }


    setup()

</script>

<style lang="scss" scoped>

    @use "@/style/import";

    .window {
        background-color: import.$primary;
        width: 100%;
        height: 100%;
    }

</style>