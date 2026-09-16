import { invoke } from "@tauri-apps/api/core";
import type { DayView } from "../types/planner";

export function getDay(date: string): Promise<DayView> {
  return invoke<DayView>("get_day", { date });
}
