<script setup lang="ts">
import { ref } from 'vue';
import { Search } from '@element-plus/icons-vue';
import { useTools } from '@/api/tools';
const { get_md5 } = useTools();
import { useGlobalStore } from '@/store';
const globalStore = useGlobalStore();

const input = ref('');
const loading = ref(false); // 控制加载动画的显示
const result = ref(''); // 存储和显示结果的地方

const handleSubmit = async (filePath) => {
  // 开始数据请求之前，设置loading为true
  loading.value = true;

  try {
    // 尝试获取md5值
    const json = await get_md5(`${globalStore.remoteConfig.host}:${globalStore.remoteConfig.port}`, filePath);
    result.value = json.data.md5;
  } catch (error) {
    // 如果出现错误，打印错误信息
    console.error(`Error getting md5 for ${filePath}: ${error}`);
  } finally {
    // 无论成功还是失败，数据请求完成后都将loading设置为false
    loading.value = false;
  }
};
</script>

<template>
  <div class="mt">
    <el-input v-model="input" placeholder="Please input file path" class="input-with-select">
      <template #append>
        <el-button :icon="Search" @click="handleSubmit(input)" />
      </template>
    </el-input>
    <!-- 加载动画和结果显示 -->
    <div v-if="loading" class="loading-container">
      <el-spinner></el-spinner>
      <span>Loading...</span>
    </div>

    <div v-else class="result-container">
      <!-- 在这里显示你的结果 -->
      {{ result }}
    </div>
  </div>
</template>

<style scoped>
.input-with-select .el-input-group__prepend {
  background-color: var(--el-fill-color-blank);
}

.loading-container {
  margin-top: 10%;
  text-align: center;
}

.result-container {
  margin-top: 10%;
  padding: 10px;
}

.mt {
  padding: 5% 10%;
}
</style>
