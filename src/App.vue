<script setup>
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup

import Menu from "./components/Menu.vue";
import Info from "./components/Info.vue"
</script>


<template>
  <div class="common-layout">
    <el-container>
      <el-aside width="200px">
        <Menu />
        <Info />
      </el-aside>
      <el-main>
        <el-header >Header</el-header>
        <router-view></router-view>
      </el-main>
    </el-container>
  </div>
</template>

<style scoped>
.common-layout{
  height: 100%;
}
.el-container{
  height: 100%;
}
.el-footer{
  border-right: solid 1px var(--el-menu-border-color);
}

.el-header{
  height: 5%;
  margin: 0;
}

.el-main{
  padding: 10px;
}
</style>
