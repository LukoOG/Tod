import { invoke } from "@tauri-apps/api/core";
import type { Day, DayView } from "../types";

export function getDay(date: string): Promise<DayView> {
  return invoke<DayView>("get_day", { date });
}

export function updateDayReflection(
  date: string,
  reflection: string | null,
  mood: number | null
): Promise<Day> {
  return invoke<Day>("update_day_reflection", { date, reflection, mood });
}