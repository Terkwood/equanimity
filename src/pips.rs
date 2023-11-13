use crate::*;

pub fn draw(v: &[MoodReading]) -> String {
    let by_day = group_by_day(v);
    
    let mut s = String::new();
    for mr in by_day {
        s.push_str(&draw_one(&mr));
    }

    s
}

/// but it will return two readings on a given day
/// if it needs to print both manic and depressive
fn group_by_day(v: &[MoodReading]) -> Vec<MoodReading> {
    panic!("group by day");
}

fn draw_one(mr: &MoodReading) -> String {
   let s = format!("{}{}{}", manic_pips(mr), equanimity_pip(mr), depressive_pips(mr));
   s
}

fn equanimity_pip(mr: &MoodReading) -> String {
    if mr.value == 0 {
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
        let s = draw_one(&mr);
        assert_eq!(s, "⚫⚫⚫⚪⚫⚫⚫");
    }
}