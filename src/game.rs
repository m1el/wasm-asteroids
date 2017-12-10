use ::math::{Vec2D};
use ::ship::{Ship};

#[derive(Debug)]
pub struct Inputs {
    pub shoot: bool,
    pub forward: bool,
    pub backward: bool,
    pub left: bool,
    pub right: bool,
}

impl Inputs {
    pub fn new() -> Inputs {
        Inputs {
            shoot: false,
            forward: false,
            backward: false,
            left: false,
            right: false,
        }
    }
}

impl Inputs {
    pub fn key_down(&mut self, code: u32, _config: &Config) {
        match code {
            90 | 32 => self.shoot = true,
            38 => self.forward = true,
            40 => self.backward = true,
            37 => self.left = true,
            39 => self.right = true,
            _ => {},
        }
    }
    pub fn key_up(&mut self, code: u32, _config: &Config) {
        match code {
            90 | 32 => self.shoot = false,
            38 => self.forward = false,
            40 => self.backward = false,
            37 => self.left = false,
            39 => self.right = false,
            _ => {},
        }
    }
}

#[derive(Debug)]
pub struct Config {
    pub acceleration: f64,
    pub speed_limit: f64,
    pub drag: f64,

    pub angular_accel: f64,
    pub angular_limit: f64,
    pub angular_drag: f64,

    pub bullet_interval: u64,
    pub bullet_speed: f64,
    pub bullet_lifetime: u64,

    pub delta_t: f64,

    pub asteroid_min_size: f64,

    pub field_size: Vec2D,
}

pub enum BulletSource {
    Player,
    UFO,
}

pub struct Bullet {
    pub pos: Vec2D,
    pub speed: Vec2D,
    pub lifetime: u64,
    pub source: BulletSource,
}

impl Bullet {
    pub fn tick(&mut self, config: &Config) {
        self.pos += self.speed.scale(config.delta_t);
        self.pos.clip(&config.field_size);
    }
}

impl Bullet {
    pub fn new(game: &Game, source: BulletSource) -> Bullet {
        let ship = &game.ship;
        let config = &game.config;
        let direction = Vec2D::one().rotate(ship.angle);
        Bullet {
            pos: ship.pos + direction.scale(20.0),
            speed: direction.scale(config.bullet_speed),
            lifetime: game.tick + config.bullet_lifetime,
            source: source,
        }
    }
}

#[derive(Clone)]
pub struct Asteroid {
    pub pos: Vec2D,
    pub speed: Vec2D,
    pub angle: f64,
    pub angle_speed: f64,
    pub size: f64,
    pub style: usize,
}

impl Asteroid {
    pub fn tick(&mut self, config: &Config) {
        self.pos += self.speed.scale(config.delta_t);
        self.pos.clip(&config.field_size);
        self.angle += self.angle_speed * config.delta_t;
    }

    pub fn split_off(&self, config: &Config) -> Vec<Asteroid> {
        let mut rv = Vec::new();
        if self.size > config.asteroid_min_size {
            let mut copy0 = self.clone();
            let mut copy1 = self.clone();
            let offset = Vec2D::one().rotate(self.angle).scale(self.size / 2.0);
            copy0.size /= 2.0;
            copy1.size /= 2.0;
            copy0.pos += offset;
            copy1.pos -= offset;
            copy0.speed += offset;
            copy1.speed -= offset;
            rv.push(copy0);
            rv.push(copy1);
        }
        rv
    }
}

pub struct Game {
    pub ship: Ship,
    pub tick: u64,
    pub bullet_tick: u64,
    pub bullets: Vec<Bullet>,
    pub asteroids: Vec<Asteroid>,
    pub inputs: Inputs,
    pub config: Config,
}

fn collide_asteroid_bullet(asteroid: &Asteroid, bullet: &Bullet) -> bool {
    let collision_distance = asteroid.size;
    let delta = bullet.pos - asteroid.pos;
    return collision_distance * collision_distance > delta.dot(&delta);
}

impl Game {
    pub fn new() -> Game {
        let mut game = Game {
            tick: 0,
            bullet_tick: 0,
            ship: Ship::new(),
            bullets: Vec::new(),
            asteroids: Vec::new(),
            inputs: Inputs::new(),
            config: Config {
                acceleration: 250.0,
                speed_limit: 1000.0,
                drag: 0.000005,

                angular_accel: 30.0,
                angular_limit: 4.0,
                angular_drag: 8.0,

                bullet_interval: 10,
                bullet_speed: 400.0,
                bullet_lifetime: 100,

                asteroid_min_size: 10.0,

                delta_t: 1.0 / 60.0,
                field_size: Vec2D { x: 1280.0, y: 720.0 },
            }
        };
        game.asteroids.push(Asteroid {
            pos: Vec2D { x: 100.0, y: 100.0 },
            speed: Vec2D { x: 30.0, y: 70.0 },
            angle: 0.0,
            angle_speed: 0.5,
            size: 50.0,
            style: 5,
        });
        game.asteroids.push(Asteroid {
            pos: Vec2D { x: 800.0, y: 100.0 },
            speed: Vec2D { x: 70.0, y: 30.0 },
            angle: 0.0,
            angle_speed: 0.7,
            size: 50.0,
            style: 5,
        });
        game.ship.pos = game.config.field_size.scale(0.5);
        game.ship.angle = ::std::f64::consts::PI * -0.5;
        game
    }

    pub fn tick(&mut self) {
        self.tick += 1;

        if self.bullets.len() > 0 && self.bullets[0].lifetime == self.tick {
            self.bullets.remove(0);
        }

        {
            let inputs = &self.inputs;
            let config = &self.config;
            if inputs.shoot && self.tick >= self.bullet_tick {
                self.bullet_tick = self.tick + config.bullet_interval;
                let bullet = Bullet::new(self, BulletSource::Player);
                self.bullets.push(bullet);
            }
        }

        {
            let asteroids = &mut self.asteroids;
            let config = &self.config;
            let mut new_asteroids = Vec::new();
            self.bullets.retain(|bullet| {
                let mut retain = true;
                asteroids.retain(|asteroid| {
                    if !retain {
                        return true;
                    }
                    if !collide_asteroid_bullet(asteroid, bullet) {
                        return true;
                    }

                    let mut shards = asteroid.split_off(&config);
                    new_asteroids.append(&mut shards);
                    retain = false;
                    return false;
                });
                retain
            });
            asteroids.append(&mut new_asteroids);
        }

        {
            let inputs = &self.inputs;
            let config = &self.config;
            let ship = &mut self.ship;
            ship.tick(inputs, config);
            for asteroid in self.asteroids.iter_mut() {
                asteroid.tick(config);
            }
            for bullet in self.bullets.iter_mut() {
                bullet.tick(config);
            }
        }
    }
}
