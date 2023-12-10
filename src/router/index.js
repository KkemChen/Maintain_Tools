import { createRouter, createWebHistory } from "vue-router"
import Home from "../views/Home.vue"
import Tools from "../views/Tools.vue"

const router = createRouter(
    {
        history: createWebHistory(), 
        routes: [
            { name:'Home', path: '/', component: Home },
            { name:'Tools', path: '/tools', component: Tools },
        ]
    }
)

export default router