import axios from "axios"
import {getToken} from "../composables/auth.ts";
import {toastByError} from "@/composables/util.ts";


// 创建 axios 实例
const service = axios.create({
    baseURL: import.meta.env.VITE_APP_BASE_API,
    timeout: 5000,
    headers: {
        "Content-Type": "application/json;charset=utf-8"
    }
})

// 请求拦截
service.interceptors.request.use((config) => {
    // config.headers = config.headers || if(localStorage.getItem('token')){
    //     config.headers.token=localStorage.getItem('token') || ""
    // }
    const token = getToken()

    if (token) {
        config.headers['token'] = token;
    }
    return config
})

// 响应拦截
service.interceptors.response.use((res) => {
    const code: number = res.data.code
    if (code != 200) {
        return Promise.reject(res.data)
    }
    return res.data
}, (err) => {
    console.log(36, err)
    let message: string = ""
    let status = err.response.status
    switch (status) {
        case 401:
            message = "登录过期，请重新登录！"
            break
        case 403:
            message = "拒绝访问！"
            break
        case 404:
            message = "请求地址错误！"
            break
        case 500:
            message = "服务器内部错误！"
            break
        default:
            message = "网络连接错误！"
    }
    toastByError(message)
})

export default service