//import './assets/main.css'

import { createApp } from 'vue'
import { useRoute, useRouter, createRouter, createMemoryHistory, createWebHistory, createWebHashHistory  } from 'vue-router'
import App from './App.vue'
import Home from "./home/home.vue"
import Component from './component/component.vue';
import AddComponent from "./component/new-component.vue"

const routes = [
    { path: "/", name: "home", component: Home},
    { path: "/addcomponent", name: "add", component: AddComponent},
    { path: "/component/:id(\\d+)", name: "component", component: Component},
]

const router = createRouter({
    history: createWebHistory(),
    routes: routes,
})
  
  createApp(App)
    .use(router)
    .mount('#app')