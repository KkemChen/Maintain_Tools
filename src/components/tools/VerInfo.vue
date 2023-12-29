<script lang="ts" setup>
import { ref, onMounted, onUnmounted } from 'vue';
import { useTools } from '@/api/tools';
const { get_ver_info, get_md5 } = useTools();
import { useGlobalStore } from '@/store';
const globalStore = useGlobalStore();

interface TableAppDataItem {
  name: string;
  date: string;
  commit: string;
}

const tableAppData = ref<TableAppDataItem[]>([]);

const columnWidths = ref({
  // date: '25%',
  // name: '25%',
  // address: '50%',
  date: '230px',
  name: '230px',
  address: '460px',
});

interface TableModelDataItem {
  name: string;
  date: string;
  md5: string;
}

const tableModelData = ref<TableModelDataItem[]>([]);

// 调整列宽的函数;
const adjustColumnWidths = () => {
  const containerWidth = document.getElementsByClassName('el-tabs__content')[0].offsetWidth - 84; // 获取表格容器的宽度

  columnWidths.value = {
    date: `${containerWidth * 0.25}px`, // 25%
    name: `${containerWidth * 0.25}px`, // 25%
    address: `${containerWidth * 0.5}px`, // 50%
  };
};

const getMd5 = async () => {
  const paths = globalStore.modelPath;

  for (const [name, path] of Object.entries(paths)) {
    await get_md5(`${globalStore.remoteConfig.host}:${globalStore.remoteConfig.port}`, path)
      .then((json) => {
        const newItem: TableModelDataItem = {
          name: name,
          date: json.data.date,
          md5: json.data.md5,
        };

        tableModelData.value.push(newItem);
      })
      .catch((error) => {
        console.error(`Error getting md5 for ${path}: ${error}`);
      });
  }
};

const getCommitHash = async () => {
  const paths = globalStore.gitInfoPath;

  for (const [name, path] of Object.entries(paths)) {
    await get_ver_info(`${globalStore.remoteConfig.host}:${globalStore.remoteConfig.port}`, path)
      .then((json) => {
        const newItem: TableAppDataItem = {
          name: name,
          date: json.data.date,
          commit: json.data.commit_hash,
        };

        tableAppData.value.push(newItem);
      })
      .catch((error) => {
        console.error(`Error getting hash for ${path}: ${error}`);
      });
  }
};

// 在组件挂载后和窗口大小变化时调整列宽
onMounted(() => {
  getCommitHash();
  getMd5();
  window.addEventListener('resize', adjustColumnWidths);
  adjustColumnWidths(); // 初始调整
});

// 在组件卸载前清理事件监听器
onUnmounted(() => {
  window.removeEventListener('resize', adjustColumnWidths);
});
</script>

<template>
  <el-table :data="tableAppData" :show-overflow-tooltip="true" stripe id="myTable">
    <el-table-column fixed prop="name" label="Name" :width="columnWidths.date" />
    <el-table-column prop="date" label="Date" :width="columnWidths.name" />
    <el-table-column prop="commit" label="Commit" :width="columnWidths.address" />
  </el-table>
  <br />
  <el-divider />
  <br />
  <el-table :data="tableModelData" :show-overflow-tooltip="true" stripe id="myTable">
    <el-table-column fixed prop="name" label="Name" :width="columnWidths.date" />
    <el-table-column prop="date" label="Date" :width="columnWidths.name" />
    <el-table-column prop="md5" label="MD5" :width="columnWidths.address" />
  </el-table>
</template>

<style scoped>
:deep(.el-table__header) {
  width: 100%;
}
:deep(.el-scrollbar__view) {
  width: 100%;
}
:deep(.el-table__body) {
  width: 100%;
}
:deep(.el-table__row) {
  width: 100%;
}
tbody {
  width: 100%;
}
</style>
