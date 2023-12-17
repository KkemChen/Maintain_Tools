<script setup>
import CPUInfo from '@/components/sysinfo/CPUInfo.vue';
import DiskInfo from '@/components/sysinfo/DiskInfo.vue';
import ProcessInfo from '@/components/sysinfo/ProcessInfo.vue';
import Pie from '@/components/v-charts/index.vue';
import IOInfo from '@/components/sysinfo/IOInfo.vue';
import { ref, onMounted, nextTick, onUnmounted, watchEffect } from 'vue';
import { invoke } from '@tauri-apps/api';
import { useGlobalStore } from '@/store';

// import { useSysinfo } from '@/api/sysinfo';

// const { fetchCPUInfo, fetchRemoteCPUInfo, fetchRemoteMemoryInfo, fetchRemoteLoadInfo } = useSysinfo();

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

const cpuTableData = ref([]);
const memTableData = ref([]);
const loadTableData = ref([]);

// const ssh_connect = () => {
//   invoke('add_ssh_connect', {
//     host: localStorage.getItem('host') + ':' + localStorage.getItem('port'),
//     user: localStorage.getItem('user'),
//     password: localStorage.getItem('password'),
//   })
//     .then((response) => {
//       console.log('SSH connection initialized', response);
//     })
//     .catch((error) => {
//       console.error('Error fetching CPU info:', error);
//     });
// };

// const disconnect_ssh = () => {
//   invoke('disconnect_ssh', {
//     host: localStorage.getItem('host') + ':' + localStorage.getItem('port'),
//   })
//     .then((response) => {
//       console.log('Disconnect ssh success: ', response);
//     })
//     .catch((error) => {
//       console.error('Disconnect ssh failed: ', error);
//     });
// };

const getCPUAverageUsage = (cpuData) => {
  if (Array.isArray(cpuData) && cpuData.length > 0) {
    const totalUsage = cpuData.reduce((sum, cpu) => sum + cpu.useage, 0);
    const averageUsage = totalUsage / cpuData.length;
    return averageUsage;
  }
  return 0.0;
};

const getMemoryUsage = (memData) => {
  if (Array.isArray(memData) && memData.length > 0) {
    const totalSize = memData[0].total_memory;
    const useage = memData[0].used_memory / totalSize;
    return useage;
  }
  return 0.0;
};

const getLoadUsage = (loadData) => {
  if (Array.isArray(loadData) && loadData.length > 0) {
    return loadData[0];
  }
  return 0.0;
};

const getRemoteInfo = async () => {
  await globalStore.getSystemInfo();
  cpuTableData.value = globalStore.systemInfo.cpuInfo;
  memTableData.value = globalStore.systemInfo.memoryInfo;
  loadTableData.value = globalStore.systemInfo.loadInfo;
};

const resizePie = () => {
  // 获取图表容器元素

  let container = document.getElementById('pie-container');
  if (!container) return;
  adjustFlexPieItems();
  // console.log(container.offsetWidth);
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
  // ssh_connect();
  resizePie();
  window.addEventListener('resize', resizePie);
  watchEffect(() => {
    if (globalStore.isConnected) {
      //先触发一次保证第一个三秒内有值
      getRemoteInfo();
      setInterval(() => {
        nextTick(() => {
          getRemoteInfo();
          chartsOption.value.cpuChart.data = getCPUAverageUsage(cpuTableData.value).toFixed(2);
          chartsOption.value.memoryChart.data = getMemoryUsage(memTableData.value).toFixed(2);
          chartsOption.value.loadChart.data = getLoadUsage(loadTableData.value).toFixed(2);
          chartsOption.value.diskChart.data = (Math.random() * 100).toFixed(2);
        });
      }, 3000);
    }
  });
});

onUnmounted(() => {
  // disconnect_ssh();
  globalStore.disconnectSsh();
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
          <DiskInfo />
        </el-card>
      </div>
    </el-col>
    <el-col :span="8">
      <div class="grid-content ep-bg-purple">
        <el-card class="box-card">
          <IOInfo />
        </el-card>
      </div>
    </el-col>
  </el-row>
  <el-row :gutter="10">
    <el-col :span="24">
      <div class="grid-content ep-bg-purple">
        <el-card class="box-card">
          <ProcessInfo />
        </el-card>
      </div>
    </el-col>
  </el-row>
</template>

<style scoped lang="scss">
.el-row {
  height: calc((calc(95%) - 15px) / 3);
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
