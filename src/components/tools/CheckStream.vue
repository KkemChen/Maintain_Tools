<script lang="ts" setup>
import { reactive, computed, ref, watch, onMounted } from 'vue';
import { Check, Select, MoreFilled, CloseBold, Close } from '@element-plus/icons-vue';
import { Icon } from '@iconify/vue';
import { useSysinfo } from '@/api/sysinfo';
const { checkStream } = useSysinfo();
import { useGlobalStore } from '@/store';
const globalStore = useGlobalStore();

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
  video: [];
  audio: [];
  tagColor: string;
  loading: Boolean;
}

const activities = ref<Activity[]>([]);

const initActivities = () => {
  activities.value = form.type.map((type) => ({
    content: type,
    timestamp: '',
    icon: null,
    size: 'large',
    color: '',
    video: [],
    audio: [],
    tagColor: 'info',
    loading: false,
  }));
};

watch(() => form.type, initActivities);

const setSuccessStatus = (activity) => {
  activity.color = '#67C23A';
  activity.icon = Select;
  activity.tagColor = 'success';
  activity.loading = false;
};

const setFailedStatus = (activity) => {
  activity.color = '#F56C6C';
  activity.icon = CloseBold;
  activity.tagColor = 'danger';
  activity.loading = false;
};

const setProcesstatus = (activity) => {
  activity.color = '#409EFF';
  activity.icon = MoreFilled;
  activity.tagColor = '';
  activity.loading = true;
};

const onSubmit = async () => {
  console.log('submit!');

  for (const activity of activities.value) {
    // 对每个 activity 执行异步操作
    setProcesstatus(activity);
    console.log(activity);
    let url = getStreamUrl(activity.content, form.name);
    try {
      const res = await checkStream(
        `${globalStore.remoteConfig.host}:${globalStore.remoteConfig.port}`,
        getStreamUrl(activity.content, form.name),
      );

      const data = res.data;
      if (res.code === 0) {
        setSuccessStatus(activity);
        console.log(data.video);
        console.log(data.audio);

        activity.video = data.video;
        activity.audio = data.audio;
      } else {
        setFailedStatus(activity);
      }
    } catch (error) {
      console.error('Error:', error);
      setFailedStatus(activity);
    }
  }
};

function channelDescription(channelCount) {
  switch (channelCount) {
    case 1:
      return 'mono';
    case 2:
      return 'stereo';
    // 添加更多的声道描述
    default:
      return `${channelCount} channels`;
  }
}

const formatStreamInfo = (stream) => {
  let formattedString = '';

  if (stream.codec_type === 'video') {
    formattedString = `Stream #${stream.index}:  Video: ${stream.codec_name},  ${stream.pix_fmt},  ${stream.width}x${stream.height}`;
    // 这里可以添加更多的视频相关信息，例如帧率等
  } else if (stream.codec_type === 'audio') {
    formattedString = `Stream #${stream.index}:  Audio: ${stream.codec_name},  ${
      stream.sample_rate
    } Hz,  ${channelDescription(stream.channels)},  ${stream.sample_fmt}`;
    // 这里可以添加更多的音频相关信息
  }

  return formattedString;
};

const getStreamUrl = (type, id) => {
  switch (type) {
    case 'RTSP':
      return `rtsp://${globalStore.remoteConfig.host}/live/${id}`;
    case 'RTMP/FLV':
      return `http://${globalStore.remoteConfig.host}:8096/live/${id}.live.flv`;
    case 'HLS':
      return `http://${globalStore.remoteConfig.host}:8096/live/${id}/hls.m3u8`;
    case 'FMP4':
      return `http://${globalStore.remoteConfig.host}:8096/live/${id}.live.mp4`;
    default:
      return ``;
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
          <el-card shadow="always">
            <template #header>
              <el-tag :type="activity.tagColor" class="item-tag">
                {{ activity.content }}
              </el-tag>
            </template>
            <el-skeleton :rows="1" :animated="activity.loading">
              <template #template>
                <div v-for="(stream, streamIndex) in activity.video" :key="streamIndex">
                  {{ formatStreamInfo(stream) }}
                </div>
                <div v-for="(stream, streamIndex) in activity.audio" :key="streamIndex">
                  {{ formatStreamInfo(stream) }}
                </div>
              </template>
            </el-skeleton>
          </el-card>
        </el-timeline-item>
      </el-timeline>
    </div>
  </div>
</template>

<style scoped lang="scss">
.item-box {
  margin-left: 0%;
  margin-top: 5%;
  padding-right: 10%;
  white-space: pre-wrap;
}

:deep(.el-card__header) {
  padding: 5px 10px;
}

:deep(.el-card__body) {
  padding: 10px;
  // height: 10vh;
  display: flex-direction;
}

.item-tag {
  font-size: 15px;
}

.el-loading-mask {
  height: auto;
}
</style>
