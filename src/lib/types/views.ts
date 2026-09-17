import type { Activity, Day, Task } from "./planner";

export interface DayView {
  day: Day;
  activities: Activity[];
  tasks: Task[];
}
