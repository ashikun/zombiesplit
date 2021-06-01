# zombiesplit

Ashikun's attempt to make a cross-platform split timer, because they
don't have enough side projects already.

Licenced under MIT.

## Operation

_Note:_ a lot of the setup is hardcoded atm, so this is likely not useful
for anything other than timing Sonic CD BTG.

zombiesplit has a semi-modal, vi-style user interface.  It has three modes:

- inactive (run not underway);
- normal;
- time editor (which can itself be focusing on a field).

Its main keybindings are:

- `RET`: start or reset run
- `j/k`: move cursor (committing any edit in progress)
- `h`: discard (if editing, drop field; otherwise, pop a split time for editing)
- `l`: commit an edit in progress
- `x`: delete (if editing, drop edit; otherwise, drop all times for split)
- `m/s/.`: edit minutes/seconds/milliseconds field (milliseconds are
  right-padded by 0, eg `5` = `500`).


## Current Features

- Edit manual IGT splits
- Multiple times per split (useful for tracking deaths/resets)
- Track total time across splits

## Intended Design

Note that none of this is implemented yet.

- Program games by TOML configuration
- Emphasis (initially at least) on manual IGT entry, maybe RTA later
- Save splits to a local SQLite file
  - Use saved splits for pacing
