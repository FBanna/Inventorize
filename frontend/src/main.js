//import './assets/main.css'

import { createApp } from 'vue'
import { useRoute, useRouter, createRouter, createMemoryHistory } from 'vue-router'
import App from './App.vue'
import Home from "./home/home.vue"
import Component from './component/component.vue';
import AddComponent from "./component/new-component.vue"

const routes = [
    { path: "/", component: Home},
    { path: "/addcomponent", component: AddComponent},
    { path: "/component", component: Component},
]

const router = createRouter({
    history: createMemoryHistory(),
    routes: routes,
})
  
  createApp(App)
    .use(router)
    .mount('#app')