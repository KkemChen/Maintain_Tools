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
    const data = JSON.parse(json.data);
    return data;
  } catch (error) {
    console.error('Error fetching CPU info:', error);
    throw error;
  }
};

const fetchRemoteMemoryInfo = async (remoteHost) => {
  try {
    const res = await invoke('get_disk_info', { host: remoteHost });
    const json = JSON.parse(res);
    return json.data;
  } catch (error) {
    console.error('Error fetching Memory info:', error);
    throw error;
  }
}

const fetchRemoteLoadInfo = async (remoteHost) => {
  try {
    const res = await invoke('get_total_info', { host: remoteHost });
    const json = JSON.parse(res);
    const data = JSON.parse(json.data);

    return data.load_info;
  } catch (error) {
    console.error('Error fetching Load info:', error);
    throw error;
  }
}

const fetchRemoteIoInfo = async (remoteHost) => {
  try {
    const res = await invoke('get_net_info', { host: remoteHost });
    const json = JSON.parse(res);
    const data = JSON.parse(json.data);

    return data;
  } catch (error) {
    console.error('Error fetching Io info:', error);
    throw error;
  }
}


export const useSysinfo = () => {
  return {
    fetchLocalCPUInfo,
    fetchRemoteCPUInfo,
    fetchRemoteMemoryInfo,
    fetchRemoteLoadInfo,
    fetchRemoteIoInfo
  }
}