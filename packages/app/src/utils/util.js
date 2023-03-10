
export const toNum = function(num, toInt) {
    if (typeof (num) !== 'number') {
        num = parseFloat(num);
    }
    if (isNaN(num)) {
        num = 0;
    }
    if (toInt) {
        num = Math.round(num);
    }
    return num;
};

export const zero = function(s, l = 2) {
    s = `${s}`;
    return s.padStart(l, '0');
};

export const clamp = function(num, min, max) {
    return Math.max(Math.min(num, max), min);
};

export const isList = function(data) {
    if (data && data instanceof Array && data.length > 0) {
        return true;
    }
    return false;
};

export const toList = function(data) {
    if (typeof (data) === 'undefined') {
        return [];
    }
    if (data instanceof Array) {
        return data;
    }
    return [data];
};

export const formatPath = function(str) {
    if (str) {
        str = str.replace(/\\/g, '/');
    }
    return str;
};

export const BF = function(v, digits = 1, base = 1024) {
    v = toNum(v, true);
    if (v === 0) {
        return '0B';
    }
    let prefix = '';
    if (v < 0) {
        v = Math.abs(v);
        prefix = '-';
    }
    const units = ['B', 'KB', 'MB', 'GB', 'TB', 'PB', 'EB', 'ZB', 'YB'];
    for (let i = 0, l = units.length; i < l; i++) {
        const min = Math.pow(base, i);
        const max = Math.pow(base, i + 1);
        if (v > min && v < max) {
            const unit = units[i];
            v = `${prefix + (v / min).toFixed(digits)} ${unit}`;
            break;
        }
    }
    return v;
};

export const NF = function(v) {
    v = toNum(v);
    return v.toLocaleString();
};

// =============================================================================
// hash
export const getHash = function(key) {
    let hash = {};
    const h = location.hash.slice(1);
    if (h) {
        const usp = new URLSearchParams(h);
        hash = Object.fromEntries(usp);
    }
    if (key) {
        return hash[key];
    }
    return hash;
};

export const setHash = function(key, value) {
    if (!key) {
        return;
    }
    let obj = key;
    if (arguments.length === 2) {
        obj = {};
        obj[key] = value;
    }
    const hash = getHash();
    Object.keys(obj).forEach((k) => {
        hash[k] = obj[k];
    });
    const usp = new URLSearchParams(hash);
    location.hash = usp.toString();
};

export const delHash = function(key) {
    if (!key) {
        location.hash = '';
        return;
    }
    let list = key;
    if (!Array.isArray(key)) {
        list = [key];
    }
    const hash = getHash();
    list.forEach((k) => {
        delete hash[k];
    });
    const usp = new URLSearchParams(hash);
    location.hash = usp.toString();
};
