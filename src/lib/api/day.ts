import { invoke } from "@tauri-apps/api/core";
import type { DayView } from "../types";

export function getDay(date: string): Promise<DayView> {
  return invoke<DayView>("get_day", { date });
}