import { createPinia, defineStore } from 'pinia';
import { useSysinfo } from '@/api/sysinfo';
import { useSSH } from '@/api/ssh';

const { fetchRemoteAllInfo } = useSysinfo();
const { sshConnect, disconnectSsh } = useSSH();
const REMOTE_CONFIG = '_Remote-config';
const IS_ACTIVED = '_Is-acived';

export const useGlobalStore = defineStore({
  id: 'GlobalState',
  state: () => {
    return {
      isConnected: false,
      isSpecialMode: false,
      remoteConfig: {
        host: '',
        port: 22,
        user: 'root',
        password: ''
      },
      systemInfo: {
        cpuInfo: {},
        cpuDetail: [],
        memoryInfo: {},
        loadInfo: {},
        networksInfo: [],
        processInfo: [],
        diskInfo: '',
        diskDetail: [],
        gpuDetail: []
      },
      appPath: {
        ivauto_ivs_server: '/opt/ivauto_ivs_server/ivauto_ivs_server',
        ivauto_quality_detection: '/opt/ivauto_ivs_server/ivauto_quality_detection',
        ivauto_summary_server: '/opt/ivauto_ivs_server/ivauto_summary_server',
        StreamServer: '/opt/StreamServer/StreamServer',
        LB_intercom: '/opt/LB_intercom/LB_intercom',
      },
      gitInfoPath: {
        ivauto_ivs_server: '/opt/ivauto_ivs_server/ivs_ver.txt',
        ivauto_quality_detection: '/opt/ivauto_ivs_server/qd_ver.txt',
        StreamServer: '/opt/StreamServer/git_commit_version.txt',
        LB_intercom: '/opt/LB_intercom/git_commit_version.txt',
        model_zoo: '/opt/ivauto_ivs_server/model_ver.txt',
      },
      modelPath: {
        prison_rt: '/opt/ivauto_ivs_server/data/prison-rt',
        coeff_prison: '/opt/ivauto_ivs_server/data/coeff-prison',
      },
      streamPort: {
        rtsp: 554,
        http: 8096,
        rtmp: 2935,
      },
    }
  },
  getters: {},
  actions: {
    initLocalStorage() {
      if (!localStorage.getItem(REMOTE_CONFIG)) {
        localStorage.setItem(REMOTE_CONFIG, JSON.stringify({}));
      }
      if (!localStorage.getItem(IS_ACTIVED)) {
        localStorage.setItem(IS_ACTIVED, '');
      }
      this.setLocalAppPath(this.appPath);
      this.setLocalGitInfoPath(this.gitInfoPath);
      this.setLocalStreamPort(this.streamPort);

    },
    addLocalRemoteConfig(key, value) {
      let remoteConfigCache = this.getRemoteConfigCache();
      remoteConfigCache[key] = value;
      this.setLocalRemoteConfig(remoteConfigCache);
    },

    getLocalAppPath() {
      return JSON.parse(localStorage.getItem('appPath') || '{}');
    },
    getLocalGitInfoPath() {
      return JSON.parse(localStorage.getItem('gitInfoPath') || '{}');
    },
    getLocalStreamPort() {
      return JSON.parse(localStorage.getItem('streamPort') || '{}');
    },
    getActiveKey() {
      return IS_ACTIVED;
    },
    // 获取完整的 remote config 缓存
    getRemoteConfigCache() {
      return JSON.parse(localStorage.getItem(REMOTE_CONFIG));
    },
    getLocalRemoteConfig(id) {
      let remoteConfigCache = this.getRemoteConfigCache();
      return remoteConfigCache[id];
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
        this.systemInfo.gpuDetail = res.gpu_detail;
      }
    },
    async getRemoteConnection(remoteConfigObj) {
      const res = await sshConnect(remoteConfigObj.host, remoteConfigObj.port, remoteConfigObj.user, remoteConfigObj.password);
      if (res.code === 0) {
        this.isConnected = true;
        const id = `${remoteConfigObj.host}:${remoteConfigObj.port}:${remoteConfigObj.user}`;
        this.setActicedConntion(id);
        this.addLocalRemoteConfig(id, remoteConfigObj);
      }
      return res;
    },
    setActicedConntion(id) {
      localStorage.setItem(IS_ACTIVED, id);
    },
    setRemoteConfig(remoteConfigObj) {
      this.remoteConfig.host = remoteConfigObj.host;
      this.remoteConfig.port = remoteConfigObj.port;
      this.remoteConfig.user = remoteConfigObj.user;
      this.remoteConfig.password = remoteConfigObj.password;
    },
    setLocalRemoteConfig(value) {
      localStorage.setItem(REMOTE_CONFIG, JSON.stringify(value));
    },
    setSystemInfo(cpuInfo, memoryInfo, loadInfo, networksInfo) {
      this.systemInfo = {
        cpuInfo,
        memoryInfo,
        loadInfo,
        networksInfo
      }
    },
    setLocalAppPath(value) {
      localStorage.setItem('appPath', JSON.stringify(value));
    },
    setLocalGitInfoPath(value) {
      localStorage.setItem('gitInfoPath', JSON.stringify(value));
    },
    setLocalStreamPort(value) {
      localStorage.setItem('streamPort', JSON.stringify(value));
    },
    async disconnectSsh(host, port) {
      const res = await disconnectSsh(host, port);
      // Todo: 1. 所有ssh连接断开是才置为 false.  2. 活跃连接 ID 已被刷新,无需删除
      // if (res.code === 0) {
      //   this.isConnected = false;
      //   return res;
      // }
      // return { code: -1, message: `Unknown error: ${res.message}` };
      return res;
    },
    deleteRemoteConfig(key) {
      let remoteConfigCache = this.getRemoteConfigCache();
      delete remoteConfigCache[key];
      this.setLocalRemoteConfig(remoteConfigCache);
    }
  }
});

const pinia = createPinia();

export default pinia;