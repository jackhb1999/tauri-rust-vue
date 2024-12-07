import {ElMessageBox, ElNotification} from 'element-plus'
import {useNProgress} from '@vueuse/integrations/useNProgress'


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
export function toastByError(message: string, duration?: number) {
    return ElNotification({
        message,
        type: 'error',
        duration: duration || 2000
    })
}


// 确认消息
export function confirm(content: string = "提示内容", title?: string) {
    return ElMessageBox.confirm(
        content,
        title || '提示',
        {
            confirmButtonText: '确认',
            cancelButtonText: '取消',
            type: 'warning',
        }
    )
}

const {isLoading} = useNProgress()

// 显示全屏 loading
export function showFullLoading() {
    isLoading.value = true
}

// 隐藏全屏 loading
export function hideFullLoading() {
    isLoading.value = false
}


