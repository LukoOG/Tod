<script lang="ts">
  import type { Activity } from "../types/planner";
  interface Props { activity: Activity; }
  let { activity }: Props = $props();
</script>

<article class:completed={activity.completed} class="item">
  <time datetime={activity.start_time ?? undefined}>{activity.start_time ?? "Any time"}</time>
  <p>{activity.title}</p>
</article>

<style>
  .item { display: grid; grid-template-columns: 5.25rem minmax(0, 1fr); gap: 1rem; align-items: baseline; padding: 0.8rem 0; border-top: 1px solid #e4e9f0; }
  time { color: #53709c; font-size: 0.8125rem; font-variant-numeric: tabular-nums; font-weight: 600; }
  p { margin: 0; color: #202a3a; }
  .completed p, .completed time { color: #8a96a8; text-decoration: line-through; }
  @media (max-width: 420px) { .item { grid-template-columns: 4.5rem minmax(0, 1fr); gap: 0.5rem; } }
</style>
