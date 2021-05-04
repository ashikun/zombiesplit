# zombiesplit

Ashikun's attempt to make a cross-platform split timer, because they
don't have enough side projects already.

Licenced under MIT.

## Intended Design

Note that none of this is implemented yet.

- Program games by TOML configuration, including current WRs
- Emphasis (initially at least) on manual IGT entry
  - Support for multiple times per split (for death tracking, etc)
  - Maybe RTA later on
- Minimalist vi-style user interface, something like:
  - j/k to focus on a split
  - h to scrub a split
  - l to add another time to a split
  - m, s, . to focus on minutes, seconds, fractional seconds
  - esc to reset
- Save splits to a local SQLite file
