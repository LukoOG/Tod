import { invoke } from "@tauri-apps/api/core";
import type { CreateTaskInput, Task } from "../types";

export function createTask(input: CreateTaskInput): Promise<Task> {
  return invoke<Task>("create_task", { input });
}

export function toggleTask(id: string): Promise<Task> {
  return invoke<Task>("toggle_task", { id });
}

export function updateTaskNotes(
  id: string,
  notes: string | null
): Promise<Task> {
  return invoke<Task>("update_task_notes", { id, notes });
}