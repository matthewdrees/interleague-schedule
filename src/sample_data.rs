use std::collections::HashSet;

use crate::interleague_schedule::*;
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
            });
        }
        last_index += team_count;
    }

    return state;
}

pub fn get_2023_majors_softball_state() -> State {
    let league_info: Vec<(String, usize)> = vec![
        ("SL".to_string(), 1),
        ("NE".to_string(), 3),
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
        ("SL".to_string(), 3),
        ("NE".to_string(), 4),
        ("MAG".to_string(), 4),
        ("QA".to_string(), 2),
        ("NW".to_string(), 1),
        ("RUG".to_string(), 3),
        ("BAL".to_string(), 2),
        ("NC".to_string(), 4),
    ];
    get_state_from_league_info(&league_info)
}

pub fn get_2024_aaa_softball_state() -> State {
    let league_info: Vec<(String, usize)> = vec![
        ("SL".to_string(), 4),
        ("NE".to_string(), 4),
        ("MAG".to_string(), 5),
        ("QA".to_string(), 2),
        ("NW".to_string(), 1),
        ("RUG".to_string(), 3),
        ("BAL".to_string(), 1),
        ("NC".to_string(), 3),
    ];
    get_state_from_league_info(&league_info)
}
fn get_day_from_day_info(day_info: (String, bool, Vec<usize>), num_teams: usize) -> Day {
    let not_these_teams = HashSet::from_iter(day_info.2);
    let teams_playing = &HashSet::from_iter(0..num_teams) - &not_these_teams;
    assert!(teams_playing.len() % 2 == 0);
    Day {
        date: day_info.0,
        is_weekend: day_info.1,
        teams_playing: teams_playing,
        games: vec![],
    }
}

pub fn get_2023_majors_softball_days(num_teams: usize) -> Vec<Day> {
    vec![
        ("3/23".to_string(), true, vec![12]),
        ("3/27".to_string(), false, vec![10]),
        ("3/30".to_string(), true, vec![11]),
        ("4/03".to_string(), false, vec![9]),
        ("4/13".to_string(), true, vec![1, 2, 3, 4, 5, 7, 10, 11, 12]),
        ("4/17".to_string(), false, vec![8]),
        ("4/20".to_string(), true, vec![0]),
        ("4/24".to_string(), false, vec![0]),
        ("4/27".to_string(), true, vec![9]),
        ("4/29".to_string(), false, vec![6]),
        ("5/01".to_string(), false, vec![5]),
        ("5/04".to_string(), true, vec![8]),
        ("5/06".to_string(), false, vec![4]),
        ("5/08".to_string(), false, vec![3]),
        ("5/11".to_string(), true, vec![7]),
        ("5/13".to_string(), false, vec![2]),
        ("5/15".to_string(), false, vec![1]),
        ("5/18".to_string(), true, vec![6]),
    ]
    .into_iter()
    .map(|di| return get_day_from_day_info(di, num_teams))
    .collect()
}

pub fn get_2023_aaa_softball_days(num_teams: usize) -> Vec<Day> {
    vec![
        ("3/23".to_string(), true, vec![22]),
        ("3/27".to_string(), false, vec![3]),
        ("3/30".to_string(), true, vec![21]),
        ("4/03".to_string(), false, vec![4]),
        (
            "4/13".to_string(),
            true,
            vec![
                4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
            ],
        ),
        ("4/17".to_string(), false, vec![5]),
        ("4/20".to_string(), true, vec![0, 1, 2]),
        ("4/24".to_string(), false, vec![0, 1, 2]),
        ("4/27".to_string(), true, vec![19]),
        ("4/29".to_string(), false, vec![6]),
        ("5/04".to_string(), true, vec![18]),
        ("5/06".to_string(), false, vec![8]),
        (
            "5/08".to_string(),
            false,
            vec![3, 7, 9, 10, 11, 12, 13, 14, 15],
        ),
        ("5/11".to_string(), true, vec![17]),
        ("5/13".to_string(), false, vec![10]),
        ("5/18".to_string(), true, vec![16]),
    ]
    .into_iter()
    .map(|di| return get_day_from_day_info(di, num_teams))
    .collect()
}

pub fn get_2024_aaa_softball_days(num_teams: usize) -> Vec<Day> {
    vec![
        ("3/23".to_string(), true, vec![]),
        ("3/26".to_string(), false, vec![]),
        ("3/30".to_string(), true, vec![]),
        ("4/02".to_string(), false, vec![]),
        ("4/16".to_string(), false, vec![]),
        ("4/20".to_string(), true, vec![]),
        ("4/23".to_string(), false, vec![]),
        ("4/27".to_string(), true, vec![]),
        ("4/30".to_string(), false, vec![]),
        ("5/04".to_string(), true, vec![]),
        ("5/07".to_string(), false, vec![]),
        ("5/11".to_string(), true, vec![]),
        ("5/14".to_string(), false, vec![]),
        ("5/18".to_string(), true, vec![]),
        ("5/21".to_string(), false, vec![]),
    ]
    .into_iter()
    .map(|di| return get_day_from_day_info(di, num_teams))
    .collect()
}
