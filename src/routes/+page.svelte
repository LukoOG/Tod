<script lang="ts">
  import { fly } from "svelte/transition";
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
  let selectedDate = $state(todayDate);
  let slideDirection = $state(1);
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
      dayView.activities = dayView.activities.map((prev) =>
        prev.id == activity.id ? activity : prev,
      );
  }

  function updateTask(task: DayView["tasks"][number]) {
    if (dayView)
      dayView.tasks = dayView.tasks.map((prev) =>
        prev.id == task.id ? task : prev,
      );
  }

  function changeDate(days: number) {
    const date = new Date(`${selectedDate}T00:00:00`);
    date.setDate(date.getDate() + days);
    selectedDate = `${date.getFullYear()}-${String(date.getMonth() + 1).padStart(2, "0")}-${String(date.getDate()).padStart(2, "0")}`;
    slideDirection = days > 0 ? 1 : -1;
    loadDay();
  }

  function returnToToday() {
    if (selectedDate === todayDate) return;
    slideDirection = selectedDate < todayDate ? 1 : -1;
    selectedDate = todayDate;
    loadDay();
  }

  async function loadDay() {
    isLoading = true;
    error = null;

    try {
      dayView = await getDay(selectedDate);
    } catch (cause) {
      error = cause instanceof Error ? cause.message : "Could not load this day.";
    } finally {
      isLoading = false;
    }
  }

  onMount(loadDay);
</script>

<svelte:head>
  <title>Today · Tod</title>
  <meta name="description" content="Your plan for today" />
</svelte:head>

<main>
  {#if isLoading}
    <p class="message" aria-live="polite">Loading day…</p>
  {:else if error}
    <div class="message error" role="alert">
      <p>{error}</p>
      <button onclick={loadDay}>Try again</button>
    </div>
  {:else if dayView}
    {#key dayView.day.date}
      <div
        class="day-content"
        in:fly={{ x: slideDirection * 16, duration: 140 }}
        out:fly={{ x: -slideDirection * 16, duration: 140 }}
      >
        <div class="date-navigation" aria-label="Date navigation">
          <button class="day-step" onclick={() => changeDate(-1)} aria-label="Previous day">‹</button>
          <DayHeader date={dayView.day.date} isToday={selectedDate === todayDate} />
          <button class="day-step" onclick={() => changeDate(1)} aria-label="Next day">›</button>
        </div>
        {#if selectedDate !== todayDate}
          <div class="today-action"><button onclick={returnToToday}>Back to today</button></div>
        {/if}

        <section aria-labelledby="activities-heading">
          <h2 id="activities-heading">Activities</h2>
          {#if activities.length}
            <div class="items">
              {#each activities as activity (activity.id)}
                <ActivityItem onActivityCompleted={updateActivity} {activity} />
              {/each}
            </div>
          {:else}
            <p class="empty">Nothing is scheduled for this day.</p>
          {/if}
        </section>

        <section aria-labelledby="tasks-heading">
          <h2 id="tasks-heading">Tasks</h2>
          {#if dayView.tasks.length}
            <div class="items">
              {#each dayView.tasks as task (task.id)}
                <TaskItem onTaskToggled={updateTask} {task} />
              {/each}
            </div>
          {:else}
            <p class="empty">No extra tasks yet.</p>
          {/if}
          <div class="add">
            <AddTask date={selectedDate} onTaskCreated={addTaskToDay} />
          </div>
        </section>
      </div>
    {/key}
  {/if}
</main>

<style>
  :global(*) {
    box-sizing: border-box;
  }
  :global(body) {
    margin: 0;
    min-width: 320px;
    background: #fafaf7;
    color: #292c28;
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
    width: min(100% - 2.5rem, 40rem);
    margin: 0 auto;
    padding: clamp(2.5rem, 9vh, 5.5rem) 0 4rem;
  }
  section + section {
    margin-top: 3rem;
  }
  .date-navigation {
    display: grid;
    grid-template-columns: 2.25rem minmax(0, 1fr) 2.25rem;
    align-items: center;
    gap: 0.5rem;
  }
  .day-step {
    display: grid;
    place-items: center;
    width: 2.25rem;
    height: 2.25rem;
    padding: 0;
    border: 1px solid transparent;
    border-radius: 50%;
    background: transparent;
    color: #6a7068;
    font: inherit;
    font-size: 1.65rem;
    font-weight: 400;
    line-height: 1;
    cursor: pointer;
    transition: background-color 120ms ease, color 120ms ease;
  }
  .day-step:hover { background: #f0f2ed; color: #3e493d; }
  .day-step:active { background: #e7ebe4; }
  .today-action { display: flex; justify-content: center; margin: 0.75rem 0 2.75rem; }
  .today-action button { padding: 0.2rem 0; border: 0; border-bottom: 1px solid #bcc8b9; background: transparent; border-radius: 0; color: #5f765f; font: inherit; font-size: 0.8125rem; font-weight: 650; cursor: pointer; }
  .today-action button:hover { color: #405a43; border-color: #698268; }
  .today-action button:active { transform: translateY(1px); }
  .day-content > section:first-of-type { margin-top: 2.75rem; }
  h2 {
    margin: 0 0 0.5rem;
    color: #696e67;
    font-size: 0.75rem;
    font-weight: 750;
    letter-spacing: 0.08em;
    text-transform: uppercase;
  }
  .items {
    border-bottom: 1px solid #e4e5df;
  }
  .empty,
  .message {
    margin: 0;
    color: #888c84;
    font-size: 0.9375rem;
  }
  .message { padding: 1.25rem 0; }
  .error { max-width: 30rem; border-left: 2px solid #bd8b79; padding-left: 0.85rem; color: #775447; }
  .error p { margin: 0 0 0.65rem; }
  .error button { padding: 0; border: 0; border-bottom: 1px solid currentColor; border-radius: 0; background: transparent; color: inherit; }
  .add { margin-top: 0.25rem; }
  :global(button:focus-visible) { outline: 3px solid #c6d9c1; outline-offset: 2px; }
  @media (max-width: 460px) {
    main { width: min(100% - 1.5rem, 40rem); padding-top: 2rem; }
    .date-navigation { grid-template-columns: 2rem minmax(0, 1fr) 2rem; gap: 0.15rem; }
    .day-step { width: 2rem; height: 2rem; }
  }
</style>
