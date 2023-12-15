import { invoke } from '@tauri-apps/api';

// const requestUrl = 'http://' + localStorage.getItem(host) + ':9888';
const requestUrl = 'http://' + '192.168.1.172' + ':9888';

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


const fetchRemoteCPUInfo = async () => {
  try {
    const response = await fetch(requestUrl + '/cpus');
    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`);
    }
    const data = await response.json();
    return data.cpu_info;
  } catch (error) {
    console.error('Error fetching CPU info:', error);
    throw error;
  }
};

const fetchRemoteMemoryInfo = async () => {
  try {
    const response = await fetch(requestUrl + '/memory');
    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`);
    }
    const data = await response.json();
    return data;
  } catch (error) {
    console.error('Error fetching Memory info:', error);
    throw error;
  }
}

const fetchRemoteLoadInfo = async () => {
  try {
    const response = await fetch(requestUrl + '/load_average');
    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`);
    }
    const data = await response.json();
    return data;
  } catch (error) {
    console.error('Error fetching Load info:', error);
    throw error;
  }
}

const fetchRemoteIoInfo = async () => {
  try {
    const response = await fetch(requestUrl + '/networks');
    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`);
    }
    const data = await response.json();
    return data;
  } catch (error) {
    console.error('Error fetching Io info:', error);
    throw error;
  }
}


export const useSysinfo = () => {
  return {
    fetchCPUInfo,
    fetchRemoteCPUInfo,
    fetchRemoteMemoryInfo,
    fetchRemoteLoadInfo,
    fetchRemoteIoInfo
  }
}