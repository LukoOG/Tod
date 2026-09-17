<script lang="ts">
    import { createTask } from "$lib/api/task";
    import type { Task } from "$lib/types";

    interface Props {
        date: string;
        onTaskCreated: (task: Task) => void;
    }

    let { date, onTaskCreated }: Props = $props();
    let title = $state("");

    async function addTask() {
        const taskTitle = title.trim();
        if (!taskTitle) return;

        const task = await createTask({ day_date: date, title: taskTitle });
        onTaskCreated(task);
        title = "";
    }
</script>

<form onsubmit={(event) => { event.preventDefault(); addTask(); }}>
    <input bind:value={title} aria-label="Task title" placeholder="Add a task" />
</form>
