<script setup>
import CPUInfo from '@/components/sysinfo/CPUInfo.vue';
import DiskInfo from '@/components/sysinfo/DiskInfo.vue';
import ProcessInfo from '@/components/sysinfo/ProcessInfo.vue';
import Pie from '@/components/v-charts/index.vue';
import IOInfo from '@/components/sysinfo/IOInfo.vue';
import { ref, onMounted, nextTick, onUnmounted, watchEffect } from 'vue';
import { useGlobalStore } from '@/store';

const globalStore = useGlobalStore();

const pieWidth = ref(200);
const pieHeight = ref(200);

const chartsOption = ref({
  cpuChart: {
    title: 'CPU',
    data: 0.0,
  },
  memoryChart: {
    title: 'Memory',
    data: 0.0,
  },
  loadChart: {
    title: 'Load',
    data: 0.0,
  },
  diskChart: {
    title: 'Disk',
    data: 0.0,
  },
});

const cpuInfo = ref('');
const cpuTableData = ref([]);
const memTableData = ref({});
const loadTableData = ref({});
const netTableData = ref([]);
const diskTableData = ref([]);
const processTableData = ref([]);
const diskInfo = ref('');

const getCPUUsage = (cpuData) => {
  if (cpuData) {
    return cpuData.usage * 100;
  }
  return 0.0;
};

const getMemoryUsage = (memData) => {
  if (memData.used && memData.total) {
    const usage = (memData.used / memData.total) * 100;
    return usage;
  }
  return 0.0;
};

const getLoadUsage = (loadData, cpuCount) => {
  if (loadData && cpuCount > 0) {
    return (loadData.load1 / cpuCount) * 100;
  }
  return 0.0;
};

const getDiskUsage = (diskData) => {
  if (diskData.length > 0) {
    return parseFloat(diskData);
  }
  return 0.0;
};

const getRemoteInfo = async () => {
  await globalStore.getSystemInfo();

  cpuInfo.value = globalStore.systemInfo.cpuInfo;
  // cpu 进度表
  cpuTableData.value = globalStore.systemInfo.cpuDetail;
  // memory pie
  memTableData.value = globalStore.systemInfo.memoryInfo;
  // load pie
  loadTableData.value = globalStore.systemInfo.loadInfo;
  // networks i/o
  netTableData.value = globalStore.systemInfo.networksInfo;
  // disk pie
  diskInfo.value = globalStore.systemInfo.diskInfo;
  // disk 表格
  diskTableData.value = globalStore.systemInfo.diskDetail;
  // process 表格
  processTableData.value = globalStore.systemInfo.processInfo;

  // pie 图表赋值
  chartsOption.value.cpuChart.data = getCPUUsage(cpuInfo.value).toFixed(2);
  chartsOption.value.memoryChart.data = getMemoryUsage(memTableData.value).toFixed(2);
  chartsOption.value.loadChart.data = getLoadUsage(loadTableData.value, cpuTableData.value.length).toFixed(2);
  chartsOption.value.diskChart.data = getDiskUsage(diskInfo.value).toFixed(2);
};

const resizePie = () => {
  // 获取图表容器元素
  let container = document.getElementById('pie-container');
  if (!container) return;
  adjustFlexPieItems();
  // 计算新的宽度和高度为容器的 25%
  nextTick(() => {
    pieWidth.value = container.offsetWidth * 0.25;
    pieHeight.value = container.offsetHeight * 1;
  });
};

function adjustFlexPieItems() {
  const container = document.querySelector('.flex-pie');
  const items = container.children;
  const itemCount = items.length;

  for (let i = 0; i < itemCount; i++) {
    items[i].style.position = 'absolute';
    items[i].style.top = '0';
    items[i].style.height = '100%';
    items[i].style.left = `${(100 / itemCount) * i}%`;
    items[i].style.width = `${100 / itemCount}%`;
  }
}

onMounted(() => {
  resizePie();
  getRemoteInfo();
  window.addEventListener('resize', resizePie);
  watchEffect(() => {
    if (globalStore.isConnected) {
      //先触发一次保证第一个三秒内有值
      getRemoteInfo();
      setInterval(() => {
        nextTick(() => {
          getRemoteInfo();
        });
      }, 2000);
    }
  });
});

onUnmounted(() => {
  window.removeEventListener('resize', resizePie);
});
</script>

<template>
  <el-row :gutter="10">
    <el-col :span="24">
      <div class="grid-content ep-bg-purple">
        <el-card class="box-card" id="pie-container">
          <div class="flex-pie">
            <Pie id="cpu" :width="pieWidth + 'px'" :height="pieHeight + 'px'" :option="chartsOption.cpuChart" />
            <Pie id="memory" :width="pieWidth + 'px'" :height="pieHeight + 'px'" :option="chartsOption.memoryChart" />
            <Pie id="load" :width="pieWidth + 'px'" :height="pieHeight + 'px'" :option="chartsOption.loadChart" />
            <Pie id="disk" :width="pieWidth + 'px'" :height="pieHeight + 'px'" :option="chartsOption.diskChart" />
          </div>
        </el-card>
      </div>
    </el-col>
  </el-row>
  <el-row :gutter="5">
    <el-col :span="8">
      <div class="grid-content ep-bg-purple">
        <el-card class="box-card">
          <CPUInfo :option="cpuTableData" />
        </el-card>
      </div>
    </el-col>
    <el-col :span="8">
      <div class="grid-content ep-bg-purple">
        <el-card class="box-card">
          <DiskInfo :option="diskTableData" />
        </el-card>
      </div>
    </el-col>
    <el-col :span="8">
      <div class="grid-content ep-bg-purple">
        <el-card class="box-card">
          <IOInfo :option="netTableData" />
        </el-card>
      </div>
    </el-col>
  </el-row>
  <el-row :gutter="10">
    <el-col :span="24">
      <div class="grid-content ep-bg-purple">
        <el-card class="box-card">
          <ProcessInfo :option="processTableData" />
        </el-card>
      </div>
    </el-col>
  </el-row>
</template>

<style scoped lang="scss">
.el-row {
  height: calc((calc(100%) - 15px) / 3);
  margin-bottom: 5px;
  .el-col {
    height: 100%;
    .grid-content {
      border: none;
      height: 100%;
      .box-card {
        width: 100%;
        height: 100%;
        /* background-color: lightgreen; */
      }
      // .flex-pie {
      //   /* display: flex;
      //   flex-direction: row;
      //   justify-content: flex-start;
      //   flex-wrap: nowrap;
      //   height: 100%;
      //   width: 100%; */
      //   position: absolute;
      //   height: 100%;
      //   width: 100%;
      // }
      .flex-pie {
        position: relative;
        height: 100%;
        width: 100%;

        > * {
          position: absolute;
          top: 0;
          height: 100%;
        }
      }

      :deep(.el-card__body) {
        padding: 0px !important;
        height: 100%;
      }
    }
  }
}
</style>
