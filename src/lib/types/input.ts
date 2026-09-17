export interface CreateTaskInput {
    day_date: string;
    title: string;
}

export interface CompleteActivityInput {
    id: string;
    complete: boolean;
}