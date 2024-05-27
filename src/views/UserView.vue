<script lang="ts">
import {defineComponent, onMounted, reactive, toRefs} from 'vue'
import {getRoleList, getUserList} from "../api/tpi.ts";
import {InitData} from "../type/user.ts";

export default defineComponent({
  setup() {
    const data = reactive(new InitData())
    onMounted(() => {
      getUser();
      getRole()
    })
    const getUser = () => {
      getUserList().then((res) => {
        console.log(12, res)
        data.list = res
      })
    }
    const getRole = () => {
      getRoleList().then(res => {
        console.log(17, res)
        data.roleList = res
      })
    }
    const deleteRow = (row) => {
      console.log(26,row)
    }
    return {
      ...toRefs(data)
    }
  }
})
</script>

<template>
  <div>
    <el-form :inline="true" :model="selectData" class="demo-form-inline">
      <el-form-item label="昵称">
        <el-input v-model="selectData.nick_name" placeholder="请输入昵称" clearable/>
      </el-form-item>
      <el-form-item label="角色">
        <el-select
            v-model="selectData.role_id"
            placeholder="Select"
            size="large"
            style="width: 240px"
        >
          <el-option label="全部" :value="0"/>
          <el-option
              v-for="item in selectData.roleList"
              :key="item.role_id"
              :label="item.role_name "
              :value="item.role_id"
          />
        </el-select>
      </el-form-item>
      <el-form-item>
        <el-button type="primary" @click="onSubmit">查询</el-button>
      </el-form-item>
    </el-form>
  </div>
  <el-table :data="list" border style="width: 100%">
    <el-table-column prop="id" label="ID" width="180"/>
    <el-table-column prop="nick_name" label="昵称" width="180"/>
    <el-table-column prop="role" label="角色">
      <template #default="scope">
        <el-button
            link
            size="small" @click="deleteRow(scope)"
            v-for="item in scope.row.role">
          {{ item.role_name }}
        </el-button>

      </template>
    </el-table-column>
  </el-table>
</template>

<style scoped>

</style>