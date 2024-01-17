import { writable } from "svelte/store";
import { invoke } from "@tauri-apps/api/tauri";


export const todo_list = writable<Array<{ id: number, text: string }>>([]);

