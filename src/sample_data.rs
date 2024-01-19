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

pub fn get_2023_majors_softball_state() -> State {
    let state: State = State {
        leagues: vec![
            League {
                name: "NE".to_string(),
                team_index_range: (0, 3),
                next_index: 0,
            },
            League {
                name: "SL".to_string(),
                team_index_range: (3, 4),
                next_index: 3,
            },
            League {
                name: "MAG".to_string(),
                team_index_range: (4, 6),
                next_index: 4,
            },
            League {
                name: "QA".to_string(),
                team_index_range: (6, 7),
                next_index: 6,
            },
            League {
                name: "NW".to_string(),
                team_index_range: (7, 8),
                next_index: 7,
            },
            League {
                name: "RUG".to_string(),
                team_index_range: (8, 9),
                next_index: 8,
            },
            League {
                name: "BAL".to_string(),
                team_index_range: (9, 11),
                next_index: 9,
            },
            League {
                name: "NC".to_string(),
                team_index_range: (11, 13),
                next_index: 11,
            },
        ],
        teams: vec![
            Team {
                league_index: 0,
                num_games: 0,
                teams_against: vec![],
            },
            Team {
                league_index: 0,
                num_games: 0,
                teams_against: vec![],
            },
            Team {
                league_index: 0,
                num_games: 0,
                teams_against: vec![],
            },
            Team {
                league_index: 1,
                num_games: 0,
                teams_against: vec![],
            },
            Team {
                league_index: 2,
                num_games: 0,
                teams_against: vec![],
            },
            Team {
                league_index: 2,
                num_games: 0,
                teams_against: vec![],
            },
            Team {
                league_index: 3,
                num_games: 0,
                teams_against: vec![],
            },
            Team {
                league_index: 4,
                num_games: 0,
                teams_against: vec![],
            },
            Team {
                league_index: 5,
                num_games: 0,
                teams_against: vec![],
            },
            Team {
                league_index: 6,
                num_games: 0,
                teams_against: vec![],
            },
            Team {
                league_index: 6,
                num_games: 0,
                teams_against: vec![],
            },
            Team {
                league_index: 7,
                num_games: 0,
                teams_against: vec![],
            },
            Team {
                league_index: 7,
                num_games: 0,
                teams_against: vec![],
            },
        ],
    };
    return state;
}
