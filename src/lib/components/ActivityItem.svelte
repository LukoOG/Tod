<script lang="ts">
  import { completeActivity } from "$lib/api";
  import type { Activity } from "../types/planner";
  interface Props { activity: Activity; onActivityCompleted: (activity: Activity) => void }
  let { activity, onActivityCompleted }: Props = $props();

  async function toggleActivity(){
    let res = await completeActivity(activity.id)
    console.log(res)
    onActivityCompleted(res)
  }
</script>

<article class:completed={activity.completed} class="item">
  <time datetime={activity.start_time ?? undefined}>{activity.start_time ?? "Any time"}</time>
  <p>{activity.title}</p>
  <button
    type="button"
    aria-pressed={activity.completed}
    aria-label={activity.completed ? `Mark ${activity.title} incomplete` : `Mark ${activity.title} complete`}
    onclick={toggleActivity}
  >
    <svg viewBox="0 0 24 24" aria-hidden="true">
      {#if activity.completed}
        <path d="m5 12 4.25 4.25L19 6.5" />
      {:else}
        <circle cx="12" cy="12" r="8" />
      {/if}
    </svg>
  </button>
</article>

<style>
  .item { display: grid; grid-template-columns: 5.25rem minmax(0, 1fr) auto; gap: 1rem; align-items: center; padding: 0.8rem 0; border-top: 1px solid #e4e9f0; }
  time { color: #53709c; font-size: 0.8125rem; font-variant-numeric: tabular-nums; font-weight: 600; }
  p { margin: 0; color: #202a3a; }
  button { display: grid; place-items: center; width: 2rem; height: 2rem; padding: 0; border: 1px solid #b9c8dd; border-radius: 50%; background: #fff; color: #315b91; cursor: pointer; }
  button svg { width: 1.1rem; height: 1.1rem; fill: none; stroke: currentColor; stroke-width: 2; stroke-linecap: round; stroke-linejoin: round; }
  button:hover { border-color: #53709c; background: #f4f8fd; transform: scale(1.06); }
  button:focus-visible { outline: 3px solid #b8d5f5; outline-offset: 2px; }
  .completed p, .completed time { color: #8a96a8; text-decoration: line-through; }
  .completed button { border-color: #52735a; background: #52735a; color: #fff; }
  @media (max-width: 420px) { .item { grid-template-columns: 4.5rem minmax(0, 1fr) auto; gap: 0.5rem; } }
</style>
