import { createRouter, createWebHistory } from "vue-router"
import Home from "../views/Home.vue"
import About from "../views/About.vue"

const router = createRouter(
    {
        history: createWebHistory(), 
        routes: [
            { path: '/', component: Home },
            { path: '/about', component: About },
        ]
    }
)

export default router