import { invoke } from "@tauri-apps/api/core";
import type { Routine } from "../types";
import type { CreateRoutineInput, UpdateRoutineInput } from "../types";

export function listRoutines(): Promise<Routine[]> {
  return invoke<Routine[]>("list_routines");
}

export function createRoutine(input: CreateRoutineInput): Promise<Routine> {
  return invoke<Routine>("create_routine", { input });
}

export function updateRoutine(input: UpdateRoutineInput): Promise<Routine> {
  return invoke<Routine>("update_routine", { input });
}

export function setRoutineActive(id: string, active: boolean): Promise<Routine> {
  return invoke<Routine>("set_routine_active", { id, active });
}

export function deleteRoutine(id: string): Promise<void> {
  return invoke<void>("delete_routine", { id });
}
