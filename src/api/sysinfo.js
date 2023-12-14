import { invoke } from '@tauri-apps/api';

const fetchCPUInfo = () => {
  return new Promise((resolve, reject) => {
    invoke('get_cpu_info')
      .then((dataStr) => {
        const data = JSON.parse(dataStr);
        resolve(data)
      })
      .catch((error) => {
        console.error('Error fetching CPU info:', error);
        reject(error)
      });
  });
};

export const useSysinfo = () => {
  return {
    fetchCPUInfo
  }
}