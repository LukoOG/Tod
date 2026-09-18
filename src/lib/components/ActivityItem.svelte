<script lang="ts">
  import { completeActivity } from "$lib/api";
  import type { Activity } from "../types/planner";
  interface Props { activity: Activity; onActivityCompleted: (activity: Activity) => void }
  let { activity, onActivityCompleted }: Props = $props();
  let isUpdating = $state(false);

  function formatDuration(minutes: number | null) {
    if (minutes === null) return null;
    const hours = Math.floor(minutes / 60);
    const remainingMinutes = minutes % 60;
    if (hours && remainingMinutes) return `${hours}h ${remainingMinutes}m`;
    return hours ? `${hours}h` : `${remainingMinutes}m`;
  }

  async function handleCompleteActivity(){
    if (isUpdating) return;
    isUpdating = true;
    try {
      onActivityCompleted(await completeActivity(activity.id));
    } finally {
      isUpdating = false;
    }
  }
</script>

<article class:completed={activity.completed} class="item">
  <time datetime={activity.start_time ?? undefined}>{activity.start_time ?? "Any time"}</time>
  <div class="details">
    <p title={activity.title}>{activity.title}</p>
    {#if formatDuration(activity.duration_minutes)}
      <span class="duration">{formatDuration(activity.duration_minutes)}</span>
    {/if}
  </div>
  <button
    type="button"
    aria-pressed={activity.completed}
    aria-label={activity.completed ? `Mark ${activity.title} incomplete` : `Mark ${activity.title} complete`}
    disabled={isUpdating}
    onclick={handleCompleteActivity}
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
  .item { display: grid; grid-template-columns: 5.5rem minmax(0, 1fr) auto; gap: 1rem; align-items: center; padding: 0.9rem 0; border-top: 1px solid #e4e5df; }
  time { color: #64736a; font-size: 0.8125rem; font-variant-numeric: tabular-nums; font-weight: 650; white-space: nowrap; }
  .details { min-width: 0; }
  p { overflow-wrap: anywhere; margin: 0; color: #292c28; font-size: 0.975rem; font-weight: 560; line-height: 1.35; }
  .duration { display: block; overflow-wrap: anywhere; margin-top: 0.15rem; color: #858780; font-size: 0.8125rem; }
  button { display: grid; place-items: center; width: 1.9rem; height: 1.9rem; padding: 0; border: 1px solid #c9d1c7; border-radius: 50%; background: transparent; color: #5f765f; cursor: pointer; transition: background-color 120ms ease, border-color 120ms ease, transform 120ms ease; }
  button svg { width: 1.1rem; height: 1.1rem; fill: none; stroke: currentColor; stroke-width: 2; stroke-linecap: round; stroke-linejoin: round; }
  button:hover:not(:disabled) { border-color: #789177; background: #f1f5ee; transform: scale(1.04); }
  button:active:not(:disabled) { transform: scale(0.96); }
  button:focus-visible { outline: 3px solid #c6d9c1; outline-offset: 2px; }
  button:disabled { cursor: wait; opacity: 0.55; }
  .completed p, .completed time, .completed .duration { color: #979a93; }
  .completed p { text-decoration: line-through; text-decoration-thickness: 1px; }
  .completed button { border-color: #698268; background: #698268; color: #fff; }
  @media (max-width: 460px) { .item { grid-template-columns: 4.45rem minmax(0, 1fr) auto; gap: 0.65rem; } }
</style>
