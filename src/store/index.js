import { createPinia, defineStore } from 'pinia';
import { useSysinfo } from '@/api/sysinfo';
import { useSSH } from '@/api/ssh';

const { fetchRemoteCPUInfo, fetchRemoteMemoryInfo, fetchRemoteLoadInfo, fetchRemoteIoInfo } = useSysinfo();
const { sshConnect, disconnectSsh } = useSSH();

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
      const requestUrl = `http://${this.host}:${this.port}`;
      this.systemInfo.cpuInfo = await fetchRemoteCPUInfo(requestUrl);
      this.systemInfo.memoryInfo = await fetchRemoteMemoryInfo(requestUrl);
      this.systemInfo.loadInfo = await fetchRemoteLoadInfo(requestUrl);
      this.systemInfo.networksInfo = await fetchRemoteIoInfo(requestUrl);
      //Todo fetch Disk Info
    },
    async getRemoteConnection() {
      await sshConnect(this.remoteConfig.host, this.remoteConfig.port, this.remoteConfig.user, this.remoteConfig.password);
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
    },
    async disconnectSsh() {
      await disconnectSsh(this.remoteConfig.host, this.remoteConfig.port);
    }
  }
});

const pinia = createPinia();

export default pinia;