export interface CreateTaskInput {
    day_date: string;
    title: string;
}

export interface CompleteActivityInput {
    id: string;
    complete: boolean;
}

export interface CreateRoutineInput {
    title: string;
    start_time: string | null;
    duration_minutes: number | null;
    days_of_week: number[];
}

export interface UpdateRoutineInput {
    id: string;
    title: string;
    start_time: string | null;
    duration_minutes: number | null;
    days_of_week: number[];
}