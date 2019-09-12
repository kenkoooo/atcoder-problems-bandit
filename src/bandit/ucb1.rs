use std::cmp::Ordering;
use std::collections::{BTreeMap, BinaryHeap};

pub struct UCB1 {
    pub sum: usize,
    pub current_second: i64,
    pub selected_sum: f64,

    queue: BinaryHeap<Q>,
    rewards: BTreeMap<String, Vec<usize>>,
    max_rewards: f64,
}

impl UCB1 {
    pub fn new(start_second: i64, max_rewards: f64) -> Self {
        Self {
            sum: 0,
            current_second: start_second,
            queue: BinaryHeap::new(),
            selected_sum: 0.0,
            rewards: BTreeMap::new(),
            max_rewards,
        }
    }

    pub fn pop_one(&mut self) -> Option<String> {
        self.queue.pop().map(|q| q.contest_id)
    }

    pub fn push_result(&mut self, contest_id: &str, count: usize) {
        let reward = self
            .rewards
            .entry(contest_id.to_string())
            .or_insert(Vec::new());
        reward.push(count);

        let n = reward.len() as f64;
        let sum = reward.iter().sum::<usize>() as f64;
        let mu = sum / n;
        let u = (self.selected_sum.ln() * 2.0 / n).sqrt() * self.max_rewards;
        let x = mu + u;

        self.queue.push(Q {
            score: x,
            contest_id: contest_id.to_string(),
        });
        self.sum += count;
        self.selected_sum += 1.0;
    }
}

struct Q {
    score: f64,
    contest_id: String,
}

impl PartialEq for Q {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score && self.contest_id == other.contest_id
    }
}
impl PartialOrd for Q {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.score.partial_cmp(&other.score)
    }
}
impl Ord for Q {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).expect("Invalid comparison!")
    }
}
impl Eq for Q {}
