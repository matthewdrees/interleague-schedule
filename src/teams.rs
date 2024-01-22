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
    pub next_index: usize,
}

impl League {
    fn num_teams(&self) -> usize {
        return self.team_index_range.1 - self.team_index_range.0;
    }

    fn bump_next_index(&mut self) {
        self.next_index += 1;
        if self.next_index >= self.team_index_range.1 {
            self.next_index = self.team_index_range.0;
        }
    }
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
            write!(
                formatter,
                "{}, next index: {}\n",
                league.name, league.next_index
            )?;
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
            let ti0 = state.leagues[comb.li0].next_index;
            let ti1 = state.leagues[comb.li1].next_index;

            if state.teams[ti0].num_games == max_games || state.teams[ti1].num_games == max_games {
                // One of the leagues is full. Stop.
                break;
            }
            add_game(&mut state.teams, teams_matrix, ti0, ti1);
            state.leagues[comb.li0].bump_next_index();
            state.leagues[comb.li1].bump_next_index();
        }
    }
}

struct TeamCompareThing {
    ti1: usize,
    li1: usize,
    distance: i32,
    num_games_against: i32,
    total_games: i32,
}

impl PartialEq for TeamCompareThing {
    fn eq(&self, other: &Self) -> bool {
        return self.distance == other.distance
            && self.num_games_against == other.num_games_against
            && self.total_games == other.total_games;
    }
}

impl PartialOrd for TeamCompareThing {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(
            self.num_games_against
                .cmp(&other.num_games_against)
                .reverse()
                .then(
                    self.distance
                        .cmp(&other.distance)
                        .reverse()
                        .then(self.total_games.cmp(&other.total_games).reverse()),
                ),
        );
    }
}

fn get_team_compare_thing(
    ti0: usize,
    ti1: usize,
    t1_total_games: i32,
    li0: usize,
    li1: usize,
    teams_matrix: &Matrix,
    league_distance_matrix: &Matrix,
) -> TeamCompareThing {
    TeamCompareThing {
        ti1: ti1,
        li1: li1,
        distance: get_matrix_val(league_distance_matrix, li0, li1),
        num_games_against: get_matrix_val(teams_matrix, ti0, ti1),
        total_games: t1_total_games,
    }
}

pub fn assign_remaining_games(
    state: &mut State,
    teams_matrix: &mut Matrix,
    league_distance_matrix: &Matrix,
    max_games: i32,
) {
    for li0 in 0..state.leagues.len() {
        let mut num_teams_remaining = state.leagues[li0].num_teams();
        while num_teams_remaining > 0 {
            let ti0 = state.leagues[li0].next_index;
            if state.teams[ti0].num_games >= max_games {
                num_teams_remaining -= 1;
                state.leagues[li0].bump_next_index();
                continue;
            }
            let mut tct: Option<TeamCompareThing> = None;
            for (ti1, team) in state.teams.iter().enumerate() {
                if ti0 == ti1 {
                    continue;
                }
                if team.num_games >= max_games {
                    continue;
                }
                // println!(
                //     "assign remaining games, state.leagues[li0] {}, ti0 {}, ti1 {}, teams remaining: {}",
                //     league.name, ti0, ti1, num_teams_remaining
                // );
                let li1 = state.teams[ti1].league_index;
                let tct2 = Some(get_team_compare_thing(
                    ti0,
                    ti1,
                    team.num_games,
                    li0,
                    li1,
                    teams_matrix,
                    league_distance_matrix,
                ));
                if tct < tct2 {
                    tct = tct2;
                }
            }
            let team1_comp_thing = tct.unwrap_or_else(|| {
                panic!(
                    "error finding game for league {}, ti0 {}",
                    state.leagues[li0].name, ti0,
                )
            });
            add_game(&mut state.teams, teams_matrix, ti0, team1_comp_thing.ti1);
            state.leagues[li0].bump_next_index();
            if li0 != team1_comp_thing.li1 {
                state.leagues[team1_comp_thing.li1].bump_next_index();
            }
        }
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
