<script lang="ts">
  import { onMount } from "svelte";
  import type { Routine } from "$lib/types";
  import type { CreateRoutineInput, UpdateRoutineInput } from "$lib/types";
  import { listRoutines, createRoutine, updateRoutine, setRoutineActive, deleteRoutine } from "$lib/api/routines";

  interface Props {
    onClose: () => void;
  }

  let { onClose }: Props = $props();

  // ── Data ───────────────────────────────────────────────────────────────
  let routines = $state<Routine[]>([]);
  let isLoading = $state(true);
  let loadError = $state<string | null>(null);

  // ── Form ───────────────────────────────────────────────────────────────
  type FormMode = { kind: "none" } | { kind: "create" } | { kind: "edit"; id: string };
  let formMode = $state<FormMode>({ kind: "none" });
  let formTitle = $state("");
  let formStartTime = $state("");
  let formDuration = $state("");
  let formDays = $state<number[]>([]);
  let formError = $state<string | null>(null);
  let isSubmitting = $state(false);
  let titleInputEl: HTMLInputElement | undefined = $state();

  // ── Delete ─────────────────────────────────────────────────────────────
  let deletingId = $state<string | null>(null);
  let isDeleting = $state(false);

  const DAYS = [
    { n: 1, label: "M" },
    { n: 2, label: "T" },
    { n: 3, label: "W" },
    { n: 4, label: "T" },
    { n: 5, label: "F" },
    { n: 6, label: "S" },
    { n: 7, label: "S" },
  ];

  const DAY_FULL = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];

  onMount(async () => {
    try {
      routines = await listRoutines();
    } catch (e) {
      loadError = e instanceof Error ? e.message : "Failed to load routines.";
    } finally {
      isLoading = false;
    }
  });

  // ── Derived ────────────────────────────────────────────────────────────
  const activeRoutines = $derived(routines.filter(r => r.active));
  const inactiveRoutines = $derived(routines.filter(r => !r.active));
  const formOpen = $derived(formMode.kind !== "none");

  // ── Form helpers ───────────────────────────────────────────────────────
  function openCreate() {
    formTitle = "";
    formStartTime = "";
    formDuration = "";
    formDays = [];
    formError = null;
    formMode = { kind: "create" };
    setTimeout(() => titleInputEl?.focus(), 0);
  }

  function openEdit(routine: Routine) {
    formTitle = routine.title;
    formStartTime = routine.start_time ?? "";
    formDuration = routine.duration_minutes?.toString() ?? "";
    formDays = [...routine.days_of_week].sort((a, b) => a - b);
    formError = null;
    formMode = { kind: "edit", id: routine.id };
    setTimeout(() => titleInputEl?.focus(), 0);
  }

  function closeForm() {
    formMode = { kind: "none" };
    formError = null;
  }

  function toggleDay(n: number) {
    if (formDays.includes(n)) {
      formDays = formDays.filter(d => d !== n);
    } else {
      formDays = [...formDays, n].sort((a, b) => a - b);
    }
  }

  async function submitForm() {
    if (isSubmitting) return;
    const title = formTitle.trim();
    const startTime = formStartTime.trim() || null;
    const rawDur = formDuration.trim();
    const durationMinutes = rawDur ? parseInt(rawDur, 10) : null;

    if (!title) { formError = "Title is required."; titleInputEl?.focus(); return; }
    if (formDays.length === 0) { formError = "Select at least one day."; return; }

    isSubmitting = true;
    formError = null;
    try {
      if (formMode.kind === "create") {
        const input: CreateRoutineInput = {
          title,
          start_time: startTime,
          duration_minutes: durationMinutes,
          days_of_week: formDays,
        };
        const created = await createRoutine(input);
        routines = [...routines, created];
      } else if (formMode.kind === "edit") {
        const input: UpdateRoutineInput = {
          id: formMode.id,
          title,
          start_time: startTime,
          duration_minutes: durationMinutes,
          days_of_week: formDays,
        };
        const updated = await updateRoutine(input);
        routines = routines.map(r => r.id === updated.id ? updated : r);
      }
      closeForm();
    } catch (e) {
      formError = e instanceof Error ? e.message : "Failed to save routine.";
    } finally {
      isSubmitting = false;
    }
  }

  async function handleToggleActive(routine: Routine) {
    try {
      const updated = await setRoutineActive(routine.id, !routine.active);
      routines = routines.map(r => r.id === updated.id ? updated : r);
    } catch {}
  }

  async function confirmDelete() {
    if (!deletingId || isDeleting) return;
    const id = deletingId;
    isDeleting = true;
    try {
      await deleteRoutine(id);
      routines = routines.filter(r => r.id !== id);
      if (formMode.kind === "edit" && formMode.id === id) closeForm();
    } catch {} finally {
      deletingId = null;
      isDeleting = false;
    }
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Escape") {
      if (deletingId) { deletingId = null; return; }
      if (formOpen) { closeForm(); return; }
      onClose();
    }
    if (e.key === "Enter" && (e.metaKey || e.ctrlKey) && formOpen) {
      e.preventDefault();
      submitForm();
    }
  }

  function formatDays(days: number[]): string {
    return [...days].sort((a, b) => a - b).map(d => DAY_FULL[d - 1]).join(", ");
  }

  function formatTime(time: string | null): string {
    if (!time) return "";
    const [hStr, mStr] = time.split(":");
    const h = parseInt(hStr, 10);
    const m = parseInt(mStr, 10);
    const period = h < 12 ? "AM" : "PM";
    const hour = h % 12 === 0 ? 12 : h % 12;
    return m === 0 ? `${hour} ${period}` : `${hour}:${mStr} ${period}`;
  }

  function formatMeta(routine: Routine): string {
    const parts: string[] = [];
    if (routine.start_time) parts.push(formatTime(routine.start_time));
    if (routine.duration_minutes) {
      const h = Math.floor(routine.duration_minutes / 60);
      const m = routine.duration_minutes % 60;
      if (h && m) parts.push(`${h}h ${m}m`);
      else if (h) parts.push(`${h}h`);
      else parts.push(`${m}m`);
    }
    return parts.join(" · ");
  }
</script>

<svelte:window onkeydown={handleKeydown} />

<div class="panel">
  <!-- Panel header -->
  <div class="panel-header">
    <h2 class="panel-title">Routines</h2>
    <button type="button" class="close-btn" onclick={onClose} aria-label="Close routines panel">
      <svg viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" aria-hidden="true">
        <path d="M3 3l10 10M13 3L3 13"/>
      </svg>
    </button>
  </div>

  <!-- Panel body -->
  <div class="panel-body">
    {#if isLoading}
      <div class="state-center">
        <div class="loading-dots" aria-label="Loading…">
          <span></span><span></span><span></span>
        </div>
      </div>
    {:else if loadError}
      <div class="state-center">
        <p class="load-error">{loadError}</p>
      </div>
    {:else}
      <!-- Active routines -->
      {#if activeRoutines.length > 0}
        <div class="routine-list" role="list">
          {#each activeRoutines as routine (routine.id)}
            <div class="routine-row" role="listitem">
              <button
                type="button"
                class="toggle-btn active"
                aria-pressed={true}
                aria-label="Deactivate {routine.title}"
                onclick={() => handleToggleActive(routine)}
                title="Active — click to deactivate"
              >
                <span class="toggle-track">
                  <span class="toggle-thumb"></span>
                </span>
              </button>
              <div class="routine-info">
                <span class="routine-title">{routine.title}</span>
                <span class="routine-sub">
                  {formatDays(routine.days_of_week)}
                  {#if formatMeta(routine)}<span class="routine-meta-sep">·</span>{formatMeta(routine)}{/if}
                </span>
              </div>
              <div class="routine-actions">
                {#if deletingId === routine.id}
                  <span class="delete-confirm-label">Delete?</span>
                  <button type="button" class="btn-danger-sm" onclick={confirmDelete} disabled={isDeleting}>
                    {isDeleting ? "…" : "Confirm"}
                  </button>
                  <button type="button" class="btn-ghost-sm" onclick={() => deletingId = null} disabled={isDeleting}>
                    Cancel
                  </button>
                {:else}
                  <button
                    type="button"
                    class="icon-btn"
                    aria-label="Edit {routine.title}"
                    onclick={() => openEdit(routine)}
                    title="Edit"
                  >
                    <svg viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
                      <path d="M2 12.5V14h1.5l7-7L9 5.5l-7 7zM13.5 3.5a1.121 1.121 0 0 0-1.586-1.586L10.5 3.33 12.17 5l1.33-1.5z"/>
                    </svg>
                  </button>
                  <button
                    type="button"
                    class="icon-btn danger"
                    aria-label="Delete {routine.title}"
                    onclick={() => deletingId = routine.id}
                    title="Delete"
                  >
                    <svg viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
                      <path d="M3 4h10M6 4V2h4v2M5 4v8a1 1 0 0 0 1 1h4a1 1 0 0 0 1-1V4"/>
                    </svg>
                  </button>
                {/if}
              </div>
            </div>
          {/each}
        </div>
      {/if}

      <!-- Inactive routines -->
      {#if inactiveRoutines.length > 0}
        <div class="inactive-section">
          <p class="inactive-label">Inactive</p>
          <div class="routine-list" role="list">
            {#each inactiveRoutines as routine (routine.id)}
              <div class="routine-row inactive" role="listitem">
                <button
                  type="button"
                  class="toggle-btn"
                  aria-pressed={false}
                  aria-label="Activate {routine.title}"
                  onclick={() => handleToggleActive(routine)}
                  title="Inactive — click to activate"
                >
                  <span class="toggle-track">
                    <span class="toggle-thumb"></span>
                  </span>
                </button>
                <div class="routine-info">
                  <span class="routine-title">{routine.title}</span>
                  <span class="routine-sub">
                    {formatDays(routine.days_of_week)}
                    {#if formatMeta(routine)}<span class="routine-meta-sep">·</span>{formatMeta(routine)}{/if}
                  </span>
                </div>
                <div class="routine-actions">
                  {#if deletingId === routine.id}
                    <span class="delete-confirm-label">Delete?</span>
                    <button type="button" class="btn-danger-sm" onclick={confirmDelete} disabled={isDeleting}>
                      {isDeleting ? "…" : "Confirm"}
                    </button>
                    <button type="button" class="btn-ghost-sm" onclick={() => deletingId = null} disabled={isDeleting}>
                      Cancel
                    </button>
                  {:else}
                    <button
                      type="button"
                      class="icon-btn"
                      aria-label="Edit {routine.title}"
                      onclick={() => openEdit(routine)}
                      title="Edit"
                    >
                      <svg viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
                        <path d="M2 12.5V14h1.5l7-7L9 5.5l-7 7zM13.5 3.5a1.121 1.121 0 0 0-1.586-1.586L10.5 3.33 12.17 5l1.33-1.5z"/>
                      </svg>
                    </button>
                    <button
                      type="button"
                      class="icon-btn danger"
                      aria-label="Delete {routine.title}"
                      onclick={() => deletingId = routine.id}
                      title="Delete"
                    >
                      <svg viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
                        <path d="M3 4h10M6 4V2h4v2M5 4v8a1 1 0 0 0 1 1h4a1 1 0 0 0 1-1V4"/>
                      </svg>
                    </button>
                  {/if}
                </div>
              </div>
            {/each}
          </div>
        </div>
      {/if}

      <!-- Empty state -->
      {#if routines.length === 0}
        <div class="empty-panel">
          <p>No routines yet.</p>
          <p class="empty-sub">Routines generate scheduled activities for you each day.</p>
        </div>
      {/if}

      <!-- Add / Edit form -->
      <div class="form-area" class:form-area--open={formOpen}>
        {#if formOpen}
          <div class="form-divider"></div>
          <form
            class="routine-form"
            onsubmit={(e) => { e.preventDefault(); submitForm(); }}
          >
            <h3 class="form-heading">{formMode.kind === "create" ? "New Routine" : "Edit Routine"}</h3>

            <label class="field">
              <span class="field-label">Title</span>
              <input
                type="text"
                bind:this={titleInputEl}
                bind:value={formTitle}
                placeholder="e.g. Morning run"
                autocomplete="off"
                disabled={isSubmitting}
              />
            </label>

            <fieldset class="field days-field">
              <legend class="field-label">Days</legend>
              <div class="day-pills">
                {#each DAYS as day, i (day.n)}
                  <button
                    type="button"
                    class="day-pill"
                    class:selected={formDays.includes(day.n)}
                    aria-pressed={formDays.includes(day.n)}
                    aria-label={DAY_FULL[i]}
                    onclick={() => toggleDay(day.n)}
                    disabled={isSubmitting}
                  >{day.label}</button>
                {/each}
              </div>
            </fieldset>

            <div class="form-row">
              <label class="field field--half">
                <span class="field-label">Start time <span class="optional">(optional)</span></span>
                <input
                  type="time"
                  bind:value={formStartTime}
                  disabled={isSubmitting}
                />
              </label>
              <label class="field field--half">
                <span class="field-label">Duration (min) <span class="optional">(optional)</span></span>
                <input
                  type="number"
                  bind:value={formDuration}
                  placeholder="e.g. 30"
                  min="1"
                  max="1440"
                  autocomplete="off"
                  disabled={isSubmitting}
                />
              </label>
            </div>

            {#if formError}
              <p class="form-error" role="alert">{formError}</p>
            {/if}

            <div class="form-actions">
              <button type="submit" class="btn-primary" disabled={isSubmitting}>
                {isSubmitting ? "Saving…" : formMode.kind === "create" ? "Add Routine" : "Save Changes"}
              </button>
              <button type="button" class="btn-ghost" onclick={closeForm} disabled={isSubmitting}>
                Cancel
              </button>
              <span class="kbd-hint">⌘↵ to save · Esc to cancel</span>
            </div>
          </form>
        {:else}
          <button type="button" class="add-routine-btn" onclick={openCreate}>
            <svg viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" aria-hidden="true">
              <path d="M8 3v10M3 8h10"/>
            </svg>
            Add Routine
          </button>
        {/if}
      </div>
    {/if}
  </div>
</div>

<style>
  /* ── Panel shell ───────────────────────────────────────────────────── */
  .panel {
    display: flex;
    flex-direction: column;
    height: 100%;
  }

  .panel-header {
    flex-shrink: 0;
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 1.25rem 1.5rem 0.875rem;
    border-bottom: 1px solid var(--c-separator);
  }

  .panel-title {
    font-size: 0.875rem;
    font-weight: 600;
    color: var(--c-label-primary);
    letter-spacing: -0.01em;
  }

  .close-btn {
    display: grid;
    place-items: center;
    width: 1.625rem;
    height: 1.625rem;
    padding: 0;
    border: none;
    border-radius: var(--radius-sm);
    background: none;
    color: var(--c-label-tertiary);
    cursor: pointer;
    transition: background 80ms ease, color 80ms ease;
  }
  .close-btn svg { width: 0.75rem; height: 0.75rem; }
  .close-btn:hover { background: var(--c-fill-secondary); color: var(--c-label-secondary); }
  .close-btn:focus-visible { outline: 2px solid var(--c-accent); outline-offset: 1px; }

  /* ── Panel body ───────────────────────────────────────────────────── */
  .panel-body {
    flex: 1;
    overflow-y: auto;
    padding: 0.75rem 1.5rem 1.5rem;
    display: flex;
    flex-direction: column;
  }

  /* ── States ───────────────────────────────────────────────────────── */
  .state-center {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    min-height: 8rem;
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

  .load-error {
    color: var(--c-destructive);
    font-size: 0.8125rem;
    text-align: center;
  }

  .empty-panel {
    padding: 2rem 0 1rem;
    text-align: center;
    color: var(--c-label-tertiary);
    font-size: 0.875rem;
  }
  .empty-sub {
    margin-top: 0.375rem;
    color: var(--c-label-quaternary);
    font-size: 0.75rem;
  }

  /* ── Routine list ─────────────────────────────────────────────────── */
  .routine-list { display: flex; flex-direction: column; }

  .routine-row {
    display: flex;
    align-items: center;
    gap: 0.625rem;
    padding: 0.5rem 0;
    border-top: 1px solid var(--c-separator);
  }
  .routine-row:first-child { border-top: none; }

  .routine-row.inactive { opacity: 0.55; }

  /* Toggle switch */
  .toggle-btn {
    flex-shrink: 0;
    padding: 0;
    border: none;
    background: none;
    cursor: pointer;
    border-radius: 999px;
  }
  .toggle-btn:focus-visible { outline: 2px solid var(--c-accent); outline-offset: 2px; }

  .toggle-track {
    display: flex;
    align-items: center;
    width: 2rem;
    height: 1.1875rem;
    padding: 0 0.1875rem;
    border-radius: 999px;
    background: var(--c-fill-primary);
    transition: background 150ms ease;
    pointer-events: none;
  }
  .toggle-btn.active .toggle-track { background: var(--c-accent); }

  .toggle-thumb {
    width: 0.875rem;
    height: 0.875rem;
    border-radius: 50%;
    background: #fff;
    box-shadow: 0 1px 2px rgba(0,0,0,0.25);
    transition: transform 150ms ease;
    pointer-events: none;
  }
  .toggle-btn.active .toggle-thumb { transform: translateX(0.8125rem); }

  /* Routine info */
  .routine-info {
    flex: 1;
    min-width: 0;
    display: flex;
    flex-direction: column;
    gap: 0.0625rem;
  }
  .routine-title {
    font-size: 0.875rem;
    color: var(--c-label-primary);
    line-height: 1.35;
    overflow-wrap: anywhere;
  }
  .routine-sub {
    font-size: 0.6875rem;
    color: var(--c-label-tertiary);
    line-height: 1.3;
  }
  .routine-meta-sep { margin: 0 0.2em; }

  /* Action buttons */
  .routine-actions {
    flex-shrink: 0;
    display: flex;
    align-items: center;
    gap: 0.25rem;
  }

  .icon-btn {
    display: grid;
    place-items: center;
    width: 1.5rem;
    height: 1.5rem;
    padding: 0;
    border: none;
    border-radius: var(--radius-sm);
    background: none;
    color: var(--c-label-quaternary);
    cursor: pointer;
    transition: background 80ms ease, color 80ms ease;
  }
  .icon-btn svg { width: 0.75rem; height: 0.75rem; }
  .icon-btn:hover { background: var(--c-fill-secondary); color: var(--c-label-secondary); }
  .icon-btn.danger:hover { background: rgba(255, 59, 48, 0.08); color: var(--c-destructive); }
  .icon-btn:focus-visible { outline: 2px solid var(--c-accent); outline-offset: 1px; }

  /* Inline delete confirm */
  .delete-confirm-label {
    font-size: 0.6875rem;
    color: var(--c-label-secondary);
    white-space: nowrap;
  }
  .btn-danger-sm {
    padding: 0.125rem 0.5rem;
    border: none;
    border-radius: var(--radius-sm);
    background: var(--c-destructive);
    color: #fff;
    font: inherit;
    font-size: 0.6875rem;
    font-weight: 500;
    cursor: pointer;
    white-space: nowrap;
    transition: opacity 80ms ease;
  }
  .btn-danger-sm:hover:not(:disabled) { opacity: 0.85; }
  .btn-danger-sm:disabled { opacity: 0.5; cursor: wait; }
  .btn-ghost-sm {
    padding: 0.125rem 0.5rem;
    border: 1px solid var(--c-separator-opaque);
    border-radius: var(--radius-sm);
    background: none;
    color: var(--c-label-secondary);
    font: inherit;
    font-size: 0.6875rem;
    cursor: pointer;
    white-space: nowrap;
    transition: background 80ms ease;
  }
  .btn-ghost-sm:hover:not(:disabled) { background: var(--c-fill-secondary); }
  .btn-ghost-sm:disabled { opacity: 0.5; cursor: wait; }

  /* ── Inactive section ─────────────────────────────────────────────── */
  .inactive-section { margin-top: 1rem; }
  .inactive-label {
    font-size: 0.625rem;
    font-weight: 700;
    letter-spacing: 0.08em;
    text-transform: uppercase;
    color: var(--c-label-quaternary);
    margin-bottom: 0.25rem;
  }

  /* ── Add button / form area ───────────────────────────────────────── */
  .form-area { margin-top: auto; padding-top: 0.75rem; }
  .form-divider { border-top: 1px solid var(--c-separator); margin-bottom: 1rem; }

  .add-routine-btn {
    display: flex;
    align-items: center;
    gap: 0.375rem;
    width: 100%;
    padding: 0.5rem 0.25rem;
    border: none;
    border-top: 1px solid var(--c-separator);
    background: none;
    color: var(--c-label-tertiary);
    font: inherit;
    font-size: 0.875rem;
    cursor: pointer;
    text-align: left;
    transition: color 80ms ease;
  }
  .add-routine-btn svg { width: 0.75rem; height: 0.75rem; flex-shrink: 0; }
  .add-routine-btn:hover { color: var(--c-label-primary); }
  .add-routine-btn:focus-visible { outline: 2px solid var(--c-accent); outline-offset: 1px; }

  /* ── Form ─────────────────────────────────────────────────────────── */
  .routine-form { display: flex; flex-direction: column; gap: 0.75rem; }
  .form-heading {
    font-size: 0.75rem;
    font-weight: 600;
    color: var(--c-label-secondary);
    letter-spacing: 0.02em;
  }

  .field { display: flex; flex-direction: column; gap: 0.25rem; }
  .field-label {
    font-size: 0.6875rem;
    font-weight: 600;
    color: var(--c-label-secondary);
    letter-spacing: 0.02em;
  }
  .optional {
    font-weight: 400;
    color: var(--c-label-quaternary);
  }

  .field input[type="text"],
  .field input[type="time"],
  .field input[type="number"] {
    width: 100%;
    padding: 0.375rem 0.5rem;
    border: 1px solid var(--c-separator-opaque);
    border-radius: var(--radius-sm);
    background: var(--c-fill-tertiary);
    color: var(--c-label-primary);
    font: inherit;
    font-size: 0.8125rem;
    box-sizing: border-box;
    transition: border-color 100ms ease, box-shadow 100ms ease;
  }
  .field input:focus {
    outline: none;
    border-color: var(--c-accent);
    box-shadow: 0 0 0 2.5px var(--c-accent-muted);
  }
  .field input:disabled { opacity: 0.55; }

  .form-row {
    display: flex;
    gap: 0.75rem;
  }
  .field--half { flex: 1; min-width: 0; }

  /* Day pills */
  .days-field { border: none; padding: 0; }
  .day-pills {
    display: flex;
    gap: 0.25rem;
    flex-wrap: wrap;
  }
  .day-pill {
    display: grid;
    place-items: center;
    width: 2rem;
    height: 2rem;
    border: 1.5px solid var(--c-separator-opaque);
    border-radius: 50%;
    background: none;
    color: var(--c-label-tertiary);
    font: inherit;
    font-size: 0.6875rem;
    font-weight: 600;
    cursor: pointer;
    transition: border-color 100ms ease, background 100ms ease, color 100ms ease;
  }
  .day-pill:hover:not(:disabled) { border-color: var(--c-accent); color: var(--c-accent); }
  .day-pill.selected {
    border-color: var(--c-accent);
    background: var(--c-accent);
    color: #fff;
  }
  .day-pill:disabled { opacity: 0.5; cursor: wait; }
  .day-pill:focus-visible { outline: 2px solid var(--c-accent); outline-offset: 2px; }

  /* Form actions */
  .form-error {
    color: var(--c-destructive);
    font-size: 0.75rem;
    margin: 0;
  }
  .form-actions {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    flex-wrap: wrap;
  }
  .btn-primary {
    padding: 0.3125rem 0.875rem;
    border: none;
    border-radius: var(--radius-sm);
    background: var(--c-accent);
    color: #fff;
    font: inherit;
    font-size: 0.8125rem;
    font-weight: 500;
    cursor: pointer;
    transition: opacity 80ms ease;
  }
  .btn-primary:hover:not(:disabled) { opacity: 0.85; }
  .btn-primary:disabled { opacity: 0.5; cursor: wait; }
  .btn-ghost {
    padding: 0.3125rem 0.75rem;
    border: 1px solid var(--c-separator-opaque);
    border-radius: var(--radius-sm);
    background: none;
    color: var(--c-label-secondary);
    font: inherit;
    font-size: 0.8125rem;
    cursor: pointer;
    transition: background 80ms ease;
  }
  .btn-ghost:hover:not(:disabled) { background: var(--c-fill-secondary); }
  .btn-ghost:disabled { opacity: 0.5; cursor: wait; }
  .kbd-hint {
    margin-left: auto;
    color: var(--c-label-quaternary);
    font-size: 0.6875rem;
  }

  /* ── Windows platform overrides ──────────────────────────────────── */
  :global([data-platform="windows"]) .panel-header {
    padding: 1rem 1.25rem 0.75rem;
  }
  :global([data-platform="windows"]) .panel-body {
    padding: 0.625rem 1.25rem 1.25rem;
  }
  :global([data-platform="windows"]) .field input[type="text"],
  :global([data-platform="windows"]) .field input[type="time"],
  :global([data-platform="windows"]) .field input[type="number"] {
    border-radius: 0.25rem;
    border-color: var(--c-separator-opaque);
  }
  :global([data-platform="windows"]) .field input:focus {
    box-shadow: none;
    border-color: var(--c-accent);
    border-bottom-width: 2px;
  }
  :global([data-platform="windows"]) .day-pill {
    border-radius: 0.25rem;
  }
</style>
