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
    console.log(json);
    return json.data;
  } catch (error) {
    console.error('Error fetching all info:', error);
    throw error;
  }
}

const checkStream = async (remoteHost, url) => {
  try {
    const res = await invoke('check_stream', { host: remoteHost, url: url });
    const json = JSON.parse(res);
    return json;
  } catch (error) {
    console.error('Error fetching Process info:', error);
  }
}

export const useSysinfo = () => {
  return {
    fetchLocalCPUInfo,
    fetchRemoteAllInfo,
    checkStream
  }
}