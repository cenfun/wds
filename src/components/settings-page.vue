<template>
  <div class="wds-page wds-page-settings vui-flex-column">
    <div class="wds-setting-header">
      <IconLabel
        icon="language"
        size="20px"
        :button="false"
        :label="t('languages')"
      />
    </div>
    <div class="wds-setting-body">
      <VuiRadio
        v-model="state.language"
        value="en"
        name="language"
      >
        English
      </VuiRadio>
      <VuiRadio
        v-model="state.language"
        value="zh"
        name="language"
      >
        简体中文
      </VuiRadio>
    </div>
    <div class="wds-setting-header" />
  </div>
</template>
<script setup>
import { inject, watch } from 'vue';
import { components } from 'vine-ui';

import IconLabel from './icon-label.vue';

import { useTranslation } from 'i18next-vue';
const { t, i18next } = useTranslation();

const { VuiRadio } = components;

const state = inject('state');

watch(() => state.language, () => {
    const lng = state.language;
    console.log(`update language to: ${lng}`);
    document.cookie = `i18next=${lng}`;
    i18next.changeLanguage(lng);
});

</script>
<style lang="scss">
.wds-page-settings {
    position: relative;
    padding: 10px;
    overflow-y: auto;
}

.wds-setting-header {
    margin-bottom: 5px;
    padding-bottom: 5px;
    font-weight: bold;
    border-bottom: 1px solid #ddd;
}

.wds-setting-body {
    margin-left: 20px;
}

</style>
