<script lang="ts">
  interface Props { date: string; isToday: boolean; }
  let { date, isToday }: Props = $props();

  const dayFormatter = new Intl.DateTimeFormat(undefined, { weekday: "long" });
  const dateFormatter = new Intl.DateTimeFormat(undefined, { month: "long", day: "numeric", year: "numeric" });
  function parseDate(value: string) { return new Date(`${value}T00:00:00`); }
</script>

<header class="day-header">
  <p class="weekday">
    {dayFormatter.format(parseDate(date))}
    {#if isToday}<span class="today-pill">Today</span>{/if}
  </p>
  <h1>{dateFormatter.format(parseDate(date))}</h1>
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
</style>
