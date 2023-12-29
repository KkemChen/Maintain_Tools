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

const fetchRemoteAllInfo = async (remoteHost) => {
  try {
    const res = await invoke('get_sysinfo', { host: remoteHost });
    const json = JSON.parse(res);
    return json.data;
  } catch (error) {
    console.error('Error fetching all info:', error);
    throw error;
  }
}





export const useSysinfo = () => {
  return {
    fetchLocalCPUInfo,
    fetchRemoteAllInfo,

  }
}