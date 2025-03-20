<script setup>
import { RouterLink, RouterView } from 'vue-router'

</script>

<!-- https://www.realtimecolors.com/?colors=1b0d0d-faf5f5-e72932-8db9e1-2b3f90&fonts=Inter-Inter -->

<template>
  <header>
    <div id="nav">
      <nav>
        <RouterLink to="/">Home</RouterLink>
        <RouterLink to="/component">Component</RouterLink>
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
    
  }

  body { 
    margin: 0px;
  }


</style>