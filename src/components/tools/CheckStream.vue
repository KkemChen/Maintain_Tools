<script lang="ts" setup>
import { reactive } from 'vue';
import { SuccessFilled, WarningFilled, MoreFilled } from '@element-plus/icons-vue';
// do not use same name with ref
const form = reactive({
  name: '',
  region: '',
  date1: '',
  date2: '',
  delivery: false,
  type: ['RTSP', 'RTMP/FLV'],
  resource: '',
  desc: '',
});

const checkItem = reactive({
  icon: '',
  color: '',
});

const onSubmit = () => {
  console.log('submit!');
};
</script>

<template>
  <div class="flex-container">
    <el-form :model="form" label-width="120px">
      <el-form-item label="Stream ID" style="width: 90%">
        <el-input v-model="form.name" placeholder="please input stream ID, Example: 202306071149391189M" />
      </el-form-item>

      <el-form-item label="Check Type">
        <el-checkbox-group v-model="form.type">
          <el-checkbox label="RTSP" name="type" />
          <el-checkbox label="RTMP/FLV" name="type" />
          <el-checkbox label="HLS" name="type" />
          <el-checkbox label="FMP4" name="type" />
        </el-checkbox-group>
      </el-form-item>

      <el-form-item>
        <el-button type="primary" @click="onSubmit">Confirm</el-button>
        <el-button>Cancel</el-button>
      </el-form-item>
    </el-form>

    <div class="item-box">
      <el-steps :active="0" align-center finish-status="success" direction="vertical">
        <el-step v-for="(type, index) in form.type" :key="index" :title="type" description="Some description" />
      </el-steps>
    </div>
  </div>
</template>

<style scoped lang="scss">
.item-box {
  margin-left: 10%;
  margin-top: 5%;
  height: 300px;
}
</style>
