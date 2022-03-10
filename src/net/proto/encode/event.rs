//! Encoding logic for events.

use super::super::{
    super::super::model::{session, short, timing, Time},
    event, Event,
};

/// Encodes an observer-level event into a protobuf event.
///
/// # Errors
///
/// Fails with `out_of_range` if any attempt counts cannot be stored as 64-bit integers.
pub fn encode(event: &session::Event) -> super::Result<Event> {
    Ok(Event {
        payload: Some(match event {
            session::Event::Total(ty, time) => event::Payload::Total(total(*ty, *time)),
            session::Event::Reset(info) => event::Payload::Reset(super::attempt_info(info)?),
            session::Event::Split(sid, event) => event::Payload::Split(split(sid, event)),
        }),
    })
}

/// Encodes a total event into its protobuf form.
fn total(ty: session::event::Total, time: Option<Time>) -> event::Total {
    event::Total {
        r#type: Some(total_type(ty)),
        value: time.map(u32::from),
    }
}

fn total_type(ty: session::event::Total) -> event::total::Type {
    match ty {
        session::event::Total::Attempt(p) => event::total::Type::Attempt(super::pace(p) as i32),
        session::event::Total::Comparison(ty) => {
            event::total::Type::Comparison(comparison_total_type(ty) as i32)
        }
    }
}

fn comparison_total_type(ty: timing::comparison::run::TotalType) -> event::total::ComparisonType {
    use timing::comparison::run::TotalType;
    match ty {
        TotalType::TotalInPbRun => event::total::ComparisonType::TotalInPbRun,
        TotalType::SumOfBest => event::total::ComparisonType::SumOfBest,
    }
}

fn split(sid: &short::Name, event: &session::event::Split) -> event::Split {
    event::Split {
        sid: sid.to_string(),
        payload: Some(split_payload(event)),
    }
}

fn split_payload(event: &session::event::Split) -> event::split::Payload {
    use {event::split::Payload, session::event::Split};
    match event {
        Split::Time(time, ty) => Payload::Time(split_time(*ty, *time)),
        Split::Pace(pace) => Payload::Pace(super::split_in_run_pace(*pace) as i32),
        Split::Popped(ty) => Payload::Pop(super::pop(*ty)),
    }
}

fn split_time(ty: session::event::Time, time: Time) -> event::split::Time {
    event::split::Time {
        r#type: split_time_type(ty) as i32,
        time: u32::from(time),
    }
}

fn split_time_type(ty: session::event::Time) -> event::split::time::Type {
    use {event::split::time::Type, session::event::Time, timing::aggregate::Kind};
    match ty {
        Time::Pushed => Type::Pushed,
        Time::Aggregate(Kind::ATTEMPT_SPLIT) => Type::AttemptTotal,
        Time::Aggregate(Kind::ATTEMPT_CUMULATIVE) => Type::AttemptCumulative,
        Time::Aggregate(Kind::COMPARISON_SPLIT) => Type::ComparisonTotal,
        Time::Aggregate(Kind::COMPARISON_CUMULATIVE) => Type::ComparisonCumulative,
    }
}
