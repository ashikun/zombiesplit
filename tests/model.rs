//! Model integration tests.

use std::{cell::RefCell, rc::Rc};

use zombiesplit::model::{
    attempt::{
        observer::{self, Event},
        Observer, Run, Session,
    },
    game::{category, Split},
};

/// Tests that a session doesn't send any observations until prompted.
#[test]
fn test_session_initial() {
    let obs = Rc::new(Obs::default());
    let _ = make_session_with_obs(obs.clone());
    obs.assert_empty()
}

/// Tests that a session sends the appropriate initial observations when asked
/// to dump.
#[test]
fn test_session_dump() {
    let obs = Rc::new(Obs::default());
    let s = make_session_with_obs(obs.clone());

    s.dump_to_observers();

    obs.assert_received(Event::GameCategory(game_category()));
    for split in splits() {
        obs.assert_received(Event::AddSplit(split.short, split.name));
    }
    obs.assert_received(Event::Attempt(attempt_info()));
    obs.assert_empty()
}

/// Makes a dummy session and an observer for it.
fn make_session_with_obs(obs: Rc<dyn Observer>) -> Session<'static> {
    let mut s = make_session();
    s.observers.add(Rc::downgrade(&obs));
    s
}

fn attempt_info() -> category::AttemptInfo {
    category::AttemptInfo {
        total: 42,
        completed: 2,
    }
}

fn game_category() -> category::Info {
    category::Info {
        game: "Game Name".to_string(),
        category: "Game Category".to_string(),
        short: category::ShortDescriptor::new("game", "category"),
    }
}

fn splits() -> [Split; 3] {
    [
        Split::new(0, "s1", "Split 1"),
        Split::new(1, "s2", "Split 2"),
        Split::new(2, "s3", "Split 3"),
    ]
}

fn make_session() -> Session<'static> {
    let splits = std::array::IntoIter::new(splits()).collect();
    let run = Run {
        attempt: attempt_info(),
        splits,
    };
    Session::new(game_category(), run)
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
        // We could use is_empty here, but this would report a less useful error
        // if there is an observation.
        assert_eq!(
            None,
            self.0.borrow().first(),
            "expected no more observations"
        )
    }
}

impl Observer for Obs {
    fn observe(&self, evt: observer::Event) {
        self.0.borrow_mut().push(evt);
    }
}
