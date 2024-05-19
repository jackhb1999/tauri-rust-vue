import { invoke } from "@tauri-apps/api/core";
import {LoginFormInt} from "../type/login.ts";


async function greet(name) {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    const greetMsg = await invoke("greet", { name: name });
    return greetMsg
}

export function login(data:LoginFormInt){
    return invoke("login",{name:data.username,pass:data.password})
}