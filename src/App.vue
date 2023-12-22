<script setup>
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import { ref, onMounted, nextTick, onUnmounted, watch } from 'vue';
import { useRoute } from 'vue-router';
import Menu from './components/menu/index.vue';
// import Info from './components/info/index.vue';

const mainPadding = ref('10px');
const route = useRoute();

watch(
  () => route.fullPath,
  () => {
    // 检查路由的meta字段并更新padding
    const padding = route.meta.padding;
    if (padding !== undefined) {
      mainPadding.value = padding;
    } else {
      mainPadding.value = '10px'; // 默认值
    }
  },
  { immediate: true },
);
</script>

<template>
  <div class="common-layout">
    <el-container>
      <Menu />
      <!-- <el-aside width="15%">
        
        <Info />
      </el-aside> -->
      <el-main :style="{ padding: mainPadding }">
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
  /* padding: 10px; */
  height: 100%;
}
</style>
