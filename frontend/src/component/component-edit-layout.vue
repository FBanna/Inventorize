<script setup>

import { onMounted, useTemplateRef, ref, defineProps } from 'vue';
import { useRoute } from 'vue-router';


const props = defineProps({
    title: String,
    is_new: Boolean
})

let c_id = Number(useRoute().params.id)


let temp = ref(null)

let c = ref({
  id: 0,
  name: "",
  size: "",
  value: "",
  info: "",
  stock: null,
  origin: "",
  label: "",
  image: false,
  datasheet: false
})

// BOOLEANS
let change_image = ref(false)
let change_datasheet = ref(false)
let remove_image = ref(false)
let remove_datasheet = ref(false)

// DATA
let image_data = ref(null)
let datasheet_data = ref(null)

// PREVIEWS
let image_preview = ref(null)
let datasheet_preview = ref(null)



async function updateImage($event) {
  const target = $event.target;
  if (!target || !target.files || target.files.length == 0) {
    image_data.value = null
    return
  }



    const arrayBuffer = await target.files[0].arrayBuffer()

    const byteArray = new Uint8Array(arrayBuffer)

    image_data.value = Array.from(byteArray)

    image_preview.value = URL.createObjectURL(target.files[0])



    change_image.value = true
    remove_image.value = false
    //console.log(image_data.value)


}


async function updateDatasheet($event) {

  const target = $event.target;
  if (!target || !target.files || target.files.length == 0) {
    datasheet_value.value = null
    return
  }

  const arrayBuffer = await target.files[0].arrayBuffer()

  const byteArray = new Uint8Array(arrayBuffer)

  datasheet_data.value = Array.from(byteArray)

  datasheet_preview.value = URL.createObjectURL(target.files[0])



  change_datasheet.value = true
  remove_datasheet.value = false

  



  // change_image.value = true
  // console.log(image_data.value)

  //   var reader = new FileReader();
  //   var fileByteArray = [];
  //   reader.readAsArrayBuffer(target.files[0]);

  //   reader.onloadend = (evt) => {
  //   if (evt.target.readyState === FileReader.DONE) {
  //     const arrayBuffer = evt.target.result,
  //       array = new Uint8Array(arrayBuffer);
  //     for (const a of array) {
  //       fileByteArray.push(a);
  //     }



  //     change_datasheet.value = true
  //     datasheet_value.value = fileByteArray


  //   }
  // }

}

function keep_image(){


  if(remove_image.value){
    return false
  }

  if(!change_image.value && !c.value.image){
    return false
  }

  console.log("KEEPING")

  return true

}

function keep_datasheet(){

  if(remove_datasheet.value){
    return false
  }

  if(!change_datasheet.value && !c.value.datasheet){
    return false
  }

  return true

}

async function submit(){
    if(props.is_new){
        await submit_new()
    } else {
        await submit_update()
    }
}

async function submit_update() {

  if (c.value.name == "" || c.value.stock == null) {
    console.log("I AM MAD!")
    return
  }

  console.log(keep_image(), keep_datasheet())


  const requestOptions = {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({


      id: c.value.id,
      component: {
        component: {
          name: c.value.name,
          size: c.value.size,
          value: c.value.value,
          info: c.value.info,
          stock: c.value.stock,
          origin: c.value.origin,
          label: c.value.label,
          image: keep_image(),
          datasheet: keep_datasheet()
        },
        image: image_data.value,
        datasheet: datasheet_data.value
      }

      
      
    })
  };

  fetch(import.meta.env.VITE_API_URL + "api/post_update_component", requestOptions)
    .then(response => {


      if(response.status == 200) {
        window.location.href = "/";

      } else {
        console.log("ERROR")
    }


  })
}



async function submit_new() {

  if (c.value.name == "" || c.value.stock == null) {
    console.log("I AM MAD!")
    return
  }

  //console.log(c.value.image)


  const requestOptions = {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({

      component: {
        name: c.value.name,
        size: c.value.size,
        value: c.value.value,
        info: c.value.info,
        stock: c.value.stock,
        origin: c.value.origin,
        label: c.value.label,
        image: keep_image(),
        datasheet: keep_datasheet()
      },
      image: image_data.value,
      datasheet: datasheet_data.value
      
    })
  };

  fetch(import.meta.env.VITE_API_URL + "api/post_component", requestOptions)
    .then(response => {


      if(response.status == 200) {
        window.location.href = "/";

      } else {
        console.log("ERROR")
      }


    })
}


async function setup() {

  const requestOptions = {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({
      i: c_id
    })
  };
  const response = await fetch(import.meta.env.VITE_API_URL+"api/post_id_get_component", requestOptions)

  c.value = await response.json();


  console.log(c.value);


}



if (!props.is_new) {

    setup();

}




function get_datasheet_src() {
  return import.meta.env.VITE_API_URL + "data/" + c_id + "/datasheet.pdf"
}

function get_image_src() {
  return import.meta.env.VITE_API_URL + "data/" + c_id + "/full.png"
}


</script>


<template>


  <div class="box info-box">

    <h1 class="heading">
      {{ title }}
    </h1>



    <br>

    <input class="input" type="text" v-model="c.name" placeholder="name" required>
    <br>
    
    <input class="input" type="text" v-model="c.size" placeholder="size">
    <br>
    
    <input class="input" type="text" v-model="c.value" placeholder="value">
    <br>

    <input class="input" type="text" v-model="c.info" placeholder="info">
    <br>

    <input class="input" type="number" v-model="c.stock" placeholder="stock" required>
    <br>

    <input class="input" type="text" v-model="c.origin" placeholder="origin">
    <br>

    <input class="input" type="text" v-model="c.label" placeholder="label">
    <br>


    <!-- <span>

      <button class="button upload-button" onclick="imageupload.click()"> <img src="../../public/upload.svg" class="favicon-upload-button"></img> Image</button>

      <input id="imageupload" type="file" class="input" style="width: 291px;" @change="updateImage"  placeholder="image"/>

    </span>
    


    <span>

      <button class="button upload-button" onclick="datasheetupload.click()"><img src="../../public/upload.svg" class="favicon-upload-button"></img> Datasheet</button>

      <input id="datasheetupload" type="file" class="input" style="width: 291px;" @change="updateDatasheet" placeholder="datasheet">

    </span> -->

    <div class="file-upload-section">



      <div class="file-upload-box">

        <button class="button upload-button" onclick="imageupload.click()"> <img src="../../public/upload.svg" class="favicon-upload-button"></img> Image</button>

        <button class="button remove-button" @click="remove_image = true; change_image = false">X</button>


        <input id="imageupload" type="file" class="file-upload-input" @change="updateImage"  placeholder="image"/>


       <img v-if="!change_image && c.image && !remove_image" class="image" :src=get_image_src()>

       <img v-if="change_image && !remove_image" class="image" :src="image_preview">

       <div v-if="(!c.image && !change_image) || remove_image" class="image">NO IMAGE</div>


      </div>




      <div class="file-upload-box">

        <button class="button upload-button" onclick="datasheetupload.click()"><img src="../../public/upload.svg" class="favicon-upload-button"></img> Datasheet</button>

        <button class="button remove-button" @click="remove_datasheet = true; change_datasheet = false">X</button>

        <input id="datasheetupload" type="file" class="file-upload-input" @change="updateDatasheet" placeholder="datasheet">
        
        <iframe v-if="!change_datasheet && c.datasheet && !remove_datasheet" :src="get_datasheet_src()" width="100%" height="300px"></iframe>

        <iframe v-if="change_datasheet && !remove_datasheet" :src="datasheet_preview" width="100%" height="300px"></iframe>

        <div v-if="(!c.datasheet && !change_datasheet) || remove_datasheet"></div>
      </div>




    </div>
<!-- 
    <span>

      <button class="button upload-button" onclick="imageupload.click()"> <img src="../../public/upload.svg" class="favicon-upload-button"></img> Image</button>

      <input id="imageupload" type="file" class="input" style="width: 291px;" @change="updateImage"  placeholder="image"/>

    </span> -->


    

  </div>
  

  <div class="box">
    <button class="submit button" @click="submit">Submit</button>

  </div>

  


</template>

<style scoped lang="scss">

@use "../../public/import";


.remove_button{
  width: 10px;
}

.image {

  width: 80%;

  align-self: center;
  
}


.file-upload-box{
  
  float: left;
  width: calc(50% - 10px);
  margin: 5px;
  height: 100pt;
  //border: solid 1px red;
}

.file-upload-section{

}

.file-upload-input{

  visibility: hidden;

}


.input{
  box-sizing: border-box;
  width: 80%;
  border-radius: 4px;
  //border-width: 1px;
  border-style: hidden;
  margin-bottom: 5px;
  height: 25px;
  background-color: import.$white;
  align-content: center;
  padding-left: 4px;
  color: import.$grey !important;

  // background-color: black;
}

input:focus{
  outline: 2px solid import.$primary;
}


input[type=file]::file-selector-button {
  display: none;
}

.submit {
  width: 100px;
  height: 25px;
}


.info-box {
  
  height: 500px;
  width: 500px;
  float: left;
}


.heading {
  margin: 0px;
  width: max-content;
  font-size: 20px !important;
}

.upload-button {
  float: left;
  margin-right: 4px;
  height: 25px;
  width: 105px;
  border-radius: 4px;
  padding-left: 2px;
  display: flex;

  

}

.favicon-upload-button {
  height: 13px;
  align-self: center;
  float: left;
  margin-right: 4px;
  margin-left: 3px;
  box-sizing: border-box;
  position: relative;
}





</style> 