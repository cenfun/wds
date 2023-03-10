<template>
  <div
    :class="classMap"
    :style="styleMap"
  >
    <div
      v-if="props.icon"
      ref="el"
      class="wds-icon"
    />
    <label v-if="props.label">
      {{ props.label }}
    </label>
  </div>
</template>
<script setup>
import {
    computed, onMounted, ref
} from 'vue';

import icons from '../utils/icons.js';

const props = defineProps({
    icon: {
        type: String,
        default: ''
    },
    label: {
        type: String,
        default: ''
    },
    size: {
        type: String,
        default: '16px'
    },
    button: {
        type: Boolean,
        default: true
    }
});

const el = ref(null);

const classMap = computed(() => {
    const list = ['wds-icon-label', 'vui-flex-row'];
    if (props.button) {
        list.push('wds-icon-label-button');
    }
    return list;
});

const styleMap = computed(() => {
    return {
        '--wds-icon-size': props.size
    };
});

const showIcon = () => {

    if (!props.icon) {
        return;
    }

    const svg = icons[props.icon];

    if (!svg) {
        return;
    }
    const $el = el.value;

    $el.innerHTML = svg;
};

onMounted(() => {
    showIcon();
});

</script>
<style lang="scss">
.wds-icon-label {
    --wds-icon-size: 16px;

    position: relative;

    label {
        margin-left: 3px;
        user-select: none;
    }
}

.wds-icon {
    display: block;
    width: var(--wds-icon-size);
    height: var(--wds-icon-size);
    background-repeat: no-repeat;
    background-position: center center;
    background-size: var(--wds-icon-size) var(--wds-icon-size);
}

.wds-icon-label-button {
    cursor: pointer;

    label {
        white-space: nowrap;
        cursor: pointer;
    }
}

.wds-icon-label-button:hover {
    color: #3a9bfc;
}

</style>
