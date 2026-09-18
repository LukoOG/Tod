<script lang="ts">
  import type { Task } from "../types/planner";
  import { toggleTask } from "$lib/api";
  interface Props {
    task: Task;
    onTaskToggled: (task: Task) => void;
  }
  let { task, onTaskToggled }: Props = $props();
  let isUpdating = $state(false);

  async function handleToggleTask() {
    if (isUpdating) return;
    isUpdating = true;
    try {
      onTaskToggled(await toggleTask(task.id));
    } finally {
      isUpdating = false;
    }
  }
</script>

<article class:completed={task.completed} class="item">
  <button
    type="button"
    class="status"
    aria-pressed={task.completed}
    aria-label={task.completed ? `Mark ${task.title} incomplete` : `Mark ${task.title} complete`}
    disabled={isUpdating}
    onclick={handleToggleTask}
  ><span aria-hidden="true">{task.completed ? "✓" : ""}</span></button>
  <p title={task.title}>{task.title}</p>
</article>

<style>
  .item {
    display: flex;
    gap: 0.75rem;
    align-items: flex-start;
    padding: 0.75rem 0;
    border-top: 1px solid #e4e5df;
  }
  .status {
    display: grid;
    flex: 0 0 auto;
    place-items: center;
    width: 1.25rem;
    height: 1.25rem;
    margin-top: 0.1rem;
    padding: 0;
    border: 1px solid #c9d1c7;
    border-radius: 0.3rem;
    background: transparent;
    color: #fff;
    cursor: pointer;
    transition: background-color 120ms ease, border-color 120ms ease, transform 120ms ease;
  }
  .status:hover:not(:disabled) { border-color: #789177; background: #f1f5ee; }
  .status:active:not(:disabled) { transform: scale(0.9); }
  .status:focus-visible { outline: 3px solid #c6d9c1; outline-offset: 2px; }
  .status:disabled { cursor: wait; opacity: 0.55; }
  .status span { font-size: 0.85rem; font-weight: 800; line-height: 1; }
  p {
    overflow-wrap: anywhere;
    margin: 0;
    color: #3a3d38;
    font-size: 0.9375rem;
    line-height: 1.4;
  }
  .completed p,
  .completed .status {
    color: #979a93;
  }
  .completed p {
    text-decoration: line-through;
    text-decoration-thickness: 1px;
  }
  .completed .status { border-color: #698268; background: #698268; }
</style>
