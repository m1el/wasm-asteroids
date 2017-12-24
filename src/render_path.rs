use ::game::{Game, Bullet, Asteroid, Explosion, InputIndex};
use ::math::{Vec2D};
use std::fmt::Write;

const SHIP_POINTS: &[Vec2D] = &[
    Vec2D { x: 10.0, y: 0.0 },
    Vec2D { x: -10.0, y: -5.0 },
    Vec2D { x: -8.0, y: -2.5 },
    Vec2D { x: -8.0, y: 2.5 },
    Vec2D { x: -10.0, y: 5.0 },
    Vec2D { x: 10.0, y: 0.0 },
];

const FLARE: &[Vec2D] = &[
    Vec2D { x: -8.0, y: 1.5 },
    Vec2D { x: -12.0, y: 0.0 },
    Vec2D { x: -8.0, y: -1.5 },
];

fn render_ship(buf: &mut String, game: &Game) {
    let ship = &game.ship;
    if ship.dead { return; }
    let inputs = &game.inputs;
    for (i, p) in SHIP_POINTS.iter().enumerate() {
        let c = if i == 0 { 'M' } else { 'L' };
        let p_c = p.scale(2.0).rotate(ship.angle) + ship.pos;
        write!(buf, "{}{:.2} {:.2} ", c, p_c.x, p_c.y)
            .expect("could not write string?");
    }
    if inputs.is_down(InputIndex::Forward) || inputs.is_down(InputIndex::Backward) {
        for (i, p) in FLARE.iter().enumerate() {
            let c = if i == 0 { 'M' } else { 'L' };
            let p_c = p.scale(2.0).rotate(ship.angle) + ship.pos;
            write!(buf, "{}{:.2} {:.2} ", c, p_c.x, p_c.y)
                .expect("could not write string?");
        }
    }
}

fn render_bullet(buf: &mut String, bullet: &Bullet) {
    let offset = Vec2D::zero();
    let start = bullet.pos + offset;
    let end = bullet.pos + bullet.speed.normalize().scale(5.0);
    write!(buf, "M{:.2} {:.2} L{:.2} {:.2} ", start.x, start.y, end.x, end.y)
        .expect("could not write string?");
}

fn render_asteroid(buf: &mut String, asteroid: &Asteroid) {
    let offset = Vec2D::zero();
    let mid = asteroid.pos + offset;
    let cnt = asteroid.style;
    let angle = ::std::f64::consts::PI * 2.0 / (cnt as f64);
    let one = Vec2D::one().scale(asteroid.size).rotate(asteroid.angle);
    for i in 0..(cnt+1) {
        let c = if i == 0 { 'M' } else { 'L' };
        let p = mid + one.rotate(angle * (i as f64));
        write!(buf, "{}{:.2} {:.2}", c, p.x, p.y)
            .expect("could not write string?");
    }
}

fn render_lives(buf: &mut String, lives: u64) {
    const LIFE_STEP: f64 = 40.0;
    const UP_ANGLE: f64 = ::std::f64::consts::PI * -0.5;
    for l in 0..lives {
        let y = -50.0;
        let x = ((l + 1) as f64) * LIFE_STEP;
        for (i, p) in SHIP_POINTS.iter().enumerate() {
            let c = if i == 0 { 'M' } else { 'L' };
            let p_c = p.scale(2.0).rotate(UP_ANGLE) + Vec2D { x, y };
            write!(buf, "{}{:.2} {:.2} ", c, p_c.x, p_c.y)
                .expect("could not write string?");
        }
    }
}

fn render_explosion(buf: &mut String, explosion: &Explosion, tick: u64) {
    const EXPLOSION_RADIUS: f64 = 30.0;
    const EXPLOSION_PARTICLES: usize = 11;
    const EXPLOSION_PARTICLE_LENGTH: f64 = 10.0;
    let explosion_da = ::std::f64::consts::PI * 2.0 / (EXPLOSION_PARTICLES as f64);
    let state = ((tick - explosion.start_tick) as f64) / ((explosion.lifetime - explosion.start_tick) as f64);

    for i in 0..EXPLOSION_PARTICLES {
        let a = explosion_da * (i as f64);
        let dir = Vec2D::one().rotate(a);
        let start = dir.scale(state * EXPLOSION_RADIUS) + explosion.pos;
        let end = dir.scale(state * EXPLOSION_RADIUS + EXPLOSION_PARTICLE_LENGTH * (1.0 + state)) + explosion.pos;
        write!(buf, "M {:.2} {:.2} L {:.2} {:.2}", start.x, start.y, end.x, end.y)
            .expect("could not write string?");
    }
}

const VECTOR_DIGITS: &[&[Vec2D]] = &[
    // 0
    &[
        Vec2D { x: 1.0, y: 0.0 },
        Vec2D { x: 2.0, y: 1.0 },
        Vec2D { x: 2.0, y: 2.0 },
        Vec2D { x: 1.0, y: 3.0 },
        Vec2D { x: 0.0, y: 2.0 },
        Vec2D { x: 0.0, y: 1.0 },
        Vec2D { x: 1.0, y: 0.0 },
    ],
    // 1
    &[
        Vec2D { x: 0.0, y: 1.0 },
        Vec2D { x: 1.0, y: 0.0 },
        Vec2D { x: 1.0, y: 3.0 },
    ],
    // 2
    &[
        Vec2D { x: 0.0, y: 1.0 },
        Vec2D { x: 1.0, y: 0.0 },
        Vec2D { x: 2.0, y: 1.0 },
        Vec2D { x: 0.0, y: 3.0 },
        Vec2D { x: 2.0, y: 3.0 },
    ],
    // 3
    &[
        Vec2D { x: 0.0, y: 0.0 },
        Vec2D { x: 2.0, y: 0.0 },
        Vec2D { x: 1.0, y: 1.0 },
        Vec2D { x: 2.0, y: 2.0 },
        Vec2D { x: 1.0, y: 3.0 },
        Vec2D { x: 0.0, y: 3.0 },
    ],
    // 4
    &[
        Vec2D { x: 1.0, y: 0.0 },
        Vec2D { x: 0.0, y: 2.0 },
        Vec2D { x: 2.0, y: 2.0 },
        Vec2D { x: 2.0, y: 0.0 },
        Vec2D { x: 2.0, y: 3.0 },
    ],
    // 5
    &[
        Vec2D { x: 2.0, y: 0.0 },
        Vec2D { x: 0.0, y: 0.0 },
        Vec2D { x: 0.0, y: 1.0 },
        Vec2D { x: 1.0, y: 1.0 },
        Vec2D { x: 2.0, y: 2.0 },
        Vec2D { x: 1.0, y: 3.0 },
        Vec2D { x: 0.0, y: 3.0 },
    ],
    // 6
    &[
        Vec2D { x: 2.0, y: 0.0 },
        Vec2D { x: 1.0, y: 0.0 },
        Vec2D { x: 0.0, y: 1.0 },
        Vec2D { x: 0.0, y: 2.0 },
        Vec2D { x: 1.0, y: 3.0 },
        Vec2D { x: 2.0, y: 2.0 },
        Vec2D { x: 1.0, y: 1.0 },
        Vec2D { x: 0.0, y: 1.0 },
    ],
    // 7
    &[
        Vec2D { x: 0.0, y: 0.0 },
        Vec2D { x: 2.0, y: 0.0 },
        Vec2D { x: 1.0, y: 3.0 },
    ],
    // 8
    &[
        Vec2D { x: 0.0, y: 0.0 },
        Vec2D { x: 2.0, y: 0.0 },
        Vec2D { x: 2.0, y: 1.0 },
        Vec2D { x: 0.0, y: 2.0 },
        Vec2D { x: 0.0, y: 3.0 },
        Vec2D { x: 2.0, y: 3.0 },
        Vec2D { x: 2.0, y: 2.0 },
        Vec2D { x: 0.0, y: 1.0 },
        Vec2D { x: 0.0, y: 0.0 },
    ],
    // 9
    &[
        Vec2D { x: 0.0, y: 3.0 },
        Vec2D { x: 1.0, y: 3.0 },
        Vec2D { x: 2.0, y: 2.0 },
        Vec2D { x: 2.0, y: 1.0 },
        Vec2D { x: 1.0, y: 0.0 },
        Vec2D { x: 0.0, y: 1.0 },
        Vec2D { x: 1.0, y: 2.0 },
        Vec2D { x: 2.0, y: 2.0 },
    ]
];

fn render_score(buf: &mut String, mut score: u64) {
    let mut digits = Vec::new();
    while score > 0 {
        digits.push(score % 10);
        score /= 10;
    }
    if digits.len() == 0 {
        digits.push(0);
    }
    const DIGIT_SCALE: f64 = 10.0;
    const DIGIT_STEP: f64 = -30.0;
    const DIGIT_RIGHTMOST: f64 = 1200.0;
    for (idx, d) in digits.iter().enumerate() {
        let digit = VECTOR_DIGITS[*d as usize];
        for (i, p) in digit.iter().enumerate() {
            let p = Vec2D { x: p.x, y: p.y }.scale(DIGIT_SCALE)
                    + Vec2D { x: DIGIT_RIGHTMOST + (idx as f64) * DIGIT_STEP, y: -DIGIT_SCALE * 5.0 };
            let c = if i == 0 { 'M' } else { 'L' };
            write!(buf, "{} {:.2} {:.2} ", c, p.x, p.y)
                .expect("could not write string?");
        }
    }
}

pub fn render_game(buf: &mut String, game: &Game) {
    render_lives(buf, game.lives);
    render_ship(buf, game);
    for bullet in game.bullets.iter() {
        render_bullet(buf, bullet);
    }
    for asteroid in game.asteroids.iter() {
        render_asteroid(buf, asteroid);
    }
    for explosion in game.explosions.iter() {
        render_explosion(buf, explosion, game.tick);
    }
    render_score(buf, game.score);
}
