<script setup>
import { RouterLink, RouterView } from 'vue-router'

</script>

<!-- https://www.realtimecolors.com/?colors=1b0d0d-faf5f5-e72932-8db9e1-2b3f90&fonts=Inter-Inter -->

<template>
  <header>
    <div class="nav">
      <nav>

        

        <RouterLink class="logo_button" to="/"><img class="image" src="../public/logo.svg"></RouterLink>
        <RouterLink class="nav_buttons" to="/addcomponent">Add</RouterLink>
        <RouterLink class="nav_buttons" to="/component">build</RouterLink>





        <div class="right_side">

          <a class="nav_buttons" href="/logout">Logout</a>

        </div>

        

      </nav>
    </div>
  </header>

  <RouterView />
</template>

<style lang="scss">

@use "../public/import";

  .nav {
    z-index: 10;
    position: relative;
    top: 0;
    left: 0;
    width: 100%;
    height: 40pt;
    background: import.$white;
    box-sizing: border-box;
    padding: 5px;
    border-bottom: 1px solid import.$white;
    box-shadow: 0 0 5px rgb(88, 88, 88);
    margin-bottom: 0px;
    
  }

  .nav_buttons {
    float: left;
    color: import.$primary;
    text-align: center;
    padding: 14px 16px;
    text-decoration: none;
    font-size: 17px;
  }

  .logo_button {
    float: left;
  }

  .image {
    height: 30pt;
  }

  body { 
    margin: 0px;
  }

  .right_side{
    float: right;
    height: 100%;
  }

</style>