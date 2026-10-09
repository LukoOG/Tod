<script lang="ts">
  import { completeActivity, updateActivityNotes } from "$lib/api";
  import { deriveStatus } from "$lib/status";
  import type { Activity } from "../types/planner";

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

  // Keep draft in sync with prop when notes are saved externally
  // (parent replaces the activity object) and on first mount.
  $effect(() => {
    if (!showNotes) draftNotes = activity.notes ?? "";
  });

  const status = $derived(deriveStatus(activity.completed, dayDate, today));
  const duration = $derived(formatDuration(activity.duration_minutes));

  function formatTime(time: string | null): string {
    if (!time) return "Any time";
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

  async function handleCompleteActivity() {
    if (isUpdating) return;
    isUpdating = true;
    try {
      onActivityCompleted(await completeActivity(activity.id));
    } finally {
      isUpdating = false;
    }
  }

  function toggleNotes() {
    if (!showNotes) {
      draftNotes = activity.notes ?? "";
      notesError = null;
    }
    showNotes = !showNotes;
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

  function cancelNotes() {
    draftNotes = activity.notes ?? "";
    notesError = null;
    showNotes = false;
  }
</script>

<article class="item" class:completed={status === "completed"} class:missed={status === "missed"}>
  <time datetime={activity.start_time ?? undefined} class:anytime={!activity.start_time}>
    {formatTime(activity.start_time)}
  </time>
  <div class="details">
    <p title={activity.title}>{activity.title}</p>
    {#if duration}
      <span class="duration">{duration}</span>
    {/if}
    {#if activity.notes && !showNotes}
      <button type="button" class="notes-preview" onclick={toggleNotes} aria-label="Edit note">
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
        ></textarea>
        {#if notesError}
          <p class="notes-error" role="alert">{notesError}</p>
        {/if}
        <div class="notes-actions">
          <button type="button" class="notes-save" onclick={saveNotes} disabled={isSavingNotes}>
            {isSavingNotes ? "Saving…" : "Save"}
          </button>
          <button type="button" class="notes-cancel" onclick={cancelNotes} disabled={isSavingNotes}>
            Cancel
          </button>
        </div>
      </div>
    {/if}
  </div>
  <div class="item-actions">
    {#if !showNotes}
      <button
        type="button"
        class="note-toggle"
        aria-label={activity.notes ? "Edit note" : "Add note"}
        aria-pressed={showNotes}
        onclick={toggleNotes}
        tabindex={-1}
      >
        <svg viewBox="0 0 24 24" aria-hidden="true">
          <path d="M12 20h9M16.5 3.5a2.121 2.121 0 013 3L7 19l-4 1 1-4L16.5 3.5z" />
        </svg>
      </button>
    {/if}
    <button
      type="button"
      class="complete-btn"
      aria-pressed={activity.completed}
      aria-label={activity.completed ? `Mark "${activity.title}" incomplete` : `Mark "${activity.title}" complete`}
      disabled={isUpdating}
      onclick={handleCompleteActivity}
    >
      <svg viewBox="0 0 24 24" aria-hidden="true">
        {#if status === "completed"}
          <path d="m5 12 4.25 4.25L19 6.5" />
        {:else if status === "missed"}
          <path d="M18 6 6 18M6 6l12 12" />
        {:else}
          <circle cx="12" cy="12" r="7.5" />
        {/if}
      </svg>
    </button>
  </div>
</article>

<style>
  .item {
    display: grid;
    grid-template-columns: 5.25rem minmax(0, 1fr) auto;
    gap: 0.85rem 1rem;
    align-items: start;
    padding: 0.85rem 0;
    border-top: 1px solid #e6e7e2;
  }
  time {
    padding-top: 0.15rem;
    color: #5c6b62;
    font-size: 0.8rem;
    font-variant-numeric: tabular-nums;
    font-weight: 600;
    letter-spacing: 0.01em;
    white-space: nowrap;
    line-height: 1.3;
  }
  time.anytime {
    color: #8f9490;
    font-weight: 500;
    font-style: italic;
    font-size: 0.775rem;
  }
  .details { min-width: 0; }
  p {
    overflow-wrap: anywhere;
    margin: 0;
    color: #252824;
    font-size: 0.9375rem;
    font-weight: 500;
    line-height: 1.35;
  }
  .duration {
    display: block;
    overflow-wrap: anywhere;
    margin-top: 0.175rem;
    color: #8a8f88;
    font-size: 0.78rem;
    letter-spacing: 0.01em;
  }

  /* ── notes preview ───────────────────────────────── */
  .notes-preview {
    display: block;
    width: 100%;
    margin-top: 0.3rem;
    padding: 0;
    border: none;
    background: none;
    color: #7a8a78;
    font: inherit;
    font-size: 0.8125rem;
    font-style: italic;
    text-align: left;
    overflow-wrap: anywhere;
    cursor: pointer;
    transition: color 100ms ease;
  }
  .notes-preview:hover { color: #4f6450; }

  /* ── notes editor ────────────────────────────────── */
  .notes-editor {
    margin-top: 0.4rem;
  }
  textarea {
    width: 100%;
    min-height: 3.5rem;
    padding: 0.4rem 0.5rem;
    border: 1px solid #c8d0c4;
    border-radius: 0.375rem;
    background: #f7f8f4;
    color: #252824;
    font: inherit;
    font-size: 0.875rem;
    line-height: 1.45;
    resize: vertical;
    transition: border-color 100ms ease;
    box-sizing: border-box;
  }
  textarea:focus {
    outline: none;
    border-color: #6f8a6f;
    box-shadow: 0 0 0 2px #b8d0b540;
  }
  textarea:disabled { opacity: 0.6; }
  .notes-error {
    margin: 0.25rem 0 0;
    color: #8c4a38;
    font-size: 0.8125rem;
  }
  .notes-actions {
    display: flex;
    gap: 0.5rem;
    margin-top: 0.35rem;
  }
  .notes-save,
  .notes-cancel {
    padding: 0.2rem 0;
    border: 0;
    border-bottom: 1px solid currentColor;
    border-radius: 0;
    background: transparent;
    font: inherit;
    font-size: 0.8125rem;
    cursor: pointer;
    transition: opacity 100ms ease;
  }
  .notes-save { color: #5f765f; font-weight: 650; }
  .notes-cancel { color: #909690; }
  .notes-save:hover:not(:disabled),
  .notes-cancel:hover:not(:disabled) { opacity: 0.7; }
  .notes-save:disabled,
  .notes-cancel:disabled { cursor: wait; opacity: 0.45; }

  /* ── action buttons column ───────────────────────── */
  .item-actions {
    display: flex;
    align-items: center;
    gap: 0.3rem;
    padding-top: 0.05rem;
  }
  .note-toggle {
    display: grid;
    place-items: center;
    width: 1.5rem;
    height: 1.5rem;
    padding: 0;
    border: none;
    border-radius: 50%;
    background: transparent;
    color: #c0c8bc;
    cursor: pointer;
    transition: color 100ms ease, background-color 100ms ease;
  }
  .note-toggle svg {
    width: 0.875rem;
    height: 0.875rem;
    fill: none;
    stroke: currentColor;
    stroke-width: 1.75;
    stroke-linecap: round;
    stroke-linejoin: round;
  }
  .note-toggle:hover { color: #6a826a; background: #f0f5ee; }
  .note-toggle:focus-visible { outline: 2px solid #b8d0b5; outline-offset: 2px; }

  .complete-btn {
    display: grid;
    place-items: center;
    width: 1.85rem;
    height: 1.85rem;
    padding: 0;
    border: 1.5px solid #bfc9bc;
    border-radius: 50%;
    background: transparent;
    color: #6a826a;
    cursor: pointer;
    transition: background-color 100ms ease, border-color 100ms ease, transform 100ms ease;
  }
  .complete-btn svg {
    width: 1.0625rem;
    height: 1.0625rem;
    fill: none;
    stroke: currentColor;
    stroke-width: 2;
    stroke-linecap: round;
    stroke-linejoin: round;
  }
  .complete-btn:hover:not(:disabled) {
    border-color: #7a9179;
    background: #f0f5ee;
    transform: scale(1.05);
  }
  .complete-btn:active:not(:disabled) { transform: scale(0.95); }
  .complete-btn:focus-visible { outline: 2px solid #b8d0b5; outline-offset: 2px; }
  .complete-btn:disabled { cursor: wait; opacity: 0.5; }

  /* ── completed state ─────────────────────────────── */
  .completed time,
  .completed .duration {
    color: #a2a8a0;
  }
  .completed p {
    color: #a2a8a0;
    text-decoration: line-through;
    text-decoration-thickness: 1px;
    text-decoration-color: #b8bdb6;
  }
  .completed .complete-btn {
    border-color: #6a826a;
    background: #6a826a;
    color: #fff;
  }
  .completed .complete-btn:hover:not(:disabled) {
    border-color: #547054;
    background: #547054;
    transform: scale(1.05);
  }

  /* ── missed state ────────────────────────────────── */
  .missed .complete-btn {
    border-color: #c8a898;
    color: #a06050;
  }
  .missed .complete-btn:hover:not(:disabled) {
    border-color: #b07060;
    background: #faf0ee;
    transform: scale(1.05);
  }

  @media (max-width: 460px) {
    .item {
      grid-template-columns: 4.5rem minmax(0, 1fr) auto;
      gap: 0.6rem 0.75rem;
    }
  }
</style>
