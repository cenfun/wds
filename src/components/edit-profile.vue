<template>
  <div
    class="wds-edit-modal"
    style="min-width: 320px;"
  >
    <table class="wds-table">
      <tbody>
        <tr>
          <td>{{ t("username") }}</td>
          <td>
            <VuiInput
              v-model="editor.data.username"
              :placeholder="t('anonymous')"
              width="180px"
            />
          </td>
        </tr>

        <tr>
          <td> {{ t("password") }}</td>
          <td>
            <VuiInput
              v-model="editor.data.password"
              type="password"
              width="180px"
            />
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

import { save_profile } from '../utils/api-private.js';
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

const onSaveClick = async () => {

    const action = editor.previous ? 'update' : 'create';

    if (action === 'update' && editor.previous) {
        const prev = editor.previous;
        if (prev.username === editor.data.username
            && prev.password === editor.data.password
        ) {
            log(t('no_changes'));
            editor.visible = false;
            return;
        }
    }

    const ok = await save_profile(action, editor.data);

    if (ok) {
        emit('updated');
    }

    editor.visible = false;

};

const onCancelClick = () => {
    editor.visible = false;
};

</script>
