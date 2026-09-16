<script lang="ts">
  interface Props { date: string; }
  let { date }: Props = $props();
  const dayFormatter = new Intl.DateTimeFormat(undefined, { weekday: "long" });
  const dateFormatter = new Intl.DateTimeFormat(undefined, { month: "long", day: "numeric" });
  function parseDate(value: string) { return new Date(`${value}T00:00:00`); }
</script>

<header class="day-header">
  <p class="weekday">{dayFormatter.format(parseDate(date))}</p>
  <h1>{dateFormatter.format(parseDate(date))}</h1>
</header>

<style>
  .day-header { margin-bottom: 2.5rem; }
  .weekday { margin: 0 0 0.2rem; color: #5f6f86; font-size: 0.875rem; font-weight: 650; letter-spacing: 0.02em; }
  h1 { margin: 0; color: #172033; font-size: clamp(2rem, 5vw, 2.65rem); font-weight: 650; letter-spacing: -0.04em; line-height: 1.05; }
</style>
