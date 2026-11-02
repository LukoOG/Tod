import { invoke } from "@tauri-apps/api/core";

export function getPlatform(): Promise<string> {
  return invoke<string>("get_platform");
}
