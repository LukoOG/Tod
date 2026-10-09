<script lang="ts">
  import { fly } from "svelte/transition";
  import { onMount } from "svelte";
  import ActivityItem from "../lib/components/ActivityItem.svelte";
  import DayHeader from "../lib/components/DayHeader.svelte";
  import TaskItem from "../lib/components/TaskItem.svelte";
  import AddTask from "$lib/components/AddTask.svelte";
  import DailyReflection from "$lib/components/DailyReflection.svelte";
  import { getDay } from "../lib/api/day";
  import { localToday } from "$lib/status";
  import type { Activity, Day, DayView } from "../lib/types";

  let dayView = $state<DayView | null>(null);
  let isLoading = $state(true);
  let error = $state<string | null>(null);

  // Use the status helper's canonical today computation so the status
  // helper and the page agree on what "today" means.
  let today = $state(localToday());
  let selectedDate = $state(today);
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
        prev.id === activity.id ? activity : prev,
      );
  }

  function updateTask(task: DayView["tasks"][number]) {
    if (dayView)
      dayView.tasks = dayView.tasks.map((prev) =>
        prev.id === task.id ? task : prev,
      );
  }

  function updateDay(day: Day) {
    if (dayView) dayView.day = day;
  }

  function changeDate(days: number) {
    const date = new Date(`${selectedDate}T00:00:00`);
    date.setDate(date.getDate() + days);
    selectedDate = `${date.getFullYear()}-${String(date.getMonth() + 1).padStart(2, "0")}-${String(date.getDate()).padStart(2, "0")}`;
    slideDirection = days > 0 ? 1 : -1;
    loadDay();
  }

  function returnToToday() {
    if (selectedDate === today) return;
    slideDirection = selectedDate < today ? 1 : -1;
    selectedDate = today;
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
  <!-- Date navigation is always visible while content loads/errors -->
  <div class="date-navigation" aria-label="Date navigation">
    <button class="day-step" onclick={() => changeDate(-1)} aria-label="Previous day">‹</button>
    <DayHeader date={selectedDate} isToday={selectedDate === today} />
    <button class="day-step" onclick={() => changeDate(1)} aria-label="Next day">›</button>
  </div>

  {#if selectedDate !== today}
    <div class="today-action">
      <button onclick={returnToToday}>Back to today</button>
    </div>
  {/if}

  {#if isLoading}
    <p class="status-message" aria-live="polite">Loading…</p>
  {:else if error}
    <div class="error-state" role="alert">
      <p>{error}</p>
      <button onclick={loadDay}>Try again</button>
    </div>
  {:else if dayView}
    {#key dayView.day.date}
      <div
        class="day-content"
        in:fly={{ x: slideDirection * 14, duration: 130, opacity: 0 }}
        out:fly={{ x: -slideDirection * 14, duration: 130, opacity: 0 }}
      >
        <section aria-labelledby="activities-heading">
          <h2 id="activities-heading">Schedule</h2>
          {#if activities.length}
            <div class="items">
              {#each activities as activity (activity.id)}
                <ActivityItem
                  {activity}
                  dayDate={activity.day_date}
                  {today}
                  onActivityCompleted={updateActivity}
                  onActivityNotesChanged={updateActivity}
                />
              {/each}
            </div>
          {:else}
            <p class="empty">Nothing scheduled.</p>
          {/if}
        </section>

        <section aria-labelledby="tasks-heading">
          <h2 id="tasks-heading">Tasks</h2>
          {#if dayView.tasks.length}
            <div class="items">
              {#each dayView.tasks as task (task.id)}
                <TaskItem
                  {task}
                  {today}
                  onTaskToggled={updateTask}
                  onTaskNotesChanged={updateTask}
                />
              {/each}
            </div>
          {:else}
            <p class="empty">No tasks.</p>
          {/if}
          <div class="add-task">
            <AddTask date={selectedDate} onTaskCreated={addTaskToDay} />
          </div>
        </section>

        <DailyReflection day={dayView.day} onDayUpdated={updateDay} />
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
    color: #252824;
    font-family:
      Inter,
      ui-sans-serif,
      system-ui,
      -apple-system,
      BlinkMacSystemFont,
      "Segoe UI",
      sans-serif;
    line-height: 1.5;
    -webkit-font-smoothing: antialiased;
  }
  :global(button:focus-visible) {
    outline: 2px solid #b8d0b5;
    outline-offset: 2px;
  }

  main {
    width: min(100% - 2.5rem, 40rem);
    margin: 0 auto;
    padding: clamp(2.5rem, 9vh, 5rem) 0 4.5rem;
  }

  /* ── Date navigation ─────────────────────────────────────── */
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
    border: none;
    border-radius: 50%;
    background: transparent;
    color: #909690;
    font: inherit;
    font-size: 1.6rem;
    font-weight: 400;
    line-height: 1;
    cursor: pointer;
    transition: background-color 100ms ease, color 100ms ease;
  }
  .day-step:hover {
    background: #eeefe9;
    color: #3b4339;
  }
  .day-step:active { background: #e5e8e1; }

  .today-action {
    display: flex;
    justify-content: center;
    margin-top: 0.7rem;
  }
  .today-action button {
    padding: 0.15rem 0;
    border: 0;
    border-bottom: 1px solid #bec8bb;
    border-radius: 0;
    background: transparent;
    color: #5f765f;
    font: inherit;
    font-size: 0.8125rem;
    font-weight: 600;
    cursor: pointer;
    transition: color 100ms ease, border-color 100ms ease;
  }
  .today-action button:hover {
    color: #3e5641;
    border-color: #6a826a;
  }
  .today-action button:active { opacity: 0.75; }

  /* ── Day content ─────────────────────────────────────────── */
  .day-content {
    margin-top: 2.5rem;
  }
  .day-content section + section {
    margin-top: 2.75rem;
  }

  h2 {
    margin: 0 0 0.5rem;
    color: #7a7f78;
    font-size: 0.7rem;
    font-weight: 700;
    letter-spacing: 0.09em;
    text-transform: uppercase;
  }
  .items {
    border-bottom: 1px solid #e6e7e2;
  }
  .empty {
    margin: 0;
    padding: 0.6rem 0 0.5rem;
    color: #a0a59e;
    font-size: 0.9rem;
    font-style: italic;
  }
  .add-task {
    margin-top: 0.15rem;
  }

  /* ── Status messages ─────────────────────────────────────── */
  .status-message {
    margin: 2rem 0 0;
    color: #a0a59e;
    font-size: 0.9rem;
    font-style: italic;
  }
  .error-state {
    margin: 2rem 0 0;
    padding-left: 0.8rem;
    border-left: 2px solid #c89982;
    color: #7a5040;
  }
  .error-state p {
    margin: 0 0 0.6rem;
    font-size: 0.9375rem;
  }
  .error-state button {
    padding: 0;
    border: 0;
    border-bottom: 1px solid currentColor;
    border-radius: 0;
    background: transparent;
    color: inherit;
    font: inherit;
    font-size: 0.875rem;
    cursor: pointer;
  }
  .error-state button:hover { opacity: 0.75; }

  /* ── Responsive ──────────────────────────────────────────── */
  @media (max-width: 460px) {
    main {
      width: min(100% - 1.5rem, 40rem);
      padding-top: 2rem;
    }
    .date-navigation {
      grid-template-columns: 2rem minmax(0, 1fr) 2rem;
      gap: 0.15rem;
    }
    .day-step {
      width: 2rem;
      height: 2rem;
    }
  }
</style>
