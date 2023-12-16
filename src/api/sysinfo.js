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


const fetchRemoteCPUInfo = async (requestUrl) => {
  return new Promise((resolve, reject) => {
    fetch(requestUrl + '/cpus')
      .then(async res => {
        const data = await res.json();
        resolve(data.cpu_info)
      })
      .catch(error => {
        console.error('Error fetching CPU info:', error);
        reject(error)
      })
  })
};

const fetchRemoteMemoryInfo = async (requestUrl) => {
  return new Promise((resolve, reject) => {
    fetch(requestUrl + '/memory')
      .then(async res => {
        const data = await res.json();
        resolve(data)
      })
      .catch(error => {
        console.error('Error fetching Memory info:', error);
        reject(error)
      })
  })
}

const fetchRemoteLoadInfo = async (requestUrl) => {
  return new Promise((resolve, reject) => {
    fetch(requestUrl + '/load_average')
      .then(async res => {
        const data = await res.json();
        resolve(data)
      })
      .catch(error => {
        console.error('Error fetching Load info:', error);
        reject(error)
      })
  })
}

const fetchRemoteIoInfo = async (requestUrl) => {
  return new Promise((resolve, reject) => {
    fetch(requestUrl + '/networks')
      .then(async res => {
        const data = await res.json();
        resolve(data)
      })
      .catch(error => {
        console.error('Error fetching Io info:', error);
        reject(error)
      })
  })
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