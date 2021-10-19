# zombiesplit architecture notes

**NOTE:** `zombiesplit`'s architecture has evolved and mutated over time, and is constantly in flux. Take these notes
with a bit of salt.

Effectively, `zombiesplit` is a C# program trapped in a Rust program's body, for which one can blame the programmer's
love of overengineering.

The top-level split is as follows:

- the _model_;
- the _database code_, which sits on top of the model;
- various _user interfaces_, which sit on top of the model and database.

## Model

The `zombiesplit` model is split into three main parts:

- the config model, which tracks all games/categories/related data that a `zombiesplit` instance
  has been taught about; 
- the historical model, which tracks all saved run data; and
- the attempt model, which tracks the run currently being processed.

### Config model

The config model is fairly thin, mapping onto both the database and the 

### Historical model

Like the config model, the historical model is a fairly thin representation of the underlying database relations.

### Attempt model

The attempt model is a 'fat' model (that is, most of the business logic of `zombiesplit` forms methods on the model
instead of being separated into higher layers).

## Database

SQLite.

Any feed-forward from the attempt model to the database is implemented as a special kind of attempt observer.

## User interfaces

### Command line porcelain

### Graphical split editor

Roughly laid out as a model-view-presenter, with the model being that described above. There is an added
complication that the presenter is tracking the modality of the user interface too (eg, normal mode/edit mode/etc).