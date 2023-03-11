<template>
  <div class="vui-app">
    <VuiLayout
      direction="column"
      height="100%"
      gutter-hover-size="2px"
      gutter-size="2px"
      class="wds-layout"
    >
      <div
        class="wds-layout-top vui-flex-column"
        size="68.2%"
      >
        <ProfilePage
          @updated="onUpdated"
          @restart="onRestart"
        />
      </div>

      <div class="wds-layout-bottom vui-flex-column">
        <LogPage />
      </div>
    </VuiLayout>

    <VuiModal
      v-model="state.settings_visible"
      :title="t('settings')"
    >
      <SettingsPage />
    </VuiModal>

    <VuiDialog
      v-model="dialog.visible"
      :message="dialog.message"
    >
      <DialogConfirm />
    </VuiDialog>

    <VuiTooltip
      :visible="tooltip.visible"
      :target="tooltip.target"
      :text="tooltip.text"
    />
  </div>
</template>
<script setup>
import {
    onMounted, reactive, provide
} from 'vue';

import { components, generateTooltips } from 'vine-ui';

import { listen } from '@tauri-apps/api/event';

import { log } from './utils/helper.js';

import { get_settings, restart } from './utils/api-private.js';

import ProfilePage from './components/profile-page.vue';

import LogPage from './components/log-page.vue';
import SettingsPage from './components/settings-page.vue';

import DialogConfirm from './components/dialog-confirm.vue';

import { useTranslation } from 'i18next-vue';

const { t, i18next } = useTranslation();

const {
    VuiLayout,
    VuiModal,
    VuiTooltip,
    VuiDialog
} = components;

// =====================================================================================

const state = reactive({

    language: i18next.language,

    port: '',
    profile_list: [],

    settings_visible: false

});

provide('state', state);

const dialog = reactive({
    visible: false,
    message: '',
    callback: null
});

provide('dialog', dialog);

const tooltip = reactive({
    visible: false,
    target: null,
    text: ''
});

provide('tooltip', tooltip);

let timeout_tooltip;
const initTooltip = () => {
    generateTooltips((target, text) => {
        clearTimeout(timeout_tooltip);

        tooltip.visible = true;
        tooltip.target = target;
        tooltip.text = text;

        timeout_tooltip = setTimeout(() => {
            tooltip.visible = false;
            tooltip.text = '';
        }, 2000);

    }, (target) => {
        clearTimeout(timeout_tooltip);
        tooltip.visible = false;
        tooltip.text = '';
    });
};

// =====================================================================================

const loadSettings = async () => {

    const settings = await get_settings();
    if (!settings) {
        return;
    }

    console.log(settings);

    Object.assign(state, settings);

};

const onUpdated = async () => {
    await loadSettings();
    await restart();
};

const onRestart = async () => {
    await restart();
};

// =====================================================================================

onMounted(() => {

    // log message
    listen('message', (e) => {
        const payload = e.payload;
        if (!payload) {
            log('invalid message', 'red');
            return;
        }
        if (typeof payload === 'object') {
            const { color, value } = payload;
            if (value) {
                log(value, color);
                return;
            }
        }
        log(payload);
    });

    loadSettings();

    initTooltip();

});

// window.addEventListener('popstate', (e) => {

// });

// window.addEventListener('keydown', (e) => {
//     if (e.code === 'Escape') {
//         console.log(e);
//     }
// });

</script>
<style lang="scss">
html,
body {
    width: 100%;
    height: 100%;
    margin: 0;
    padding: 0;
    font-size: 14px;
    font-family: Helvetica, Arial, sans-serif;
}

/*
layout
*/

.vui-app {
    position: relative;
    width: 100%;
    height: 100%;
    color: #333;
}

.wds-layout {
    background-color: #fff;
}

.wds-layout-top {
    position: relative;
    border-bottom: 1px solid #ccc;

    .vui-flyover {
        position: absolute;
    }
}

.wds-layout-bottom {
    position: relative;
}

/*
pane
*/

.wds-page {
    height: 100%;
}

.wds-page-header {
    padding: 0 10px;
    font-weight: bold;
    border-top: 1px solid #eee;
    border-bottom: 1px solid #eee;
    background: #eee;

    .wds-icon-label label {
        max-width: 300px;
        white-space: nowrap;
        text-overflow: ellipsis;
        overflow: hidden;
    }
}

.wds-page-list {
    padding: 10px;
    overflow-x: hidden;
    overflow-y: auto;
}

.wds-page-toolbar {
    padding: 10px;
    border-top: 1px solid #eee;
    background: #f5f5f5;
}

/*
table
*/

.wds-table {
    width: 100%;
    border-collapse: collapse;

    th {
        white-space: nowrap;
        background-color: #eee;
    }

    td,
    th {
        padding: 5px;
        border: 1px solid #ddd;
    }

    .wds-table-left {
        text-align: left;
    }
}

</style>
