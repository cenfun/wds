<template>
  <div
    ref="el"
    class="wds-edit-modal"
    style="min-width: 320px;"
  >
    <table class="wds-table">
      <tbody>
        <tr>
          <td>{{ t("name") }}</td>
          <td>
            <VuiFlex
              gap="10px"
              padding="5px"
            >
              <VuiInput
                v-model="editor.data.name"
                class="wds-dir-name"
                width="180px"
              />
            </VuiFlex>
          </td>
        </tr>

        <tr>
          <td> {{ t("path") }}</td>
          <td>
            <VuiFlex
              gap="10px"
              padding="5px"
            >
              <VuiInput
                v-model="editor.data.path"
                class="wds-dir-path"
                width="180px"
              />

              <IconLabel
                icon="open"
                @click="onOpenFolder"
              />
            </VuiFlex>
          </td>
        </tr>

        <tr>
          <td> {{ t("permission") }}</td>
          <td>
            <VuiFlex
              gap="10px"
              padding="5px"
            >
              <VuiSelect v-model="editor.data.permission">
                <option>read</option>
                <option>write</option>
              </VuiSelect>
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
import {
    inject, ref, onMounted
} from 'vue';

import { components } from 'vine-ui';

import { open } from '@tauri-apps/plugin-dialog';
import { basename } from '@tauri-apps/api/path';

import { save_dir } from '../utils/api-private.js';
import { log } from '../utils/helper.js';
import { formatPath } from '../utils/util.js';

import IconLabel from './icon-label.vue';

import { useTranslation } from 'i18next-vue';
const { t } = useTranslation();

const emit = defineEmits(['updated']);

const {
    VuiButton,
    VuiFlex,
    VuiInput,
    VuiSelect
} = components;


const editor = inject('editor');
const state = inject('state');

const el = ref(null);
let $el;

// eslint-disable-next-line complexity
const onSaveClick = async () => {
    if (!editor.data.name) {
        $el.querySelector('.wds-dir-name input').focus();
        return;
    }
    if (!editor.data.path) {
        $el.querySelector('.wds-dir-path input').focus();
        return;
    }

    const action = editor.previous ? 'update' : 'create';

    if (action === 'update' && editor.previous) {
        const prev = editor.previous;
        if (prev.name === editor.data.name
            && prev.path === editor.data.path
            && prev.permission === editor.data.permission
        ) {
            log(t('no_changes'));
            editor.visible = false;
            return;
        }
    }

    // check duplicate dir name (case-insensitive)
    const profile = state.profile_list.find((p) => p.id === editor.profile_id);
    if (profile) {
        const newName = (editor.data.name || '').toLowerCase();
        const duplicate = profile.dir_list.some((d) => {
            if (action === 'update' && d.id === editor.previous?.id) {
                return false;
            }
            return (d.name || '').toLowerCase() === newName;
        });
        if (duplicate) {
            log(t('name_exists'), 'red');
            return;
        }
    }

    const id = editor.profile_id;
    const ok = await save_dir(action, id, editor.data);

    if (ok) {
        emit('updated');
    }

    editor.visible = false;

};

const onCancelClick = () => {
    editor.visible = false;
};

const onOpenFolder = async () => {
    const path = await open({
        multiple: false,
        directory: true
    });

    if (!path) {
        return;
    }

    const p = formatPath(path);
    editor.data.path = p;


    if (!editor.data.name) {
        editor.data.name = await basename(p);
    }

};

onMounted(() => {
    $el = el.value;
});

</script>
