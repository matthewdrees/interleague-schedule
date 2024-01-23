use crate::teams::*;

pub fn get_district8_city_league_distances() -> Matrix {
    let mut league_distance_matrix = create_matrix_with_dimension(8);
    league_distance_matrix[0][1] = 3; // NE - SL
    league_distance_matrix[0][2] = 3; // NE - MAG
    league_distance_matrix[0][3] = 3; // NE - QA
    league_distance_matrix[0][4] = 3; // NE - NW
    league_distance_matrix[0][5] = 1; // NE - RUG
    league_distance_matrix[0][6] = 3; // NE - BAL
    league_distance_matrix[0][7] = 2; // NE - NC
    league_distance_matrix[1][2] = 4; // SL - MAG
    league_distance_matrix[1][3] = 4; // SL - QA
    league_distance_matrix[1][4] = 1; // SL - NW
    league_distance_matrix[1][5] = 1; // SL - RUG
    league_distance_matrix[1][6] = 2; // SL - BAL
    league_distance_matrix[1][7] = 2; // SL - NC
    league_distance_matrix[2][3] = 1; // MAG - QA
    league_distance_matrix[2][4] = 2; // MAG - NW
    league_distance_matrix[2][5] = 2; // MAG - RUG
    league_distance_matrix[2][6] = 1; // MAG - BAL
    league_distance_matrix[2][7] = 1; // MAG - NC
    league_distance_matrix[3][4] = 2; // QA - NW
    league_distance_matrix[3][5] = 2; // QA - RUG
    league_distance_matrix[3][6] = 1; // QA - BAL
    league_distance_matrix[3][7] = 1; // QA - NC
    league_distance_matrix[4][5] = 2; // NW - RUG
    league_distance_matrix[4][6] = 1; // NW - BAL
    league_distance_matrix[4][7] = 1; // NW - NC
    league_distance_matrix[5][6] = 2; // RUG - BAL
    league_distance_matrix[5][7] = 1; // RUG - NC
    league_distance_matrix[6][7] = 1; // BAL - NC
    return league_distance_matrix;
}

pub fn get_state_from_league_info(league_info: &Vec<(String, usize)>) -> State {
    let mut state = State {
        leagues: vec![],
        teams: vec![],
    };
    let mut last_index: usize = 0;
    for (li, info) in league_info.iter().enumerate() {
        let name: String = info.0.clone();
        let team_count: usize = info.1;
        state.leagues.push(League {
            name: name,
            team_index_range: (last_index, last_index + team_count),
        });
        for _ in 0..team_count {
            state.teams.push(Team {
                league_index: li,
                num_games: 0,
                teams_against: vec![],
                total_distance: 0,
            });
        }
        last_index += team_count;
    }

    return state;
}

pub fn get_2023_majors_softball_state() -> State {
    let league_info: Vec<(String, usize)> = vec![
        ("NE".to_string(), 3),
        ("SL".to_string(), 1),
        ("MAG".to_string(), 2),
        ("QA".to_string(), 1),
        ("NW".to_string(), 1),
        ("RUG".to_string(), 1),
        ("BAL".to_string(), 2),
        ("NC".to_string(), 2),
    ];
    get_state_from_league_info(&league_info)
}

pub fn get_2023_aaa_softball_state() -> State {
    let league_info: Vec<(String, usize)> = vec![
        ("NE".to_string(), 4),
        ("SL".to_string(), 3),
        ("MAG".to_string(), 4),
        ("QA".to_string(), 2),
        ("NW".to_string(), 1),
        ("RUG".to_string(), 3),
        ("BAL".to_string(), 2),
        ("NC".to_string(), 4),
    ];
    get_state_from_league_info(&league_info)
}
