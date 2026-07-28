import Classes from '@/app/views/classes.vue'
import { createRouter, createWebHistory } from 'vue-router'
import Home from '@/app/views/home.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {path: "/", component: Home},
    {path: "/classes", component: Classes}
  ],
})

export default router
