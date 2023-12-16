import { createPinia, defineStore } from 'pinia';
import { useSysinfo } from '@/api/sysinfo';

const { fetchRemoteCPUInfo, fetchRemoteMemoryInfo, fetchRemoteLoadInfo, fetchRemoteIoInfo } = useSysinfo();

export const useGlobalStore = defineStore({
  id: 'GlobalState',
  state: () => {
    return {
      remoteConfig: {
        host: '',
        port: 0,
        user: '',
        password: '',
        sysInfoHttpPort: 9888 //默认请求远端主机的9888端口获取sysinfo信息
      },
      systemInfo: {
        cpuInfo: [],
        memoryInfo: [],
        loadInfo: [],
        networksInfo: []
      }
    }
  },
  getters: {},
  actions: {
    async getSystemInfo() {
      this.systemInfo.cpuInfo = await fetchRemoteCPUInfo();
      this.systemInfo.memoryInfo = await fetchRemoteMemoryInfo();
      this.systemInfo.loadInfo = await fetchRemoteLoadInfo();
      this.systemInfo.networksInfo = await fetchRemoteIoInfo();
      //Todo fetch Disk Info
    },
    setRemoteConfig(host, port, user, password, sysInfoHttpPort = 9888) {
      this.remoteConfig = {
        host,
        port,
        user,
        password,
        sysInfoHttpPort
      };
    },
    setSystemInfo(cpuInfo, memoryInfo, loadInfo, networksInfo) {
      this.systemInfo = {
        cpuInfo,
        memoryInfo,
        loadInfo,
        networksInfo
      }
    }
  }
});

const pinia = createPinia();

export default pinia;