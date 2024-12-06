import {useCookies} from "@vueuse/integrations/useCookies";


const cookie = useCookies()

const TokenKey = "token";


// 获取 token
export function getToken(): any {
    return cookie.get(TokenKey);
}

// 设置 token
export function setToken(token: String): void {
    cookie.set(TokenKey, token);
}

// 清除 token
export function removeToken(): void {
    cookie.remove(TokenKey);
}