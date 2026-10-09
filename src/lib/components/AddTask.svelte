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
  let inputEl: HTMLInputElement;

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

<form
  class="add-task-form"
  onsubmit={(e) => { e.preventDefault(); addTask(); }}
>
  <span class="plus-icon" aria-hidden="true">
    <svg viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round">
      <path d="M8 3v10M3 8h10"/>
    </svg>
  </span>
  <label class="sr-only" for="new-task-input">New task title</label>
  <input
    id="new-task-input"
    bind:this={inputEl}
    bind:value={title}
    placeholder="Add a task…"
    disabled={isSubmitting}
    autocomplete="off"
  />
  {#if title.trim()}
    <button type="submit" class="submit-btn" disabled={isSubmitting}>
      {isSubmitting ? "…" : "Add"}
    </button>
  {/if}
</form>

<style>
  .sr-only {
    position: absolute;
    width: 1px; height: 1px;
    padding: 0; overflow: hidden;
    clip: rect(0,0,0,0);
    white-space: nowrap;
    border: 0;
  }
  .add-task-form {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.4375rem 0.25rem 0.4375rem 0;
    border-top: 1px solid var(--c-separator);
  }
  .plus-icon {
    flex-shrink: 0;
    display: grid;
    place-items: center;
    width: 1.125rem;
    height: 1.125rem;
    color: var(--c-label-quaternary);
  }
  .plus-icon svg { width: 0.75rem; height: 0.75rem; }
  input {
    flex: 1;
    min-width: 0;
    padding: 0;
    border: none;
    background: transparent;
    color: var(--c-label-primary);
    font: inherit;
    font-size: 0.875rem;
    outline: none;
  }
  input::placeholder { color: var(--c-label-quaternary); }
  .submit-btn {
    flex-shrink: 0;
    padding: 0.125rem 0.5rem;
    border: none;
    border-radius: var(--radius-sm);
    background: var(--c-accent);
    color: #fff;
    font: inherit;
    font-size: 0.6875rem;
    font-weight: 600;
    cursor: pointer;
    transition: opacity 80ms ease;
  }
  .submit-btn:hover:not(:disabled) { opacity: 0.85; }
  .submit-btn:disabled { opacity: 0.55; cursor: wait; }
</style>
