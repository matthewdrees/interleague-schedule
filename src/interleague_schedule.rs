use backtrack::Config;
use std::collections::HashSet;
use std::fmt;

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Game {
    pub ti0: usize,
    pub ti1: usize,
    pub distance: i32,
}

#[derive(Clone)]
pub struct Day {
    pub date: String,
    pub is_weekend: bool,
    pub teams_remaining: HashSet<usize>,
    pub games: Vec<Game>,
}

#[derive(Clone)]
pub struct ScheduleConfig {
    pub days: Vec<Day>,
    pub days_index: usize,
    pub remaining_games: Vec<Game>,
}

fn day_is_valid_rec(
    games_remaining_map: &Vec<(usize, Vec<usize>)>,
    idx: usize,
    teams_remaining: HashSet<usize>,
) -> bool {
    if teams_remaining.is_empty() {
        return true;
    }
    if idx == games_remaining_map.len() {
        return false;
    }
    let ti0 = games_remaining_map[idx].0;
    if teams_remaining.contains(&ti0) {
        for ti1 in games_remaining_map[idx].1.iter() {
            if teams_remaining.contains(ti1) {
                let mut new_teams_remaining = teams_remaining.clone();
                assert!(new_teams_remaining.remove(&ti0));
                assert!(new_teams_remaining.remove(ti1));
                if day_is_valid_rec(games_remaining_map, idx + 1, new_teams_remaining) {
                    return true;
                }
            }
        }
    }
    false
}

impl Day {
    fn is_valid(&self, sorted_deduped_remaining_games: &Vec<Game>) -> bool {
        let mut remaining_games_map: Vec<(usize, Vec<usize>)> = Vec::new();
        let mut ti0: usize = 0;
        let mut ti1_vals: Vec<usize> = Vec::new();
        for g in sorted_deduped_remaining_games.iter() {
            if self.teams_remaining.contains(&g.ti0) && self.teams_remaining.contains(&g.ti1) {
                if ti0 == g.ti0 {
                    ti1_vals.push(g.ti1);
                } else {
                    if !ti1_vals.is_empty() {
                        remaining_games_map.push((ti0, ti1_vals.clone()));
                        ti1_vals.clear();
                    }
                    ti0 = g.ti0;
                    ti1_vals.push(g.ti1);
                }
            }
        }
        if !ti1_vals.is_empty() {
            remaining_games_map.push((ti0, ti1_vals));
        }
        return day_is_valid_rec(&remaining_games_map, 0, self.teams_remaining.clone());
    }
}

impl ScheduleConfig {
    pub fn new(days: Vec<Day>, remaining_games: Vec<Game>) -> ScheduleConfig {
        ScheduleConfig {
            days,
            days_index: 0,
            remaining_games,
        }
    }

    fn from_next_day(old_config: &ScheduleConfig) -> ScheduleConfig {
        ScheduleConfig {
            days: old_config.days.to_vec(),
            days_index: old_config.days_index + 1,
            remaining_games: old_config.remaining_games.to_vec(),
        }
    }
}

impl fmt::Debug for ScheduleConfig {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "ScheduleConfig:\n")?;
        for (i, day) in self.days.iter().enumerate() {
            write!(formatter, "{}. {}: ", i, day.date)?;
            let mut day_distance = 0;
            for g in day.games.iter() {
                day_distance += g.distance;
                write!(formatter, "({}, {}), ", g.ti0, g.ti1)?;
            }
            write!(formatter, "dist: {}, remaining teams: ", day_distance)?;
            let mut v = Vec::from_iter(day.teams_remaining.clone());
            v.sort();
            write!(formatter, "{:?}, ", v)?;
            write!(formatter, "\n")?;
            // write!(formatter, "remaining games: {:?}\n", self.remaining_games)?;
        }
        Ok(())
    }
}

impl Config for ScheduleConfig {
    fn successors(&self) -> Vec<ScheduleConfig> {
        if self.days[self.days_index].teams_remaining.is_empty() {
            return vec![ScheduleConfig::from_next_day(&self)];
        }

        let mut successors = Vec::with_capacity(self.remaining_games.len());

        for (gi, game) in self.remaining_games.iter().enumerate() {
            if self.days[self.days_index]
                .teams_remaining
                .contains(&game.ti0)
                && self.days[self.days_index]
                    .teams_remaining
                    .contains(&game.ti1)
            {
                let mut new_remaining_games = self.remaining_games.to_vec();
                let next_game = new_remaining_games.remove(gi);
                let mut new_days = self.days.clone();
                new_days[self.days_index]
                    .teams_remaining
                    .remove(&next_game.ti0);
                new_days[self.days_index]
                    .teams_remaining
                    .remove(&next_game.ti1);
                new_days[self.days_index].games.push(next_game);

                if new_remaining_games.len() < new_days[self.days_index].teams_remaining.len() / 2 {
                    break;
                }
                successors.push(ScheduleConfig {
                    days: new_days,
                    days_index: self.days_index,
                    remaining_games: new_remaining_games,
                });
            }
        }
        successors
    }

    fn is_valid(&self) -> bool {
        let mut sorted_deduped_remaining_games = self.remaining_games.clone();
        sorted_deduped_remaining_games.sort();
        sorted_deduped_remaining_games.dedup();
        return self.days[self.days_index + 1..]
            .iter()
            .all(|d| d.is_valid(&sorted_deduped_remaining_games));
    }
    fn is_goal(&self) -> bool {
        //self.days_index == self.days.len()
        self.days_index == 16
    }
}

mod test {

    #[test]
    fn day_is_valid() {
        use super::*;

        let mut teams_remaining = HashSet::new();
        teams_remaining.insert(0);
        teams_remaining.insert(1);
        teams_remaining.insert(2);
        teams_remaining.insert(3);

        let day = Day {
            date: "".to_string(),
            is_weekend: false,
            teams_remaining,
            games: vec![],
        };

        assert!(!day.is_valid(&vec![Game {
            ti0: 0,
            ti1: 1,
            distance: 1,
        },]));

        assert!(day.is_valid(&vec![
            Game {
                ti0: 0,
                ti1: 1,
                distance: 1,
            },
            Game {
                ti0: 2,
                ti1: 3,
                distance: 1,
            },
        ]));

        assert!(day.is_valid(&vec![
            Game {
                ti0: 0,
                ti1: 1,
                distance: 1,
            },
            Game {
                ti0: 0,
                ti1: 2,
                distance: 1,
            },
            Game {
                ti0: 2,
                ti1: 3,
                distance: 1,
            },
        ]));
    }
}
