<script lang="ts" setup>
import { ref } from 'vue';
import { Document, Menu as IconMenu, Location, Setting, Expand, Fold, HomeFilled } from '@element-plus/icons-vue';

const isCollapse = ref(true);
const handleOpen = (key: string, keyPath: string[]) => {
  console.log(key, keyPath);
};
const handleClose = (key: string, keyPath: string[]) => {
  console.log(key, keyPath);
};
const toggleCollapse = () => {
  isCollapse.value = !isCollapse.value;
};
</script>

<style>
.el-menu-vertical-demo:not(.el-menu--collapse) {
  width: 200px;
  min-height: 400px;
}
</style>

<template>
  <!--   <el-menu router>
    <el-menu-item index="/">Home</el-menu-item>
    <el-menu-item index="/tools">Tools</el-menu-item>
    <el-menu-item index="/search">Search</el-menu-item>
  </el-menu> -->

  <el-menu
    default-active="2"
    class="el-menu-vertical-demo"
    :collapse="isCollapse"
    @open="handleOpen"
    @close="handleClose"
    router
  >
    <el-menu-item index="/">
      <el-icon><HomeFilled /></el-icon>
      <template #title>Home</template>
    </el-menu-item>
    <el-menu-item index="/tools">
      <el-icon><icon-menu /></el-icon>
      <template #title>Tools</template>
    </el-menu-item>
    <el-menu-item index="/document">
      <el-icon><document /></el-icon>
      <template #title>Document</template>
    </el-menu-item>
    <el-menu-item index="/setting">
      <el-icon><setting /></el-icon>
      <template #title>Setting</template>
    </el-menu-item>

    <el-menu-item class="menu-item-bottom" @click="toggleCollapse">
      <el-icon v-if="isCollapse"><Expand /></el-icon>
      <el-icon v-else><Fold /></el-icon>
    </el-menu-item>
  </el-menu>
</template>

<style scoped>
.el-menu {
  height: 100%;
  position: relative;
}

.menu-item-bottom {
  position: absolute;
  bottom: 0;
  width: 100%;
}
</style>
