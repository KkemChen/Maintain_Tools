import { createRouter, createWebHistory } from 'vue-router';
import Home from '../views/Home.vue';
import Tools from '../views/Tools.vue';
import Document from '../views/Document.vue';
import Setting from '../views/Setting.vue';

const router = createRouter({
  history: createWebHistory(),
  routes: [
    { name: 'Home', path: '/', component: Home },
    { name: 'Tools', path: '/tools', component: Tools },
    { name: 'Document', path: '/document', component: Document },
    { name: 'Setting', path: '/setting', component: Setting },
  ],
});

export default router;
