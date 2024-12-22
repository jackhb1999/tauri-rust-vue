<script setup lang="ts">
import {ref} from 'vue'
import {useAsideWidthStore} from "@/store/asideWidth.ts";

const asideWidthStore = useAsideWidthStore()

let tabIndex = 2
const activeTab = ref('2')
const tabList = ref([
  {
    title: '后台首页',
    path: '/'
  },

])


const removeTab = (targetName: string) => {

}
</script>

<template>
  <div class="tag-list" :style="{left:asideWidthStore.asideWidth}">
    <el-tabs
        v-model="activeTab"
        type="card"
        class="demo-tabs"
        closable
        @tab-remove="removeTab"
        style="min-width: 100px"
    >
      <el-tab-pane
          v-for="item in tabList"
          :key="item.path"
          :label="item.title"
          :name="item.path"
      >

      </el-tab-pane>
    </el-tabs>
    <span class="tag-btn">
 <el-dropdown>
    <span class="el-dropdown-link">
      <el-icon>
        <arrow-down/>
      </el-icon>
    </span>
    <template #dropdown>
      <el-dropdown-menu>
        <el-dropdown-item>Action 1</el-dropdown-item>
        <el-dropdown-item>Action 2</el-dropdown-item>
        <el-dropdown-item>Action 3</el-dropdown-item>
        <el-dropdown-item disabled>Action 4</el-dropdown-item>
        <el-dropdown-item divided>Action 5</el-dropdown-item>
      </el-dropdown-menu>
    </template>
  </el-dropdown>
  </span>
  </div>
</template>

<style scoped>
.tag-list {
  @apply fixed bg-gray-100 flex items-center px-2;
  top: 7vh;
  right: 0;
  height: 5vh;
  z-index: 100;
}

.tag-btn {
  @apply bg-white rounded ml-auto flex items-center justify-center px-2;
  height: 4.5vh;

}

:deep(.el-tabs__header) {
  @apply mb-0;
  border: 0 !important;
}

:deep(.el-tabs__nav) {
  border: 0 !important;
}

:deep(.el-tabs__item) {
  border: 0 !important;
  @apply bg-white mx-1 rounded;
  height: 4.5vh;
}

:deep(.el-tabs__nav-next), :deep(.el-tabs__nav-prev) {
  height: 4.5vh;
  line-height: 4.5vh;
}

:deep(.is-disabled) {
  cursor: not-allowed;
  @apply text-gray-300;
}
</style>