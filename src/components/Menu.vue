<script setup>


</script>

<template>
    <el-menu router>
      <el-menu-item index="/">Home</el-menu-item>
      <el-menu-item index="/tools">Tools</el-menu-item>
      <el-menu-item index="/tools">Setting</el-menu-item>
      <!-- 其他菜单项... -->
    </el-menu>
    
</template>


<style scoped>
.el-menu{
    height: 90%;
}
.el-footer {
  margin-top: auto; /* 将 footer 推到底部 */
}
</style>
