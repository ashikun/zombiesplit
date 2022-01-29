# zombiesplit

zombiesplit is a WIP speedrun split timer.
The value proposition over, say, [livesplit](https://livesplit.org), is that
zombiesplit is cross-platform by design and focused on making manual IGT runs
easy to handle.  Other design decisions of note include:

- nerdy client-server architecture (one client implemented so far, plans to
  make a streamdeck plugin as another);
- modal split editor for those of us with way too much vi muscle memory;
- extensive use of SQLite as a backing store, because why not.

Of course, zombiesplit is new, is mostly a hobby project, and is never gonna
be as featureful as livesplit etc.  Features currently _missing_ include:

- RTA timing;
- run export/import;
- display customisability;
- user friendliness and onboarding;
- stability (there are lots of bugs!)

zombiesplit is licenced under MIT.


## Usage

### Copying over the assets

In the absence of a proper installation script, using `zombiesplit` will
require you to do some copying or symlinking of files into the places it
expects them.  The main directory you'll need to copy/symlink is the `assets`
directory, which contains the default font set.  This generally
needs to go into:

- Linux: `~/.local/share/zombiesplit`
- macOS: `~/Library/Application Support/xyz.ashikun/zombiesplit`
- Windows: `C:\Users\NAME\AppData\Roaming\Ashikun\Zombiesplit`

See [the `directories` documentation](https://docs.rs/directories/latest/directories/struct.ProjectDirs.html#method.data_dir)
for details.

### Configuration

The main way to configure zombiesplit is through two TOML files, `client.toml`
and `server.toml`, typically located in:

- Linux: `~/.config/zombiesplit`
- macOS: `~/Library/Application Support/xyz.ashikun.zombiesplit`
- Windows: `C:\Users\NAME\AppData\Roaming\Ashikun\Zombiesplit`

See [the `directories` documentation](https://docs.rs/directories/latest/directories/struct.ProjectDirs.html#method.config_dir)
for details.

These configuration files can be overridden by configuration in another
file by passing `--config PATH` to programs.

For examples of what these files look like, see `examples/config`.

### Initialising the database

zombiesplit uses a SQLite database to store game and (eventually) run data;
before using zombiesplit you'll need to use `zsdb` to set up the database:

```
$ zsdb init
```

To teach zombiesplit about a game, use

```
$ zsdb add-game scd11.toml
```

where `scd11.toml` is a game specification file (conveniently, this is the
one pre-packed with zombiesplit as an example).  The game will be stored into
the database as the filename less its extension (so `scd11`.)

The default location for the zombiesplit database, if not configured otherwise
in `server.toml`, is `zombiesplit.db` in the same directory as `assets`.

### Operation

Supposing we've added a game `scd11` with a category `btg-sonic`, run:

```
$ zsserver scd11/btg-sonic
```

This launches the zombiesplit server on the host/port configured in the main
configuration file.  Then, in another terminal, use

```
$ zsclient
```

to run the client.

`zsclient` has a semi-modal, vi-style user interface.  It has three modes:

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
