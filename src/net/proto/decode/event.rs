//! Decoding logic for events.

use super::{
    super::{
        super::super::model::{session::observer, short, timing},
        event, Event,
    },
    error::{Missing, Result},
};
use crate::net::proto::decode::error::Unknown;

/// Decodes a protobuf representation of an event into an observer event.
///
/// # Errors
///
/// Fails if any of the indices inside the event don't fit within `usize` on this machine, or if
///
pub fn decode(e: Event) -> Result<Option<observer::Event>> {
    e.payload.map(payload).transpose()
}

fn payload(e: event::Payload) -> Result<observer::Event> {
    Ok(match e {
        event::Payload::Total(t) => total(&t)?,
        event::Payload::Reset(info) => observer::Event::Reset(super::attempt_info(&info)?),
        event::Payload::Split(s) => split(&s)?,
    })
}

fn total(t: &event::Total) -> Result<observer::Event> {
    let ty = total_type(Unknown::TotalType.require(t.r#type.as_ref())?)?;
    let value = t.value.map(super::time).transpose()?;
    Ok(observer::Event::Total(ty, value))
}

/// Decodes an aggregate as a total.
pub fn total_type(t: &event::total::Type) -> Result<observer::Total> {
    Ok(match t {
        event::total::Type::Attempt(p) => attempt_total_type(*p)?,
        event::total::Type::Comparison(ty) => comparison_total_type(*ty)?,
    })
}

fn attempt_total_type(pace_num: i32) -> Result<observer::Total> {
    let raw_pace = Unknown::Pace.require(super::super::Pace::from_i32(pace_num))?;
    Ok(observer::Total::Attempt(super::pace(raw_pace)))
}

fn comparison_total_type(type_num: i32) -> Result<observer::Total> {
    let raw_type =
        Unknown::ComparisonTotalType.require(event::total::ComparisonType::from_i32(type_num))?;
    Ok(observer::Total::Comparison(comparison_total_type_inner(
        raw_type,
    )))
}

fn comparison_total_type_inner(
    ty: event::total::ComparisonType,
) -> timing::comparison::run::TotalType {
    use event::total::ComparisonType;
    match ty {
        ComparisonType::TotalInPbRun => timing::comparison::run::TotalType::TotalInPbRun,
        ComparisonType::SumOfBest => timing::comparison::run::TotalType::SumOfBest,
    }
}

fn split(s: &event::Split) -> Result<observer::Event> {
    let sid = short::Name::from(&s.sid);
    let event = match Missing::SplitEvent.require(s.payload.as_ref())? {
        event::split::Payload::Time(t) => split_time(t)?,
        event::split::Payload::Pace(p) => split_pace(*p)?,
    };
    Ok(observer::Event::Split(sid, event))
}

fn split_time(t: &event::split::Time) -> Result<observer::split::Event> {
    let time = super::time(t.time)?;
    Ok(observer::split::Event::Time(
        time,
        split_time_type(t.r#type()),
    ))
}

fn split_time_type(ty: event::split::time::Type) -> observer::time::Event {
    use event::split::time::Type;
    match ty {
        Type::Pushed => observer::time::Event::Pushed,
        Type::Popped => observer::time::Event::Popped,
        Type::AttemptTotal => {
            observer::time::Event::Aggregate(timing::aggregate::Kind::ATTEMPT_SPLIT)
        }
        Type::AttemptCumulative => {
            observer::time::Event::Aggregate(timing::aggregate::Kind::ATTEMPT_CUMULATIVE)
        }
        Type::ComparisonTotal => {
            observer::time::Event::Aggregate(timing::aggregate::Kind::COMPARISON_SPLIT)
        }
        Type::ComparisonCumulative => {
            observer::time::Event::Aggregate(timing::aggregate::Kind::COMPARISON_CUMULATIVE)
        }
    }
}

fn split_pace(pace_index: i32) -> Result<observer::split::Event> {
    let pace = Unknown::Pace.require(super::super::Pace::from_i32(pace_index))?;
    Ok(observer::split::Event::Pace(super::split_in_run_pace(pace)))
}
