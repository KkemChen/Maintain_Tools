import { invoke } from '@tauri-apps/api';

const checkStream = async (remoteHost, url) => {
    try {
        const res = await invoke('check_stream', { host: remoteHost, url: url });
        const json = JSON.parse(res);
        return json;
    } catch (error) {
        console.error('Error fetching Process info:', error);
    }
}

const get_md5 = async (remoteHost, path) => {
    try {
        const res = await invoke('get_md5', { host: remoteHost, path: path });
        const json = JSON.parse(res);
        return json;
    } catch (error) {
        console.error('Error get_md5:', error);
    }
}

const get_ver_info = async (remoteHost, path) => {
    try {
        const res = await invoke('get_ver_info', { host: remoteHost, path: path });
        const json = JSON.parse(res);
        return json;
    } catch (error) {
        console.error('Error get_commit_hash:', error);
    }
}



export const useTools = () => {
    return {
        get_md5,
        get_ver_info,
        checkStream,
    }
}