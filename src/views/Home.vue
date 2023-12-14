<script setup>
import CPUInfo from '@/components/sysinfo/CPUInfo.vue';
import DiskInfo from '@/components/sysinfo/DiskInfo.vue';
import ProcessInfo from '@/components/sysinfo/ProcessInfo.vue';
import Pie from '@/components/v-charts/index.vue';
import IOInfo from '@/components/sysinfo/IOInfo.vue';
import { ref, onMounted, nextTick, onUnmounted } from 'vue';
import { invoke } from '@tauri-apps/api';

import { useSysinfo } from '@/api/sysinfo';

const { fetchCPUInfo } = useSysinfo();

const pieWidth = '200px';
const chartsOption = ref({
  cpuChart: {
    title: 'CPU',
    data: 0.0,
  },
  memoryChart: {
    title: 'Memory',
    data: 65.89,
  },
  loadChart: {
    title: 'Load',
    data: 65.89,
  },
  diskChart: {
    title: 'Disk',
    data: 65.89,
  },
});

const cpuTableData = ref([]);

const ssh_connect = () => {
  invoke('add_ssh_connect', {
    host: localStorage.getItem('host') + ':' + localStorage.getItem('port'),
    user: localStorage.getItem('user'),
    password: localStorage.getItem('password'),
  })
    .then((response) => {
      console.log('SSH connection initialized', response);
    })
    .catch((error) => {
      console.error('Error fetching CPU info:', error);
    });
};

const disconnect_ssh = () => {
  invoke('disconnect_ssh', {
    host: host,
  })
    .then((response) => {
      console.log('Disconnect ssh success: ', response);
    })
    .catch((error) => {
      console.error('Disconnect ssh failed: ', error);
    });
};

const getCPUAverageUsage = (cpuData) => {
  if (Array.isArray(cpuData) && cpuData.length > 0) {
    const totalUsage = cpuData.reduce((sum, cpu) => sum + cpu.usage, 0);
    const averageUsage = totalUsage / cpuData.length;
    return averageUsage;
  }
  return 0.0;
};

onMounted(() => {
  ssh_connect();
  //先触发一次保证第一个三秒内有值

  fetchCPUInfo().then((data) => {
    cpuTableData.value = data;
  });
  setInterval(() => {
    nextTick(() => {
      fetchCPUInfo().then((data) => {
        cpuTableData.value = data;
      });
      chartsOption.value.cpuChart.data = getCPUAverageUsage(cpuTableData.value).toFixed(2);
      chartsOption.value.memoryChart.data = (Math.random() * 100).toFixed(2);
      chartsOption.value.loadChart.data = (Math.random() * 100).toFixed(2);
      chartsOption.value.diskChart.data = (Math.random() * 100).toFixed(2);
    });
  }, 3000);
});

onUnmounted(() => {
  disconnect_ssh();
});
</script>

<template>
  <el-row :gutter="10">
    <el-col :span="24">
      <div class="grid-content ep-bg-purple">
        <el-card class="box-card">
          <div class="flex-pie">
            <Pie id="cpu" :width="pieWidth" :option="chartsOption.cpuChart" />
            <Pie id="memory" :width="pieWidth" :option="chartsOption.memoryChart" />
            <Pie id="load" :width="pieWidth" :option="chartsOption.loadChart" />
            <Pie id="disk" :width="pieWidth" :option="chartsOption.diskChart" />
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
      .flex-pie {
        display: flex;
        flex-direction: row;
        justify-content: flex-start;
        flex-wrap: nowrap;
      }
      :deep(.el-card__body) {
        padding: 0px !important;
        height: 100%;
      }
    }
  }
}
</style>
