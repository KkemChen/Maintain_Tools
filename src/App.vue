<script setup>
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup

import Menu from './components/menu/index.vue';
import Info from './components/info/index.vue';

const host = import.meta.env.VITE_HOST;
const port = import.meta.env.VITE_PORT;
const user = import.meta.env.VITE_USER;
const password = import.meta.env.VITE_PASSWORD;

localStorage.setItem('host', host);
localStorage.setItem('port', port);
localStorage.setItem('user', user);
localStorage.setItem('password', password);
</script>

<template>
  <div class="common-layout">
    <el-container>
      <el-aside width="15%">
        <Menu />
        <Info />
      </el-aside>
      <el-main>
        <el-header>Header</el-header>
        <router-view></router-view>
      </el-main>
    </el-container>
  </div>
</template>

<style scoped>
.common-layout {
  height: 100%;
}
.el-container {
  height: 100%;
}

/* .el-aside{
  border-right: solid 1px var(--el-menu-border-color);
} */
.el-header {
  height: 5%;
  margin: 0;
}

.el-main {
  padding: 10px;
}
</style>
