/**
 * Derived item statuses.
 *
 * Date comparisons use local calendar-day semantics: both `itemDate` and today
 * are represented as `YYYY-MM-DD` strings, which sort/compare correctly with
 * plain string comparison — no timezone-related off-by-one errors.
 */

export type ItemStatus = "completed" | "missed" | "pending";

/**
 * Returns today's date as a `YYYY-MM-DD` string in the local timezone.
 * Exported so callers can pass it in for testability or to avoid repeated calls.
 */
export function localToday(): string {
  const d = new Date();
  return (
    `${d.getFullYear()}-` +
    `${String(d.getMonth() + 1).padStart(2, "0")}-` +
    `${String(d.getDate()).padStart(2, "0")}`
  );
}

/**
 * Derives the display status of an activity or task.
 *
 * @param completed - the backend `completed` boolean
 * @param dayDate   - the item's `day_date` as `YYYY-MM-DD`
 * @param today     - today's date as `YYYY-MM-DD` (defaults to `localToday()`)
 */
export function deriveStatus(
  completed: boolean,
  dayDate: string,
  today: string = localToday()
): ItemStatus {
  if (completed) return "completed";
  if (dayDate < today) return "missed";
  return "pending";
}
