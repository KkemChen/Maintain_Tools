<script setup>
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import CPUInfo from "./components/CPUInfo.vue";
import Aside from "./components/Aside.vue";
</script>


<template>
  <div id="app">
  <div class="common-layout">
    <el-container>
      <el-aside width="200px">
        <Aside />
      </el-aside>
      <el-main>
        <el-header>Header</el-header>
        <router-view></router-view>
      </el-main>
    </el-container>
  </div>
</div>
</template>

<style scoped>

</style>
