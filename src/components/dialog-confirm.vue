<template>
  <div>
    <div class="vui-confirm-message">
      {{ props.message }}
    </div>
    <div class="vui-confirm-buttons">
      <VuiButton
        primary
        width="80px"
        @click="onOkClick"
      >
        {{ t("ok") }}
      </VuiButton>
      <VuiButton
        width="80px"
        @click="onCancelClick"
      >
        {{ t("cancel") }}
      </VuiButton>
    </div>
  </div>
</template>
<script setup>
import { inject } from 'vue';
import { components } from 'vine-ui';

import { useTranslation } from 'i18next-vue';
const { t } = useTranslation();

const { VuiButton } = components;

const props = defineProps({
    message: {
        type: String,
        default: ''
    }
});

const dialog = inject('dialog');

const onOkClick = () => {
    dialog.visible = false;
    if (dialog.callback) {
        dialog.callback();
    }
    dialog.message = '';
    dialog.callback = null;
};

const onCancelClick = () => {
    dialog.visible = false;
    dialog.message = '';
    dialog.callback = null;
};

</script>
<style>
.vui-confirm-message {
    padding: 10px 0 20px;
}

.vui-confirm-buttons {
    display: flex;
    gap: 10px;
    justify-content: flex-end;
}

</style>
