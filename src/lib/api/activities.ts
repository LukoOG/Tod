import { invoke } from "@tauri-apps/api/core";
import type { Activity, CompleteActivityInput } from "../types";


export function completeActivity(id: string): Promise<Activity>{
    return invoke<Activity>("complete_activity", { id })
}