//! Decoding logic for events.

use super::{
    super::{
        super::super::model::{session, short, timing},
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
pub fn decode(e: Event) -> Result<Option<session::Event>> {
    e.payload.map(payload).transpose()
}

fn payload(e: event::Payload) -> Result<session::Event> {
    Ok(match e {
        event::Payload::Total(t) => total(&t)?,
        event::Payload::Reset(info) => session::Event::Reset(super::attempt_info(&info)?),
        event::Payload::Split(s) => split(&s)?,
    })
}

fn total(t: &event::Total) -> Result<session::Event> {
    let ty = total_type(Unknown::TotalType.require(t.r#type.as_ref())?)?;
    let value = t.value.map(super::time).transpose()?;
    Ok(session::Event::Total(ty, value))
}

/// Decodes an aggregate as a total.
pub fn total_type(t: &event::total::Type) -> Result<session::event::Total> {
    Ok(match t {
        event::total::Type::Attempt(p) => attempt_total_type(*p)?,
        event::total::Type::Comparison(ty) => comparison_total_type(*ty)?,
    })
}

fn attempt_total_type(pace_num: i32) -> Result<session::event::Total> {
    super::pace_from_index(pace_num).map(session::event::Total::Attempt)
}

fn comparison_total_type(type_num: i32) -> Result<session::event::Total> {
    let raw_type =
        Unknown::ComparisonTotalType.require(event::total::ComparisonType::from_i32(type_num))?;
    Ok(session::event::Total::Comparison(
        comparison_total_type_inner(raw_type),
    ))
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

fn split(s: &event::Split) -> Result<session::Event> {
    let sid = short::Name::from(&s.sid);
    let event = match Missing::SplitEvent.require(s.payload.as_ref())? {
        event::split::Payload::Time(t) => split_time(t)?,
        event::split::Payload::Pace(p) => split_pace(*p)?,
        event::split::Payload::Pop(p) => split_pop(*p)?,
    };
    Ok(session::Event::Split(sid, event))
}

fn split_time(t: &event::split::Time) -> Result<session::event::Split> {
    let time = super::time(t.time)?;
    Ok(session::event::split::Split::Time(
        time,
        split_time_type(t.r#type()),
    ))
}

fn split_time_type(ty: event::split::time::Type) -> session::event::Time {
    use event::split::time::Type;
    match ty {
        Type::Pushed => session::event::Time::Pushed,
        Type::AttemptTotal => {
            session::event::Time::Aggregate(timing::aggregate::Kind::ATTEMPT_SPLIT)
        }
        Type::AttemptCumulative => {
            session::event::Time::Aggregate(timing::aggregate::Kind::ATTEMPT_CUMULATIVE)
        }
        Type::ComparisonTotal => {
            session::event::Time::Aggregate(timing::aggregate::Kind::COMPARISON_SPLIT)
        }
        Type::ComparisonCumulative => {
            session::event::Time::Aggregate(timing::aggregate::Kind::COMPARISON_CUMULATIVE)
        }
    }
}

fn split_pace(pace_index: i32) -> Result<session::event::Split> {
    super::split_in_run_pace_from_index(pace_index).map(session::event::Split::Pace)
}

fn split_pop(pop_index: i32) -> Result<session::event::Split> {
    Ok(session::event::Split::Popped(super::pop(pop_index)?))
}
