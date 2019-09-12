use serde::Deserialize;
use std::collections::{BTreeMap, VecDeque};

pub struct AtCoderMock {
    map: BTreeMap<String, (VecDeque<Submission>)>,
    contests: VecDeque<Contest>,
}

impl AtCoderMock {
    pub fn new(mut submissions: Vec<Submission>, mut contests: Vec<Contest>) -> Self {
        submissions.sort_by_key(|s| s.id);
        let map = submissions.into_iter().fold(BTreeMap::new(), |mut map, s| {
            map.entry(s.contest_id.clone())
                .or_insert(VecDeque::new())
                .push_back(s);
            map
        });

        contests.sort_by_key(|c| c.start_epoch_second + c.duration_second);
        let contests = contests
            .into_iter()
            .filter(|c| map.contains_key(&c.id))
            .collect();
        Self { map, contests }
    }

    pub fn get_new_contest(&mut self, current_second: i64) -> Option<Contest> {
        let c = self.contests.pop_front()?;

        if c.start_epoch_second + c.duration_second > current_second {
            self.contests.push_front(c);
            None
        } else {
            Some(c)
        }
    }

    pub fn pop(&mut self, contest_id: &str, current_second: i64) -> Option<usize> {
        let q = self.map.get_mut(contest_id)?;

        let mut count = 0;
        while let Some(s) = q.pop_front() {
            if s.epoch_second > current_second {
                q.push_front(s);
                break;
            } else {
                count += 1;
            }
        }
        Some(count)
    }
}

#[derive(Debug, Deserialize)]
pub struct Submission {
    pub id: i64,
    pub epoch_second: i64,
    pub problem_id: String,
    pub contest_id: String,
    pub user_id: String,
    pub language: String,
    pub point: f64,
    pub length: i64,
    pub result: String,
    pub execution_time: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct Contest {
    pub id: String,
    pub start_epoch_second: i64,
    pub duration_second: i64,
}
