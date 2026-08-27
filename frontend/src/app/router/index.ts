import { createRouter, createWebHistory } from 'vue-router'
import ComponentSearch from '@/app/views/ComponentSearch.vue'
import Classes from '@/app/views/Classes.vue'
import Labels from '../views/Labels.vue'
import Manufacturers from '../views/Manufacturers.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {path: "/:uuid", component: ComponentSearch, props: true},
    {path: "/", component: ComponentSearch},
    {path: "/classes", component: Classes},
    {path: "/labels", component: Labels},
    {path: "/manufacturers", component: Manufacturers}
  ],
})

export default router
