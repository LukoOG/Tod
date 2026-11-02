<script lang="ts">
  interface Props {
    date: string;
    isToday: boolean;
    onDatePicked?: (date: string) => void;
  }
  let { date, isToday, onDatePicked }: Props = $props();

  const dayFormatter = new Intl.DateTimeFormat(undefined, { weekday: "long" });
  const dateFormatter = new Intl.DateTimeFormat(undefined, { month: "long", day: "numeric", year: "numeric" });
  function parseDate(value: string) { return new Date(`${value}T00:00:00`); }
</script>

<header class="day-header">
  <p class="weekday">
    {dayFormatter.format(parseDate(date))}
    {#if isToday}<span class="today-pill">Today</span>{/if}
  </p>

  {#if onDatePicked}
    <!-- Overlay pattern: transparent date input sits over the h1 -->
    <div class="date-jump" title="Click to jump to a date">
      <h1>{dateFormatter.format(parseDate(date))}</h1>
      <input
        type="date"
        value={date}
        class="date-input"
        aria-label="Jump to date"
        onchange={(e) => { if (e.currentTarget.value) onDatePicked?.(e.currentTarget.value); }}
      />
    </div>
  {:else}
    <h1>{dateFormatter.format(parseDate(date))}</h1>
  {/if}
</header>

<style>
  .day-header {
    flex: 1;
    min-width: 0;
    text-align: center;
  }
  .weekday {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.375rem;
    margin: 0 0 0.125rem;
    color: var(--c-label-secondary);
    font-size: 0.6875rem;
    font-weight: 600;
    letter-spacing: 0.06em;
    text-transform: uppercase;
  }
  .today-pill {
    padding: 0.0625rem 0.375rem;
    border-radius: 999px;
    background: var(--c-accent);
    color: #fff;
    font-size: 0.5625rem;
    font-weight: 700;
    letter-spacing: 0.06em;
    text-transform: uppercase;
  }
  h1 {
    margin: 0;
    color: var(--c-label-primary);
    font-size: 1.125rem;
    font-weight: 600;
    letter-spacing: -0.02em;
    line-height: 1.2;
  }

  /* Date jump: transparent input overlaid on the h1 */
  .date-jump {
    position: relative;
    display: inline-block;
  }
  .date-jump h1 {
    transition: color 80ms ease;
  }
  .date-jump:hover h1 {
    color: var(--c-accent);
  }
  .date-input {
    position: absolute;
    inset: 0;
    width: 100%;
    height: 100%;
    opacity: 0;
    cursor: pointer;
    border: none;
    padding: 0;
    -webkit-app-region: no-drag;
  }
</style>
