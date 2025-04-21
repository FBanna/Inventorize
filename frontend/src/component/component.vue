<script setup>
import { onMounted, ref } from 'vue';
import { useRoute } from 'vue-router';

let output = ref(1)
let c = ref({
  id: 0,
  name: "",
  size: "",
  value: "",
  info: "",
  stock: 0,
  origin: "",
  label: ""

})

let id = useRoute().params.id

const error = ref("")


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

async function build_label() {

  const requestOptions = {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({
      i: Number(id)
    })
  };

  await fetch(import.meta.env.VITE_API_URL+"api/post_build", requestOptions)
    .then(async response => {


      if (response.status == 200) {

        

        let data = await response.bytes()

        // CHANGE FOR AN ACTUAL NAME
        const file = new File([data], 'output.pdf', {
          type: 'application/pdf',
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

        error.value = "Error making label"

      }

    })




  
  
}

setup()

</script>

<template>
  <div class="info-box box">

    <h1 class="heading">
      {{ c.name }}
    </h1>
    
    <br>

    <table>
      <tbody>

        <tr>
          <td data-row class="title">size:</td>
          <td data-row class="info">{{ c.size }}</td>
        </tr>

        <tr>
          <td data-row class="title">value:</td>
          <td data-row class="info">{{ c.value }}</td>
        </tr>

        <tr>
          <td data-row class="title">info:</td>
          <td data-row class="info">{{ c.info }}</td>
        </tr>

        <tr>
          <td data-row class="title">origin:</td>
          <td data-row class="info">{{ c.origin }}</td>
        </tr>

        <tr>
          <td data-row class="title">label:</td>
          <td data-row class="info">{{ c.label }}</td>
        </tr>
      </tbody>
    </table>

  </div>

  <div class="actions-box box" @click="build_label">
    <button class="button build-button">Build Label</button>


    <br>
    {{ error }}
  </div>

  


</template>

<style lang="scss">
@use "../../public/import";

.info-box {
  

  height: 500px;
  width: 500px;
  float: left;
}

.heading {
  margin: 0px;
  width: min-content;
}

td[data-row] {
  border-style: none;
  border-color: none;
  border: none;
}

.title {
  font-weight: bold;
  width: 100px;
}

.info {

  margin-left: 100px;

}

.actions-box {
  
  width: auto;
}

.box {
  margin: 10px;
  padding: 10px;
  background-color: import.$secondary;
  border-radius: 10px;
  display: inline-block;
}

.build-button {
  width: 100px;
  height: 30px;
}



</style> 