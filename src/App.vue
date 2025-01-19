<script setup lang="ts">

import {RouterView} from "vue-router";
</script>

<template>
  {{ $route.fullPath }}
  <RouterView />
</template>

<style lang="scss">
body{
  @apply bg-gray-100;
}

*{
  padding: 0;
  margin: 0;
}
html,body,#app{
  width: 100%;
  height: 100%;
}

</style>
