//! Integration tests for sessions, comparisons, etc without involving the database.

use zombiesplit::model::{
    game,
    session::{event, Attempt, Session},
    short,
    timing::{
        aggregate,
        comparison::{self, delta, Comparison},
    },
    Loadable, Time,
};

const SAMPLE_GAME_PATH: &str = "scd11.toml";
const SAMPLE_GAME_NAME: &str = "scd11";
const SAMPLE_CATEGORY_NAME: &str = "btg-sonic";

fn load_game() -> game::Config {
    // TODO: deduplicate with db
    game::Config::from_toml_file(SAMPLE_GAME_PATH).expect("couldn't load sample game")
}

fn make_attempt() -> Attempt {
    Attempt::from_config(
        &load_game(),
        game::category::ShortDescriptor::new(SAMPLE_GAME_NAME, SAMPLE_CATEGORY_NAME),
    )
    .expect("couldn't load game/category")
}

fn split(name: &str, h: u32, m: u32, s: u32, ms: u32) -> (short::Name, comparison::Split) {
    let in_pb_run = aggregate::Set {
        split: Time::new(h, m, s, ms).expect("time overflowed"),
        cumulative: Default::default(),
    };
    (
        name.into(),
        comparison::Split {
            split_pb: Time::default(),
            in_pb_run,
        },
    )
}

/// Constructs a test comparison using a sample run of Sonic CD (2011).
fn comparison() -> Comparison {
    let mut splits = [
        split("pp1", 0, 0, 25, 60),
        split("pp2", 0, 0, 25, 300),
        split("pp3", 0, 0, 24, 260),
        split("cc1", 0, 0, 36, 130),
        split("cc2", 0, 0, 46, 50),
        split("cc3", 0, 0, 29, 610),
        split("tt1", 0, 0, 27, 600),
        split("tt2", 0, 0, 54, 310),
        split("tt3", 0, 0, 48, 980),
        split("qq1", 0, 0, 29, 400),
        split("qq2", 0, 0, 45, 280),
        split("qq3", 0, 1, 5, 110),
        split("ww1", 0, 0, 46, 600),
        split("ww2", 0, 0, 51, 730),
        split("ww3", 0, 1, 11, 150),
        split("ss1", 0, 0, 35, 430),
        split("ss2", 0, 0, 34, 980),
        split("ss3", 0, 1, 13, 450),
        split("mm1", 0, 0, 36, 10),
        split("mm2", 0, 0, 45, 110),
        split("mm3", 0, 1, 37, 550),
    ];

    // Fix up the cumulatives
    let mut accum = Time::default();
    for (_, split) in &mut splits {
        accum += split.in_pb_run.split;
        split.in_pb_run.cumulative = accum;
    }

    Comparison {
        splits: short::Map::from(splits),
        run: Default::default(),
    }
}

fn make_session<'o, T: event::observer::Observer>(obs: &'o T) -> Session<'static, 'o, T> {
    let mut s = Session::new(make_attempt(), obs);

    s.set_comparison_provider(Box::new(Some(comparison())));

    s
}

fn delta(string: &str) -> delta::Delta {
    string.parse().expect("couldn't parse delta")
}

fn split_delta(split: &str, run: &str) -> delta::Split {
    delta::Split::new(delta(split), delta(run))
}

/// Tests a short sample run where we push in splits and check the split and run deltas.
#[test]
fn test_session_deltas() {
    let obs = DeltaLogger::default();
    let mut s = make_session(&obs);

    // 1 second ahead of previous time
    push(&mut s, "pp1", 0, 0, 24, 60);
    assert_eq!(Some(split_delta("-1s", "-1s")), obs.delta("pp1"));

    // 2 seconds behind previous time
    push(&mut s, "pp2", 0, 0, 27, 300);
    assert_eq!(Some(split_delta("-1s", "-1s")), obs.delta("pp1"));
    assert_eq!(Some(split_delta("+2s", "+1s")), obs.delta("pp2"));

    // No change in time
    push(&mut s, "pp3", 0, 0, 24, 260);
    assert_eq!(Some(split_delta("-1s", "-1s")), obs.delta("pp1"));
    assert_eq!(Some(split_delta("+2s", "+1s")), obs.delta("pp2"));
    assert_eq!(Some(split_delta("-0s", "+1s")), obs.delta("pp3"));

    // Recalculate
    push(&mut s, "pp1", 0, 0, 1, 0);
    assert_eq!(Some(split_delta("-0s", "-0s")), obs.delta("pp1"));
    assert_eq!(Some(split_delta("+2s", "+2s")), obs.delta("pp2"));
    assert_eq!(Some(split_delta("-0s", "+2s")), obs.delta("pp3"));
}

fn push(session: &mut Session<DeltaLogger>, name: &str, h: u32, m: u32, s: u32, ms: u32) {
    let time = Time::new(h, m, s, ms).expect("time construction error");
    session.push_to(short::Name::from(name), time);
}

#[derive(Default)]
struct DeltaLogger {
    log: std::sync::Mutex<std::collections::HashMap<short::Name, delta::Split>>,
}

impl DeltaLogger {
    fn delta(&self, split: impl Into<short::Name>) -> Option<delta::Split> {
        let log = self.log.lock().expect("couldn't lock log");
        log.get(&split.into()).copied()
    }
}

impl event::observer::Observer for DeltaLogger {
    fn observe(&self, evt: event::Event) {
        if let event::Event::Split(n, event::Split::Delta(d)) = evt {
            let mut log = self.log.lock().expect("couldn't lock log");
            log.insert(n, d);
        }
    }
}
