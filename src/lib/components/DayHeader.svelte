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
    {#if isToday}<span class="today-badge">Today</span>{/if}
  </p>
  <h1>{dateFormatter.format(parseDate(date))}</h1>
</header>

<style>
  .day-header {
    min-width: 0;
    text-align: center;
  }
  .weekday {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.4rem;
    margin: 0 0 0.2rem;
    color: #7a7f78;
    font-size: 0.8rem;
    font-weight: 600;
    letter-spacing: 0.03em;
    text-transform: capitalize;
  }
  .today-badge {
    padding: 0.075rem 0.4rem;
    border: 1px solid #cdd8c9;
    border-radius: 999px;
    color: #547054;
    font-size: 0.675rem;
    font-weight: 700;
    letter-spacing: 0.04em;
    text-transform: uppercase;
  }
  h1 {
    margin: 0;
    color: #222621;
    font-size: clamp(1.6rem, 4vw, 2.1rem);
    font-weight: 650;
    letter-spacing: -0.04em;
    line-height: 1.1;
  }
</style>
