use ::game::{Game, Bullet, Asteroid, InputIndex};
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
    let config = &game.config;
    let inputs = &game.inputs;
    let offset_x = if ship.pos.x * 2.0 < config.field_size.x { 1.0 } else { -1.0 } * config.field_size.x;
    let offset_y = if ship.pos.y * 2.0 < config.field_size.y { 1.0 } else { -1.0 } * config.field_size.y;
    let offsets = [
        Vec2D { x: 0.0, y: 0.0 },
        Vec2D { x: offset_x, y: 0.0 },
        Vec2D { x: 0.0, y: offset_y },
        Vec2D { x: offset_x, y: offset_y },
    ];
    for offset in offsets.iter() {
        for (i, p) in SHIP_POINTS.iter().enumerate() {
            let c = if i == 0 { 'M' } else { 'L' };
            let p_c = p.scale(2.0).rotate(ship.angle) + *offset + ship.pos;
            write!(buf, "{}{:.2} {:.2} ", c, p_c.x, p_c.y)
                .expect("could not write string?");
        }
        if inputs.is_down(InputIndex::Forward) || inputs.is_down(InputIndex::Backward) {
            for (i, p) in FLARE.iter().enumerate() {
                let c = if i == 0 { 'M' } else { 'L' };
                let p_c = p.scale(2.0).rotate(ship.angle) + *offset + ship.pos;
                write!(buf, "{}{:.2} {:.2} ", c, p_c.x, p_c.y)
                    .expect("could not write string?");
            }
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

pub fn render_game(buf: &mut String, game: &Game) {
    render_ship(buf, game);
    for bullet in game.bullets.iter() {
        render_bullet(buf, bullet);
    }
    for asteroid in game.asteroids.iter() {
        render_asteroid(buf, asteroid);
    }
}
