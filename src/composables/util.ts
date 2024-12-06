import {ElNotification} from 'element-plus'


// 成功提示
export function toastBySuccess(message: string, duration: number = 1500) {
    return ElNotification({
        message,
        type: 'success',
        duration
    })
}

// 失败提示
export function toastByFail(message: string) {
    return ElNotification({
        title: 'Error',
        message,
        type: 'error',
    })
}

// 报错提示
export function toastByError(message: string,duration?: number) {
    return ElNotification({
        message,
        type: 'error',
        duration: duration || 2000
    })
}



