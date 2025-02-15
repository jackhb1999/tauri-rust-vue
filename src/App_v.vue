<script setup lang="ts">
import {onMounted, reactive, ref} from 'vue'
import {User, Lock} from '@element-plus/icons-vue'
import type {FormInstance} from "element-plus";
import {login} from "@/ipc/tpi.ts";
import {LoginData} from "@/type/login.ts";

// 使用时需要引入插件包@visactor/vtable-editors
import * as VTable_editors from '@visactor/vtable-editors';
import * as VTable from "@visactor/vue-vtable";

import {ListTable} from "@visactor/vtable";

const listTableRef = ref();

// 正常使用方式
// const input_editor = new VTable.editors.InputEditor();
// 官网编辑器中将 VTable.editors重命名成了VTable_editors

const input_editor = new VTable_editors.InputEditor();
const date_input_editor = new VTable_editors.DateInputEditor();
const list_editor = new VTable_editors.ListEditor({values: ['nan', 'nv']});
VTable.register.editor('input-editor', input_editor);
VTable.register.editor('date-input-editor', date_input_editor);
VTable.register.editor('list-editor', list_editor);

function generateRandomString(length) {
  let result = '';
  const characters = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz';
  for (let i = 0; i < length; i++) {
    result += characters.charAt(Math.floor(Math.random() * characters.length));
  }
  return result;
}

function generateRandomHobbies() {
  const hobbies = [
    'Reading books',
    'Playing video games',
    'Watching movies',
    'Cooking',
    'Hiking',
    'Traveling',
    'Photography',
    'Playing \n musical /n instruments',
    'Gardening',
    'Painting',
    'Writing',
    'Swimming'
  ];

  const numHobbies = Math.floor(Math.random() * 3) + 1; // 生成 1-3 之间的随机整数
  const selectedHobbies = [];

  for (let i = 0; i < numHobbies; i++) {
    const randomIndex = Math.floor(Math.random() * hobbies.length);
    const hobby = hobbies[randomIndex];
    selectedHobbies.push(hobby);
    hobbies.splice(randomIndex, 1); // 确保每个爱好只选一次
  }

  return selectedHobbies.join(', ');
}

function generateRandomBirthday() {
  const start = new Date('1970-01-01');
  const end = new Date('2000-12-31');
  const randomDate = new Date(start.getTime() + Math.random() * (end.getTime() - start.getTime()));
  const year = randomDate.getFullYear();
  const month = randomDate.getMonth() + 1;
  const day = randomDate.getDate();
  return `${year}-${month < 10 ? '0' + month : month}-${day < 10 ? '0' + day : day}`;
}

function generateRandomPhoneNumber() {
  const areaCode = [
    '130',
    '131',
    '132',
    '133',
    '134',
    '135',
    '136',
    '137',
    '138',
    '139',
    '150',
    '151',
    '152',
    '153',
    '155',
    '156',
    '157',
    '158',
    '159',
    '170',
    '176',
    '177',
    '178',
    '180',
    '181',
    '182',
    '183',
    '184',
    '185',
    '186',
    '187',
    '188',
    '189'
  ];
  const prefix = areaCode[Math.floor(Math.random() * areaCode.length)];
  const suffix = String(Math.random()).substr(2, 8);
  return prefix + suffix;
}

const generatePersons = count => {
  return Array.from(new Array(count)).map((_, i) => {
    const first = generateRandomString(10);
    const last = generateRandomString(4);
    return {
      id: i + 1,
      email1: `${first}_${last}@xxx.com`,
      name: first,
      lastName: last,
      hobbies: generateRandomHobbies(),
      birthday: generateRandomBirthday(),
      tel: generateRandomPhoneNumber(),
      address: `No.${i + 100} ${generateRandomString(10)} ${generateRandomString(5)}\n${generateRandomString(5)}`,
      sex: i % 2 === 0 ? 'boy' : 'girl',
      work: i % 2 === 0 ? 'back-end engineer' : 'front-end engineer',
      city: 'beijing'
    };
  });
};
const records = generatePersons(100);
const columns = [
  {
    field: 'id',
    title: 'ID',
    width: 80,
    sort: true
  },
  {
    field: 'full name',
    title: 'Full name',
    columns: [
      {
        field: 'name',
        title: 'First Name\n(input editor)',
        width: 120,
        editor: 'input-editor',
        sort: true
      },
      {
        field: 'lastName',
        title: 'Last Name\n(input editor)',
        width: 100,
        editor: 'input-editor',
        sort: true
      }
    ]
  },
  {
    field: 'birthday',
    title: 'birthday\n(date editor)',
    width: 120,
    editor: 'date-input-editor',
    sort: true
  },
  {
    field: 'sex',
    title: 'sex\n(list editor)',
    width: 100,
    editor: 'list-editor',
    sort: true
  },
  {
    field: 'address',
    title: 'address\n(textArea editor)',
    width: 300,
    editor: 'textArea-editor',
    sort: true
  },
  {
    field: 'tel',
    title: 'telephone',
    width: 150,
    sort: true
  },
  {
    field: 'work',
    title: 'job',
    width: 200,
    sort: true
  },
  {
    field: 'city',
    title: 'city',
    width: 150,
    sort: true
  }
];
const option = reactive({
  records,
  columns,
  enableLineBreak: true,
  autoWrapText: true,
  limitMaxAutoWidth: 600,
  heightMode: 'autoHeight',
  editCellTrigger: 'click',
  keyboardOptions: {
    copySelected: true,
    pasteValueToCell: true,
    selectAllOnCtrlA: true
  },
  menu: {
    contextMenuItems: ["向下插入数据", "向下插入空行", '修改掉整行值', '修改值', '删除该行']
  },
  editor: 'input-editor',
  multipleSort: true
});

onMounted(() => {
  const listTable = new ListTable(listTableRef.value, option);

  listTable.on("dropdown_menu_click", (args) => {
    console.log('dropdown_menu_click', args);
    if (args.menuKey === "向下插入数据") {
      const recordIndex = listTable.getRecordShowIndexByCell(args.col, args.row);
      listTable.addRecords(generatePersons(1), recordIndex + 1);
    } else if (args.menuKey === "向下插入空行") {
      const recordIndex = listTable.getRecordShowIndexByCell(args.col, args.row);
      listTable.addRecord({}, recordIndex + 1);
    } else if (args.menuKey === "删除该行") {
      const recordIndex = listTable.getRecordShowIndexByCell(args.col, args.row);
      listTable.deleteRecords([recordIndex]);
    } else if (args.menuKey === "修改掉整行值") {
      const recordIndex = listTable.getRecordShowIndexByCell(args.col, args.row);
      listTable.updateRecords([{
        "id": 1111,
        "email1": "changed Value",
        "name": "changed Value",
        "lastName": "changed Value",
        "hobbies": "changed Value",
        "birthday": "1974-09-25",
        "tel": "13237599651",
        "sex": "boy",
        "work": "back-end engineer",
        "city": "beijing"
      }], [recordIndex]);
    } else if (args.menuKey === "修改值") {
      listTable.startEditCell(args.col, args.row);
    }
  })
});


</script>

<template>
  <div ref="listTableRef" style="width: 1280px; height: 400px"></div>

</template>

<style scoped>

.login-container {
  @apply min-h-screen bg-indigo-500;
}

.login-container .left, .login-container .right {
  @apply flex items-center justify-center;
}

.login-container .right {
  @apply bg-light-50 flex-col;
}

.left > div > div:first-child {
  @apply font-bold text-5xl text-light-50 mb-4;
}

.left > div > div:last-child {
  @apply text-gray-200 text-lg;
}
</style>