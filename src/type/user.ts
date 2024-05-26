export interface ListInt {
    id: number
    nick_name: string
    role: RoleInt[]
    user_name: string
}

interface RoleInt {
    role: number,
    roleName: string
}

interface SelectDataInt {
    role: number,
    role_name: string
}

interface RoleListInt {
    authority: number[]
    roleId: number
    role_name: string
}

export class InitData {
    selectData: SelectDataInt = {
        role: 0,
        role_name: ""
    }
    list: ListInt[]
    roleList: RoleListInt[] = []
}