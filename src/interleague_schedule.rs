use backtrack::Config;
use std::fmt;

pub struct ScheduleConfig {
    num_games: i32,
    game_num: i32,
}

impl ScheduleConfig {
    pub fn new(num_games: i32) -> ScheduleConfig {
        ScheduleConfig {
            num_games,
            game_num: 0,
        }
    }

    fn from(old_config: &ScheduleConfig, game_num: i32) -> ScheduleConfig {
        ScheduleConfig {
            num_games: old_config.num_games,
            game_num,
        }
    }
}

impl Config for ScheduleConfig {
    fn successors(&self) -> Vec<ScheduleConfig> {
        let mut successors = Vec::with_capacity(self.num_games as usize);
        for i in self.game_num + 1..self.num_games {
            successors.push(ScheduleConfig::from(&self, i));
        }
        successors
    }
    fn is_valid(&self) -> bool {
        true
    }
    fn is_goal(&self) -> bool {
        return self.num_games == self.game_num;
    }
}

impl fmt::Display for ScheduleConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "num_games: {}, game_num: {}",
            self.num_games, self.game_num
        )?;
        Ok(())
    }
}

mod test {

    #[test]
    fn is_valid() {
        use super::*;
        let config = ScheduleConfig::new(5);

        assert!(config.is_valid(), "starting config is valid");
    }
}
