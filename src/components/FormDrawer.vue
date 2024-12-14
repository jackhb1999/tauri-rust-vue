<script setup lang="ts">

import {ref} from "vue";

const showDrawer = ref(false)

const loading = ref(false)

const props = defineProps({
  title: String,
  size: {
    type: String,
    default: "45%",
  },
  destroyOnClose: {
    type: Boolean,
    default: false
  },
  confirmText: {
    type: String,
    default: "提交",
  },
})

const emit = defineEmits(["submit"])

const open = () => showDrawer.value = true

const close = () => showDrawer.value = false

const submit = () => emit("submit")

const showLoading = () => loading.value = true

const hideLoading = () => loading.value = false

// 向父组件暴露方法
defineExpose({
  open, close,
  showLoading,hideLoading
})
</script>

<template>
  <el-drawer v-model="showDrawer" :title="title" :size="size"
             :close-on-click-modal="false" :destroy-on-close="destroyOnClose">
    <div class="formDrawer">
      <div class="body">
        <slot/>
      </div>
      <div class="actions">
        <el-button type="primary" @click="submit" :loading="loading"
        >{{confirmText}}
        </el-button>
        <el-button @click="close">取 消
        </el-button>
      </div>
    </div>
  </el-drawer>
</template>

<style scoped>
.formDrawer {
  width: 100%;
  height: 100%;
  position: relative;
  @apply flex flex-col;
}

.formDrawer .body {
  flex: 1;
  overflow: auto;
}

.formDrawer .actions {
  height: 50px;
  @apply mt-auto flex items-center;
}
</style>