<script setup>

import { onMounted, useTemplateRef, ref, defineProps } from 'vue';
import { useRoute } from 'vue-router';


const props = defineProps({
    title: String,
    is_new: Boolean
})




let c = ref({
  id: 0,
  name: "",
  size: "",
  value: "",
  info: "",
  stock: null,
  origin: "",
  label: "",
})

let image = null
let datasheet = null



function updateImage($event) {
    const target = $event.target;
    if (!target || !target.files || target.files.length == 0) {
      image = null
      return
    }

    var reader = new FileReader();
    var fileByteArray = [];
    reader.readAsArrayBuffer(target.files[0]);

    reader.onloadend = (evt) => {
    if (evt.target.readyState === FileReader.DONE) {
      const arrayBuffer = evt.target.result,
        array = new Uint8Array(arrayBuffer);
      for (const a of array) {
        fileByteArray.push(a);
      }
      console.log(fileByteArray)
      image = fileByteArray
    }
  }
}


function updateDatasheet($event) {

  const target = $event.target;
    if (!target || !target.files || target.files.length == 0) {
      datasheet = null
      return
    }

    var reader = new FileReader();
    var fileByteArray = [];
    reader.readAsArrayBuffer(target.files[0]);

    reader.onloadend = (evt) => {
    if (evt.target.readyState === FileReader.DONE) {
      const arrayBuffer = evt.target.result,
        array = new Uint8Array(arrayBuffer);
      for (const a of array) {
        fileByteArray.push(a);
      }
      console.log(fileByteArray)
      datasheet = fileByteArray
    }
  }

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

  console.log(c.value.image)


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
          image: false,
          datasheet: false
        },
        image: image,
        datasheet: datasheet
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

  console.log(c.value.image)


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
        image: false,
        datasheet: false
      },
      image: c.value.image,
      datasheet: c.value.datasheet
      
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
      i: Number(useRoute().params.id)
    })
  };
  const response = await fetch(import.meta.env.VITE_API_URL+"api/post_id_get_component", requestOptions)

  c.value = await response.json();


  console.log(c.value);


}



if (!props.is_new) {

    setup();




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


    <span>

      <button class="button upload-button" onclick="imageupload.click()"> <img src="../../public/upload.svg" class="favicon-upload-button"></img> Image</button>

      <input id="imageupload" type="file" class="input" style="width: 291px;" @change="updateImage"  placeholder="image"/>

    </span>
    


    <span>
      <button class="button upload-button" onclick="datasheetupload.click()"><img src="../../public/upload.svg" class="favicon-upload-button"></img> Datasheet</button>

      <input id="datasheetupload" type="file" class="input" style="width: 291px;" @change="updateDatasheet" placeholder="datasheet">

    </span>

  </div>
  

  <div class="box">
    <button class="submit button" @click="submit">Submit</button>

  </div>

  


</template>

<style scoped lang="scss">

@use "../../public/import";


.input{
  box-sizing: border-box;
  width: 400px;
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