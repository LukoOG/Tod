<script lang="ts">
  import type { Task } from "../types/planner";
  import { toggleTask } from "$lib/api";
  interface Props {
    task: Task;
    onTaskToggled: (task: Task) => void;
  }
  let { task, onTaskToggled }: Props = $props();

  async function handleToggleTask() {
    let res = await toggleTask(task.id);
    console.log(res);
    onTaskToggled(res);
  }
</script>

<article class:completed={task.completed} class="item">
  <button
    type="button"
    class="status"
    aria-pressed={task.completed}
    aria-label={task.completed ? `Mark ${task.title} incomplete` : `Mark ${task.title} complete`}
    onclick={handleToggleTask}
  >{task.completed ? "✓" : "○"}</button>
  <p>{task.title}</p>
</article>

<style>
  .item {
    display: flex;
    gap: 0.75rem;
    align-items: baseline;
    padding: 0.8rem 0;
    border-top: 1px solid #e4e9f0;
  }
  .status {
    width: 1rem;
    padding: 0;
    border: 0;
    background: none;
    color: #53709c;
    font-size: 1.05rem;
    line-height: 1.2;
    text-align: center;
    cursor: pointer;
  }
  .status:focus-visible { outline: 3px solid #b8d5f5; outline-offset: 2px; }
  p {
    margin: 0;
    color: #202a3a;
  }
  .completed p,
  .completed .status {
    color: #8a96a8;
  }
  .completed p {
    text-decoration: line-through;
  }
</style>
