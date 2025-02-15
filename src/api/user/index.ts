import service from "@/request";

enum Api {
    getMenu = '/menu/getMenu',
}

export const getMenu = () => {
    return service.get(Api.getMenu)
}