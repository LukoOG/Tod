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
  <input
    id="new-task"
    bind:value={title}
    placeholder="Add a task…"
    disabled={isSubmitting}
    autocomplete="off"
  />
  <button type="submit" disabled={!title.trim() || isSubmitting}>Add</button>
</form>

<style>
  .sr-only {
    position: absolute;
    width: 1px;
    height: 1px;
    padding: 0;
    overflow: hidden;
    clip: rect(0, 0, 0, 0);
    white-space: nowrap;
    border: 0;
  }
  form {
    display: flex;
    gap: 0.55rem;
    padding-top: 0.35rem;
  }
  input {
    min-width: 0;
    flex: 1;
    padding: 0.5rem 0;
    border: 0;
    border-bottom: 1px solid #d0d5cd;
    border-radius: 0;
    outline: none;
    background: transparent;
    color: #252824;
    font: inherit;
    font-size: 0.9375rem;
    transition: border-color 100ms ease;
  }
  input::placeholder { color: #9a9e97; }
  input:hover { border-color: #aab5a7; }
  input:focus {
    border-color: #6f8a6f;
    box-shadow: 0 2px 0 -1px #6f8a6f;
  }
  button {
    padding: 0.3rem 0.15rem;
    border: 0;
    background: transparent;
    color: #5f765f;
    font: inherit;
    font-size: 0.875rem;
    font-weight: 650;
    cursor: pointer;
    transition: color 100ms ease;
  }
  button:hover:not(:disabled) { color: #3f5a42; }
  button:focus-visible { outline: 2px solid #b8d0b5; outline-offset: 2px; }
  button:disabled { cursor: default; opacity: 0.4; }
</style>
