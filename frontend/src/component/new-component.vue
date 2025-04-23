<script setup>
import { onMounted, useTemplateRef, ref } from 'vue';

// const add_component_api = ref(import.meta.env.VITE_API_URL + "api/post_component")


const image = useTemplateRef("image");
const datasheet = useTemplateRef("datasheet");


onMounted(() => {
  image.value.focus()
  datasheet.value.focus()
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
  image: null,
  datasheet: null
})

function updateImage() {

  var files = image.value.files;

  var reader = new FileReader();

  reader.addEventListener("load", function() {
    some_file.src = reader.result;
  })

  reader.readAsDataURL(files[0])

  var data = new FormData();
  data.append("file", files[0])

  c.value.image = data

  console.log(c.value.image, "ok ok ", files.body)

}

function updateDatasheet() {

  var files = datasheet.value.files;

  var data = new FormData();
  data.append("file", files[0])

  c.value.datasheet = data

}

async function submit() {

  if (c.value.name == "" || c.value.stock == null) {
    console.log("I AM MAD!")
    return
  }

  console.log(c.value.image)


  const requestOptions = {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({
      name: c.value.name,
      size: c.value.size,
      value: c.value.value,
      info: c.value.info,
      stock: c.value.stock,
      origin: c.value.origin,
      label: c.value.label,
      image: c.value.image.files[0].arrayBuffer(),
      datasheet: c.value.datasheet.files[0].arrayBuffer(),
    })
  };


  //const response = await fetch(import.meta.env.VITE_API_URL+"/post_id_get_component", requestOptions)
  fetch(import.meta.env.VITE_API_URL + "api/add_component", requestOptions)
    .then(response => {


      if(response.redirected) {
        window.location.href = response.url;

        error.value = "You should be redirected now"


      } else if (response.status == 401) {

        error.value = "incorrect login!"
      } else if (response.status == 500) {
        error.value = "internal server error!"
      }

    })



    // }).catch(error => {
    //   console.log("what" + error)
    // }) 
      
    

  }


</script>

<template>
  
  <div class="window">
    ADD COMPONENT
  
  <!-- <form :action="add_component_api" method="POST"> -->
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

    <input class="input" type="file" ref="image" @change="updateImage"  placeholder="image">
    <br>

    <input class="input" type="file" ref="datasheet" @change="updateDatasheet"  placeholder="datasheet">
    <br>



    <button class="submit" @click="submit">Submit</button>

  </div>
  


</template>

<style lang="scss">

@use "../../public/import";

.window{
  margin: 5px;
}

.input{
  box-sizing: border-box;
  width: 200px;
  border-radius: 4px;
  border-width: 1px;
  border-style: solid;
  margin-bottom: 5px;
}

.submit {
  background-color: import.$accent;
  color: white !important;
  border-style: hidden;
  border-radius: 1px;
  width: 100px;
  height: 25px;
}


</style> 