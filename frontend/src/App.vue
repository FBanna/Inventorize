<script setup>
import { RouterLink, RouterView } from 'vue-router'

</script>

<template>
  <header>
    <div id="nav">
      hi
      <nav>
        <RouterLink to="/">Home</RouterLink>
        <RouterLink to="/test">About</RouterLink>
      </nav>
    </div>
  </header>

  <RouterView />
</template>

<style>

  #nav {
    top: 0;
    left: 0;
    width: 100%;
    height: 40pt;
    background: red;
    margin: 0pt, 0pt, 0pt, 0pt;
  }

</style>