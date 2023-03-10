<template>
  <div class="wds-fps-detector" />
</template>
<script setup>
import { onMounted } from 'vue';

import FpsDetector from 'fps-detector';

onMounted(() => {
    new FpsDetector('.wds-fps-detector', {
        width: 120,
        height: 30
    });

});

</script>
<style>
.wds-fps-detector {
    margin: 0 5px;
}
</style>
