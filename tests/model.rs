//! Model integration tests.

use std::sync::mpsc;

use zombiesplit::model::{
    attempt::{observer, Observer, Run, Session},
    game::{
        category::{self, AttemptInfo, ShortDescriptor},
        Split,
    },
    short,
};

#[test]
fn test_mini_session() {
    let game = short::Name::from("game");
    let category = short::Name::from("category");

    let metadata = category::Info {
        game: "Game Name".to_string(),
        category: "Game Category".to_string(),
        short: ShortDescriptor::new(game, category),
    };

    let splits = [Split {
        id: 0,
        short: short::Name::from("s1"),
        name: "Split 1".to_string(),
    }];
    let splits = std::array::IntoIter::new(splits).collect();

    let run = Run {
        attempt: AttemptInfo::default(),
        splits,
    };

    let mut s = Session::new(metadata, run);

    let (cs, _) = mpsc::channel();
    s.observers.add(Box::new(Obs { sender: cs }));

    // TODO(@MattWindsor91): actually test.
}

// TODO(@MattWindsor91): possibly unify with the presenter version.
pub struct Obs {
    sender: mpsc::Sender<observer::Event>,
}

impl Observer for Obs {
    fn observe(&self, evt: observer::Event) {
        // TODO(@MattWindsor91): handle errors properly?
        if let Err(e) = self.sender.send(evt) {
            log::warn!("error sending event to presenter: {}", e);
        }
    }
}
