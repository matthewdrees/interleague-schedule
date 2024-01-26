use backtrack::Config;
use std::collections::HashSet;
use std::fmt;

#[derive(Copy, Clone, Debug)]
pub struct Game {
    pub ti0: usize,
    pub ti1: usize,
    pub distance: i32,
}

#[derive(Clone)]
pub struct Day {
    pub date: String,
    pub is_weekend: bool,
    pub teams_playing: HashSet<usize>,
    pub games: Vec<Game>,
}

#[derive(Clone)]
pub struct ScheduleConfig {
    pub days: Vec<Day>,
    pub days_index: usize,
    pub remaining_games: Vec<Game>,
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
            let mut v = Vec::from_iter(day.teams_playing.clone());
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
        if self.days[self.days_index].teams_playing.is_empty() {
            return vec![ScheduleConfig::from_next_day(&self)];
        }

        let mut successors = Vec::with_capacity(self.remaining_games.len());

        for (gi, game) in self.remaining_games.iter().enumerate() {
            if self.days[self.days_index].teams_playing.contains(&game.ti0)
                && self.days[self.days_index].teams_playing.contains(&game.ti1)
            {
                let mut new_remaining_games = self.remaining_games.to_vec();
                let next_game = new_remaining_games.remove(gi);
                let mut new_days = self.days.clone();
                new_days[self.days_index]
                    .teams_playing
                    .remove(&next_game.ti0);
                new_days[self.days_index]
                    .teams_playing
                    .remove(&next_game.ti1);
                new_days[self.days_index].games.push(next_game);

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
        true
    }
    fn is_goal(&self) -> bool {
        self.days_index == self.days.len()
    }
}

// mod test {

//     #[test]
//     fn is_valid() {
//         use super::*;
//         let config = ScheduleConfig::new(5);

//         assert!(config.is_valid(), "starting config is valid");
//     }
// }
