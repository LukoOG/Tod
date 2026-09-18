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
    {#if isToday}<span class="today-label">Today</span>{/if}
  </p>
  <h1>{dateFormatter.format(parseDate(date))}</h1>
</header>

<style>
  .day-header { min-width: 0; text-align: center; }
  .weekday { display: flex; align-items: center; justify-content: center; gap: 0.45rem; margin: 0 0 0.25rem; color: #70736d; font-size: 0.8125rem; font-weight: 650; letter-spacing: 0.035em; }
  .today-label { padding: 0.1rem 0.4rem; border: 1px solid #d8ded4; border-radius: 999px; color: #5f765f; font-size: 0.6875rem; font-weight: 700; letter-spacing: 0.04em; text-transform: uppercase; }
  h1 { margin: 0; color: #272a26; font-size: clamp(1.65rem, 4vw, 2.15rem); font-weight: 650; letter-spacing: -0.045em; line-height: 1.1; }
</style>
