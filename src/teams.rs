use itertools::Itertools;
use std::cmp;
use std::cmp::Ordering;
use std::fmt;
use std::vec::Vec;

pub type Matrix = Vec<Vec<i32>>;

pub fn increment_matrix(matrix: &mut Matrix, i0: usize, i1: usize) {
    if i0 <= i1 {
        matrix[i0][i1] += 1;
    } else {
        matrix[i1][i0] += 1;
    }
}

pub fn get_matrix_val(matrix: &Matrix, i0: usize, i1: usize) -> i32 {
    if i0 <= i1 {
        return matrix[i0][i1];
    } else {
        return matrix[i1][i0];
    }
}

pub fn create_matrix_with_dimension(dimension: usize) -> Matrix {
    let m: Matrix = vec![vec![0; dimension]; dimension];
    return m;
}

pub fn dump_matrix(matrix: Matrix) {
    for row in matrix.iter() {
        println!("{:?}", row)
    }
}

pub struct League {
    pub name: String,
    pub team_index_range: (usize, usize),
}

impl League {
    fn num_teams(&self) -> usize {
        return self.team_index_range.1 - self.team_index_range.0;
    }
}

pub fn dump_travel_scores(state: &State, league_distance_matrix: &Matrix) {
    let mut total_travel_score = 0;

    println!("Travel scores:");
    for (ti0, team) in state.teams.iter().enumerate() {
        let li0 = state.teams[ti0].league_index;

        let mut team_travel_score = 0;
        for ti1 in team.teams_against.iter() {
            let li1 = state.teams[*ti1].league_index;
            team_travel_score += get_matrix_val(league_distance_matrix, li0, li1);
        }
        let league_name = &state.leagues[li0].name;
        let team_num = ti0 - state.leagues[li0].team_index_range.0;
        println!(" - {}{}: {}", league_name, team_num, team_travel_score);
        total_travel_score += team_travel_score;
    }
    println!("Total travel score: {}", total_travel_score);
}

pub struct Team {
    pub league_index: usize,
    pub num_games: i32,
    pub teams_against: Vec<usize>,
}

pub struct State {
    pub leagues: Vec<League>,
    pub teams: Vec<Team>,
}

impl fmt::Debug for State {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        for league in self.leagues.iter() {
            write!(formatter, "{}\n", league.name)?;
            for (i, ti) in (league.team_index_range.0..league.team_index_range.1).enumerate() {
                let team = &self.teams[ti];
                write!(
                    formatter,
                    " {}. team idx: {}, #games: {}, ",
                    i + 1,
                    ti,
                    team.num_games
                )?;
                for ti in team.teams_against.iter() {
                    let league_index = self.teams[*ti].league_index;
                    write!(
                        formatter,
                        "{}{},",
                        self.leagues[league_index].name,
                        *ti - self.leagues[league_index].team_index_range.0 + 1
                    )?;
                }
                write!(formatter, "\n")?;
            }
        }
        Ok(())
    }
}

pub fn add_game(teams: &mut Vec<Team>, teams_matrix: &mut Matrix, ti0: usize, ti1: usize) {
    teams[ti0].num_games += 1;
    teams[ti0].teams_against.push(ti1);

    teams[ti1].num_games += 1;
    teams[ti1].teams_against.push(ti0);

    increment_matrix(teams_matrix, ti0, ti1);
}

pub fn assign_in_league_games(state: &mut State, teams_matrix: &mut Matrix, max_games: i32) {
    for league in state.leagues.iter() {
        assert!(
            (league.num_teams() - 1) * 2 <= max_games as usize,
            "Too many games in league for inter-league."
        );
        for combo in (league.team_index_range.0..league.team_index_range.1)
            .into_iter()
            .combinations(2)
        {
            // Teams play other in-league teams twice.
            add_game(&mut state.teams, teams_matrix, combo[0], combo[1]);
            add_game(&mut state.teams, teams_matrix, combo[0], combo[1]);
        }
    }
}

// All leagues play each other by min(team nums) games, rotating teams, sorted by league distance.
pub fn assign_minimum_interleague_games(
    state: &mut State,
    teams_matrix: &mut Matrix,
    league_distance_matrix: &Matrix,
    max_games: i32,
) {
    let mut leagues_next_index: Vec<usize> =
        state.leagues.iter().map(|l| l.team_index_range.0).collect();

    struct LeagueDistanceCombo {
        li0: usize,
        li1: usize,
        distance: i32,
    }
    let mut league_combos: Vec<LeagueDistanceCombo> = Vec::new();
    for v in (0..state.leagues.len()).into_iter().combinations(2) {
        league_combos.push(LeagueDistanceCombo {
            li0: v[0],
            li1: v[1],
            distance: league_distance_matrix[v[0]][v[1]],
        });
    }
    league_combos.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());

    for comb in league_combos {
        let num_games = cmp::min(
            state.leagues[comb.li0].num_teams(),
            state.leagues[comb.li1].num_teams(),
        );
        for _ in 0..num_games {
            let ti0 = leagues_next_index[comb.li0];
            let ti1 = leagues_next_index[comb.li1];

            if state.teams[ti0].num_games == max_games || state.teams[ti1].num_games == max_games {
                // One of the leagues is full. Stop.
                break;
            }
            add_game(&mut state.teams, teams_matrix, ti0, ti1);
            leagues_next_index[comb.li0] += 1;
            if leagues_next_index[comb.li0] >= state.leagues[comb.li0].team_index_range.1 {
                leagues_next_index[comb.li0] = state.leagues[comb.li0].team_index_range.0;
            }
            leagues_next_index[comb.li1] += 1;
            if leagues_next_index[comb.li1] >= state.leagues[comb.li1].team_index_range.1 {
                leagues_next_index[comb.li1] = state.leagues[comb.li1].team_index_range.0;
            }
        }
    }
}

struct GameCompare {
    // Needed for adding the game afterwards.
    ti0: usize,
    ti1: usize,

    // Compare by these fields, in order
    num_games_against: i32,
    num_league_games_against: i32,
    max_total_games: i32,
    min_total_games: i32,
    distance: i32,
}

impl PartialEq for GameCompare {
    fn eq(&self, other: &Self) -> bool {
        return self.num_games_against == other.num_games_against
            && self.num_league_games_against == other.num_league_games_against
            && self.max_total_games == other.max_total_games
            && self.min_total_games == other.min_total_games
            && self.distance == other.distance;
    }
}

impl PartialOrd for GameCompare {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(
            self.num_games_against
                .cmp(&other.num_games_against)
                .reverse()
                .then(
                    self.num_league_games_against
                        .cmp(&other.num_league_games_against)
                        .reverse()
                        .then(
                            self.max_total_games
                                .cmp(&other.max_total_games)
                                .reverse()
                                .then(
                                    self.min_total_games
                                        .cmp(&other.min_total_games)
                                        .reverse()
                                        .then(self.distance.cmp(&other.distance).reverse()),
                                ),
                        ),
                ),
        )
    }
}

pub fn assign_remaining_games(
    state: &mut State,
    teams_matrix: &mut Matrix,
    league_distance_matrix: &Matrix,
    max_games: i32,
) {
    loop {
        let mut gco: Option<GameCompare> = None;
        for (ti0, team0) in state.teams.iter().enumerate() {
            if team0.num_games == max_games {
                continue;
            }
            for ti1 in ti0 + 1..state.teams.len() {
                let team1 = &state.teams[ti1];
                if team1.num_games == max_games {
                    continue;
                }
                let gco_candidate = Some(GameCompare {
                    ti0,
                    ti1: ti1,
                    max_total_games: cmp::max(team0.num_games, team1.num_games),
                    min_total_games: cmp::min(team0.num_games, team1.num_games),
                    num_games_against: get_matrix_val(teams_matrix, ti0, ti1),
                    num_league_games_against: 0,
                    distance: get_matrix_val(
                        league_distance_matrix,
                        team0.league_index,
                        team1.league_index,
                    ),
                });

                if gco_candidate > gco {
                    gco = gco_candidate;
                }
            }
        }
        if gco.is_none() {
            break;
        }
        let gc = gco.unwrap();
        add_game(&mut state.teams, teams_matrix, gc.ti0, gc.ti1);
    }
}

pub fn get_teams_to_play_against(
    state: &mut State,
    league_distance_matrix: &Matrix,
    max_games: i32,
) -> Matrix {
    let mut teams_matrix = create_matrix_with_dimension(state.teams.len());

    // Assign games.
    assign_in_league_games(state, &mut teams_matrix, max_games);
    assign_minimum_interleague_games(state, &mut teams_matrix, &league_distance_matrix, max_games);
    assign_remaining_games(state, &mut teams_matrix, &league_distance_matrix, max_games);

    return teams_matrix;
}

#[cfg(test)]
mod tests {
    use crate::teams::*;

    #[test]
    fn matrix_tests() {
        let mut exp = create_matrix_with_dimension(2);
        exp[0][0] = 0;
        let act = create_matrix_with_dimension(2);
        assert_eq!(exp, act);
    }
}
