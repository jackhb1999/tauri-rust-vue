import {createStore} from 'vuex'

// 创建一个新的 store 实例
const store = createStore({
    state() {
        return {
            // 用户信息
            user: {}
        }
    },
    mutations: {
        // 记录用户信息
        SET_USERINFO(state, user) {
            state.user = user
        }
    },
    actions:{

        // 获取当前登录用户信息
        getinfo({commit}){
            return new Promise((resolve,reject) => {
                // getinfo().then(res=>{
                //     commit('SET_USERINFO',res)
                // resolve(res)
                // }).catch(err => reject(err))
            })
        }
    }
})

export default store