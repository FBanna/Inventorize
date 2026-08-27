
<template>

  <div class="window">

        

    <div class="search-container">



      <span class="search-tools">

        <button class="button search-button" @click="table?.search">Search</button>
        <button class="button search-button" @click="add_component">Add Component</button>

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

            <select v-model=search[unit.class_instance_id].facets[key] @change="get_facets_and_update(unit.class_instance_id, key)" multiple class="facet-selector">
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

    <div class="results-container">

      <!-- <ComponentTable ref="table" :uuid="props.uuid" :search="search" /> -->

      <Table
      :get_search="search_function"
      :transform_row_data="transform_function"
      :get_column_groups="column_function"
      :row_click="row_click_function"
      :get_id="id_function"
      :limited_pages="true"
      
      
      ref="table"></Table>

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
import { get_fields_from_class_instance } from '@/api/class_instance';
import { post_search_get_component_with_attributes, post_search_get_component_with_attributes_paged, post_search_get_facets } from '@/api/search';
import { pushAppError } from '@/error/error_state';
import { onBeforeMount, ref, useTemplateRef, watch, type Ref } from 'vue';
import { onBeforeRouteUpdate, useRoute } from 'vue-router';
import router from '../router/index.ts';
import Table, { type TableState } from '../components/table/Table.vue';
import { Popups, setActivePopup } from '../popup/popup_state.ts';

  const route = useRoute();

    const props = defineProps(["uuid"])

    //const results = ref()         // components found
    const prompts: Ref<Array<any>> = ref<any>([])  // prompts found that populate facets
    const search = ref<any>({})   // the users searched request


    const table = useTemplateRef("table");


    
    

    async function get_facets() {

      try {

          let res = await post_search_get_facets(
              props.uuid,
              Object.values(search.value)
          )

          return res
      } catch(e: any) {
          pushAppError(e)
      }

    }


    async function get_facets_and_update(id: any, facet: any) {


      let saved: any = prompts.value.find((u: any) => u.class_instance_id == id).facets[facet]

      let res: any = await get_facets()

      if (res == null) {
        prompts.value = []
      } else {
        prompts.value = res
      }

      let index = prompts.value.findIndex((u: any) => u.class_instance_id == id)

      prompts.value[index].facets[facet] = saved

    }

    
    async function add_component() {     

       let opts = {
            class_instance_id: props.uuid,
        }

        setActivePopup(
            Popups.AddComponent,
            opts,
            async () => {
              await setup()
            }
        )

    }


    
    

    function initialiseSearch(prompts: any) {

      for (const unit of prompts) {

        search.value[unit.class_instance_id] = {
          class_instance_id: unit.class_instance_id,
          facets: {}
        }

        for (const key of Object.keys(unit.facets)) {
          search.value[unit.class_instance_id].facets[key] = []
        }
      }
    }

    async function setup() {
        //await search_components()



        await table.value?.reset()
        
        get_facets().then((res: any) => {

          if (res == null) {
            res = []
          }

          initialiseSearch(res)


          prompts.value = res
        })
        
    }


    // TABLE FUNCTIONS

    let raw_fields: any;

    async function search_function(state: TableState): Promise<Array<any>> {

        let res: any = await post_search_get_component_with_attributes_paged(
            props.uuid,
            state.page,
            Object.values(search.value),
        )

        state.has_next = res.has_next

        return res.results

    }

    function transform_function(row: any): Array<any> {

      let out = ["IMAGE"]

      for (let field of raw_fields.core) {
        out.push(row[field])
      }

      for (let class_ of raw_fields.attributes) {
        for (let field of class_.fields) {
          out.push(
            row.attributes.find(
              (a: any) => a.class_instance_id === class_.class_instance_id
            ).attributes[field.name]
          )
        }
      }

      return out
    }

    async function column_function(): Promise<Array<any>> {
      let res: any = await get_fields_from_class_instance(
        props.uuid
      )

      raw_fields = res

      let processed: Array<any> = [];

      let core = ["Image"]

      for (let field of res.core) {
        core.push(field)
      }

      processed.push(core)

      for (let class_ of res.attributes) {

        let class_group = []

        for (let field of class_.fields) {

          if (field.unit == "" || field == null) {          
            class_group.push(field.name)
          } else {
            class_group.push(field.name + " (" + field.unit + ")")
          }
        }

        processed.push(class_group)

      }

      return processed
    }
    
    function id_function(row: any): any {
      return row.component_id
    }

    function row_click_function(row: any) {
      console.log("click")
    }



    setup()

</script>

<style lang="scss" scoped>

    @use "@/style/import";

    .window {
        //background-color: rgba(255, 0, 0, 0.068);
        width: 100%;
        height: 100%;
        min-width: 0;
        display: flex;
        flex-direction: column;
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

    .results-container {
      overflow-y: scroll;
      overflow-x: hidden;
      flex: 1;

    }

    

    

</style>