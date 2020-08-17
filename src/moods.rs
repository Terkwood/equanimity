use crate::*;
use chrono::prelude::*;

const DAYS_TO_DISPLAY: u8 = 14;

pub fn recent(
    mrs: &[MoodReading],
    right_now_ms: u64,
    local_datetime: fn(u64) -> DateTime<FixedOffset>,
) -> Vec<MoodReading> {
    let grouped = group_by(mrs, |mr| local_datetime(mr.epoch_millis).date());

    let cutoff =
        local_datetime(right_now_ms).date() - chrono::Duration::days(DAYS_TO_DISPLAY as i64);

    let recent_grouped = grouped.iter().filter(|(date, _)| date > &cutoff);

    let max_in_each = recent_grouped.map(|(_date, mood_readings)| wildest(mood_readings));

    let mut maybe_more_entries_than_allowed: Vec<MoodReading> = max_in_each
        .map(|at_most_two| match at_most_two {
            HighLowMoods::Nothing => vec![],
            HighLowMoods::One(mr) => vec![mr],
            HighLowMoods::MaxMin(h, l) => vec![h, l],
        })
        .flatten()
        .collect();

    maybe_more_entries_than_allowed.sort();

    if maybe_more_entries_than_allowed.len() > DAYS_TO_DISPLAY as usize {
        maybe_more_entries_than_allowed
            .iter()
            .skip(maybe_more_entries_than_allowed.len() - DAYS_TO_DISPLAY as usize)
            .cloned()
            .collect()
    } else {
        maybe_more_entries_than_allowed
    }
}

enum HighLowMoods {
    Nothing,
    One(MoodReading),
    MaxMin(MoodReading, MoodReading),
}

fn wildest(readings: &Vec<&MoodReading>) -> HighLowMoods {
    let mut lowest: Option<MoodReading> = None;
    let mut nil: Option<MoodReading> = None;
    let mut highest: Option<MoodReading> = None;
    for mr in readings {
        if mr.value < 0 && mr.value < lowest.map(|l| l.value).unwrap_or(0) {
            lowest = Some(**mr)
        } else if mr.value == 0 && nil.is_none() {
            nil = Some(**mr)
        } else if mr.value > 0 && mr.value > highest.map(|h| h.value).unwrap_or(0) {
            highest = Some(**mr)
        }
    }

    match (lowest, nil, highest) {
        (None, None, None) => HighLowMoods::Nothing,
        (None, Some(mr), None) => HighLowMoods::One(mr),
        (Some(l), _, None) => HighLowMoods::One(l),
        (None, _, Some(h)) => HighLowMoods::One(h),
        (Some(l), _, Some(h)) => HighLowMoods::MaxMin(h, l),
    }
}

fn group_by<I, F, K, T>(xs: I, mut key_fn: F) -> Vec<(K, Vec<T>)>
where
    I: IntoIterator<Item = T>,
    F: FnMut(&T) -> K,
    K: Eq,
{
    let mut groups = Vec::<(K, Vec<T>)>::new();
    for item in xs {
        let key = key_fn(&item);
        let last = groups.last_mut();
        if let Some((_, group)) = last.filter(|(k, _)| k == &key) {
            group.push(item);
        } else {
            groups.push((key, vec![item]));
        }
    }
    groups
}

#[cfg(test)]
mod test {
    use super::*;
    use chrono::prelude::*;

    const ONE_DAY_MS: u64 = 86_400_000;

    #[test]
    fn test_recent_moods_dedup() {
        let right_now = Utc::now().timestamp_millis() as u64;

        let exp00 = MoodReading {
            value: -2,
            epoch_millis: right_now,
        };

        let exp01 = MoodReading {
            value: 3,
            epoch_millis: right_now - ONE_DAY_MS,
        };

        let exp02_a = MoodReading {
            value: 3,
            epoch_millis: right_now - 2 * ONE_DAY_MS,
        };

        let exp02_b = MoodReading {
            value: -3,
            epoch_millis: right_now - 2 * ONE_DAY_MS - 1,
        };

        let exp03_a = MoodReading {
            value: -1,
            epoch_millis: right_now - 3 * ONE_DAY_MS,
        };

        let exp03_b = MoodReading {
            value: 2,
            epoch_millis: right_now - 3 * ONE_DAY_MS - 1,
        };

        let convoluted = vec![
            MoodReading {
                value: 0,
                epoch_millis: right_now,
            },
            MoodReading {
                value: -1,
                epoch_millis: right_now,
            },
            exp00,
            exp01,
            MoodReading {
                value: 2,
                epoch_millis: right_now - ONE_DAY_MS,
            },
            MoodReading {
                value: 1,
                epoch_millis: right_now - ONE_DAY_MS,
            },
            exp02_a,
            exp02_b,
            MoodReading {
                value: 0,
                epoch_millis: right_now - 2 * ONE_DAY_MS,
            },
            exp03_a,
            MoodReading {
                value: 0,
                epoch_millis: right_now - 3 * ONE_DAY_MS,
            },
            exp03_b,
        ];

        let actual = recent(right_now, &convoluted);

        let mut last_timestamp = 0;
        for a in &actual {
            assert!(a.epoch_millis > last_timestamp);
            last_timestamp = a.epoch_millis;
        }

        let mut expected = vec![exp00, exp01, exp02_a, exp02_b, exp03_a, exp03_b];
        expected.reverse();

        assert_eq!(actual, expected)
    }

    #[test]
    fn test_recent_mood_dedup_2() {
        let right_now = Utc::now().timestamp_millis() as u64;

        let mut many_dup_vals = vec![];
        for i in 0..10 {
            many_dup_vals.push(MoodReading {
                value: 0,
                epoch_millis: right_now - i * ONE_DAY_MS,
            });
            many_dup_vals.push(MoodReading {
                value: 0,
                epoch_millis: right_now - i * ONE_DAY_MS,
            });
            many_dup_vals.push(MoodReading {
                value: 0,
                epoch_millis: right_now - i * ONE_DAY_MS,
            });
            many_dup_vals.push(MoodReading {
                value: 0,
                epoch_millis: right_now - i * ONE_DAY_MS,
            });
        }

        let result = recent(right_now, &many_dup_vals);
        assert_eq!(10, result.len());
    }
}
