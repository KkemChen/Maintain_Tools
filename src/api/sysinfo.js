import { invoke } from '@tauri-apps/api';

const fetchLocalCPUInfo = () => {
  return new Promise((resolve, reject) => {
    invoke('get_cpu_info')
      .then((res) => {
        const data = JSON.parse(res);
        resolve(data)
      })
      .catch((error) => {
        console.error('Error fetching CPU info:', error);
        reject(error)
      });
  });
};


const fetchRemoteCPUInfo = async (remoteHost) => {
  return new Promise((resolve, reject) => {
    invoke('get_cpu_detail', { host: remoteHost })
      .then((res) => {
        const json = JSON.parse(res);
        const data = JSON.parse(json.data);
        resolve(data)
      })
      .catch((error) => {
        console.error('Error fetching CPU info:', error);
        reject(error)
      });
  });
};

const fetchRemoteMemoryInfo = async (remoteHost) => {
  return new Promise((resolve, reject) => {
    invoke('get_disk_info', { host: remoteHost })
      .then((res) => {
        const json = JSON.parse(res);
        // const data = JSON.parse(json.data);
        resolve(json.data)
      })
      .catch((error) => {
        console.error('Error fetching CPU info:', error);
        reject(error)
      });
  });
}

const fetchRemoteLoadInfo = async (remoteHost) => {
  return new Promise((resolve, reject) => {
    invoke('get_total_info', { host: remoteHost })
      .then((res) => {
        const json = JSON.parse(res);
        const data = JSON.parse(json.data);

        resolve(data.load_info)
      })
      .catch((error) => {
        console.error('Error fetching IO info:', error);
        reject(error)
      });
  });
}

const fetchRemoteIoInfo = async (remoteHost) => {
  return new Promise((resolve, reject) => {
    invoke('get_net_info', { host: remoteHost })
      .then((res) => {
        const json = JSON.parse(res);
        const data = JSON.parse(json.data);
        // console.log(data);
        resolve(data)
      })
      .catch((error) => {
        console.error('Error fetching IO info:', error);
        reject(error)
      });
  });
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