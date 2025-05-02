<script setup>
import { onMounted, useTemplateRef, ref } from 'vue';

// const add_component_api = ref(import.meta.env.VITE_API_URL + "api/post_component")


const image = useTemplateRef("image");
const datasheet = useTemplateRef("datasheet");




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



function updateImage($event) {
    const target = $event.target;
    if (!target || !target.files || target.files.length == 0) {
      c.value.image = null
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
      c.value.image = fileByteArray
    }
  }
}


function updateDatasheet($event) {

  const target = $event.target;
    if (!target || !target.files || target.files.length == 0) {
      c.value.datasheet = null
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
      c.value.datasheet = fileByteArray
    }
  }

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

        //error.value = "You should be redirected now"


      } else {
        console.log("ERROR")
      }
      //  else if (response.status == 401) {

      //   error.value = "incorrect login!"
      // } else if (response.status == 500) {
      //   error.value = "internal server error!"
      // }

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

    <input class="input" type="file" @change="updateImage"  placeholder="image">
    <br>

    <input class="input" type="file" @change="updateDatasheet"  placeholder="datasheet">
    <br>



    <button class="submit" @click="submit">Submit</button>

  </div>
  


</template>

<style scoped lang="scss">

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