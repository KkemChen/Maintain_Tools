<template>
    <el-row :gutter="10">
      <el-col :span="24"><div class="grid-content ep-bg-purple">
        <el-card class="box-card">
          <div v-for="o in 4" :key="o" class="text item">{{ 'List item ' + o }}</div>
        </el-card>
      </div></el-col>
    </el-row>
    <el-row :gutter="5">
      <el-col :span="8" ><div class="grid-content ep-bg-purple" >
        <el-card class="box-card">
          <div v-for="o in 4" :key="o" class="text item">{{ 'List item ' + o }}</div>
        </el-card>
      </div></el-col>
      <el-col :span="8" ><div class="grid-content ep-bg-purple" >
        <el-card class="box-card">
          <div v-for="o in 4" :key="o" class="text item">{{ 'List item ' + o }}</div>
        </el-card>
      </div></el-col>
      <el-col :span="8" ><div class="grid-content ep-bg-purple" >
        <el-card class="box-card">
          <div v-for="o in 4" :key="o" class="text item">{{ 'List item ' + o }}</div>
        </el-card>
      </div></el-col>
    </el-row>
    <el-row :gutter="10">
      <el-col :span="24"><div class="grid-content ep-bg-purple" >
        <el-card class="box-card">
          <div v-for="o in 4" :key="o" class="text item">{{ 'List item ' + o }}</div>
        </el-card>
      </div></el-col>
    </el-row>
  </template>
  
<style scoped>
.el-row {
  /* margin-bottom: 20px; */
  min-height: calc((calc(95%) - 15px) /3) ;
  margin-bottom: 5px;
}


.grid-content {
  border: none;
  height: 100%;
}

.box-card {
  width: 100%;
  height: 100%;
  background-color: lightgreen;
}


</style>
  