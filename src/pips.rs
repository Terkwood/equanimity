use std::{collections::HashMap};

use crate::{*, moods::{HighLowMoods, WithEquanimity}};

pub fn draw(v: &[MoodReading]) -> String {
    let by_day = group_by_day(v);
    
    let mut s = String::new();
    for mr in by_day {
        match mr.0  {
            HighLowMoods::Nothing => {},
            HighLowMoods::One(mr, eq) => {s.
                push_str(&draw_one(&mr, eq));},
            HighLowMoods::MaxMin(max, min,eq) => {
                s.push_str(&draw_one(&max, eq));
                s.push_str(&draw_one(&min, eq));
            }

        }
        
    }

    s
}

/// but it will return two readings on a given day
/// if it needs to print both manic and depressive
fn group_by_day(v: &[MoodReading]) -> Vec<(HighLowMoods, WithEquanimity)> {
    let mut by_day: HashMap<chrono::NaiveDate, (MoodReading, WithEquanimity)> = HashMap::new();
    
    for mr in v {
        let time = chrono::NaiveDateTime::from_timestamp_millis(mr.epoch_millis as i64);
        let date = time.map(|t | t.date());
        if let Some(d) = date {
            let entry = by_day.entry(d).or_insert(
                (
                    mr.clone(), 
                (mr.value == 0).into()
            ));
            let new_eq = if mr.value == 0  {
               WithEquanimity::Yes 
            } else { WithEquanimity::No};
        }
    }
    panic!("group by day");
}

fn draw_one(mr: &MoodReading, with_equanimity: WithEquanimity) -> String {
   let s = format!("{}{}{}", manic_pips(mr), equanimity_pip(mr, with_equanimity == WithEquanimity::Yes), depressive_pips(mr));
   s
}

fn equanimity_pip(mr: &MoodReading, with_equanimity: bool) -> String {
    if with_equanimity || mr.value == 0 {
        "⚪".to_string()
    } else {
        "⚫".to_string()
    }
}

fn manic_pips(mr: &MoodReading) -> String {
    let mut s = String::new();
    for _ in 0..mr.value.abs() {
        s.push_str("🔴");
    }
    for _ in mr.value.abs()..3 {
        s.push_str("⚫");
    }
    let r = s.chars().rev().collect();
    r
}
fn depressive_pips (mr: &MoodReading) -> String {
    let mut s = String::new();
    for _ in 0..mr.value.abs() {
        s.push_str("🔵");
    }
    for _ in mr.value.abs()..3 {
        s.push_str("⚫");
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_draw_one() {
        let mr = MoodReading {
            epoch_millis: 0,
            value: 0,
        };
        let s = draw_one(&mr, WithEquanimity::Yes);
        assert_eq!(s, "⚫⚫⚫⚪⚫⚫⚫");
    }
    #[test]
    fn test_draw_one_no_eq() {
        let mr = MoodReading {
            epoch_millis: 0,
            value: 1,
        };
        let s = draw_one(&mr, WithEquanimity::No);
        assert_eq!(s, "⚫⚫⚫⚫🔴⚫⚫");
    }
    #[test]
    fn test_draw_one_1() {
        let mr = MoodReading {
            epoch_millis: 0,
            value: 1,
        };
        let s = draw_one(&mr, WithEquanimity::Yes);
        assert_eq!(s, "⚫⚫⚫⚪🔴⚫⚫");
    }
    #[test]
    fn test_draw_one_2() {
        let mr = MoodReading {
            epoch_millis: 0,
            value: 2,
        };
        let s = draw_one(&mr, WithEquanimity::No);
        assert_eq!(s, "⚫⚫⚫⚫🔴🔴⚫");
    }
    #[test]
    fn test_draw_one_2_a() {
        let mr = MoodReading {
            epoch_millis: 0,
            value: 2,
        };
        let s = draw_one(&mr, WithEquanimity::Yes);
        assert_eq!(s, "⚫⚫⚫⚪🔴🔴⚫");
    }
}