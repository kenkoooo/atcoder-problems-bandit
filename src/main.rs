use std::error::Error;
use std::fs::File;
use std::io::BufReader;

use atcoder_problems_bandit::bandit::UCB1;
use atcoder_problems_bandit::{AtCoderMock, Contest, Submission};

const ONE_MONTH: i64 = 3600 * 24 * 30;

fn main() -> Result<(), Box<dyn Error>> {
    eprintln!("Loading JSON file...");
    let contests = load_json("contests.json")?;
    eprintln!("Loaded {} contests.", contests.len());

    eprintln!("Loading CSV file...");
    let submissions = load_csv("submissions.csv")?;
    eprintln!("Loaded {} submissions.", submissions.len());

    eprintln!("Aggregate submissions...");
    let mut mock = AtCoderMock::new(submissions, contests);

    // 2016/01/01 00:00:00
    let start_time = 1451574000;
    let mut ucb1 = UCB1::new(start_time, 20.0);

    while ucb1.current_second < 1568274067 {
        let contest_id = mock
            .get_new_contest(ucb1.current_second)
            .map(|c| c.id)
            .or_else(|| ucb1.pop_one());

        if let Some(contest_id) = contest_id {
            let count = mock.pop(&contest_id, ucb1.current_second).expect("Invalid");

            ucb1.push_result(&contest_id, count);
        }

        if ucb1.current_second % ONE_MONTH == 0 {
            let avg = (ucb1.sum as f64) / ucb1.selected_sum;
            eprintln!(
                "turn{} time={} sum={} avg={}",
                ucb1.selected_sum, ucb1.current_second, ucb1.sum, avg
            );
        }

        ucb1.current_second += 20;
    }
    Ok(())
}

fn load_json(filename: &str) -> Result<Vec<Contest>, Box<dyn Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let contests = serde_json::from_reader(reader)?;
    Ok(contests)
}
fn load_csv(filename: &str) -> Result<Vec<Submission>, Box<dyn Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut reader = csv::Reader::from_reader(reader);
    let submissions = reader
        .deserialize()
        .collect::<Result<Vec<Submission>, _>>()?;
    Ok(submissions)
}
