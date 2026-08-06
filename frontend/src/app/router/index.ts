import Classes from '@/app/views/classes.vue'
import { createRouter, createWebHistory } from 'vue-router'
import ComponentSearch from '../views/ComponentSearch.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {path: "/:uuid", component: ComponentSearch, props: true},
    {path: "/classes", component: Classes}
  ],
})

export default router
