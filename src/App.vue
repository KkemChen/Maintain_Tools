<script setup>
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup

import Menu from './components/menu/index.vue';
// import Info from './components/info/index.vue';
</script>

<template>
  <div class="common-layout">
    <el-container>
      <Menu />
      <!-- <el-aside width="15%">
        
        <Info />
      </el-aside> -->
      <el-main>
        <!-- <el-header></el-header> -->
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
