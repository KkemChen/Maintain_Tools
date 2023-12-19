import { invoke } from '@tauri-apps/api';

const fetchLocalCPUInfo = async () => {
  try {
    const res = await invoke('get_cpu_info');
    const data = JSON.parse(res);
    return data;
  } catch (error) {
    console.error('Error fetching CPU info:', error);
    throw error;
  }
};


const fetchRemoteCPUInfo = async (remoteHost) => {
  try {
    const res = await invoke('get_cpu_detail', { host: remoteHost });
    const json = JSON.parse(res);
    return json.data;
  } catch (error) {
    console.error('Error fetching CPU info:', error);
    throw error;
  }
};

const fetchRemoteMemoryInfo = async (remoteHost) => {
  try {
    const res = await invoke('get_mem_info', { host: remoteHost });
    const json = JSON.parse(res);
    return json.data;
  } catch (error) {
    console.error('Error fetching Memory info:', error);
    throw error;
  }
}

const fetchRemoteLoadInfo = async (remoteHost) => {
  try {
    const res = await invoke('get_load_info', { host: remoteHost });
    const json = JSON.parse(res);
    return json.data;
  } catch (error) {
    console.error('Error fetching Load info:', error);
    throw error;
  }
}

const fetchRemoteIoInfo = async (remoteHost) => {
  try {
    const res = await invoke('get_net_info', { host: remoteHost });
    const json = JSON.parse(res);
    return json.data;
  } catch (error) {
    console.error('Error fetching net info:', error);
    throw error;
  }
}

const fetchRemoteDiskDetail = async (remoteHost) => {
  try {
    const res = await invoke('get_disk_detail', { host: remoteHost });
    const json = JSON.parse(res);
    return json.data;
  } catch (error) {
    console.error('Error fetching Disk info:', error);
  }
}

const fetchRemoteDiskInfo = async (remoteHost) => {
  try {
    const res = await invoke('get_disk_info', { host: remoteHost });
    const json = JSON.parse(res);
    console.log(json.data);
    return json.data;
  } catch (error) {
    console.error('Error fetching Disk info:', error);
  }
}

const fetchRemoteProcessInfo = async (remoteHost) => {
  try {
    const res = await invoke('get_process_info', { host: remoteHost });
    const json = JSON.parse(res);
    return json.data;
  } catch (error) {
    console.error('Error fetching Process info:', error);
  }
}

export const useSysinfo = () => {
  return {
    fetchLocalCPUInfo,
    fetchRemoteCPUInfo,
    fetchRemoteMemoryInfo,
    fetchRemoteLoadInfo,
    fetchRemoteIoInfo,
    fetchRemoteDiskDetail,
    fetchRemoteProcessInfo,
    fetchRemoteDiskInfo
  }
}