<script lang="ts">
  import { updateDayReflection } from "$lib/api/day";
  import type { Day } from "$lib/types";

  interface Props {
    day: Day;
    onDayUpdated: (day: Day) => void;
  }

  let { day, onDayUpdated }: Props = $props();

  // Local editor state — separate from the prop so cancel always restores cleanly.
  // Initialised via $effect below so Svelte 5 knows these track the prop reactively.
  let draftReflection = $state("");
  let draftMood = $state<number | null>(null);
  let isSaving = $state(false);
  let error = $state<string | null>(null);
  let isDirty = $derived(
    draftReflection.trim() !== (day.reflection ?? "") ||
    draftMood !== (day.mood ?? null)
  );

  // Sync local state with the prop on mount and whenever the parent swaps in a
  // different day (date navigation).
  $effect(() => {
    draftReflection = day.reflection ?? "";
    draftMood = day.mood ?? null;
    error = null;
  });

  async function save() {
    if (isSaving) return;
    isSaving = true;
    error = null;
    const reflection = draftReflection.trim() === "" ? null : draftReflection.trim();
    try {
      const updated = await updateDayReflection(day.date, reflection, draftMood);
      onDayUpdated(updated);
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

  const MOODS = [
    { value: 1, label: "Rough" },
    { value: 2, label: "Meh" },
    { value: 3, label: "Okay" },
    { value: 4, label: "Good" },
    { value: 5, label: "Great" },
  ] as const;
</script>

<section class="reflection" aria-labelledby="reflection-heading">
  <h2 id="reflection-heading">Reflection</h2>

  <div class="mood-row">
    {#each MOODS as m}
      <button
        type="button"
        class="mood-btn"
        class:selected={draftMood === m.value}
        aria-pressed={draftMood === m.value}
        aria-label={`Mood: ${m.label}`}
        onclick={() => { draftMood = draftMood === m.value ? null : m.value; }}
      >
        {m.value}
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
  ></textarea>

  {#if error}
    <p class="save-error" role="alert">{error}</p>
  {/if}

  {#if isDirty || day.reflection !== null || day.mood !== null}
    <div class="actions">
      <button
        type="button"
        class="btn-save"
        onclick={save}
        disabled={isSaving || !isDirty}
      >
        {isSaving ? "Saving…" : "Save"}
      </button>
      {#if isDirty}
        <button
          type="button"
          class="btn-cancel"
          onclick={cancel}
          disabled={isSaving}
        >
          Cancel
        </button>
      {/if}
    </div>
  {/if}
</section>

<style>
  .reflection {
    margin-top: 2.75rem;
  }
  h2 {
    margin: 0 0 0.75rem;
    color: #7a7f78;
    font-size: 0.7rem;
    font-weight: 700;
    letter-spacing: 0.09em;
    text-transform: uppercase;
  }

  /* ── mood row ────────────────────────────────────── */
  .mood-row {
    display: flex;
    gap: 0.35rem;
    margin-bottom: 0.75rem;
  }
  .mood-btn {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.15rem;
    flex: 1;
    padding: 0.35rem 0.2rem;
    border: 1.5px solid #dde3da;
    border-radius: 0.5rem;
    background: transparent;
    color: #7a7f78;
    font: inherit;
    font-size: 0.875rem;
    font-weight: 600;
    cursor: pointer;
    transition: border-color 100ms ease, background-color 100ms ease, color 100ms ease;
  }
  .mood-label {
    font-size: 0.6rem;
    font-weight: 500;
    letter-spacing: 0.02em;
    text-transform: uppercase;
    color: #a0a59e;
  }
  .mood-btn:hover:not(.selected) {
    border-color: #b5c8b1;
    background: #f4f7f2;
  }
  .mood-btn:focus-visible { outline: 2px solid #b8d0b5; outline-offset: 2px; }
  .mood-btn.selected {
    border-color: #6a826a;
    background: #eef4ec;
    color: #3e5641;
  }
  .mood-btn.selected .mood-label { color: #6a826a; }

  /* ── textarea ────────────────────────────────────── */
  textarea {
    width: 100%;
    padding: 0.6rem 0.7rem;
    border: 1px solid #d0d5cd;
    border-radius: 0.5rem;
    background: transparent;
    color: #252824;
    font: inherit;
    font-size: 0.9375rem;
    line-height: 1.55;
    resize: vertical;
    transition: border-color 100ms ease;
    box-sizing: border-box;
  }
  textarea::placeholder { color: #9a9e97; }
  textarea:hover:not(:disabled) { border-color: #aab5a7; }
  textarea:focus {
    outline: none;
    border-color: #6f8a6f;
    box-shadow: 0 0 0 2px #b8d0b540;
  }
  textarea:disabled { opacity: 0.6; }

  /* ── save error ──────────────────────────────────── */
  .save-error {
    margin: 0.4rem 0 0;
    padding-left: 0.6rem;
    border-left: 2px solid #c89982;
    color: #7a5040;
    font-size: 0.875rem;
  }

  /* ── action row ──────────────────────────────────── */
  .actions {
    display: flex;
    gap: 0.75rem;
    align-items: center;
    margin-top: 0.6rem;
  }
  .btn-save,
  .btn-cancel {
    padding: 0.2rem 0;
    border: 0;
    border-bottom: 1px solid currentColor;
    border-radius: 0;
    background: transparent;
    font: inherit;
    font-size: 0.875rem;
    cursor: pointer;
    transition: opacity 100ms ease;
  }
  .btn-save { color: #5f765f; font-weight: 650; }
  .btn-cancel { color: #909690; }
  .btn-save:hover:not(:disabled),
  .btn-cancel:hover:not(:disabled) { opacity: 0.7; }
  .btn-save:disabled { cursor: default; opacity: 0.4; }
  .btn-cancel:disabled { cursor: wait; opacity: 0.45; }
</style>
