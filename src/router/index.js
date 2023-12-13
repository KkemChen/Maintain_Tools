import { createRouter, createWebHistory } from 'vue-router';
import Home from '../views/Home.vue';
import Tools from '../views/Tools.vue';
import Search from '../views/Search.vue';

const router = createRouter({
  history: createWebHistory(),
  routes: [
    { name: 'Home', path: '/', component: Home },
    { name: 'Tools', path: '/tools', component: Tools },
    { name: 'Search', path: '/search', component: Search },
  ],
});

export default router;
