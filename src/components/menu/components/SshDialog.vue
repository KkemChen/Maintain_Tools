<script setup>
import { ref, reactive, watchEffect, nextTick } from 'vue';
import { ElMessage } from 'element-plus';
import { useGlobalStore } from '@/store';

defineOptions({ name: 'SSHDialog' });
const props = defineProps({
  isVisible: {
    type: Boolean,
    default: false,
  },
});

const globalStore = useGlobalStore();

const isVisible = ref(props.isVisible);
watchEffect(() => {
  isVisible.value = props.isVisible;
});

const formLabelWidth = '140px';

const form = reactive({
  host: '',
  port: 22,
  user: 'root',
  password: '',
});

const emit = defineEmits(['modify-connectionStatus']);

const storeRemoteConfig = () => {
  emit('modify-connectionStatus', 1);
  globalStore.setRemoteConfig(form.host, form.port, form.user, form.password);
  isVisible.value = false;
  nextTick(async () => {
    const res = await globalStore.getRemoteConnection();
    ElMessage({
      type: res.code === 0 ? 'success' : 'error',
      message: res.message,
    });
  });
};
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
