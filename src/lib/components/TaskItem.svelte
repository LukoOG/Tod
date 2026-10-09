<script lang="ts">
  import type { Task } from "../types/planner";
  import { toggleTask, updateTaskNotes } from "$lib/api";
  import { deriveStatus } from "$lib/status";

  interface Props {
    task: Task;
    today: string;
    onTaskToggled: (task: Task) => void;
    onTaskNotesChanged: (task: Task) => void;
  }

  let { task, today, onTaskToggled, onTaskNotesChanged }: Props = $props();

  let isUpdating = $state(false);
  let showNotes = $state(false);
  let draftNotes = $state("");
  let isSavingNotes = $state(false);
  let notesError = $state<string | null>(null);

  // Keep draft in sync with prop when notes are saved externally
  // (parent replaces the task object) and on first mount.
  $effect(() => {
    if (!showNotes) draftNotes = task.notes ?? "";
  });

  const status = $derived(deriveStatus(task.completed, task.day_date, today));

  async function handleToggleTask() {
    if (isUpdating) return;
    isUpdating = true;
    try {
      onTaskToggled(await toggleTask(task.id));
    } finally {
      isUpdating = false;
    }
  }

  function toggleNotes() {
    if (!showNotes) {
      draftNotes = task.notes ?? "";
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
      const updated = await updateTaskNotes(task.id, value);
      onTaskNotesChanged(updated);
      showNotes = false;
    } catch (e) {
      notesError = e instanceof Error ? e.message : "Failed to save notes.";
    } finally {
      isSavingNotes = false;
    }
  }

  function cancelNotes() {
    draftNotes = task.notes ?? "";
    notesError = null;
    showNotes = false;
  }
</script>

<article class="item" class:completed={status === "completed"} class:missed={status === "missed"}>
  <button
    type="button"
    class="checkbox"
    aria-pressed={task.completed}
    aria-label={task.completed ? `Mark "${task.title}" incomplete` : `Mark "${task.title}" complete`}
    disabled={isUpdating}
    onclick={handleToggleTask}
  >
    {#if status === "completed"}
      <svg viewBox="0 0 24 24" aria-hidden="true">
        <path d="m5 12 4.25 4.25L19 6.5" />
      </svg>
    {:else if status === "missed"}
      <svg viewBox="0 0 24 24" aria-hidden="true">
        <path d="M18 6 6 18M6 6l12 12" />
      </svg>
    {/if}
  </button>

  <div class="details">
    <p title={task.title}>{task.title}</p>
    {#if task.notes && !showNotes}
      <button type="button" class="notes-preview" onclick={toggleNotes} aria-label="Edit note">
        {task.notes}
      </button>
    {/if}
    {#if showNotes}
      <div class="notes-editor">
        <textarea
          aria-label="Notes for {task.title}"
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

  {#if !showNotes}
    <button
      type="button"
      class="note-toggle"
      aria-label={task.notes ? "Edit note" : "Add note"}
      aria-pressed={showNotes}
      onclick={toggleNotes}
      tabindex={-1}
    >
      <svg viewBox="0 0 24 24" aria-hidden="true">
        <path d="M12 20h9M16.5 3.5a2.121 2.121 0 013 3L7 19l-4 1 1-4L16.5 3.5z" />
      </svg>
    </button>
  {/if}
</article>

<style>
  .item {
    display: flex;
    gap: 0.75rem;
    align-items: flex-start;
    padding: 0.7rem 0;
    border-top: 1px solid #e6e7e2;
  }
  .checkbox {
    display: grid;
    flex: 0 0 auto;
    place-items: center;
    width: 1.3rem;
    height: 1.3rem;
    margin-top: 0.15rem;
    padding: 0;
    border: 1.5px solid #bfc9bc;
    border-radius: 0.3rem;
    background: transparent;
    color: #fff;
    cursor: pointer;
    transition: background-color 100ms ease, border-color 100ms ease, transform 100ms ease;
  }
  .checkbox svg {
    width: 0.875rem;
    height: 0.875rem;
    fill: none;
    stroke: currentColor;
    stroke-width: 2.5;
    stroke-linecap: round;
    stroke-linejoin: round;
  }
  .checkbox:hover:not(:disabled) {
    border-color: #7a9179;
    background: #f0f5ee;
  }
  .checkbox:active:not(:disabled) { transform: scale(0.88); }
  .checkbox:focus-visible { outline: 2px solid #b8d0b5; outline-offset: 2px; }
  .checkbox:disabled { cursor: wait; opacity: 0.5; }

  .details {
    flex: 1;
    min-width: 0;
  }
  p {
    overflow-wrap: anywhere;
    margin: 0;
    color: #252824;
    font-size: 0.9375rem;
    line-height: 1.4;
  }

  /* ── notes preview ───────────────────────────────── */
  .notes-preview {
    display: block;
    width: 100%;
    margin-top: 0.25rem;
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
    margin-top: 0.35rem;
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

  /* ── note toggle ─────────────────────────────────── */
  .note-toggle {
    flex: 0 0 auto;
    display: grid;
    place-items: center;
    width: 1.5rem;
    height: 1.5rem;
    margin-top: 0.1rem;
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

  /* ── completed state ─────────────────────────────── */
  .completed p {
    color: #a2a8a0;
    text-decoration: line-through;
    text-decoration-thickness: 1px;
    text-decoration-color: #b8bdb6;
  }
  .completed .checkbox {
    border-color: #6a826a;
    background: #6a826a;
  }
  .completed .checkbox:hover:not(:disabled) {
    border-color: #547054;
    background: #547054;
  }

  /* ── missed state ────────────────────────────────── */
  .missed .checkbox {
    border-color: #c8a898;
    color: #a06050;
  }
  .missed .checkbox:hover:not(:disabled) {
    border-color: #b07060;
    background: #faf0ee;
  }
</style>
