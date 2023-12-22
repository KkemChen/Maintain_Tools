<script setup>
import { ref, onBeforeUnmount, watchEffect } from 'vue';
import {
  Document,
  Menu as IconMenu,
  Location,
  Setting,
  Expand,
  Fold,
  HomeFilled,
  SuccessFilled,
  WarningFilled,
} from '@element-plus/icons-vue';
import CustomLoading from './components/CustomLoading.vue';
import SSHDialog from './components/SshDialog.vue';
import { useGlobalStore } from '@/store';

defineOptions({ name: 'Menu' });

const globalStore = useGlobalStore();

const connectionStatus = ref(0); //0-未连接 1-loading 2-连接成功
const dialogVisible = ref(false);
const isCollapse = ref(false);

const handleOpen = (key, keyPath) => {
  console.log(key, keyPath);
};
const handleClose = (key, keyPath) => {
  console.log(key, keyPath);
};
const toggleCollapse = () => {
  isCollapse.value = !isCollapse.value;
};
const toggleConnectionStatus = async (e) => {
  dialogVisible.value = !dialogVisible.value;
};
const modifyConnectionStatus = (status) => {
  connectionStatus.value = status;
};

let hasReloaded = false;
const navigationEntries = window.performance.getEntriesByType('navigation');
const unwatch = watchEffect(() => {
  if (globalStore.isConnected) {
    connectionStatus.value = 2;
  } else {
    connectionStatus.value = 0;
  }
  if (!hasReloaded && navigationEntries.length > 0 && navigationEntries[0].type === 'reload') {
    globalStore.getRemoteConnection();
    hasReloaded = true;
  }
});

// 在组件被销毁前执行清理操作
onBeforeUnmount(() => {
  unwatch();
});
</script>

<template>
  <el-menu
    default-active="/"
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
    <el-menu-item index="/setting" disable>
      <el-icon><setting /></el-icon>
      <template #title>Setting</template>
    </el-menu-item>

    <el-menu-item class="menu-item-above-bottom" @click="toggleConnectionStatus">
      <el-icon>
        <WarningFilled style="color: var(--el-color-warning)" v-if="connectionStatus === 0" />
        <CustomLoading v-else-if="connectionStatus === 1" />
        <SuccessFilled style="color: var(--el-color-success)" v-else />
      </el-icon>
      <template #title>
        <span v-if="connectionStatus === 0">Not Connected</span>
        <span v-else-if="connectionStatus === 1">Connecting...</span>
        <span v-else>{{ globalStore.remoteConfig.host }} : {{ globalStore.remoteConfig.port }}</span>
      </template>
    </el-menu-item>

    <el-menu-item class="menu-item-bottom" @click="toggleCollapse">
      <el-icon>
        <Expand v-if="isCollapse" />
        <Fold v-else />
      </el-icon>
    </el-menu-item>
  </el-menu>

  <SSHDialog :is-visible="dialogVisible" @modify-connectionStatus="modifyConnectionStatus" />
</template>

<style scoped>
.el-menu {
  height: 100%;
  position: relative;
}
.el-menu-vertical-demo:not(.el-menu--collapse) {
  width: 200px;
  min-height: 400px;
}
.menu-item-bottom {
  position: absolute;
  bottom: 0;
  width: 100%;
}
.menu-item-above-bottom {
  position: absolute;
  bottom: 50px;
  width: 100%;
}
</style>
