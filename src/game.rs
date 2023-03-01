use std::time::{Instant, Duration};

use legion::*;

use crate::config::GameConfig;

pub mod entity;
pub mod update;

pub struct GameTime {
    pub delta: f32
}

pub struct Game {
    config: GameConfig,
    world: World,
    resources: Resources,
    schedule: Schedule
}

unsafe impl Send for Game {}
unsafe impl Sync for Game {}

impl Game {
    pub fn new(config: GameConfig) -> Self {
        let mut game = Self {
            config, 
            world: World::default(),
            resources: Resources::default(),
            schedule: Schedule::builder()
                .add_system(self::update::update_positions_system())
                .build()
        };

        game.resources.insert(GameTime { delta: 0.1 });

        game
    }

    pub fn update(&mut self) {
        let mut last_tick = Instant::now();
        let tick_dur = Duration::from_secs(1) / self.config.tps;
        loop {
            let now = Instant::now();

            let elapsed = now - last_tick;

            if elapsed >= tick_dur {
                let mut game_time = self.resources.get_mut::<GameTime>().unwrap();
                game_time.delta = elapsed.as_secs_f32();

                drop(game_time);

                self.tick();
                last_tick += tick_dur;
            }
        }
    }

    fn tick(&mut self) {
        self.schedule.execute(&mut self.world, &mut self.resources);
    }
}
