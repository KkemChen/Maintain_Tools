import { createPinia, defineStore } from 'pinia';
import { useSysinfo } from '@/api/sysinfo';
import { useSSH } from '@/api/ssh';

const { fetchRemoteAllInfo } = useSysinfo();
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
        cpuInfo: {},
        cpuDetail: [],
        memoryInfo: {},
        loadInfo: {},
        networksInfo: [],
        processInfo: [],
        diskInfo: '',
        diskDetail: []
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
        const res = await fetchRemoteAllInfo(requestUrl);
        this.systemInfo.cpuInfo = res.cpu_info;
        this.systemInfo.cpuDetail = res.cpu_detail;
        this.systemInfo.memoryInfo = res.mem_info;
        this.systemInfo.loadInfo = res.load_info;
        this.systemInfo.networksInfo = res.net_info;
        this.systemInfo.processInfo = res.process_info;
        this.systemInfo.diskInfo = res.disk_info;
        this.systemInfo.diskDetail = res.disk_detail;
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