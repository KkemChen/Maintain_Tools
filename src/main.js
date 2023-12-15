import { createApp } from 'vue';
import pinia from '@/store/index';
import ElementPlus from 'element-plus';
import 'element-plus/dist/index.css';

import App from './App.vue';
import router from './router';
import './styles.css';

const app = createApp(App);
app.use(pinia);
app.use(ElementPlus);
app.use(router);
app.mount('#app');
