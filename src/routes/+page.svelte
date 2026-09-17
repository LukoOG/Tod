<script lang="ts">
  import { onMount } from "svelte";
  import ActivityItem from "../lib/components/ActivityItem.svelte";
  import DayHeader from "../lib/components/DayHeader.svelte";
  import TaskItem from "../lib/components/TaskItem.svelte";
  import AddTask from "$lib/components/AddTask.svelte";
  import { getDay } from "../lib/api/day";
  import type { Activity, DayView } from "../lib/types";

  let dayView = $state<DayView | null>(null);
  let isLoading = $state(true);
  let error = $state<string | null>(null);

  const now = new Date();
  const todayDate = `${now.getFullYear()}-${String(now.getMonth() + 1).padStart(2, "0")}-${String(now.getDate()).padStart(2, "0")}`;
  const activities = $derived(
    dayView?.activities.toSorted(compareActivities) ?? [],
  );

  function compareActivities(a: Activity, b: Activity) {
    if (a.start_time === null) return b.start_time === null ? 0 : 1;
    if (b.start_time === null) return -1;
    return a.start_time.localeCompare(b.start_time);
  }

  function addTaskToDay(task: DayView["tasks"][number]) {
    if (dayView) dayView.tasks = [...dayView.tasks, task];
  }

  function updateActivity(activity: DayView["activities"][number]) {
    if (dayView)
      dayView.activities = dayView.activities.map((a) =>
        a.id == activity.id ? activity : a,
      );
  }

  async function loadToday() {
    isLoading = true;
    error = null;

    try {
      dayView = await getDay(todayDate);
    } catch (cause) {
      error = cause instanceof Error ? cause.message : "Could not load today.";
    } finally {
      isLoading = false;
    }
  }

  onMount(loadToday);
</script>

<svelte:head>
  <title>Today · Tod</title>
  <meta name="description" content="Your plan for today" />
</svelte:head>

<main>
  {#if isLoading}
    <p class="message" aria-live="polite">Loading today…</p>
  {:else if error}
    <div class="message error" role="alert">
      <p>{error}</p>
      <button onclick={loadToday}>Try again</button>
    </div>
  {:else if dayView}
    <DayHeader date={dayView.day.date} />

    <section aria-labelledby="activities-heading">
      <h2 id="activities-heading">Activities</h2>
      {#if activities.length}
        <div class="items">
          {#each activities as activity (activity.id)}
            <ActivityItem onActivityCompleted={updateActivity} {activity} />
          {/each}
        </div>
      {:else}
        <p class="empty">No activities scheduled for today.</p>
      {/if}
    </section>

    <section aria-labelledby="tasks-heading">
      <h2 id="tasks-heading">Tasks</h2>
      {#if dayView.tasks.length}
        <div class="items">
          {#each dayView.tasks as task (task.id)}
            <TaskItem {task} />
          {/each}
        </div>
      {:else}
        <p class="empty">No tasks for today.</p>
      {/if}
      <div class="add">
        <AddTask date={todayDate} onTaskCreated={addTaskToDay} />
      </div>
    </section>
  {/if}
</main>

<style>
  :global(*) {
    box-sizing: border-box;
  }
  :global(body) {
    margin: 0;
    min-width: 320px;
    background: #f8fafc;
    color: #202a3a;
    font-family:
      Inter,
      ui-sans-serif,
      system-ui,
      -apple-system,
      BlinkMacSystemFont,
      "Segoe UI",
      sans-serif;
    line-height: 1.5;
  }
  main {
    width: min(100% - 2.5rem, 42rem);
    margin: 0 auto;
    padding: clamp(3rem, 10vh, 6.5rem) 0 4rem;
  }
  section + section {
    margin-top: 2.75rem;
  }
  h2 {
    margin: 0 0 0.65rem;
    color: #172033;
    font-size: 0.875rem;
    font-weight: 700;
    letter-spacing: 0.04em;
    text-transform: uppercase;
  }
  .items {
    border-bottom: 1px solid #e4e9f0;
  }
  .empty,
  .message {
    margin: 0;
    color: #718096;
    font-size: 0.9375rem;
  }
  .error p {
    margin: 0 0 0.75rem;
    color: #9b2c2c;
  }
  button {
    padding: 0.4rem 0.7rem;
    border: 1px solid #b9c8dd;
    border-radius: 0.375rem;
    background: #fff;
    color: #315b91;
    font: inherit;
    font-size: 0.875rem;
    cursor: pointer;
  }
  button:hover {
    border-color: #53709c;
  }
  button:focus-visible {
    outline: 3px solid #b8d5f5;
    outline-offset: 2px;
  }
</style>
