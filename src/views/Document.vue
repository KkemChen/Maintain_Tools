<template>
  <div class="main">
    <iframe src="/dist/vitepress/index.html" height="100%" width="100%" frameborder="0"></iframe>
  </div>
</template>

<script lang="ts" setup>
import { ref } from 'vue';
const input = ref('');
</script>

<style scoped>
.main {
  height: 100%;
  overflow: hidden;
}
</style>
