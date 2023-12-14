<template>
  <el-footer>
    <el-row>
      <el-col :span="4">
        <div class="HostIcon"></div>
      </el-col>
      <el-col :span="20">
        <el-select v-model="selectedHost" class="m-2" :placeholder="selectedHost">
          <el-option
            v-for="(item, index) in options"
            :key="index"
            :label="item.IP"
            :value="item.Value"
            :disabled="!item.Available"
          />
        </el-select>
      </el-col>
    </el-row>

    <el-button plain @click="drawer = true">
      <el-icon class="el-icon--left"><Histogram /></el-icon>
      查看系统信息
    </el-button>
    <el-drawer v-model="drawer" direction="rtl">
      <template #header>
        <h4>系统信息</h4>
      </template>
      <template #default>
        <el-row :gutter="20">
          <el-col :sm="24" :xs="24" :md="24" :lg="11" v-for="(item, index) in items" :key="index" class="CardItem">
            <el-card class="box-card">
              <el-image class="CardImageHead" :src="item.Image" fit="contain" />
              <h3 v-cloak class="CardTitle">{{ item.Title }}</h3>
              <p class="CardValue">
                {{ item.Value }}
              </p>
            </el-card>
          </el-col>
        </el-row>
      </template>
    </el-drawer>
  </el-footer>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import image from '../../../assets/hamburger.png';

interface ItemData {
  Image: String;
  Title: String;
  Value: String;
}

interface Hosts {
  IP: String;
  Value: String;
  Available: Boolean;
}

const drawer = ref<Boolean>(false);

const selectedHost = ref<String | null>('');

const options = ref<Hosts[]>([
  {
    IP: '192.168.0.1',
    Value: '192.168.0.1',
    Available: true,
  },
  {
    IP: '192.168.0.2',
    Value: '192.168.0.2',
    Available: false,
  },
  {
    IP: '192.168.0.3',
    Value: '192.168.0.3',
    Available: false,
  },
  {
    IP: '192.168.0.4',
    Value: '192.168.0.4',
    Available: true,
  },
]);

const items = ref<ItemData[]>([
  {
    Image: image,
    Title: '内核信息',
    Value: '5.3.8',
  },
  {
    Image: image,
    Title: '内核信息',
    Value: '5.3.8',
  },
  {
    Image: image,
    Title: '内核信息',
    Value: '5.3.8',
  },
  {
    Image: image,
    Title: '内核信息',
    Value: '5.3.8',
  },
  {
    Image: image,
    Title: '内核信息',
    Value: '5.3.8',
  },
  {
    Image: image,
    Title: '内核信息',
    Value: '5.3.8',
  },
]);
</script>

<style scoped>
.el-footer {
  border-right: solid 1px var(--el-menu-border-color);
  height: 15%;
  margin-top: -35px;
  display: flex;
  flex-direction: column;
  justify-content: flex-start;

  .HostIcon {
    width: 16px;
    height: 16px;
    border-radius: 35px;
    margin: 7px auto;
    background-color: rgb(0, 187, 3);
  }

  .el-button {
    padding: 20px 25px;
    margin: 15px auto;
  }

  .CardItem {
    margin-bottom: 15px;

    .CardImageHead {
      min-width: 120px;
      max-width: 240px;
      margin: 0 auto;
    }

    .CardTitle,
    .CardValue {
      text-align: center;
    }
  }
}
</style>
