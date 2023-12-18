<script lang="ts" setup>
import { reactive, computed, ref, watch, onMounted } from 'vue';
import { Check, Select, MoreFilled, CloseBold, Close } from '@element-plus/icons-vue';
import { Icon } from '@iconify/vue';
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

interface Activity {
  content: string;
  timestamp: string;
  icon: any;
  size: string;
  color: string;
}

const activities = ref<Activity[]>([]);

const initActivities = () => {
  activities.value = form.type.map((type) => ({
    content: type,
    timestamp: '',
    icon: null,
    size: 'large',
    color: '',
  }));
};

watch(() => form.type, initActivities);

const test1 = {
  code: 0,
  message: 'success',
};
const test2 = {
  code: -1,
  message: 'failed',
};

const onSubmit = async () => {
  console.log('submit!');

  for (const activity of activities.value) {
    // 对每个 activity 执行异步操作
    activity.color = '#409EFF';
    activity.icon = MoreFilled;
    try {
      //   const response = await fetch('your-api-url');
      //   const data = await response.json();
      const data = test2;
      activity.color = data.code === 0 ? '#67C23A' : '#F56C6C';
      activity.icon = data.code === 0 ? Select : CloseBold;
    } catch (error) {
      console.error('Error:', error);
      activity.color = '#F56C6C'; // 错误时设置为红色
      activity.icon = CloseBold;
    }
  }
};

onMounted(() => {
  initActivities();
});
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
        <el-button @click="initActivities">Cancel</el-button>
      </el-form-item>
    </el-form>

    <div class="item-box">
      <el-timeline style="height: 100%">
        <el-timeline-item
          v-for="(activity, index) in activities"
          :key="index"
          :icon="activity.icon"
          :color="activity.color"
          :size="activity.size"
          :timestamp="activity.timestamp"
          style="height: 20%"
        >
          {{ activity.content }}
        </el-timeline-item>
      </el-timeline>
    </div>
  </div>
</template>

<style scoped lang="scss">
.item-box {
  margin-left: 10%;
  margin-top: 5%;
  height: 50vh;
}
</style>
