let $logs;
let timeout_log;
const max_logs = 100;
export const log = (str, color) => {
    if (str && typeof str === 'object') {
        // remove "" for rust struct
        str = JSON.stringify(str, null, 4).replace(/"/g, '');
    }

    const $line = document.createElement('div');
    if (color) {
        $line.style.color = color;
    }
    $line.innerHTML = str;

    if (!$logs) {
        $logs = document.querySelector('.wds-log-container');
    }

    $logs.appendChild($line);

    $logs.scrollTo({
        top: $logs.scrollHeight,
        behavior: 'auto'
    });

    clearTimeout(timeout_log);
    timeout_log = setTimeout(() => {
        const len = $logs.children.length;
        if (len <= max_logs) {
            return;
        }

        const removeLen = len - max_logs;
        if (removeLen < 5) {
            return;
        }

        const list = [];
        for (let i = 0; i < removeLen; i++) {
            list.push($logs.children[i]);
        }
        list.forEach((c) => {
            c.remove();
        });

    }, 500);
};

export const log_duration = (msg, time_start) => {
    const duration = Date.now() - time_start;
    log(`${msg} 【${duration}ms】`);
};


export const log_clear = () => {
    if ($logs) {
        $logs.innerHTML = '';
    }
};
