<script setup>

import { createMemoryHistory, createRouter } from 'vue-router';

import Home from "./home/home.vue"
import Component from './component/component.vue';
import { createApp} from 'vue';

const routes = [
  { path: "/", component: Home},
  { path: "/test", component: Component}
]

const router = createRouter({
  history: createMemoryHistory(),
  routes,
})

createApp(App)
  .use(router)
  .mount('#app')

</script>
