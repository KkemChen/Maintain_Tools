<script setup>
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import CPUInfo from "./components/CPUInfo.vue";
import Sidebar from "./components/Sidebar.vue";
</script>

<template>
  <div class="container">
    <div class="sidebar">
      <Sidebar />
    </div>
    <div class="main-content">
      <router-view></router-view> 
    </div>
  </div>
</template>

<style scoped>
.container {
  display: flex;
}

.sidebar {
  width: 250px; 
  background-color: #f0f0f0; 
}

.main-content {
  flex-grow: 1;
  padding: 20px; 
 
}


</style>
