import { invoke } from "@tauri-apps/api/core";
import type { Activity } from "../types";

export function completeActivity(id: string): Promise<Activity> {
  return invoke<Activity>("complete_activity", { id });
}

export function updateActivityNotes(
  id: string,
  notes: string | null
): Promise<Activity> {
  return invoke<Activity>("update_activity_notes", { id, notes });
}