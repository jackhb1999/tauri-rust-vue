import {reactive, ref, useTemplateRef} from "vue";
import type {FormInstance} from "element-plus";
import {updatePassword} from "@/api/tpi.ts";
import {toastByError, toastByFail, toastBySuccess} from "@/composables/util.ts";
import FormDrawer from '@/components/FormDrawer.vue'
import {useUserInfoStore} from "@/store/userinfo.ts";
import {useRouter} from "vue-router";


export function useRepassword() {
    const store = useUserInfoStore()
    const router = useRouter()
// 修改密码drawer
    type FormDrawerType = InstanceType<typeof FormDrawer>
    const formDrawer = useTemplateRef<FormDrawerType>('formDrawerRef')
    const pws = reactive({
        oldpassword: '',
        newpassword: '',
        newpasswordagen: '',
    })

    const rules = {
        oldpassword: [
            {required: true, message: '旧密码不能为空！', trigger: 'blur'},
        ],
        newpassword: [{required: true, message: '新密码不能为空！', trigger: 'blur'},
        ],
        newpasswordagen: [{required: true, message: '确认密码不能为空！', trigger: 'blur'},
        ],
    }
    const formRef = ref<FormInstance>()
    const onSubmit = () => {
        formRef.value?.validate((isValid) => {
            if (isValid) {
                formDrawer.value?.showLoading()
                updatePassword(pws).then(res => {
                    toastBySuccess('修改密码成功，请重新登录', 1500)

                    // 跳转页面
                    store.removeInfo()
                    router.push("/login")
                }).catch(err => {
                    toastByFail('网络连接错误')
                }).finally(() => {
                    formDrawer.value?.hideLoading()
                })
            } else {
                toastByError('输入有误，请检查', 1600)
            }
        })
        console.log('submit!')
    }

    const openRePWForm = () => formDrawer.value?.open()
    return {
        formDrawer,
        pws,
        rules,
        onSubmit,
        openRePWForm
    }
}