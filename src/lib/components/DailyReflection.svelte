<script lang="ts">
  import { updateDayReflection } from "$lib/api/day";
  import type { Day } from "$lib/types";

  function focus(node: HTMLElement) { node.focus(); }

  interface Props {
    day: Day;
    onDayUpdated: (day: Day) => void;
  }

  let { day, onDayUpdated }: Props = $props();

  let draftReflection = $state("");
  let draftMood = $state<number | null>(null);
  let isSaving = $state(false);
  let saveSuccess = $state(false);
  let error = $state<string | null>(null);
  let isExpanded = $state(false);

  let isDirty = $derived(
    draftReflection.trim() !== (day.reflection ?? "") ||
    draftMood !== (day.mood ?? null)
  );

  // Sync local state with the prop on mount and when the day changes
  $effect(() => {
    draftReflection = day.reflection ?? "";
    draftMood = day.mood ?? null;
    error = null;
    saveSuccess = false;
    isExpanded = !!(day.reflection || day.mood !== null);
  });

  async function save() {
    if (isSaving) return;
    isSaving = true;
    error = null;
    saveSuccess = false;
    const reflection = draftReflection.trim() === "" ? null : draftReflection.trim();
    try {
      const updated = await updateDayReflection(day.date, reflection, draftMood);
      onDayUpdated(updated);
      saveSuccess = true;
      setTimeout(() => { saveSuccess = false; }, 2000);
    } catch (e) {
      error = e instanceof Error ? e.message : "Failed to save reflection.";
    } finally {
      isSaving = false;
    }
  }

  function cancel() {
    draftReflection = day.reflection ?? "";
    draftMood = day.mood ?? null;
    error = null;
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Enter" && (e.metaKey || e.ctrlKey)) { e.preventDefault(); save(); }
    if (e.key === "Escape" && isDirty) { cancel(); }
  }

  function expand() {
    isExpanded = true;
  }

  const MOODS = [
    { value: 1, label: "Rough", emoji: "😞" },
    { value: 2, label: "Meh",   emoji: "😐" },
    { value: 3, label: "Okay",  emoji: "🙂" },
    { value: 4, label: "Good",  emoji: "😊" },
    { value: 5, label: "Great", emoji: "😄" },
  ] as const;

  const selectedMoodEmoji = $derived(MOODS.find(m => m.value === draftMood)?.emoji ?? null);
</script>

<section class="reflection" aria-labelledby="reflection-heading">
  <div class="section-header">
    <h2 id="reflection-heading">Reflection</h2>
    {#if saveSuccess}
      <span class="save-badge" aria-live="polite">Saved</span>
    {/if}
  </div>

  {#if !isExpanded}
    <button type="button" class="expand-btn" onclick={expand}>
      <span class="expand-placeholder">How did today go?</span>
      <span class="expand-hint">Click to write a reflection…</span>
    </button>
  {:else}
    <div class="mood-row" aria-label="Mood rating">
      {#each MOODS as m}
        <button
          type="button"
          class="mood-btn"
          class:selected={draftMood === m.value}
          aria-pressed={draftMood === m.value}
          aria-label={`Mood: ${m.label}`}
          onclick={() => { draftMood = draftMood === m.value ? null : m.value; }}
        >
          <span class="mood-emoji">{m.emoji}</span>
          <span class="mood-label">{m.label}</span>
        </button>
      {/each}
    </div>

    <textarea
      id="reflection-textarea"
      aria-label="Daily reflection"
      placeholder="How did today go?"
      rows={4}
      bind:value={draftReflection}
      disabled={isSaving}
      onkeydown={handleKeydown}
      use:focus
    ></textarea>

    {#if error}
      <p class="save-error" role="alert">{error}</p>
    {/if}

    {#if isDirty}
      <div class="actions">
        <button
          type="button"
          class="btn-save"
          onclick={save}
          disabled={isSaving}
        >
          {isSaving ? "Saving…" : "Save reflection"}
        </button>
        <button
          type="button"
          class="btn-cancel"
          onclick={cancel}
          disabled={isSaving}
        >
          Cancel
        </button>
        <span class="kbd-hint">⌘↵ to save</span>
      </div>
    {:else if !isDirty && (day.reflection !== null || day.mood !== null)}
      <div class="actions">
        <button type="button" class="btn-save" onclick={save} disabled={isSaving || !isDirty}>
          {isSaving ? "Saving…" : "Saved"}
        </button>
      </div>
    {/if}
  {/if}
</section>

<style>
  .reflection {
    padding-top: 1.25rem;
    border-top: 1px solid var(--c-separator);
  }
  .section-header {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    margin-bottom: 0.75rem;
  }
  h2 {
    margin: 0;
    color: var(--c-label-tertiary);
    font-size: 0.625rem;
    font-weight: 700;
    letter-spacing: 0.1em;
    text-transform: uppercase;
  }
  .save-badge {
    padding: 0.0625rem 0.375rem;
    border-radius: 999px;
    background: color-mix(in srgb, var(--c-accent) 15%, transparent);
    color: var(--c-accent);
    font-size: 0.625rem;
    font-weight: 600;
    letter-spacing: 0.04em;
    text-transform: uppercase;
  }

  /* expand prompt */
  .expand-btn {
    display: flex;
    flex-direction: column;
    gap: 0.125rem;
    width: 100%;
    padding: 0.625rem 0.75rem;
    border: 1px dashed var(--c-separator-opaque);
    border-radius: var(--radius);
    background: none;
    text-align: left;
    cursor: pointer;
    transition: border-color 80ms ease, background 80ms ease;
  }
  .expand-btn:hover {
    border-color: var(--c-label-quaternary);
    background: var(--c-fill-secondary);
  }
  .expand-btn:focus-visible { outline: 2px solid var(--c-accent); outline-offset: 2px; }
  .expand-placeholder {
    color: var(--c-label-secondary);
    font-size: 0.875rem;
  }
  .expand-hint {
    color: var(--c-label-quaternary);
    font-size: 0.6875rem;
  }

  /* mood row */
  .mood-row {
    display: flex;
    gap: 0.3125rem;
    margin-bottom: 0.625rem;
  }
  .mood-btn {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.125rem;
    padding: 0.375rem 0.25rem;
    border: 1px solid var(--c-separator-opaque);
    border-radius: var(--radius-sm);
    background: none;
    cursor: pointer;
    transition: border-color 80ms ease, background 80ms ease;
  }
  .mood-btn:hover:not(.selected) {
    border-color: var(--c-label-quaternary);
    background: var(--c-fill-secondary);
  }
  .mood-btn:focus-visible { outline: 2px solid var(--c-accent); outline-offset: 1px; }
  .mood-btn.selected {
    border-color: var(--c-accent);
    background: color-mix(in srgb, var(--c-accent) 10%, transparent);
  }
  .mood-emoji { font-size: 1rem; line-height: 1; }
  .mood-label {
    color: var(--c-label-tertiary);
    font-size: 0.5625rem;
    font-weight: 500;
    letter-spacing: 0.03em;
    text-transform: uppercase;
  }
  .mood-btn.selected .mood-label { color: var(--c-accent); }

  /* textarea */
  textarea {
    width: 100%;
    padding: 0.5rem 0.625rem;
    border: 1px solid var(--c-separator-opaque);
    border-radius: var(--radius-sm);
    background: var(--c-fill-tertiary);
    color: var(--c-label-primary);
    font: inherit;
    font-size: 0.875rem;
    line-height: 1.6;
    resize: vertical;
    transition: border-color 100ms ease, box-shadow 100ms ease;
    box-sizing: border-box;
  }
  textarea::placeholder { color: var(--c-label-quaternary); }
  textarea:hover:not(:disabled) { border-color: var(--c-label-quaternary); }
  textarea:focus {
    outline: none;
    border-color: var(--c-accent);
    box-shadow: 0 0 0 2.5px var(--c-accent-muted);
  }
  textarea:disabled { opacity: 0.55; }

  /* error */
  .save-error {
    margin: 0.375rem 0 0;
    padding: 0.3125rem 0.5rem;
    border-radius: var(--radius-sm);
    background: color-mix(in srgb, var(--c-destructive) 8%, transparent);
    color: var(--c-destructive);
    font-size: 0.8125rem;
  }

  /* action row */
  .actions {
    display: flex;
    align-items: center;
    gap: 0.625rem;
    margin-top: 0.5rem;
  }
  .btn-save {
    padding: 0.25rem 0.75rem;
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
  .btn-save:hover:not(:disabled) { opacity: 0.85; }
  .btn-save:disabled { opacity: 0.5; cursor: default; }
  .btn-cancel {
    padding: 0.25rem 0.5rem;
    border: 1px solid var(--c-separator-opaque);
    border-radius: var(--radius-sm);
    background: none;
    color: var(--c-label-secondary);
    font: inherit;
    font-size: 0.8125rem;
    cursor: pointer;
    transition: background 80ms ease;
  }
  .btn-cancel:hover:not(:disabled) { background: var(--c-fill-secondary); }
  .btn-cancel:disabled { opacity: 0.5; cursor: wait; }
  .kbd-hint {
    margin-left: auto;
    color: var(--c-label-quaternary);
    font-size: 0.6875rem;
  }
</style>
