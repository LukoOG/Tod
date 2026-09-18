<script lang="ts">
    import { createTask } from "$lib/api/task";
    import type { Task } from "$lib/types";

    interface Props {
        date: string;
        onTaskCreated: (task: Task) => void;
    }

    let { date, onTaskCreated }: Props = $props();
    let title = $state("");
    let isSubmitting = $state(false);

    async function addTask() {
        const taskTitle = title.trim();
        if (!taskTitle || isSubmitting) return;

        isSubmitting = true;
        try {
            const task = await createTask({ day_date: date, title: taskTitle });
            onTaskCreated(task);
            title = "";
        } finally {
            isSubmitting = false;
        }
    }
</script>

<form onsubmit={(event) => { event.preventDefault(); addTask(); }}>
    <label class="sr-only" for="new-task">Task title</label>
    <input id="new-task" bind:value={title} placeholder="Add a task" disabled={isSubmitting} />
    <button type="submit" disabled={!title.trim() || isSubmitting}>Add</button>
</form>

<style>
  .sr-only { position: absolute; width: 1px; height: 1px; padding: 0; overflow: hidden; clip: rect(0, 0, 0, 0); white-space: nowrap; border: 0; }
  form { display: flex; gap: 0.55rem; padding-top: 0.3rem; }
  input { min-width: 0; flex: 1; padding: 0.55rem 0; border: 0; border-bottom: 1px solid #cfd4cc; border-radius: 0; outline: none; background: transparent; color: #292c28; font: inherit; font-size: 0.9375rem; }
  input::placeholder { color: #989b94; }
  input:hover { border-color: #aab5a7; }
  input:focus { border-color: #6f8a6f; box-shadow: 0 2px 0 -1px #6f8a6f; }
  button { padding: 0.3rem 0.1rem; border: 0; background: transparent; color: #5f765f; font: inherit; font-size: 0.875rem; font-weight: 650; cursor: pointer; }
  button:hover:not(:disabled) { color: #405a43; }
  button:focus-visible { outline: 3px solid #c6d9c1; outline-offset: 2px; }
  button:disabled { cursor: default; opacity: 0.45; }
</style>
