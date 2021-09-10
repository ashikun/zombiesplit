//! Model integration tests.

use std::{cell::RefCell, rc::Rc};

use zombiesplit::model::{
    attempt::{observer, Observer, Run, Session},
    game::{
        category::{self, AttemptInfo, ShortDescriptor},
        Split,
    },
    short,
};

/// Tests that a session doesn't send any observations until prompted.
#[test]
fn test_session_initial() {
    let (_, obs) = make_session_and_obs();
    obs.assert_empty()
}

/// Makes a dummy session and an observer for it.
fn make_session_and_obs() -> (Session<'static>, Obs) {
    let mut s = make_session();
    let obs = Obs::default();
    s.observers.add(Box::new(obs.clone()));

    (s, obs)
}

fn make_session() -> Session<'static> {
    let game = short::Name::from("game");
    let category = short::Name::from("category");
    let metadata = category::Info {
        game: "Game Name".to_string(),
        category: "Game Category".to_string(),
        short: ShortDescriptor::new(game, category),
    };
    let splits = [
        Split::new(0, "s1", "Split 1"),
        Split::new(1, "s2", "Split 2"),
        Split::new(2, "s3", "Split 3"),
    ];
    let splits = std::array::IntoIter::new(splits).collect();
    let run = Run {
        attempt: AttemptInfo::default(),
        splits,
    };
    Session::new(metadata, run)
}

// TODO(@MattWindsor91): possibly unify with the presenter version.
#[derive(Clone, Default)]
pub struct Obs(Rc<RefCell<Vec<observer::Event>>>);

impl Obs {
    /// Asserts that the observer found a message that is equal to `evt`.
    pub fn assert_received(&self, evt: observer::Event) {
        let mut queue = self.0.borrow_mut();
        assert!(!queue.is_empty(), "expected an observation");
        assert_eq!(evt, queue.remove(0));
    }

    /// Asserts that the observer is empty.
    pub fn assert_empty(&self) {
        assert!(self.0.borrow().is_empty(), "expected no more observations")
    }
}

impl Observer for Obs {
    fn observe(&self, evt: observer::Event) {
        self.0.borrow_mut().push(evt);
    }
}
