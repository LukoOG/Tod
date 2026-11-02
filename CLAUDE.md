# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this is

**Tod** is a macOS daily life planner — a Tauri v2 desktop app with a SvelteKit + TypeScript frontend and a Rust backend. The app lets users view a day-by-day schedule (activities generated from recurring routines), ad-hoc tasks, and a daily reflection (mood + free text). All data is stored in a local SQLite database via `rusqlite`.

## Commands

### Development

```sh
# Run the full Tauri app (starts Vite dev server + Rust backend)
pnpm tauri dev

# Type-check the Svelte/TS frontend
pnpm check

# Build the frontend only
pnpm build
```

### Rust backend

```sh
# Run all Rust tests
cd src-tauri && cargo test

# Run a single test by name
cd src-tauri && cargo test <test_name>

# Run all tests in a specific module
cd src-tauri && cargo test tests::day

# Type-check the Rust crate without building
cd src-tauri && cargo check
```

### Tauri build (production)

```sh
pnpm tauri build
```

## Architecture

### Layer overview

```
Frontend (SvelteKit/Svelte 5)
  └─ src/lib/api/*.ts          # thin wrappers around tauri invoke()
  └─ src/lib/components/       # Svelte components
  └─ src/routes/+page.svelte   # single page, day-view state machine

Tauri bridge
  └─ src-tauri/src/commands/   # #[tauri::command] handlers (entry points)

Rust backend
  └─ src-tauri/src/domain/     # business logic (e.g. generate_day)
  └─ src-tauri/src/db/         # Database struct, SQL queries, migrations
  └─ src-tauri/src/models/     # plain Rust structs for DB rows
  └─ src-tauri/src/views/      # response shapes serialised to the frontend
  └─ src-tauri/src/error.rs    # AppError enum
```

### Data flow

1. The frontend calls `invoke("get_day", { date })` (or another command).
2. The matching `#[tauri::command]` in `commands/` receives the call with a `tauri::State<'_, Database>` handle.
3. Commands delegate to `domain/` for business logic, which calls `db/` for persistence.
4. Results are returned as `views/` types (serialised to JSON by serde), which the frontend receives as typed TS interfaces defined in `src/lib/types/`.

### Key domain concept: `generate_day`

`domain/day.rs::generate_day` is the core read operation. On every `get_day` call it:
- Creates the `days` row if it doesn't exist yet.
- Looks up all active `routines` matching the requested weekday.
- Inserts a corresponding `activities` row for each routine using `INSERT OR IGNORE` (idempotent; existing completion state is never overwritten).
- Returns a `DayView` containing the day record, its activities, and its tasks.

### Database

- SQLite via `rusqlite` with `features = ["bundled"]` (no system SQLite dependency).
- `Database` is a `Mutex<Connection>` managed as Tauri app state.
- Migrations are embedded Rust code in `db/migrations.rs`; schema is also visible in `migrations/001_initial.sql` and `migrations/002_seed_routines.sql`.
- Dates are stored as `TEXT` in `YYYY-MM-DD` format; timestamps as ISO-8601 UTC strings.
- `days_of_week` on routines is stored as a JSON array of integers (`1` = Monday … `7` = Sunday), serialised/deserialised by the model layer.

### Frontend state

The single page (`+page.svelte`) owns `dayView` as Svelte 5 `$state`. Mutations from child components are fed back via callback props (`onActivityCompleted`, `onTaskToggled`, etc.) rather than shared stores, so every update flows through the top-level state.

### Testing

Tests live in `src-tauri/src/tests/`. `test_utils.rs` provides `setup_test_db()` (in-memory SQLite with migrations applied and FK enforcement on) and fixture helpers. Tests call domain/db functions directly — no mocking.
