# Rustaire

Terminal Klondike solitaire built in Rust with ratatui/crossterm.

## Architecture

The game is split into four modules with clear separation of concerns:

### `src/card.rs` — Data types
Defines `Card`, `Suit`, and `Rank`. Cards know their suit, rank, and whether they're face-up. Contains the stacking logic (`can_stack_on_tableau`, `can_stack_on_foundation`) so game rules live close to the data.

### `src/game.rs` — Game state & logic
The `Game` struct owns all piles (stock, waste, 4 foundations, 7 tableau columns) and the move history stack. Every mutation (draw, move between piles) is a method on `Game` that:
1. Validates the move
2. Mutates state
3. Pushes a `Move` record onto `history` for undo

The `Move` struct captures enough info (source, destination, cards moved, whether a card was revealed) to fully reverse any action. Hints scan all possible moves in priority order.

### `src/theme.rs` — Color schemes
`ThemeId` enum lists available themes; each maps to a `Theme` struct containing ~24 named colors. All rendering code references theme colors — no hardcoded colors in the UI module.

### `src/ui.rs` — Rendering & app state
`AppState` holds the `Game`, cursor position, selection, current theme, and transient UI state (hints, messages). The `render()` function draws the full frame each tick using ratatui widgets. Layout is vertical (title bar → top row → tableau → status bar) with horizontal sub-layouts for columns.

### `src/main.rs` — Event loop & input
Opens `/dev/tty` directly (works even when stdout is captured). Runs the ratatui event loop: draw frame → poll for input → dispatch. Handles keyboard (arrows, space, enter, hotkeys, number keys) and mouse (click, double-click for auto-move, right-click to deselect).

## Key design decisions

- **Undo via move history**: every action pushes a reversible record, no state snapshots needed.
- **Direct `/dev/tty`**: avoids "Device not configured" errors when launched from task runners that capture stdout.
- **Theme as a data struct**: adding a theme means adding one enum variant and one constructor — no rendering code changes.
- **Auto-move priority**: Enter tries foundation first, then tableau-to-tableau, matching how most players think.

## Build & run

```sh
mise run          # default task: cargo run --release (needs direct TTY)
mise run build    # just compile
```

Or directly: `cargo run --release` / `./target/release/rustaire`

## Controls

| Key | Action |
|-----|--------|
| Arrows | Move cursor |
| Space | Select / place card |
| Enter | Auto-move (foundation first, then tableau) |
| 1-7 | Select/place on tableau column |
| 0 | Select/place on waste |
| D | Draw from stock |
| H | Hint |
| U | Undo |
| A | Auto-complete (when all face-up) |
| T | Cycle theme |
| N | New game |
| Q / Esc | Quit |
| Left click | Select/place |
| Double-click | Auto-move |
| Right click | Clear selection |
