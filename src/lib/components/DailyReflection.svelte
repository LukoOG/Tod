<script lang="ts">
  import { updateDayReflection } from "$lib/api/day";
  import type { Day } from "$lib/types";

  interface Props {
    day: Day;
    isToday: boolean;
    onDayUpdated: (day: Day) => void;
    saveRef?: { fn: (() => Promise<void>) | null };
  }

  let { day, isToday, onDayUpdated, saveRef }: Props = $props();

  let draftReflection = $state("");
  let draftMood = $state<number | null>(null);
  let isSaving = $state(false);
  let saveStatus = $state<"idle" | "saving" | "saved" | "error">("idle");
  let errorMsg = $state<string | null>(null);
  let syncedDate = $state("");

  let isDirty = $derived(
    draftReflection.trim() !== (day.reflection ?? "") ||
    draftMood !== (day.mood ?? null)
  );

  // Reset draft only when navigating to a different day, not after each autosave
  $effect(() => {
    if (day.date === syncedDate) return;
    syncedDate = day.date;
    draftReflection = day.reflection ?? "";
    draftMood = day.mood ?? null;
    errorMsg = null;
    saveStatus = (day.reflection || day.mood !== null) ? "saved" : "idle";
  });

  // Expose save so parent can flush before navigation
  $effect(() => {
    if (saveRef) saveRef.fn = save;
    return () => { if (saveRef) saveRef.fn = null; };
  });

  // Autosave: fires 500ms after the user stops editing
  $effect(() => {
    void draftReflection;
    void draftMood;
    if (!isDirty) return;
    const timer = setTimeout(save, 500);
    return () => clearTimeout(timer);
  });

  async function save() {
    if (!isDirty || isSaving) return;
    isSaving = true;
    saveStatus = "saving";
    errorMsg = null;
    const trimmed = draftReflection.trim();
    const reflection = trimmed === "" ? null : trimmed;
    try {
      const updated = await updateDayReflection(day.date, reflection, draftMood);
      onDayUpdated(updated);
      saveStatus = "saved";
    } catch (e) {
      saveStatus = "error";
      errorMsg = e instanceof Error ? e.message : "Failed to save.";
    } finally {
      isSaving = false;
    }
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Enter" && (e.metaKey || e.ctrlKey)) { e.preventDefault(); save(); }
  }

  function autosize(node: HTMLTextAreaElement) {
    function resize() {
      node.style.height = "auto";
      node.style.height = `${node.scrollHeight}px`;
    }
    node.addEventListener("input", resize);
    resize();
    return { destroy() { node.removeEventListener("input", resize); } };
  }

  const MOODS = [
    { value: 1, label: "Rough", emoji: "😞" },
    { value: 2, label: "Meh",   emoji: "😐" },
    { value: 3, label: "Okay",  emoji: "🙂" },
    { value: 4, label: "Good",  emoji: "😊" },
    { value: 5, label: "Great", emoji: "😄" },
  ] as const;
</script>

<section class="reflection" aria-labelledby="reflection-heading">
  <div class="section-header">
    <h2 id="reflection-heading" class="section-label">Reflection</h2>
    {#if saveStatus === "saving"}
      <span class="save-status" aria-live="polite">Saving…</span>
    {:else if saveStatus === "saved" && !isDirty}
      <span class="save-status saved" aria-live="polite">Saved</span>
    {:else if saveStatus === "error"}
      <span class="save-status error-badge" aria-live="polite">Error saving</span>
    {/if}
  </div>

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
    placeholder={isToday ? "How did today go?" : "Reflection…"}
    bind:value={draftReflection}
    disabled={isSaving}
    onkeydown={handleKeydown}
    onblur={() => { if (isDirty) save(); }}
    use:autosize
  ></textarea>

  {#if errorMsg}
    <p class="save-error" role="alert">
      {errorMsg}
      <button type="button" class="retry-btn" onclick={save}>Retry</button>
    </p>
  {/if}

  {#if isDirty && !isSaving}
    <p class="kbd-hint">⌘↵ saves immediately</p>
  {/if}
</section>

<style>
  .reflection {
    display: flex;
    flex-direction: column;
    gap: 0.625rem;
  }

  .section-header {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .section-label {
    margin: 0;
    color: var(--c-label-tertiary);
    font-size: 0.625rem;
    font-weight: 700;
    letter-spacing: 0.1em;
    text-transform: uppercase;
  }

  .save-status {
    padding: 0.0625rem 0.375rem;
    border-radius: 999px;
    color: var(--c-label-quaternary);
    font-size: 0.625rem;
    font-weight: 500;
    letter-spacing: 0.04em;
  }
  .save-status.saved {
    color: var(--c-accent);
    background: color-mix(in srgb, var(--c-accent) 12%, transparent);
  }
  .save-status.error-badge {
    color: var(--c-destructive);
    background: color-mix(in srgb, var(--c-destructive) 10%, transparent);
  }

  /* mood row */
  .mood-row {
    display: flex;
    gap: 0.3125rem;
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

  /* textarea — autosizes via JS; no fixed rows */
  textarea {
    width: 100%;
    min-height: 7rem;
    padding: 0.5rem 0.625rem;
    border: 1px solid var(--c-separator-opaque);
    border-radius: var(--radius-sm);
    background: var(--c-fill-tertiary);
    color: var(--c-label-primary);
    font: inherit;
    font-size: 0.875rem;
    line-height: 1.65;
    resize: none;
    overflow: hidden;
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
    display: flex;
    align-items: center;
    gap: 0.5rem;
    margin: 0;
    padding: 0.3125rem 0.5rem;
    border-radius: var(--radius-sm);
    background: color-mix(in srgb, var(--c-destructive) 8%, transparent);
    color: var(--c-destructive);
    font-size: 0.8125rem;
  }
  .retry-btn {
    padding: 0;
    border: none;
    background: none;
    color: var(--c-destructive);
    font: inherit;
    font-size: 0.8125rem;
    font-weight: 600;
    text-decoration: underline;
    cursor: pointer;
  }

  .kbd-hint {
    margin: 0;
    color: var(--c-label-quaternary);
    font-size: 0.625rem;
    text-align: right;
  }
</style>
