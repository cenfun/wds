<template>
  <div class="wds-page wds-page-profile vui-flex-column">
    <div class="wds-page-header">
      <VuiFlex
        gap="10px"
      >
        <IconLabel
          icon="location"
          :label="t('address')+':'"
          :button="false"
        />
        <div class="wds-address">
          {{ state.address }}
        </div>

        <div class="vui-flex-empty" />

        <IconLabel
          icon="edit"
          :label="t('edit_port')"
          @click="onEditPortClick"
        />

        <IconLabel
          icon="restart"
          :label="t('restart')"
          @click="onRestartClick"
        />
      </VuiFlex>
    </div>
    <div class="wds-page-list wds-profile-list vui-flex-auto">
      <div v-if="isList(state.profile_list)">
        <table class="wds-table">
          <tbody
            v-for="(item, i) in state.profile_list"
            :key="i"
            class="wds-profile-item"
          >
            <tr>
              <th colspan="3">
                <VuiFlex gap="10px">
                  <IconLabel
                    icon="user"
                    :button="false"
                  />
                  <div>{{ t('username') }}:</div>
                  <div class="wds-username">
                    {{ item.username || t('anonymous') }}
                  </div>
                  <div>{{ t('password') }}:</div>
                  <VuiFlex
                    v-if="item.password"
                    gap="10px"
                  >
                    <div
                      v-if="item.see_password"
                      class="wds-password"
                    >
                      {{ item.password }}
                    </div>
                    <div v-else>
                      {{ "".padEnd(item.password.length,"*") }}
                    </div>
                    <IconLabel
                      icon="eye"
                      @click="item.see_password=!item.see_password"
                    />
                  </VuiFlex>
                </VuiFlex>
              </th>
              <th>
                <VuiFlex gap="10px">
                  <div class="vui-flex-auto" />
                  <IconLabel
                    icon="edit"
                    :tooltip="t('edit_profile')"
                    @click="onEditProfileClick(item)"
                  />

                  <IconLabel
                    icon="delete"
                    :tooltip="t('delete')"
                    @click="onDeleteProfileClick(item)"
                  />
                </VuiFlex>
              </th>
            </tr>

            <tr
              v-for="(dir, j) in item.dir_list"
              :key="j"
              class="wds-dir-item"
            >
              <td>
                <IconLabel
                  icon="folder"
                  :label="dir.name"
                  @click="onEditDirClick(item,dir)"
                />
              </td>
              <td>
                <div class="wds-dir-path">
                  {{ dir.path }}
                </div>
              </td>
              <td>
                <div class="wds-dir-permission">
                  {{ dir.permission }}
                </div>
              </td>
              <td>
                <VuiFlex
                  gap="10px"
                >
                  <div class="vui-flex-auto" />
                  <IconLabel
                    icon="edit"
                    :tooltip="t('edit_dir')"
                    @click="onEditDirClick(item,dir)"
                  />

                  <IconLabel
                    icon="delete"
                    :tooltip="t('delete')"
                    @click="onDeleteDirClick(item,dir)"
                  />
                </VuiFlex>
              </td>
            </tr>
            <tr>
              <td colspan="4">
                <VuiFlex
                  gap="10px"
                  class="wds-dir-add"
                >
                  <IconLabel
                    icon="folder-add"
                    :label="t('add_dir')"
                    @click="onAddDirClick(item)"
                  />

                  <div class="vui-flex-empty" />
                </VuiFlex>
              </td>
            </tr>
            <tr>
              <td
                colspan="2"
                class="wds-profile-space"
              />
            </tr>
          </tbody>
        </table>
      </div>
      <VuiButton
        v-else
        primary
        @click="onAddProfileClick"
      >
        {{ t('add_profile') }}
      </VuiButton>
    </div>

    <div class="wds-page-toolbar">
      <IconLabel
        icon="user-add"
        :label="t('add_profile')"
        @click="onAddProfileClick"
      />
    </div>

    <VuiModal
      v-model="editor.visible"
      :title="editor.title"
    >
      <EditProfile
        v-if="editor.type==='profile'"
        @updated="onUpdated"
      />
      <EditPort
        v-if="editor.type==='port'"
        @updated="onUpdated"
      />
      <EditDir
        v-if="editor.type==='dir'"
        @updated="onUpdated"
      />
    </VuiModal>
  </div>
</template>
<script setup>
import {
    inject, reactive, provide, toRaw
} from 'vue';

import { components } from 'vine-ui';

// import { open } from '@tauri-apps/api/dialog';

import { isList } from '../utils/util.js';
// import { log } from '../utils/helper.js';

import { save_profile, save_dir } from '../utils/api-private.js';

import IconLabel from './icon-label.vue';

import EditProfile from './edit-profile.vue';
import EditPort from './edit-port.vue';
import EditDir from './edit-dir.vue';


import { useTranslation } from 'i18next-vue';
const { t } = useTranslation();

const emit = defineEmits(['updated', 'restart']);

const {
    VuiButton,
    VuiFlex,
    VuiModal
} = components;


const state = inject('state');

const dialog = inject('dialog');

const editor = reactive({
    title: '',
    data: {
        dir_list: []
    },
    previous: null,
    visible: false,
    window_list: []
});
provide('editor', editor);

// =====================================================================================
const onAddDirClick = (item) => {
    editor.type = 'dir';
    editor.profile_id = item.id;
    editor.data = {
        permission: 'read'
    };
    editor.previous = null;
    editor.title = t('add_dir');
    editor.visible = true;
};

const onEditDirClick = (item, dir) => {
    editor.type = 'dir';
    editor.profile_id = item.id;
    editor.data = JSON.parse(JSON.stringify(toRaw(dir)));
    editor.previous = dir;
    editor.title = t('edit_dir');
    editor.visible = true;
};

const onDeleteDirClick = (item, dir) => {
    dialog.message = t('delete_confirm', {
        target: dir.name
    });
    dialog.visible = true;
    dialog.callback = async () => {
        const ok = await save_dir('delete', item.id, dir);
        if (ok) {
            onUpdated();
        }
    };
};


// =====================================================================================

const onAddProfileClick = () => {
    editor.type = 'profile';
    editor.data = {};
    editor.previous = null;
    editor.title = t('add_profile');
    editor.visible = true;
};

const onEditProfileClick = (previous) => {
    editor.type = 'profile';
    editor.data = {
        id: previous.id,
        username: previous.username,
        password: previous.password
    };
    editor.previous = previous;
    editor.title = t('edit_profile');
    editor.visible = true;
};

const onDeleteProfileClick = (item) => {
    dialog.message = t('delete_confirm', {
        target: item.username || t('anonymous')
    });
    dialog.visible = true;
    dialog.callback = async () => {
        const ok = await save_profile('delete', item);
        if (ok) {
            onUpdated();
        }
    };
};

// =====================================================================================

const onEditPortClick = () => {
    editor.type = 'port';
    editor.data = {
        port: toRaw(state.port)
    };
    editor.title = t('edit_port');
    editor.visible = true;
};

const onRestartClick = () => {
    emit('restart');
};

const onUpdated = () => {
    emit('updated');
};

</script>

<style lang="scss">
.wds-profile-item tr:hover {
    background-color: rgb(66 129 236 / 5%);
}

.wds-table {
    .wds-profile-space {
        border: none;
    }
}

.wds-address {
    margin-left: -5px;
    color: green;
    font-weight: normal;
    font-family: Menlo, Consolas, monospace;
}

.wds-username {
    padding: 0 3px;
    font-weight: normal;
    font-family: Menlo, Consolas, monospace;
    border-radius: 5px;
    background-color: #eee;
}

.wds-password {
    padding: 0 3px;
    color: #fff;
    font-weight: normal;
    font-family: Menlo, Consolas, monospace;
    border-radius: 5px;
    background-color: #666;
}

.wds-dir-permission {
    display: inline-block;
    padding: 3px 5px;
    font-size: 12px;
    border-radius: 5px;
    background-color: #ddd;
}

.wds-dir-name {
    color: #000;
}

.wds-dir-path {
    color: gray;
}

</style>
