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
import { ElMessage } from 'element-plus';
import CustomLoading from './components/CustomLoading.vue';
import SSHDialog from './components/SshDialog.vue';
import { useGlobalStore } from '@/store';

defineOptions({ name: 'Menu' });

const globalStore = useGlobalStore();

const connectionStatus = ref(0); //0-未连接 1-loading 2-连接成功
const dialogVisible = ref(false);
const isCollapse = ref(false);
const remoteConfigCache = ref({});
const currentHost = ref('');
const currentPort = ref('');

const handleOpen = (key, keyPath) => {
  console.log(key, keyPath);
};
const handleClose = (key, keyPath) => {
  console.log(key, keyPath);
};
const toggleCollapse = () => {
  isCollapse.value = !isCollapse.value;
};
const toggleConnectionStatus = async () => {
  dialogVisible.value = !dialogVisible.value;
  const activeConntionId = localStorage.getItem(globalStore.getActiveKey());
  if (activeConntionId) {
    remoteConfigCache.value = globalStore.getLocalRemoteConfig(activeConntionId);
  }
};
const modifyConnectionStatus = (status) => {
  connectionStatus.value = status;
};
const getCurrentSshInfo = (host, port) => {
  currentHost.value = host;
  currentPort.value = port;
};

let hasReloaded = false;
const navigationEntries = window.performance.getEntriesByType('navigation');
const unwatch = watchEffect(async () => {
  if (globalStore.isConnected) {
    connectionStatus.value = 2;
  } else {
    connectionStatus.value = 0;
  }
  if (!hasReloaded && navigationEntries.length > 0 && navigationEntries[0].type === 'reload') {
    hasReloaded = true;
    const activeConntionId = localStorage.getItem(globalStore.getActiveKey());
    if (activeConntionId) {
      connectionStatus.value = 1;
      const activeConntion = globalStore.getLocalRemoteConfig(activeConntionId);
      currentHost.value = activeConntion.host;
      currentPort.value = activeConntion.port;

      const res = await globalStore.getRemoteConnection(activeConntion);
      connectionStatus.value = res.code === 0 ? 2 : 0;
      if (res.code === 0) {
        globalStore.setRemoteConfig(activeConntion);
      }
      ElMessage({
        type: res.code === 0 ? 'success' : 'error',
        message: res.message,
      });
    } else {
      ElMessage({
        type: 'warning',
        message: '当前没有活跃缓存连接',
      });
    }
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
    <el-menu-item index="/setting">
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
        <span v-else>{{ currentHost }} : {{ currentPort }}</span>
      </template>
    </el-menu-item>

    <el-menu-item class="menu-item-bottom" @click="toggleCollapse">
      <el-icon>
        <Expand v-if="isCollapse" />
        <Fold v-else />
      </el-icon>
    </el-menu-item>
  </el-menu>

  <SSHDialog
    :is-visible="dialogVisible"
    :remote-config-cache="remoteConfigCache"
    @modify-connectionStatus="modifyConnectionStatus"
    @get-current-ssh-info="getCurrentSshInfo"
  />
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
