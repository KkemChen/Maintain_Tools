<script lang="ts" setup>
import { onMounted, reactive, ref } from 'vue';
import { useGlobalStore } from '@/store';
import { ElMessage } from 'element-plus';

const globalStore = useGlobalStore();

const appPath = reactive({
  ivauto_ivs_server: '',
  ivauto_quality_detection: '',
  ivauto_summary_server: '',
  StreamServer: '',
  LB_intercom: '',
});

const gitInfoPath = reactive({
  ivauto_ivs_server: '',
  ivauto_quality_detection: '',
  StreamServer: '',
  LB_intercom: '',
  model_zoo: '',
});

const modelPath = reactive({
  prison_rt: '',
  coeff_prison: '',
});

const streamPort = reactive({
  rtsp: 554,
  http: 8096,
  rtmp: 2935,
});

onMounted(() => {
  Object.assign(appPath, globalStore.getLocalAppPath());
  Object.assign(gitInfoPath, globalStore.getLocalGitInfoPath());
  Object.assign(streamPort, globalStore.getLocalStreamPort());
});

const onSubmit = () => {
  // 更新全局状态和localStorage
  globalStore.setLocalAppPath(appPath);
  globalStore.setLocalGitInfoPath(gitInfoPath);
  globalStore.setLocalStreamPort(streamPort);
  ElMessage({
    type: 'success',
    message: '保存成功',
  });
};
</script>
<template>
  <div class="container">
    <el-tag class="ml-2" type="success">App Path</el-tag>

    <el-form label-width="15rem" class="form">
      <el-form-item label="ivauto_ivs_server">
        <el-input v-model="appPath.ivauto_ivs_server" />
      </el-form-item>
      <el-form-item label="ivauto_quality_detection">
        <el-input v-model="appPath.ivauto_quality_detection" />
      </el-form-item>
      <el-form-item label="ivauto_summary_server">
        <el-input v-model="appPath.ivauto_summary_server" />
      </el-form-item>
      <el-form-item label="StreamServer">
        <el-input v-model="appPath.StreamServer" />
      </el-form-item>
      <el-form-item label="LB_intercom">
        <el-input v-model="appPath.LB_intercom" />
      </el-form-item>
    </el-form>
    <el-divider />
    <el-form label-width="15rem" class="form">
      <el-tag class="ml-2" type="success">Git Commit Hash Path</el-tag>
      <el-form-item label="ivauto_ivs_server">
        <el-input v-model="gitInfoPath.ivauto_ivs_server" />
      </el-form-item>
      <el-form-item label="ivauto_quality_detection">
        <el-input v-model="gitInfoPath.ivauto_quality_detection" />
      </el-form-item>
      <el-form-item label="StreamServer">
        <el-input v-model="gitInfoPath.StreamServer" />
      </el-form-item>
      <el-form-item label="LB_intercom">
        <el-input v-model="gitInfoPath.LB_intercom" />
      </el-form-item>
      <el-form-item label="LB_intercom">
        <el-input v-model="gitInfoPath.model_zoo" />
      </el-form-item>
    </el-form>
    <el-divider />
    <el-form label-width="15rem" class="form">
      <el-tag class="ml-2" type="success">Stream Port</el-tag>
      <el-form-item label="rtsp">
        <el-input v-model="streamPort.rtsp" />
      </el-form-item>
      <el-form-item label="http">
        <el-input v-model="streamPort.http" />
      </el-form-item>
      <el-form-item label="rtmp">
        <el-input v-model="streamPort.rtmp" />
      </el-form-item>
      <el-form-item class="saveButton">
        <el-button type="primary" @click="onSubmit">Save</el-button>
        <el-button>Cancel</el-button>
      </el-form-item>
    </el-form>
  </div>
</template>

<style scoped>
.container {
  width: 100%;
}

.form {
  width: 90%;
}

.ml-2 {
  margin-bottom: 12px;
  margin-left: 3%;
}

.saveButton {
  margin-top: 50px;
}
</style>
