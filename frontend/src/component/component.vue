<script setup>
import { onMounted, ref } from 'vue';
import { useRoute } from 'vue-router';

let output = ref(1)
let component = ref({
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


async function setup() {
  console.log(id)


  const requestOptions = {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({
      i: Number(id)
    })
  };
  const response = await fetch(import.meta.env.VITE_API_URL+"/post_id_get_component", requestOptions)
  //const response = await fetch(import.meta.env.VITE_API_URL+"/post_build", requestOptions);
  component.value = await response.json();
  //this.postId = data.id;

}

setup()

</script>

<template>
  what the what the hell???!!

  {{ output }}

  <input type="number" v-model="output">

  <button @click="get_message">click!</button>
  <br>

  {{ component.name }}

  <br>

  {{ component.size }}


</template>

<style>

</style> 