# zombiesplit architecture notes

**NOTE:** `zombiesplit`'s architecture has evolved and mutated over time, and is
*constantly in flux. Take these notes with a bit of salt.

Effectively, `zombiesplit` is a C# program trapped in a Rust program's body, for
which one can blame the programmer's love of overengineering.

The top-level split is as follows:

- the _model_;
- the _database code_, which sits on top of the model;
- the _netcode_ (client and server), which sits on top of the model and
  database;
- various _user interfaces_, which sit on top of the model, netcode, and 
  database.

## Model

The `zombiesplit` model is split into four main parts, roughly from low to high
level:

- the timing model, which tracks notions of time, comparisons, and aggregates;
- the game model, which tracks all games/categories/related data that a
  `zombiesplit` instance has been taught about; 
- the historical model, which tracks all saved run data; and
- the session model, which tracks the run currently being processed.

### Timing model

The timing model contains various building blocks for representing times,
comparisons between times, and aggregates (various computed summing operations
on times).

### Game model

This is a fairly thin representation of the underlying database relations.

### Historical model

Like the game model, the historical model is a fairly thin representation of the
underlying database relations.

### Session model

The session model is a 'fat' model (that is, most of the business logic of
`zombiesplit` forms methods on the model instead of being separated into higher
layers).

An in-flight attempt, along with other parts of session _state_, is contained in a _session_, which exposes protocols
for sending _actions_ and subscribing to _events_ (an observer pattern). These form the backbone of the client/server
protocol in the netcode.

## Database

SQLite.

The session can both pull data from the database (as both comparisons and initial game/category data) and push data to
the database (newly-saved historic runs).

## Netcode

`zombiesplit` has a client/server architecture, with multiple clients able to
connect simultaneously to one server.  This protocol is a `gRPC` encoding of
the action/event API, with the protocol buffers description at
`proto/zombiesplit.proto`.

This protocol is _not yet stable_; revisions of zombiesplit may change the
protobuf at will.  You have been warned.

### Client

Some of the user interfaces below are `zombiesplit` clients, but all use the
same netcode.

### Server

## User interfaces

### Command line porcelain

This is `zsdb`, at the moment.

### Graphical split editor

This is `zsclient`.

Roughly laid out as a model-view-presenter, with the model being that described
above. There is an added complication that the presenter is tracking the
modality of the user interface too (eg, normal mode/edit mode/etc).
