<script setup>
import { onMounted, useTemplateRef, ref } from 'vue';
import { useRoute } from 'vue-router';

// const add_component_api = ref(import.meta.env.VITE_API_URL + "api/post_component")


const image = ref(null)
const datasheet = ref(null)

let update_image = false
let update_datasheet = false

let id = useRoute().params.id


let c = ref({
  id: 0,
  name: "",
  size: "",
  value: "",
  info: "",
  stock: null,
  origin: "",
  label: ""
})

async function setup() {

  const requestOptions = {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({
      i: Number(id)
    })
  };
  const response = await fetch(import.meta.env.VITE_API_URL+"api/post_id_get_component", requestOptions)
  //const response = await fetch(import.meta.env.VITE_API_URL+"/post_build", requestOptions);
  c.value = await response.json();
  //this.postId = data.id;

}



function updateImage($event) {
    const target = $event.target;
    if (!target || !target.files || target.files.length == 0) {
      image.value = null
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
      

      update_image = true;
      image.value = fileByteArray
    }
  }
}


function updateDatasheet($event) {

  const target = $event.target;
    if (!target || !target.files || target.files.length == 0) {
      datasheet.value = null
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
      update_datasheet = true;
      datasheet.value = fileByteArray
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


      id: Number(id),
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
        image: image.value,
        datasheet: datasheet.value
      }

      
      
    })
  };

  fetch(import.meta.env.VITE_API_URL + "api/post_update_component", requestOptions)
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

  setup()


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