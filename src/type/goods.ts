export interface ListInt{
    user_id:number,
    id:number,
    title:string,
    introduce:string
}

interface selectDataInt{
    title:string,
    introduce:string,
    page:number,
    count:number,
    pagesize:number,
}

export class InitData{
    selectData:selectDataInt={
        title:"",
        introduce:"",
        page:1,
        count:0,
        pagesize:10
    }
    list:ListInt[]=[]
}