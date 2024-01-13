use crate::interleague_schedule::ScheduleConfig;

pub mod interleague_schedule;

fn main() {
    let config = ScheduleConfig::new(5);
    println!("Solving for the schedule.");
    println!(
        "{}",
        match backtrack::solve(config) {
            Some(solution) => format!("{}", solution),
            None => "No solution found".to_string(),
        }
    );
}
