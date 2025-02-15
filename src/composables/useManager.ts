import {reactive, ref, useTemplateRef} from "vue";
import type {FormInstance, FormRules} from "element-plus";
import {updatePassword} from "@/ipc/tpi.ts";
import {toastByError, toastByFail, toastBySuccess} from "@/composables/util.ts";
import FormDrawer from '@/components/FormDrawer.vue'
import {useUserInfoStore} from "@/store/userinfo.ts";
import {useRouter} from "vue-router";


export function useRepassword() {
    const userStore = useUserInfoStore()
    const router = useRouter()
// 修改密码drawer
    type FormDrawerType = InstanceType<typeof FormDrawer>
    const formDrawer = useTemplateRef<FormDrawerType>('formDrawerRef')
    const pws = reactive({
        oldpassword: '',
        newpassword: '',
        newpasswordagen: '',
    })

    const rules = reactive<FormRules>({
        oldpassword: [
            {required: true, message: '旧密码不能为空！', trigger: 'blur'},
        ],
        newpassword: [{required: true, message: '新密码不能为空！', trigger: 'blur'},
        ],
        newpasswordagen: [{required: true, message: '确认密码不能为空！', trigger: 'blur'},
        ],
    })
    const formRef = ref<FormInstance>()
    const onSubmit = () => {
        console.log(33, formRef.value)
        formRef.value?.validate((isValid) => {
            console.log(34, isValid)
            if (isValid) {
                formDrawer.value?.showLoading()
                if (pws.newpassword != pws.newpasswordagen) {
                    toastByFail('两次密码不一致')
                    return
                }
                updatePassword(pws).then(res => {
                    toastBySuccess('修改密码成功，请重新登录', 1500)

                    // 跳转页面
                    userStore.removeInfo()
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
        console.log(55, 'submit!')
    }

    const openRePWForm = () => {
        formDrawer.value?.open()
    }
    return {
        formDrawer,
        pws,
        formRef,
        rules,
        onSubmit,
        openRePWForm
    }
}