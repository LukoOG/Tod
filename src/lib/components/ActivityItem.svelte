<script lang="ts">
  import { completeActivity, updateActivityNotes } from "$lib/api";
  import { deriveStatus } from "$lib/status";
  import type { Activity } from "../types/planner";

  function focus(node: HTMLElement) { node.focus(); }

  interface Props {
    activity: Activity;
    dayDate: string;
    today: string;
    onActivityCompleted: (activity: Activity) => void;
    onActivityNotesChanged: (activity: Activity) => void;
  }

  let {
    activity,
    dayDate,
    today,
    onActivityCompleted,
    onActivityNotesChanged,
  }: Props = $props();

  let isUpdating = $state(false);
  let showNotes = $state(false);
  let draftNotes = $state("");
  let isSavingNotes = $state(false);
  let notesError = $state<string | null>(null);
  let rowHovered = $state(false);

  $effect(() => {
    if (!showNotes) draftNotes = activity.notes ?? "";
  });

  const status = $derived(deriveStatus(activity.completed, dayDate, today));
  const duration = $derived(formatDuration(activity.duration_minutes));

  function formatTime(time: string | null): string {
    if (!time) return "";
    const [hStr, mStr] = time.split(":");
    const h = parseInt(hStr, 10);
    const m = parseInt(mStr, 10);
    const period = h < 12 ? "AM" : "PM";
    const hour = h % 12 === 0 ? 12 : h % 12;
    return m === 0 ? `${hour} ${period}` : `${hour}:${mStr} ${period}`;
  }

  function formatDuration(minutes: number | null): string | null {
    if (minutes === null) return null;
    const hours = Math.floor(minutes / 60);
    const rem = minutes % 60;
    if (hours && rem) return `${hours}h ${rem}m`;
    return hours ? `${hours}h` : `${rem}m`;
  }

  async function handleComplete() {
    if (isUpdating) return;
    isUpdating = true;
    try {
      onActivityCompleted(await completeActivity(activity.id));
    } finally {
      isUpdating = false;
    }
  }

  function openNotes() {
    draftNotes = activity.notes ?? "";
    notesError = null;
    showNotes = true;
  }

  function closeNotes() {
    draftNotes = activity.notes ?? "";
    notesError = null;
    showNotes = false;
  }

  async function saveNotes() {
    if (isSavingNotes) return;
    const trimmed = draftNotes.trim();
    const value = trimmed === "" ? null : trimmed;
    isSavingNotes = true;
    notesError = null;
    try {
      const updated = await updateActivityNotes(activity.id, value);
      onActivityNotesChanged(updated);
      showNotes = false;
    } catch (e) {
      notesError = e instanceof Error ? e.message : "Failed to save notes.";
    } finally {
      isSavingNotes = false;
    }
  }

  function handleNotesKeydown(e: KeyboardEvent) {
    if (e.key === "Enter" && (e.metaKey || e.ctrlKey)) { e.preventDefault(); saveNotes(); }
    if (e.key === "Escape") { closeNotes(); }
  }
</script>

<article
  class="row"
  class:is-completed={status === "completed"}
  class:is-missed={status === "missed"}
  onmouseenter={() => rowHovered = true}
  onmouseleave={() => rowHovered = false}
  role="listitem"
>
  <!-- Time column -->
  <div class="time-col">
    {#if activity.start_time}
      <span class="time">{formatTime(activity.start_time)}</span>
      {#if duration}<span class="dur">{duration}</span>{/if}
    {:else}
      <span class="time anytime">—</span>
    {/if}
  </div>

  <!-- Content column -->
  <div class="content">
    <button
      type="button"
      class="title-btn"
      aria-pressed={activity.completed}
      aria-label={activity.completed ? `Mark "${activity.title}" incomplete` : `Complete "${activity.title}"`}
      disabled={isUpdating}
      onclick={handleComplete}
    >
      <span class="check-ring" aria-hidden="true">
        {#if status === "completed"}
          <!-- checkmark -->
          <svg viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="M3 8l3.5 3.5L13 4.5"/>
          </svg>
        {:else if status === "missed"}
          <!-- x mark -->
          <svg viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round">
            <path d="M4 4l8 8M12 4l-8 8"/>
          </svg>
        {/if}
      </span>
      <span class="title-text">{activity.title}</span>
    </button>

    {#if activity.notes && !showNotes}
      <button type="button" class="note-chip" onclick={openNotes} aria-label="View/edit note">
        <svg viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
          <path d="M2 12.5V14h1.5l7-7L9 5.5l-7 7zM13.5 3.5a1.121 1.121 0 0 0-1.586-1.586L10.5 3.33 12.17 5l1.33-1.5z"/>
        </svg>
        {activity.notes}
      </button>
    {/if}

    {#if showNotes}
      <div class="notes-editor">
        <textarea
          aria-label="Notes for {activity.title}"
          placeholder="Add a note…"
          rows={2}
          bind:value={draftNotes}
          disabled={isSavingNotes}
          onkeydown={handleNotesKeydown}
          use:focus
        ></textarea>
        {#if notesError}
          <p class="field-error" role="alert">{notesError}</p>
        {/if}
        <div class="editor-actions">
          <button type="button" class="btn-primary-sm" onclick={saveNotes} disabled={isSavingNotes}>
            {isSavingNotes ? "Saving…" : "Save"}
          </button>
          <button type="button" class="btn-ghost-sm" onclick={closeNotes} disabled={isSavingNotes}>Cancel</button>
          <span class="kbd-hint">⌘↵ to save · Esc to cancel</span>
        </div>
      </div>
    {/if}
  </div>

  <!-- Actions column (shown on hover or when notes open) -->
  {#if !showNotes}
    <button
      type="button"
      class="note-btn"
      class:visible={rowHovered || !!activity.notes}
      aria-label={activity.notes ? "Edit note" : "Add note"}
      onclick={openNotes}
      tabindex={rowHovered ? 0 : -1}
    >
      <svg viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
        <path d="M2 12.5V14h1.5l7-7L9 5.5l-7 7zM13.5 3.5a1.121 1.121 0 0 0-1.586-1.586L10.5 3.33 12.17 5l1.33-1.5z"/>
      </svg>
    </button>
  {/if}
</article>

<style>
  .row {
    display: grid;
    grid-template-columns: 4.5rem 1fr 1.625rem;
    gap: 0 0.75rem;
    align-items: start;
    padding: 0.5rem 0.25rem 0.5rem 0;
    border-top: 1px solid var(--c-separator);
    transition: background-color 80ms ease;
  }
  .row:hover { background: var(--c-hover); border-radius: var(--radius-sm); }

  /* Time column */
  .time-col {
    display: flex;
    flex-direction: column;
    gap: 0.0625rem;
    padding-top: 0.1875rem;
    text-align: right;
  }
  .time {
    color: var(--c-label-secondary);
    font-size: 0.6875rem;
    font-weight: 600;
    font-variant-numeric: tabular-nums;
    letter-spacing: 0.01em;
    white-space: nowrap;
    line-height: 1.3;
  }
  .time.anytime { color: var(--c-label-tertiary); }
  .dur {
    color: var(--c-label-tertiary);
    font-size: 0.625rem;
    font-variant-numeric: tabular-nums;
    letter-spacing: 0.01em;
  }

  /* Content column */
  .content { min-width: 0; }

  .title-btn {
    display: flex;
    align-items: baseline;
    gap: 0.5rem;
    width: 100%;
    padding: 0;
    border: none;
    background: none;
    text-align: left;
    cursor: pointer;
    color: inherit;
  }
  .title-btn:focus-visible { outline: none; }
  .title-btn:focus-visible .check-ring { box-shadow: 0 0 0 2.5px var(--c-accent-muted); }

  .check-ring {
    flex-shrink: 0;
    display: grid;
    place-items: center;
    width: 1.125rem;
    height: 1.125rem;
    margin-top: 0.125rem;
    border: 1.5px solid var(--c-check-border);
    border-radius: 50%;
    background: var(--c-check-bg);
    color: var(--c-check-icon);
    transition: border-color 100ms ease, background 100ms ease, transform 100ms ease;
  }
  .check-ring svg {
    width: 0.625rem;
    height: 0.625rem;
  }
  .title-btn:hover:not(:disabled) .check-ring {
    border-color: var(--c-accent);
    transform: scale(1.08);
  }
  .title-btn:active:not(:disabled) .check-ring { transform: scale(0.92); }
  .title-btn:disabled { cursor: wait; }
  .title-btn:disabled .check-ring { opacity: 0.5; }

  .title-text {
    flex: 1;
    overflow-wrap: anywhere;
    font-size: 0.875rem;
    font-weight: 400;
    line-height: 1.4;
    color: var(--c-label-primary);
    transition: color 80ms ease;
  }

  /* completed */
  .is-completed .check-ring {
    --c-check-border: var(--c-accent);
    --c-check-bg: var(--c-accent);
    --c-check-icon: #fff;
    border-color: var(--c-accent);
    background: var(--c-accent);
    color: #fff;
  }
  .is-completed .title-text {
    color: var(--c-label-tertiary);
    text-decoration: line-through;
    text-decoration-thickness: 1px;
    text-decoration-color: var(--c-label-quaternary);
  }
  .is-completed .time,
  .is-completed .dur { color: var(--c-label-quaternary); }

  /* missed */
  .is-missed .check-ring {
    --c-check-border: var(--c-destructive-muted);
    border-color: var(--c-destructive-muted);
    color: var(--c-destructive-muted);
  }
  .is-missed .title-text { color: var(--c-label-secondary); }

  /* note chip (inline note preview) */
  .note-chip {
    display: flex;
    align-items: flex-start;
    gap: 0.3125rem;
    margin-top: 0.25rem;
    margin-left: calc(1.125rem + 0.5rem); /* align with title text */
    padding: 0;
    border: none;
    background: none;
    color: var(--c-label-tertiary);
    font: inherit;
    font-size: 0.75rem;
    font-style: italic;
    line-height: 1.4;
    text-align: left;
    overflow-wrap: anywhere;
    cursor: pointer;
    transition: color 80ms ease;
  }
  .note-chip svg { flex-shrink: 0; width: 0.6875rem; height: 0.6875rem; margin-top: 0.125rem; }
  .note-chip:hover { color: var(--c-label-secondary); }

  /* notes editor */
  .notes-editor {
    margin-top: 0.375rem;
    margin-left: calc(1.125rem + 0.5rem);
  }
  textarea {
    width: 100%;
    min-height: 3.25rem;
    padding: 0.375rem 0.5rem;
    border: 1px solid var(--c-separator-opaque);
    border-radius: var(--radius-sm);
    background: var(--c-fill-tertiary);
    color: var(--c-label-primary);
    font: inherit;
    font-size: 0.8125rem;
    line-height: 1.5;
    resize: vertical;
    transition: border-color 100ms ease, box-shadow 100ms ease;
    box-sizing: border-box;
  }
  textarea:focus {
    outline: none;
    border-color: var(--c-accent);
    box-shadow: 0 0 0 2.5px var(--c-accent-muted);
  }
  textarea:disabled { opacity: 0.55; }
  .field-error {
    margin: 0.25rem 0 0;
    color: var(--c-destructive);
    font-size: 0.75rem;
  }
  .editor-actions {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    margin-top: 0.3125rem;
    flex-wrap: wrap;
  }
  .btn-primary-sm {
    padding: 0.1875rem 0.625rem;
    border: none;
    border-radius: var(--radius-sm);
    background: var(--c-accent);
    color: #fff;
    font: inherit;
    font-size: 0.75rem;
    font-weight: 500;
    cursor: pointer;
    transition: opacity 80ms ease;
  }
  .btn-primary-sm:hover:not(:disabled) { opacity: 0.85; }
  .btn-primary-sm:disabled { opacity: 0.5; cursor: wait; }
  .btn-ghost-sm {
    padding: 0.1875rem 0.5rem;
    border: 1px solid var(--c-separator-opaque);
    border-radius: var(--radius-sm);
    background: none;
    color: var(--c-label-secondary);
    font: inherit;
    font-size: 0.75rem;
    cursor: pointer;
    transition: background 80ms ease;
  }
  .btn-ghost-sm:hover:not(:disabled) { background: var(--c-fill-secondary); }
  .btn-ghost-sm:disabled { opacity: 0.5; cursor: wait; }
  .kbd-hint {
    margin-left: auto;
    color: var(--c-label-quaternary);
    font-size: 0.6875rem;
  }

  /* note toggle button (hover-reveal) */
  .note-btn {
    display: grid;
    place-items: center;
    width: 1.375rem;
    height: 1.375rem;
    margin-top: 0.125rem;
    padding: 0;
    border: none;
    border-radius: var(--radius-sm);
    background: none;
    color: var(--c-label-quaternary);
    cursor: pointer;
    opacity: 0;
    transition: opacity 80ms ease, background 80ms ease, color 80ms ease;
  }
  .note-btn.visible { opacity: 1; }
  .note-btn svg { width: 0.75rem; height: 0.75rem; }
  .note-btn:hover { background: var(--c-fill-secondary); color: var(--c-label-secondary); }
  .note-btn:focus-visible { opacity: 1; outline: 2px solid var(--c-accent); outline-offset: 1px; }
</style>
