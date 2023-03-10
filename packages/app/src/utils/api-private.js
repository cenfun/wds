import { invoke } from '@tauri-apps/api/tauri';

import { log, log_duration } from './helper.js';

export const invoke_api = async (name, args = {}, duration = true) => {
    const time_start = Date.now();

    // log(`invoke_api: ${name}`, 'cyan');

    let failed = false;
    const res = await invoke(`invoke_${name}`, args).catch((e) => {
        const msg = e.message || e;
        log(`${name}: ${msg}`, 'red');
        failed = true;
    });
    if (duration) {
        log_duration(name, time_start);
    }

    if (failed) {
        // console.log('failed');
        return;
    }
    return res;
};

export const get_settings = () => {
    return invoke_api('get_settings');
};

export const save_port = (port) => {
    return invoke_api('save_port', {
        port
    });
};


export const save_profile = (action, data) => {
    return invoke_api('save_profile', {
        action,
        data
    });
};

export const save_dir = (action, id, data) => {
    return invoke_api('save_dir', {
        action,
        id,
        data
    });
};


export const restart = () => {
    return invoke_api('restart');
};
