<template>
  <div
    class="wds-edit-modal"
    style="min-width: 320px;"
  >
    <table class="wds-table">
      <tbody>
        <tr>
          <td>{{ t("port") }}</td>
          <td>
            <VuiFlex
              gap="10px"
              padding="5px"
            >
              <VuiInput
                v-model="editor.data.port"
                type="number"
                min="0"
                max="65535"
              />
              <div>(0~65535)</div>
            </VuiFlex>
          </td>
        </tr>
        <tr>
          <td />
          <td>
            <VuiFlex
              gap="10px"
              padding="5px"
            >
              <VuiButton
                width="80px"
                primary
                @click="onSaveClick"
              >
                {{ t("save") }}
              </VuiButton>
              <VuiButton
                width="80px"
                @click="onCancelClick"
              >
                {{ t("cancel") }}
              </VuiButton>
            </VuiFlex>
          </td>
        </tr>
      </tbody>
    </table>
  </div>
</template>
<script setup>
import { inject } from 'vue';

import { components } from 'vine-ui';

// import { open } from '@tauri-apps/plugin-dialog';

import { save_server_config } from '../utils/api-private.js';
import { log } from '../utils/helper.js';

import { useTranslation } from 'i18next-vue';
const { t } = useTranslation();

const emit = defineEmits(['updated']);

const {
    VuiButton,
    VuiFlex,
    VuiInput
} = components;


const editor = inject('editor');
const state = inject('state');

const onSaveClick = async () => {
    let port = editor.data.port;
    if (!port) {
        port = 8090;
    }

    if (port === state.port) {
        log(t('no_changes'));
        editor.visible = false;
        return;
    }

    const ok = await save_server_config({ port });
    if (ok) {
        emit('updated');
    }
    editor.visible = false;

};

const onCancelClick = () => {
    editor.visible = false;
};

</script>
