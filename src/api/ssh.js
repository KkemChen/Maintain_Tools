import { invoke } from '@tauri-apps/api';

const sshConnect = (host, port, user, password) => {
  return new Promise((resolve, reject) => {
    invoke('add_ssh_connect', {
      host: `${host}:${port}`,
      user: user,
      password: password,
    })
      .then((response) => {
        console.log('SSH connection initialized', response);
        resolve(response);
      })
      .catch((error) => {
        console.error('Error fetching CPU info:', error);
        reject(response);
      });
  });
};

const disconnectSsh = (host, port) => {
  return new Promise((resolve, reject) => {
    invoke('disconnect_ssh', {
      host: `${host}:${port}`,
    })
      .then((response) => {
        console.log('Disconnect ssh success: ', response);
        resolve(response);
      })
      .catch((error) => {
        console.error('Disconnect ssh failed: ', error);
        reject(response);
      });
  });
};

export const useSSH = () => {
  return {
    sshConnect,
    disconnectSsh
  };
};
