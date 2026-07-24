//import './assets/main.css'

import { createApp } from 'vue'
import { useRoute, useRouter, createRouter, createMemoryHistory, createWebHistory, createWebHashHistory  } from 'vue-router'
import App from './App.vue'
import Home from "./home/home.vue"
import Component from './component/component.vue';
import AddComponent from "./component/new-component.vue"
import updateComponent from './component/update-component.vue';
import UpdateComponent from './component/update-component.vue';

const routes = [
    { path: "/", component: Home},
    { path: "/addcomponent", component: AddComponent},
    { path: "/component/:id(\\d+)", component: Component},
    { path: "/component/:id(\\d+)/update", component: UpdateComponent},

]

const router = createRouter({
    history: createWebHistory(),
    routes: routes,
})
  
  createApp(App)
    .use(router)
    .mount('#app')