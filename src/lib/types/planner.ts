export interface Activity {
  id: string;
  day_date: string;
  title: string;
  start_time: string | null;
  duration_minutes: number | null;
  completed: boolean;
  completed_at: string | null;
  notes: string | null;
  source_routine_id: string | null;
  created_at: string;
  updated_at: string;
}

export interface Routine {
  id: string;
  title: string;
  start_time: string | null;
  duration_minutes: number | null;
  days_of_week: number[];
  active: boolean;
  created_at: string;
  updated_at: string;
}

export interface Task {
  id: string;
  day_date: string;
  title: string;
  completed: boolean;
  completed_at: string | null;
  notes: string | null;
  created_at: string;
  updated_at: string;
}

export interface Day {
  date: string;
  mood: number | null;
  reflection: string | null;
  created_at: string;
  updated_at: string;
}

export interface DayView {
  day: Day;
  activities: Activity[];
  tasks: Task[];
}
