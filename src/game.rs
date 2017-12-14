use ::math::{Vec2D};
use ::ship::{Ship};
pub use ::input::{Inputs, InputIndex};
use ::geom::{test_circle_point, test_circle_triangle};

pub struct Config {
    pub acceleration: f64,
    pub speed_limit: f64,
    pub drag: f64,

    pub angular_accel: f64,
    pub angular_limit: f64,
    pub angular_drag: f64,

    pub bullet_interval: f64,
    pub bullet_speed: f64,
    pub bullet_lifetime: f64,

    pub delta_t: f64,

    pub asteroid_min_size: f64,

    pub field_size: Vec2D,
    pub key_binds: Vec<(u32, InputIndex)>,
}

const DEFAULT_KEYBINDS: &[(u32, InputIndex)] = &[
    (90, InputIndex::Shoot),    // Z
    (32, InputIndex::Shoot),    // Space
    (38, InputIndex::Forward),  // Up
    (40, InputIndex::Backward), // Down
    (37, InputIndex::Left),     // Left
    (39, InputIndex::Right),    // Right
];

impl Config {
    pub fn new() -> Config {
        Config {
            acceleration: 250.0,
            speed_limit: 1000.0,
            drag: 0.000005,

            angular_accel: 30.0,
            angular_limit: 4.0,
            angular_drag: 8.0,

            bullet_interval: 0.15,
            bullet_speed: 400.0,
            bullet_lifetime: 1.7,

            asteroid_min_size: 10.0,

            delta_t: 1.0 / 60.0,
            field_size: Vec2D { x: 1280.0, y: 720.0 },
            key_binds: DEFAULT_KEYBINDS.to_vec(),
        }
    }

    pub fn lookup_input_key(&self, code: u32) -> Option<InputIndex> {
        for &(key, val) in self.key_binds.iter() {
            if key == code {
                return Some(val);
            }
        }
        None
    }
}

#[derive(PartialEq)]
pub enum BulletSource {
    Player,
    UFO,
}

pub struct Bullet {
    pub pos: Vec2D,
    pub speed: Vec2D,
    pub lifetime: u64,
    pub dead: bool,
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
            lifetime: game.tick + (config.bullet_lifetime / config.delta_t) as u64,
            dead: false,
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
    pub dead: bool,
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

pub struct UFO {}
impl UFO {
    pub fn tick(&mut self) {}
}

pub struct Game {
    pub ship: Ship,
    pub ufo: Option<UFO>,
    pub ufo_spawn_tick: u64,
    pub lives: usize,
    pub score: usize,
    pub tick: u64,
    pub next_bullet_tick: u64,
    pub bullets: Vec<Bullet>,
    pub asteroids: Vec<Asteroid>,
    pub inputs: Inputs,
    pub config: Config,
}

fn collide_asteroid_bullet(asteroid: &Asteroid, bullet: &Bullet) -> bool {
    test_circle_point(asteroid.pos, asteroid.size, bullet.pos)
}

fn collide_asteroid_ship(asteroid: &Asteroid, ship: &Ship) -> bool {
    let tr: Vec<Vec2D> = [
        Vec2D { x: 10.0, y: 0.0 },
        Vec2D { x: -10.0, y: -5.0 },
        Vec2D { x: -10.0, y: 5.0 },
    ].iter().map(|p| ship.pos + p.scale(2.2).rotate(ship.angle)).collect();
    test_circle_triangle(asteroid.pos, asteroid.size, tr[0], tr[1], tr[2])
}

impl Game {
    pub fn new() -> Game {
        let mut game = Game {
            tick: 0,
            lives: 4,
            score: 0,
            next_bullet_tick: 0,
            ship: Ship::new(),
            ufo: None,
            ufo_spawn_tick: ::std::u64::MAX,
            bullets: Vec::new(),
            asteroids: Vec::new(),
            inputs: Inputs::new(),
            config: Config::new(),
        };
        game.asteroids.push(Asteroid {
            pos: Vec2D { x: 100.0, y: 100.0 },
            speed: Vec2D { x: 30.0, y: 70.0 },
            angle: 0.0,
            angle_speed: 0.5,
            size: 50.0,
            style: 5,
            dead: false,
        });
        game.asteroids.push(Asteroid {
            pos: Vec2D { x: 800.0, y: 100.0 },
            speed: Vec2D { x: 70.0, y: 30.0 },
            angle: 0.0,
            angle_speed: 0.7,
            size: 50.0,
            style: 5,
            dead: false,
        });
        game.ship.pos = game.config.field_size.scale(0.5);
        game.ship.angle = ::std::f64::consts::PI * -0.5;
        game
    }

    pub fn reset(&mut self) {
        *self = Game::new();
    }

    pub fn tick(&mut self) {
        self.tick += 1;
        if self.ship.dead {
            let ship = &mut self.ship;
            if self.inputs.is_down(InputIndex::Shoot) {
                ship.speed = Vec2D::zero();
                ship.pos = self.config.field_size.scale(0.5);
                ship.angle = ::std::f64::consts::PI * -0.5;
                ship.dead = false;
            }
        }

        // decay bullets
        if self.bullets.len() > 0 && self.bullets[0].lifetime == self.tick {
            self.bullets.remove(0);
        }

        {
            // move entities
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
            for ufo in self.ufo.iter_mut() {
                ufo.tick();
            }
        }

        if !self.ship.dead {
            // shoot
            let inputs = &self.inputs;
            let config = &self.config;
            if inputs.been_pressed(InputIndex::Shoot) && self.tick >= self.next_bullet_tick {
                self.next_bullet_tick = self.tick + (config.bullet_interval / config.delta_t) as u64;
                let bullet = Bullet::new(self, BulletSource::Player);
                self.bullets.push(bullet);
            }
        }


        // COLLISIONS
        {
            // collide asteroids with bullets
            let asteroids = &mut self.asteroids;
            let bullets = &mut self.bullets;
            let config = &self.config;
            let mut new_asteroids = Vec::new();
            for asteroid in asteroids.iter_mut() {
                for bullet in bullets.iter_mut().filter(|bullet| bullet.source == BulletSource::Player) {
                    // bullets and asteroids may collide multiple times
                    // the alternative is having order-dependent logic
                    if collide_asteroid_bullet(asteroid, bullet) {
                        if !asteroid.dead {
                            new_asteroids.append(&mut asteroid.split_off(&config));
                        }
                        asteroid.dead = true;
                        bullet.dead = true;
                    }
                }
            }

            bullets.retain(|bullet| !bullet.dead);
            asteroids.retain(|asteroid| !asteroid.dead);

            asteroids.append(&mut new_asteroids);
        }

        {
            // collide bullets with ship & ufo
            let bullets = &mut self.bullets;
            let ship = &mut self.ship;
            let ufo = &mut self.ufo;
            let collide_ship_bullet = |_: &Ship, _: &Bullet| false;
            let collide_ufo_bullet = |_: &UFO, _: &Bullet| false;
            for bullet in bullets.iter_mut() {
                match bullet.source {
                    BulletSource::UFO => {
                        if !ship.dead && collide_ship_bullet(ship, bullet) {
                            ship.dead = true;
                            bullet.dead = true;
                        }
                    },
                    BulletSource::Player => {
                        if ufo.as_ref().map_or(false, |ufo| collide_ufo_bullet(ufo, bullet)) {
                            *ufo = None;
                            bullet.dead = true;
                        }
                    },
                }
            }

            bullets.retain(|bullet| !bullet.dead);
        }


        {
            // collide asteroids with ship & ufo
            let asteroids = &mut self.asteroids;
            let mut new_asteroids = Vec::new();
            let config = &self.config;
            let ship = &mut self.ship;
            let ufo = &mut self.ufo;

            let collide_asteroid_ufo = |_: &Asteroid, _: &UFO| false;

            for asteroid in asteroids.iter_mut() {
                let mut collided = false;

                if !ship.dead && collide_asteroid_ship(asteroid, ship) {
                    ship.dead = true;
                    collided = true;
                }

                if ufo.as_ref().map_or(false, |ufo| collide_asteroid_ufo(asteroid, ufo)) {
                    *ufo = None;
                    collided = true;
                }

                if collided {
                    if !asteroid.dead {
                        new_asteroids.append(&mut asteroid.split_off(&config));
                        asteroid.dead = true;
                    }
                }
            }

            asteroids.retain(|asteroid| !asteroid.dead);
            asteroids.append(&mut new_asteroids);
        }

        {
            let collide_ship_ufo = |_, _| false;
            if collide_ship_ufo(&self.ship, &self.ufo) {
            }
        }
        // END COLLISIONS

        // forget pressed inputs
        self.inputs.tick();
    }
}
