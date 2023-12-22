<template>
  <div class="main">
    <iframe :src="iframeSrc" height="100%" width="100%" frameborder="0"></iframe>
  </div>
</template>

<script lang="ts" setup>
import { ref, onMounted } from 'vue';
const iframeSrc = ref('/vitepress/index.html');

onMounted(() => {
  if (import.meta.env.MODE === 'development') {
    iframeSrc.value = '/dist/vitepress/index.html'; // 开发环境的路径
  } else {
    iframeSrc.value = '/vitepress/index.html'; // 生产环境的路径
  }
});
</script>

<style scoped>
.main {
  height: 100%;
  overflow: hidden;
}
</style>
