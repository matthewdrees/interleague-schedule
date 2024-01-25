use ::interleague_schedule::interleague_schedule::*;
use ::interleague_schedule::sample_data::*;
use ::interleague_schedule::teams::*;

pub mod interleague_schedule;

fn main() {
    let mut state = get_2023_majors_softball_state();
    let league_distance_matrix = get_district8_city_league_distances();
    let team_matrix = get_teams_to_play_against(&mut state, &league_distance_matrix, 16);
    let mut remaining_games: Vec<Game> = Vec::new();
    for ti0 in 0..state.teams.len() {
        for ti1 in ti0 + 1..state.teams.len() {
            for _ in 0..get_matrix_val(&team_matrix, ti0, ti1) {
                let distance = get_matrix_val(
                    &league_distance_matrix,
                    state.teams[ti0].league_index,
                    state.teams[ti1].league_index,
                );
                remaining_games.push(Game { ti0, ti1, distance });
            }
        }
    }
    remaining_games.sort_by_key(|g| g.distance);

    let days = get_2023_majors_softball_days(state.teams.len());

    let schedule_config = ScheduleConfig {
        days,
        days_index: 0,
        remaining_games,
    };
    println!("{:?}", schedule_config);

    // println!(
    //     "{}",
    //     match backtrack::solve(config) {
    //         Some(solution) => format!("{:?}", solution),
    //         None => "No solution found".to_string(),
    //     }
    // );
}

// fn main() {
//     let mut state = get_2023_aaa_softball_state();
//     let league_distance_matrix = get_district8_city_league_distances();
//     let team_matrix = get_teams_to_play_against(&mut state, &league_distance_matrix, 16);
//     dump_matrix(team_matrix);
//     println!("{:?}", state);
//     dump_travel_scores(&state, &league_distance_matrix);
// }
