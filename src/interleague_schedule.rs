// use backtrack::Config;
use std::collections::HashSet;
use std::fmt;

pub struct Game {
    pub ti0: usize,
    pub ti1: usize,
    pub distance: i32,
}
pub struct Day {
    pub date: String,
    pub is_weekend: bool,
    pub teams_playing: HashSet<usize>,
    pub games: Vec<Game>,
}

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

    // fn from(old_config: &ScheduleConfig, days_index: usize) -> ScheduleConfig {
    //     ScheduleConfig {
    //         days: old_config.days,
    //         days_index: old_config.days_index,
    //         remaining_games: old_config.remaining_games,
    //     }
    // }
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
        }
        Ok(())
    }
}

// impl Config for ScheduleConfig {
//     fn successors(&self) -> Vec<ScheduleConfig> {
//         let mut successors = Vec::new();
//         if self.days[self.days_index].teams_playing.is_empty() {
//             successors.push(ScheduleConfig::from(&self, self.days_index + 1));
//             return successors;
//         }

//         // for ti0 in self.days[self.days_index].teams_playing {
//         //     for ti1 in self.teams[ti0].teams_against {
//         //         successors.push(ScheduleConfig::from(&self, self.days_index));
//         //     }
//         // }
//         // for ti0 in self.days[self.days_index] + 1..self.num_games {}
//         successors
//     }
//     fn is_valid(&self) -> bool {
//         self.days.len() == self.days_index
//     }
//     fn is_goal(&self) -> bool {
//         self.days.len() == self.days_index
//     }
// }

// mod test {

//     #[test]
//     fn is_valid() {
//         use super::*;
//         let config = ScheduleConfig::new(5);

//         assert!(config.is_valid(), "starting config is valid");
//     }
// }
