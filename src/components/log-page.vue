<template>
  <div class="wds-page wds-page-log vui-flex-column">
    <div class="wds-page-list wds-log-container vui-flex-auto" />
    <div class="wds-page-toolbar">
      <VuiFlex gap="10px">
        <IconLabel
          icon="clear"
          :label="t('clear')"
          @click="onClearClick"
        />

        <IconLabel
          icon="reload"
          :label="t('reload')"
          @click="onReloadClick"
        />

        <div class="vui-flex-empty" />

        <FpsDetector />

        <IconLabel
          icon="settings"
          size="20px"
          :tooltip="t('settings')"
          @click="onSettingsClick"
        />
      </VuiFlex>
    </div>
  </div>
</template>
<script setup>
import { inject } from 'vue';
import { components } from 'vine-ui';

import { log_clear } from '../utils/helper.js';

import IconLabel from './icon-label.vue';
import FpsDetector from './fps-detector.vue';

import { useTranslation } from 'i18next-vue';
const { t } = useTranslation();

const { VuiFlex } = components;

const state = inject('state');

const onClearClick = () => {
    log_clear();
};

const onReloadClick = () => {
    window.location.reload();
};

const onSettingsClick = () => {
    state.settings_visible = true;
};
</script>
<style lang="scss">
.wds-page-log {
    .wds-page-toolbar {
        padding: 5px 10px;
    }
}

.wds-log-container {
    height: 100%;
    padding: 5px;
    border-left: 1px solid #ddd;
    overflow: auto;

    > div {
        padding: 2px 0;
        word-break: break-all;
    }

    > canvas {
        display: block;
    }
}

</style>
