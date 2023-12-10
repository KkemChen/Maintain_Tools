<script setup>
    import { useRouter ,useRoute} from 'vue-router';
    const router = useRouter()
    const route = useRoute()

const toHome = () => {
    router.push({
        name: 'Home',

    })
}

const toTools = () => {
    router.push({
        name: 'Tools',
    })
}


</script>

<template>
    <el-menu>
      <el-menu-item index="1" @click="toHome()">Home</el-menu-item>
      <el-menu-item index="2" @click="toTools()">Tools</el-menu-item>
      <el-menu-item index="3" @click="toTools()">Setting</el-menu-item>
      <!-- 其他菜单项... -->
    </el-menu>
    <el-footer>Footer</el-footer>
</template>


<style scoped>

.el-footer {
  margin-top: auto; /* 将 footer 推到底部 */
}
</style>
