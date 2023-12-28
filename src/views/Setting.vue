<script lang="ts" setup>
import { reactive, ref } from 'vue';

const appPath = reactive({
  ivauto_ivs_server: '/opt/ivauto_ivs_server/ivauto_ivs_server',
  ivauto_quality_detection: '/opt/ivauto_ivs_server/ivauto_quality_detection',
  ivauto_summary_server: '/opt/ivauto_ivs_server/ivauto_summary_server',
  StreamServer: '/opt/StreamServer/StreamServer',
  LB_intercom: '/opt/LB_intercom/LB_intercom',
});

const gitInfoPath = reactive({
  ivauto_ivs_server: '/opt/ivauto_ivs_server/ivs_ver.txt',
  ivauto_quality_detection: '/opt/ivauto_ivs_server/qd_ver.txt',
  StreamServer: '/opt/StreamServer/git_commit_version.txt',
  LB_intercom: '/opt/LB_intercom/git_commit_version.txt',
  model_zoo: '/opt/ivauto_ivs_server/model_ver.txt',
});

const modelPath = reactive({
  prison_rt: '/opt/ivauto_ivs_server/data/prison-rt',
  coeff_prison: '/opt/ivauto_ivs_server/data/coeff-prison',
});

const streamPort = reactive({
  rtsp: 554,
  http: 8096,
  rtmp: 2935,
});

const form = reactive({
  name: '',
  region: '',
  date1: '',
  date2: '',
  delivery: false,
  type: [],
  resource: '',
  desc: '',
});

const onSubmit = () => {
  console.log('submit!');
};
</script>
<template>
  <div class="container">
    app path
    <el-form :model="form" label-width="15rem" class="form">
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
      <el-divider />
      git commit hash path
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
      <el-divider />
      stream port
      <el-form-item label="rtsp">
        <el-input v-model="streamPort.rtsp" />
      </el-form-item>
      <el-form-item label="http">
        <el-input v-model="streamPort.http" />
      </el-form-item>
      <el-form-item label="rtmp">
        <el-input v-model="streamPort.rtmp" />
      </el-form-item>
    </el-form>
  </div>
</template>

<style scoped>
.container {
  width: 100vh;
}

.form {
  width: 100%;
}
</style>
