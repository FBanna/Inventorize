import Classes from '@/app/views/classes.vue'
import { createRouter, createWebHistory } from 'vue-router'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {path: "/classes", component: Classes}
  ],
})

export default router
