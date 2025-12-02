# 🎄 Advent of Code — Rust Solutions (Rust, 2025)

Welcome to a festive collection of Advent of Code solutions written in Rust — currently focused on 2025 with room to grow into other years. This repo is a personal playground for puzzle-solving, performance tuning, and clean code for each day's challenge.

🚀 Highlights
- Modular day-by-day solutions for clarity and fast iteration.
- Simple input/samples loader to keep I/O consistent.
- Designed so adding a new day or year is frictionless.

Progress (2025)
- [x] Day 1
- [x] Day 2
- [ ] Day 3
- [ ] Day 4
- [ ] Day 5
- [ ] Day 6
- [ ] Day 7
- [ ] Day 8
- [ ] Day 9
- [ ] Day 10
- [ ] Day 11
- [ ] Day 12


Quickstart
- Build (requires Rust stable):

  cargo build --release

- Run the full runner:

  cargo run --release

- Run a specific day (if supported by main.rs):

  cargo run --release -- 1

Output format
- The runner prints each day's results in a compact form. Example:

  Day 1 — Part 1: <answer>
  Day 1 — Part 2: <answer>

Inputs and samples
- Real puzzle inputs: inputs/dayN.txt
- Sample/test inputs: inputs/samples/dayN.txt or inputs/samples/testN.txt
- Use file_loader.rs helpers to keep input parsing consistent across days.

Project layout (important files)
- Cargo.toml / Cargo.lock — Rust workspace configuration
- src/
  - main.rs — program entry and runner for days
  - file_loader.rs — file I/O helpers
  - day1.rs, day2.rs, ... — solution modules for each day
- inputs/
  - dayX.txt — real input for that day
  - samples/ — sample inputs used while developing or testing

Add a new day (fast checklist)
1. Create src/dayN.rs and implement the parsing + solve functions (follow existing modules for conventions).
2. Hook the module into main.rs so the runner can call it.
3. Add inputs/dayN.txt and optional inputs/samples/dayN.txt.

Add a new year
- Two options:
  - Keep everything in one crate and namespace modules (e.g., year2026_day1.rs), or
  - Convert to a workspace with a crate per year if you want stronger separation.

Tips & Tricks
- Keep parsing logic near the top of a day's module and separate Part 1/Part 2 functions for testability.
- Use the sample files for quick test-driven iteration; add unit tests that load inputs/samples automatically.
- If you want faster iteration while developing, run in debug mode (cargo run) and only switch to --release for final runs.

Contributing / Notes
- This is a personal repo, but contributions are welcome — open a PR with a short note explaining the change.
- Consider adding a LICENSE if you plan to make this public.

Roadmap / TODO
- Add badges and CI to run tests/solutions on push.
- Add example outputs for each day in a results/ folder.
- Expand to include previous/future years (2024, 2026, ...).

If you want more pizzazz (badges, CI workflow, example outputs, or ASCII art header), tell me which and I will add it.
