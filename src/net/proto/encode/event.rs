//! Encoding logic for events.

use super::super::{
    super::super::model::{attempt::observer, timing::comparison, Time},
    event, Event,
};

/// Encodes an observer-level event into a protobuf event.
///
/// # Errors
///
/// Fails with `out_of_range` if any attempt counts cannot be stored as 64-bit integers.
pub fn encode(event: &observer::Event) -> super::Result<Event> {
    Ok(Event {
        payload: match event {
            observer::Event::Total(ty, time) => Some(event::Payload::Total(total(*ty, *time))),
            observer::Event::Reset(info) => Some(event::Payload::Reset(super::attempt_info(info)?)),
            observer::Event::Split(_, _) => None,
        },
    })
}

/// Encodes a total event into its protobuf form.
fn total(ty: observer::Total, time: Option<Time>) -> event::Total {
    event::Total {
        r#type: Some(total_type(ty)),
        value: time.map(u32::from),
    }
}

fn total_type(ty: observer::Total) -> event::total::Type {
    match ty {
        observer::Total::Attempt(p) => event::total::Type::Attempt(super::pace(p) as i32),
        observer::Total::Comparison(ty) => {
            event::total::Type::Comparison(comparison_total_type(ty) as i32)
        }
    }
}

fn comparison_total_type(ty: comparison::run::TotalType) -> event::total::ComparisonType {
    match ty {
        comparison::run::TotalType::TotalInPbRun => event::total::ComparisonType::TotalInPbRun,
        comparison::run::TotalType::SumOfBest => event::total::ComparisonType::SumOfBest,
    }
}
