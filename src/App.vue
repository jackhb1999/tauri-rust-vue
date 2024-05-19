<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import Greet from "./components/Greet.vue";
import {RouterView} from "vue-router";
</script>

<template>
  <RouterView />
</template>

<style lang="scss">
*{
  padding: 0px;
  margin: 0px;
}
html,body,#app{
  width: 100%;
  height: 100%;
}

</style>
