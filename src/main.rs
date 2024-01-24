use ::interleague_schedule::sample_data::*;
use ::interleague_schedule::teams::*;

// use crate::interleague_schedule::ScheduleConfig;

pub mod interleague_schedule;

// fn main() {
//     let config = ScheduleConfig::new(5);
//     println!("Solving for the schedule.");
//     println!(
//         "{}",
//         match backtrack::solve(config) {
//             Some(solution) => format!("{}", solution),
//             None => "No solution found".to_string(),
//         }
//     );
// }

fn main() {
    let mut state = get_2023_aaa_softball_state();
    let league_distance_matrix = get_district8_city_league_distances();
    let team_matrix = get_teams_to_play_against(&mut state, &league_distance_matrix, 16);
    dump_matrix(team_matrix);
    println!("{:?}", state);
    dump_travel_scores(&state, &league_distance_matrix);
}
