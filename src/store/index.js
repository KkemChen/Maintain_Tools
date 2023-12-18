import { createPinia, defineStore } from 'pinia';
import { useSysinfo } from '@/api/sysinfo';
import { useSSH } from '@/api/ssh';

const { fetchRemoteCPUInfo, fetchRemoteMemoryInfo, fetchRemoteLoadInfo, fetchRemoteIoInfo, fetchRemoteDiskInfo, fetchRemoteProcessInfo } = useSysinfo();
const { sshConnect, disconnectSsh } = useSSH();

export const useGlobalStore = defineStore({
  id: 'GlobalState',
  state: () => {
    return {
      isConnected: false,
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
        networksInfo: [],
        processInfo: [],
        diskInfo: []
      }
    }
  },
  getters: {},
  actions: {
    getRemoteConfigStatus() {
      return this.remoteConfig.host.trim() === '';
    },
    async getSystemInfo() {
      if (this.isConnected) {
        const requestUrl = `${this.remoteConfig.host}:${this.remoteConfig.port}`;
        this.systemInfo.cpuInfo = await fetchRemoteCPUInfo(requestUrl);
        this.systemInfo.memoryInfo = await fetchRemoteMemoryInfo(requestUrl);
        this.systemInfo.loadInfo = await fetchRemoteLoadInfo(requestUrl);
        this.systemInfo.networksInfo = await fetchRemoteIoInfo(requestUrl);
        this.systemInfo.processInfo = await fetchRemoteProcessInfo(requestUrl);
        this.systemInfo.diskInfo = await fetchRemoteDiskInfo(requestUrl);
      }
    },
    async getRemoteConnection() {
      if (this.getRemoteConfigStatus()) {
        this.remoteConfig = JSON.parse(localStorage.getItem('remoteConfig'));
      }
      if (this.remoteConfig) {
        const res = await sshConnect(this.remoteConfig.host, this.remoteConfig.port, this.remoteConfig.user, this.remoteConfig.password);
        if (res.code === 0) {
          this.isConnected = true;
        }
        return res;
      }
      console.error('Local host value is null.');
      return { code: -1, message: 'Please check your host, it\'s null.' };
    },
    setRemoteConfig(host, port, user, password, sysInfoHttpPort = 9888) {
      this.remoteConfig = {
        host,
        port,
        user,
        password,
        sysInfoHttpPort
      };
      localStorage.setItem('remoteConfig', JSON.stringify(this.remoteConfig));
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
      this.isConnected = false;
    }
  }
});

const pinia = createPinia();

export default pinia;