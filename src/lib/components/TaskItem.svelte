<script lang="ts">
  import type { Task } from "../types/planner";
  import { toggleTask, updateTaskNotes } from "$lib/api";
  import { deriveStatus } from "$lib/status";

  function focus(node: HTMLElement) { node.focus(); }

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
  let rowHovered = $state(false);

  $effect(() => {
    if (!showNotes) draftNotes = task.notes ?? "";
  });

  const status = $derived(deriveStatus(task.completed, task.day_date, today));

  async function handleToggle() {
    if (isUpdating) return;
    isUpdating = true;
    try {
      onTaskToggled(await toggleTask(task.id));
    } finally {
      isUpdating = false;
    }
  }

  function openNotes() {
    draftNotes = task.notes ?? "";
    notesError = null;
    showNotes = true;
  }

  function closeNotes() {
    draftNotes = task.notes ?? "";
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
      const updated = await updateTaskNotes(task.id, value);
      onTaskNotesChanged(updated);
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
  <button
    type="button"
    class="checkbox"
    aria-pressed={task.completed}
    aria-label={task.completed ? `Mark "${task.title}" incomplete` : `Complete "${task.title}"`}
    disabled={isUpdating}
    onclick={handleToggle}
  >
    {#if status === "completed"}
      <svg viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
        <path d="M3 8l3.5 3.5L13 4.5"/>
      </svg>
    {:else if status === "missed"}
      <svg viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" aria-hidden="true">
        <path d="M4 4l8 8M12 4l-8 8"/>
      </svg>
    {/if}
  </button>

  <div class="content">
    <p>{task.title}</p>

    {#if task.notes && !showNotes}
      <button type="button" class="note-chip" onclick={openNotes} aria-label="View/edit note">
        <svg viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
          <path d="M2 12.5V14h1.5l7-7L9 5.5l-7 7zM13.5 3.5a1.121 1.121 0 0 0-1.586-1.586L10.5 3.33 12.17 5l1.33-1.5z"/>
        </svg>
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

  {#if !showNotes}
    <button
      type="button"
      class="note-btn"
      class:visible={rowHovered || !!task.notes}
      aria-label={task.notes ? "Edit note" : "Add note"}
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
    display: flex;
    align-items: flex-start;
    gap: 0.625rem;
    padding: 0.4375rem 0.25rem 0.4375rem 0;
    border-top: 1px solid var(--c-separator);
    transition: background 80ms ease;
  }
  .row:hover { background: var(--c-hover); border-radius: var(--radius-sm); }

  .checkbox {
    flex-shrink: 0;
    display: grid;
    place-items: center;
    width: 1.125rem;
    height: 1.125rem;
    margin-top: 0.125rem;
    padding: 0;
    border: 1.5px solid var(--c-check-border, var(--c-separator-opaque));
    border-radius: 0.25rem;
    background: var(--c-check-bg, transparent);
    color: var(--c-check-icon, transparent);
    cursor: pointer;
    transition: border-color 100ms ease, background 100ms ease, transform 100ms ease;
  }
  .checkbox svg { width: 0.625rem; height: 0.625rem; }
  .checkbox:hover:not(:disabled) {
    border-color: var(--c-accent);
    transform: scale(1.06);
  }
  .checkbox:active:not(:disabled) { transform: scale(0.88); }
  .checkbox:focus-visible { outline: 2px solid var(--c-accent-muted); outline-offset: 1px; }
  .checkbox:disabled { cursor: wait; opacity: 0.5; }

  .content { flex: 1; min-width: 0; }

  p {
    overflow-wrap: anywhere;
    margin: 0;
    color: var(--c-label-primary);
    font-size: 0.875rem;
    font-weight: 400;
    line-height: 1.4;
    transition: color 80ms ease;
  }

  .note-chip {
    display: flex;
    align-items: flex-start;
    gap: 0.3125rem;
    margin-top: 0.25rem;
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

  .notes-editor { margin-top: 0.375rem; }
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
  .field-error { margin: 0.25rem 0 0; color: var(--c-destructive); font-size: 0.75rem; }
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
  .kbd-hint { margin-left: auto; color: var(--c-label-quaternary); font-size: 0.6875rem; }

  .note-btn {
    flex-shrink: 0;
    display: grid;
    place-items: center;
    width: 1.375rem;
    height: 1.375rem;
    margin-top: 0.0625rem;
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

  /* completed */
  .is-completed .checkbox {
    --c-check-border: var(--c-accent);
    --c-check-bg: var(--c-accent);
    --c-check-icon: #fff;
  }
  .is-completed .checkbox:hover:not(:disabled) {
    --c-check-bg: color-mix(in srgb, var(--c-accent) 85%, #000);
  }
  .is-completed p {
    color: var(--c-label-tertiary);
    text-decoration: line-through;
    text-decoration-thickness: 1px;
    text-decoration-color: var(--c-label-quaternary);
  }

  /* missed */
  .is-missed .checkbox {
    --c-check-border: var(--c-destructive-muted);
    --c-check-icon: var(--c-destructive-muted);
  }
  .is-missed p { color: var(--c-label-secondary); }
</style>
