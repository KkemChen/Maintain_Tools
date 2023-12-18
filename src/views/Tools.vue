<script lang="ts" setup>
import { ref } from 'vue';
import type { TabsPaneContext } from 'element-plus';
import CheckStream from '@/components/tools/CheckStream.vue';

const activeName = ref('first');

const handleClick = (tab: TabsPaneContext, event: Event) => {
  console.log(tab, event);
};
</script>

<template>
  <el-tabs v-model="activeName" class="demo-tabs" @tab-click="handleClick">
    <el-tab-pane label="Check Stream" name="first"><CheckStream /></el-tab-pane>
    <el-tab-pane label="TODO" name="second">Config</el-tab-pane>
    <el-tab-pane label="TODO" name="third">Role</el-tab-pane>
    <el-tab-pane label="TODO" name="fourth">Task</el-tab-pane>
  </el-tabs>
</template>

<style>
.demo-tabs > .el-tabs__content {
  padding: 32px;
  color: #6b778c;
  font-size: 32px;
  font-weight: 600;
}
</style>
