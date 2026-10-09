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

  function handleKeydown(e: KeyboardEvent) {
    // Left/Right arrow keys for date navigation (when no input focused)
    const target = e.target as HTMLElement;
    if (target.tagName === "INPUT" || target.tagName === "TEXTAREA") return;
    if (e.key === "ArrowLeft") { e.preventDefault(); changeDate(-1); }
    if (e.key === "ArrowRight") { e.preventDefault(); changeDate(1); }
    if (e.key === "t" || e.key === "T") { e.preventDefault(); returnToToday(); }
  }

  onMount(loadDay);
</script>

<svelte:head>
  <title>{selectedDate === today ? "Today" : selectedDate} · Tod</title>
</svelte:head>

<svelte:window onkeydown={handleKeydown} />

<div class="app-window">
  <!-- Toolbar / title bar area -->
  <header class="toolbar">
    <div class="toolbar-left">
      <!-- macOS traffic-light spacer -->
      <div class="window-controls-spacer" aria-hidden="true"></div>
    </div>

    <div class="toolbar-center">
      <nav class="date-nav" aria-label="Date navigation">
        <button
          type="button"
          class="nav-btn"
          onclick={() => changeDate(-1)}
          aria-label="Previous day"
          title="Previous day (←)"
        >
          <svg viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
            <path d="M10 12L6 8l4-4"/>
          </svg>
        </button>

        <DayHeader date={selectedDate} isToday={selectedDate === today} />

        <button
          type="button"
          class="nav-btn"
          onclick={() => changeDate(1)}
          aria-label="Next day"
          title="Next day (→)"
        >
          <svg viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
            <path d="M6 4l4 4-4 4"/>
          </svg>
        </button>
      </nav>

      {#if selectedDate !== today}
        <button type="button" class="today-btn" onclick={returnToToday} title="Return to today (T)">
          Today
        </button>
      {/if}
    </div>

    <div class="toolbar-right">
      <!-- reserved for future actions -->
    </div>
  </header>

  <!-- Main scrollable content -->
  <main class="content-area" tabindex="-1">
    {#if isLoading}
      <div class="state-view" aria-live="polite" aria-busy="true">
        <div class="loading-dots" aria-label="Loading…">
          <span></span><span></span><span></span>
        </div>
      </div>
    {:else if error}
      <div class="state-view error-view" role="alert">
        <p class="error-message">{error}</p>
        <button type="button" class="retry-btn" onclick={loadDay}>Try again</button>
      </div>
    {:else if dayView}
      {#key dayView.day.date}
        <div
          class="day-view"
          in:fly={{ x: slideDirection * 16, duration: 120, opacity: 0 }}
          out:fly={{ x: -slideDirection * 16, duration: 120, opacity: 0 }}
        >
          <!-- Schedule section -->
          <section class="day-section" aria-labelledby="schedule-heading">
            <h2 class="section-label" id="schedule-heading">Schedule</h2>
            {#if activities.length}
              <div class="item-list" role="list">
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
              <p class="empty-state">Nothing scheduled</p>
            {/if}
          </section>

          <!-- Tasks section -->
          <section class="day-section" aria-labelledby="tasks-heading">
            <h2 class="section-label" id="tasks-heading">Tasks</h2>
            {#if dayView.tasks.length}
              <div class="item-list" role="list">
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
              <p class="empty-state">No tasks</p>
            {/if}
            <AddTask date={selectedDate} onTaskCreated={addTaskToDay} />
          </section>

          <!-- Reflection section -->
          <DailyReflection day={dayView.day} onDayUpdated={updateDay} />
        </div>
      {/key}
    {/if}
  </main>

  <!-- Status bar -->
  <footer class="statusbar">
    <span class="statusbar-date">{selectedDate}</span>
    <span class="statusbar-hint" aria-label="Keyboard shortcuts">← → to navigate · T for today</span>
  </footer>
</div>

<style>
  /* ═══════════════════════════════════════════════
     DESIGN SYSTEM — macOS-inspired tokens
     Light + dark appearances via prefers-color-scheme
  ═══════════════════════════════════════════════ */
  :global(:root) {
    /* Spacing scale */
    --space-1: 0.25rem;
    --space-2: 0.5rem;
    --space-3: 0.75rem;
    --space-4: 1rem;
    --space-5: 1.25rem;
    --space-6: 1.5rem;
    --space-8: 2rem;

    /* Radii */
    --radius-sm: 0.3125rem;
    --radius:    0.5rem;
    --radius-lg: 0.75rem;

    /* Typography */
    --font-system: -apple-system, "SF Pro Text", "Helvetica Neue", system-ui, sans-serif;
    --font-size-base: 0.875rem;
    --line-height-base: 1.5;

    /* Light mode palette — macOS Aqua inspired */
    --c-bg:                 #f5f5f5;
    --c-surface:            #ffffff;
    --c-surface-raised:     #fafafa;
    --c-toolbar:            rgba(246, 246, 246, 0.9);
    --c-toolbar-border:     rgba(0, 0, 0, 0.10);

    --c-label-primary:      #1c1c1e;
    --c-label-secondary:    #48484a;
    --c-label-tertiary:     #8e8e93;
    --c-label-quaternary:   #aeaeb2;

    --c-separator:          rgba(60, 60, 67, 0.12);
    --c-separator-opaque:   #d1d1d6;

    --c-fill-primary:       rgba(120, 120, 128, 0.20);
    --c-fill-secondary:     rgba(120, 120, 128, 0.14);
    --c-fill-tertiary:      rgba(118, 118, 128, 0.08);
    --c-hover:              rgba(0, 0, 0, 0.03);

    --c-accent:             #007aff;
    --c-accent-muted:       rgba(0, 122, 255, 0.20);

    --c-check-border:       #c7c7cc;
    --c-check-bg:           transparent;
    --c-check-icon:         transparent;

    --c-destructive:        #ff3b30;
    --c-destructive-muted:  #ff9f94;

    --c-statusbar:          rgba(246, 246, 246, 0.92);
    --c-statusbar-border:   rgba(0, 0, 0, 0.08);
  }

  @media (prefers-color-scheme: dark) {
    :global(:root) {
      --c-bg:                 #1c1c1e;
      --c-surface:            #2c2c2e;
      --c-surface-raised:     #3a3a3c;
      --c-toolbar:            rgba(28, 28, 30, 0.90);
      --c-toolbar-border:     rgba(255, 255, 255, 0.08);

      --c-label-primary:      #f2f2f7;
      --c-label-secondary:    #ebebf5cc;
      --c-label-tertiary:     #ebebf599;
      --c-label-quaternary:   #ebebf560;

      --c-separator:          rgba(84, 84, 88, 0.60);
      --c-separator-opaque:   #3a3a3c;

      --c-fill-primary:       rgba(120, 120, 128, 0.36);
      --c-fill-secondary:     rgba(120, 120, 128, 0.28);
      --c-fill-tertiary:      rgba(120, 120, 128, 0.18);
      --c-hover:              rgba(255, 255, 255, 0.04);

      --c-accent:             #0a84ff;
      --c-accent-muted:       rgba(10, 132, 255, 0.22);

      --c-check-border:       #48484a;
      --c-check-bg:           transparent;
      --c-check-icon:         transparent;

      --c-destructive:        #ff453a;
      --c-destructive-muted:  #ff6961;

      --c-statusbar:          rgba(28, 28, 30, 0.92);
      --c-statusbar-border:   rgba(255, 255, 255, 0.06);
    }
  }

  /* ═══════════════════════════════════════════════
     GLOBAL RESET
  ═══════════════════════════════════════════════ */
  :global(*) {
    box-sizing: border-box;
    margin: 0;
    padding: 0;
  }
  :global(html) {
    height: 100%;
  }
  :global(body) {
    height: 100%;
    overflow: hidden; /* desktop window — no scroll on body */
    background: var(--c-bg);
    color: var(--c-label-primary);
    font-family: var(--font-system);
    font-size: var(--font-size-base);
    line-height: var(--line-height-base);
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
  }
  :global(button) {
    font-family: inherit;
  }
  :global(button:focus-visible) {
    outline: 2px solid var(--c-accent);
    outline-offset: 2px;
  }

  /* ═══════════════════════════════════════════════
     APP WINDOW
  ═══════════════════════════════════════════════ */
  .app-window {
    display: flex;
    flex-direction: column;
    height: 100vh;
    background: var(--c-surface);
  }

  /* ═══════════════════════════════════════════════
     TOOLBAR (title-bar-style header)
  ═══════════════════════════════════════════════ */
  .toolbar {
    flex-shrink: 0;
    display: grid;
    grid-template-columns: 1fr auto 1fr;
    align-items: center;
    gap: 0;
    height: 3.25rem;
    padding: 0 0.75rem;
    background: var(--c-toolbar);
    border-bottom: 1px solid var(--c-toolbar-border);
    backdrop-filter: saturate(180%) blur(20px);
    -webkit-backdrop-filter: saturate(180%) blur(20px);
    /* Tauri draggable region */
    -webkit-app-region: drag;
    user-select: none;
  }
  /* Prevent drag on interactive children */
  .toolbar button {
    -webkit-app-region: no-drag;
  }

  .toolbar-left { display: flex; align-items: center; }
  .toolbar-right { display: flex; align-items: center; justify-content: flex-end; }
  .toolbar-center {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.125rem;
  }

  .window-controls-spacer {
    width: 4.25rem; /* standard macOS traffic-light area */
    height: 1rem;
  }

  /* Date navigation */
  .date-nav {
    display: flex;
    align-items: center;
    gap: 0.125rem;
  }
  .nav-btn {
    display: grid;
    place-items: center;
    width: 1.75rem;
    height: 1.75rem;
    padding: 0;
    border: none;
    border-radius: var(--radius-sm);
    background: none;
    color: var(--c-label-tertiary);
    cursor: pointer;
    -webkit-app-region: no-drag;
    transition: background 80ms ease, color 80ms ease;
  }
  .nav-btn svg { width: 0.875rem; height: 0.875rem; }
  .nav-btn:hover { background: var(--c-fill-primary); color: var(--c-label-primary); }
  .nav-btn:active { background: var(--c-fill-secondary); }
  .nav-btn:focus-visible { outline: 2px solid var(--c-accent); outline-offset: 1px; }

  .today-btn {
    padding: 0.0625rem 0.5rem;
    border: 1px solid var(--c-separator-opaque);
    border-radius: 999px;
    background: none;
    color: var(--c-accent);
    font: inherit;
    font-size: 0.625rem;
    font-weight: 600;
    letter-spacing: 0.03em;
    text-transform: uppercase;
    cursor: pointer;
    -webkit-app-region: no-drag;
    transition: background 80ms ease;
  }
  .today-btn:hover { background: var(--c-accent-muted); border-color: var(--c-accent); }
  .today-btn:focus-visible { outline: 2px solid var(--c-accent); outline-offset: 2px; }

  /* ═══════════════════════════════════════════════
     CONTENT AREA
  ═══════════════════════════════════════════════ */
  .content-area {
    flex: 1;
    overflow-y: auto;
    /* inset scrollbar for macOS overlay scrollbars */
    scrollbar-gutter: stable;
  }
  .content-area:focus { outline: none; }

  /* Day view wrapper for transitions */
  .day-view {
    width: min(100% - 3rem, 34rem);
    margin: 0 auto;
    padding: 1.5rem 0 4rem;
    display: flex;
    flex-direction: column;
    gap: 1.75rem;
  }

  /* ═══════════════════════════════════════════════
     DAY SECTION
  ═══════════════════════════════════════════════ */
  .day-section { display: flex; flex-direction: column; }

  .section-label {
    margin-bottom: 0.375rem;
    color: var(--c-label-tertiary);
    font-size: 0.625rem;
    font-weight: 700;
    letter-spacing: 0.1em;
    text-transform: uppercase;
  }

  .empty-state {
    padding: 0.625rem 0;
    border-top: 1px solid var(--c-separator);
    color: var(--c-label-quaternary);
    font-size: 0.8125rem;
    font-style: italic;
  }

  /* ═══════════════════════════════════════════════
     LOADING STATE
  ═══════════════════════════════════════════════ */
  .state-view {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 0.75rem;
    height: 100%;
    min-height: 12rem;
  }
  .loading-dots {
    display: flex;
    gap: 0.375rem;
  }
  .loading-dots span {
    width: 0.3125rem;
    height: 0.3125rem;
    border-radius: 50%;
    background: var(--c-label-quaternary);
    animation: dot-pulse 1.2s ease-in-out infinite;
  }
  .loading-dots span:nth-child(2) { animation-delay: 0.2s; }
  .loading-dots span:nth-child(3) { animation-delay: 0.4s; }
  @keyframes dot-pulse {
    0%, 80%, 100% { opacity: 0.3; transform: scale(0.8); }
    40%            { opacity: 1;   transform: scale(1.1); }
  }

  /* ═══════════════════════════════════════════════
     ERROR STATE
  ═══════════════════════════════════════════════ */
  .error-view { gap: 0.625rem; }
  .error-message {
    color: var(--c-label-secondary);
    font-size: 0.875rem;
    text-align: center;
    max-width: 22rem;
  }
  .retry-btn {
    padding: 0.3125rem 0.875rem;
    border: 1px solid var(--c-separator-opaque);
    border-radius: var(--radius-sm);
    background: none;
    color: var(--c-accent);
    font: inherit;
    font-size: 0.8125rem;
    cursor: pointer;
    transition: background 80ms ease;
  }
  .retry-btn:hover { background: var(--c-accent-muted); border-color: var(--c-accent); }

  /* ═══════════════════════════════════════════════
     STATUS BAR
  ═══════════════════════════════════════════════ */
  .statusbar {
    flex-shrink: 0;
    display: flex;
    align-items: center;
    justify-content: space-between;
    height: 1.5rem;
    padding: 0 0.75rem;
    background: var(--c-statusbar);
    border-top: 1px solid var(--c-statusbar-border);
    backdrop-filter: saturate(180%) blur(12px);
    -webkit-backdrop-filter: saturate(180%) blur(12px);
  }
  .statusbar-date {
    color: var(--c-label-quaternary);
    font-size: 0.625rem;
    font-variant-numeric: tabular-nums;
  }
  .statusbar-hint {
    color: var(--c-label-quaternary);
    font-size: 0.625rem;
  }

  /* ═══════════════════════════════════════════════
     RESPONSIVE
  ═══════════════════════════════════════════════ */
  @media (max-width: 520px) {
    .window-controls-spacer { width: 0; }
    .toolbar { padding: 0 0.5rem; }
    .day-view {
      width: calc(100% - 2rem);
      padding-top: 1.25rem;
    }
  }
</style>
