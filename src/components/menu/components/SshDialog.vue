<script setup>
import { onBeforeUnmount, ref, reactive, watchEffect, nextTick } from 'vue';
import { ElMessage } from 'element-plus';
import { useGlobalStore } from '@/store';

defineOptions({ name: 'SSHDialog' });
const props = defineProps({
  isVisible: {
    type: Boolean,
    default: false,
  },
  remoteConfigCache: {
    type: Object,
    default: {},
  },
});

const globalStore = useGlobalStore();

const isVisible = ref(false);
const formLabelWidth = '140px';
let form = reactive({
  host: '',
  port: 22,
  user: 'root',
  password: '',
});

const unwatch = watchEffect(() => {
  isVisible.value = props.isVisible;
  if (props.remoteConfigCache) {
    form.host = props.remoteConfigCache.host;
    form.port = props.remoteConfigCache.port;
    form.user = props.remoteConfigCache.user;
    form.password = props.remoteConfigCache.password;
  } else {
    ElMessage({
      type: 'info',
      message: `当前缓存为空 cache: ${props.remoteConfigCache}`,
    });
  }
});

// 定义修改连接状态的事件
const emit = defineEmits(['modify-connectionStatus', 'get-current-ssh-info']);

const storeRemoteConfig = () => {
  emit('modify-connectionStatus', 1);
  const currentActiveConntionId = localStorage.getItem(globalStore.getActiveKey());
  isVisible.value = false;
  nextTick(async () => {
    if (
      globalStore.isConnected &&
      globalStore.remoteConfig.host === form.host &&
      globalStore.remoteConfig.port === form.port &&
      globalStore.remoteConfig.user === form.user
    ) {
      ElMessage({ type: 'warning', message: '你正在重复连接同一台设备!!!' });
      emit('modify-connectionStatus', 2);
      return;
    }

    const res = await globalStore.getRemoteConnection(form);
    if (res.code !== 0) {
      // 获取连接结果且出现 error(code 不为 0)
      emit('modify-connectionStatus', 0);

      // 先报错误信息
      ElMessage({
        type: 'error',
        message: res.message,
      });

      if (currentActiveConntionId && globalStore.isConnected) {
        //提示2秒后的状态恢复
        ElMessage({
          type: 'warning',
          message: '2秒后重新显示原有连接状态.',
        });

        //修改为断开连接2秒后重新显示原有的成功连接状态
        setTimeout(() => {
          emit('modify-connectionStatus', 2);
        }, 2000);
      }
    } else {
      // 连接成功, 当前活跃连接 ID 已被更新, 当前连接状态被置为 true.
      emit('modify-connectionStatus', 2);
      emit('get-current-ssh-info', form.host, form.port);
      globalStore.setRemoteConfig(form);

      // 连接成功
      ElMessage({
        type: 'success',
        message: res.message,
      });
      // 当存在连接时, 再次连接成功, 判断原有活跃连接和连接状态
      if (currentActiveConntionId && globalStore.isConnected) {
        const oldRemoteConfig = globalStore.getLocalRemoteConfig(currentActiveConntionId);
        // 断开原有ssh连接
        const disconnectedRes = JSON.parse(await globalStore.disconnectSsh(oldRemoteConfig.host, oldRemoteConfig.port));
        ElMessage({
          type: disconnectedRes.code === 0 ? 'success' : 'error',
          message: disconnectedRes.message,
          duration: 3000,
        });
      }
    }
  });
};
// 在组件被销毁前执行清理操作
onBeforeUnmount(() => {
  unwatch();
});
</script>
<template>
  <el-dialog v-model="isVisible" title="Connnect to remote SSH" align-center>
    <el-form :model="form">
      <el-form-item label="Host" :label-width="formLabelWidth">
        <el-input v-model="form.host" placeholder="Please input host" autocomplete="off" />
      </el-form-item>
      <el-form-item label="Port" :label-width="formLabelWidth">
        <el-input v-model="form.port" placeholder="Please input port" autocomplete="off" />
      </el-form-item>
      <el-form-item label="User" :label-width="formLabelWidth">
        <el-input v-model="form.user" placeholder="Please input user" autocomplete="off" />
      </el-form-item>
      <el-form-item label="Password" :label-width="formLabelWidth">
        <el-input
          v-model="form.password"
          type="password"
          placeholder="Please input password"
          show-password
          autocomplete="off"
        />
      </el-form-item>
    </el-form>
    <template #footer>
      <span class="dialog-footer">
        <el-button @click="isVisible = false">取消</el-button>
        <el-button type="primary" @click="storeRemoteConfig">发起连接</el-button>
      </span>
    </template>
  </el-dialog>
</template>
<style scoped>
.dialog-footer button:first-child {
  margin-right: 10px;
}
</style>
