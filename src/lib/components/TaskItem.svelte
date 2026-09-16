<script lang="ts">
  import type { Task } from "../types/planner";
  interface Props { task: Task; }
  let { task }: Props = $props();
</script>

<article class:completed={task.completed} class="item">
  <span aria-hidden="true" class="status">{task.completed ? "✓" : "○"}</span>
  <p>{task.title}</p>
</article>

<style>
  .item { display: flex; gap: 0.75rem; align-items: baseline; padding: 0.8rem 0; border-top: 1px solid #e4e9f0; }
  .status { width: 1rem; color: #53709c; font-size: 1.05rem; line-height: 1.2; text-align: center; }
  p { margin: 0; color: #202a3a; }
  .completed p, .completed .status { color: #8a96a8; }
  .completed p { text-decoration: line-through; }
</style>
