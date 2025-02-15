import {createApp} from 'vue'
import ElementPlus from 'element-plus'
import 'element-plus/dist/index.css'
import App from './App.vue'
import {router} from './router'
import 'virtual:windi.css'

import "@/router/permission"
import {createPinia} from "pinia";
import "nprogress/nprogress.css"
import installComponents from "@/components/index.ts"

const pinia = createPinia()

const app = createApp(App)

app.use(pinia)
app.use(router)
app.use(ElementPlus)
app.use(installComponents)

app.mount("#app");

