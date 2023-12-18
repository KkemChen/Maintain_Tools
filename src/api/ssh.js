import { invoke } from '@tauri-apps/api';

const sshConnect = async (host, port, user, password) => {
  try {
    const response = await invoke('add_ssh_connect', {
      host: `${host}:${port}`,
      user: user,
      password: password,
    });

    console.log('SSH connection initialized', response);
    const data = JSON.parse(response);
    return data;
  } catch (error) {
    console.error('Error in SSH connection:', error);
    throw error;
  }
};

const disconnectSsh = async (host, port) => {
  try {
    const response = await invoke('disconnect_ssh', {
      host: `${host}:${port}`,
    });

    console.log('Disconnect ssh success: ', response);
    return response;
  } catch (error) {
    console.error('Disconnect ssh failed: ', error);
    throw error;
  }
};

export const useSSH = () => {
  return {
    sshConnect,
    disconnectSsh
  };
};
