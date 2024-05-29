export interface ListInt {
    id: number
    nick_name: string
    role: RoleInt[]
    user_name: string
}

interface RoleInt {
    role_id: number,
    role_name: string
}

interface SelectDataInt {
    role_id: number,
    nick_name: string
}

interface RoleListInt {
    authority: number[]
    role_id: number
    role_name: string
}

export class InitData {
    selectData: SelectDataInt = {
        role_id: 0,
        nick_name: ""
    }
    list: ListInt[]
    roleList: RoleListInt[] = []
}