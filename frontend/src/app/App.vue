<script setup lang="ts">
import ClassInstanceTreeSidebar from '@/app/components/ClassInstanceTreeSidebar.vue';
import Popup_controller from './popup/popup_controller.vue';
import Error_controller from '@/error/error_controller.vue';


</script>

<template>    

  <header>
    <div class="nav">
      <nav>

        

        <RouterLink class="logo_button" to="/"><img class="image" src="/images/logo.svg"></RouterLink>
        <!-- <RouterLink class="nav_buttons" to="/addcomponent">Add</RouterLink> -->
        <!-- <RouterLink class="nav_buttons" to="/component/1">build</RouterLink> -->





        <div class="right_side">

          <a class="nav_buttons" href="/logout">Logout</a>

        </div>

        

      </nav>
    </div>
  </header>


  <div class="screen">

    <ClassInstanceTreeSidebar />

    <RouterView />

    <Popup_controller class="popup" />

    <Error_controller />

  </div>

  
  

</template>

<style scoped lang="scss">

  @use "@/style/import";

  .nav {
    margin: 0;
    z-index: 1;
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 50px;
    background: import.$white;
    box-sizing: border-box;
    padding: 5px;
    border-bottom: 1px solid import.$white;
    box-shadow: 0 0 5px rgb(88, 88, 88);
    margin-bottom: 0px;
    
  }

  .screen {
    display: flex;
    height: calc(100vh - 50px);
    
    margin-top: 50px;
  }

  .popup {
    z-index: 2;
  }

  .nav_buttons {
    float: left;
    color: import.$primary;
    padding: 10px;
    vertical-align: middle;

    text-decoration: none;
    font-size: 17px;
  }

  .logo_button {
    float: left;
  }

  .image {
    height: 40px;
    align-self: center;
    vertical-align: middle;
  }

  .right_side{
    float: right;
    height: 100%;
  }



</style>
