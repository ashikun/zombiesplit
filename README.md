# zombiesplit

Ashikun's attempt to make a cross-platform split timer, because they
don't have enough side projects already.

Licenced under MIT.

## Initialisation

_NOTE:_ zombiesplit currently looks in `.` for pretty much everything - config,
database, game files.  This might be fixed later, but for now the easiest way to
try zombiesplit is `cargo run`ning it out of a working copy.

zombiesplit uses a SQLite database to store game and (eventually) run data;
before using zombiesplit you'll need to run:

```
# or `zombiesplit init`
$ cargo run -- init
```

To teach zombiesplit about a game, use

```
$ cargo run -- add-game scd11.toml
```

where `scd11.toml` is a game specification file (conveniently, this is the
one pre-packed with zombiesplit as an example).  The game will be stored into
the database as the filename less its extension (so `scd11`.)

## Operation

Supposing we've added a game `scd11` with a category `btg-sonic`, run:

```
$ cargo run -- run scd11 btg-sonic
```

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


## Current features

- Edit manual IGT splits
- Multiple times per split (useful for tracking deaths/resets)
- Track total time across splits

## Planned features

Note that none of this is implemented yet.  Also see the GitHub issues page.

- Program games by easily shareable TOML configuration
  - Rudimentary access for editing, removing, and backing up game data
- Emphasis (initially at least) on manual IGT entry, maybe RTA later
- Save splits to a local SQLite file
  - Use saved splits for pacing
