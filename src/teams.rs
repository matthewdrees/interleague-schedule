use itertools::Itertools;
use std::cmp;
use std::fmt;
use std::vec::Vec;

pub type Matrix = Vec<Vec<i32>>;

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

    if ti0 <= ti1 {
        teams_matrix[ti0][ti1] += 1;
    } else {
        teams_matrix[ti1][ti0] += 1;
    }
}

pub fn assign_in_league_games(state: &mut State, team_matrix: &mut Matrix, max_games: i32) {
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
            add_game(&mut state.teams, team_matrix, combo[0], combo[1]);
            add_game(&mut state.teams, team_matrix, combo[0], combo[1]);
        }
    }
}

// All leagues play each other by min(team nums) games, rotating teams, sorted by league distance.
pub fn assign_minimum_interleague_games(
    state: &mut State,
    team_matrix: &mut Matrix,
    league_matrix: Matrix,
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
            distance: league_matrix[v[0]][v[1]],
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
            add_game(&mut state.teams, team_matrix, ti0, ti1);
            state.leagues[comb.li0].bump_next_index();
            state.leagues[comb.li1].bump_next_index();
        }
    }
}

pub fn get_teams_to_play_against(
    state: &mut State,
    league_matrix: Matrix,
    max_games: i32,
) -> Matrix {
    let mut team_matrix = create_matrix_with_dimension(state.teams.len());

    // Assign games.
    assign_in_league_games(state, &mut team_matrix, max_games);
    assign_minimum_interleague_games(state, &mut team_matrix, league_matrix, max_games);

    return team_matrix;
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
