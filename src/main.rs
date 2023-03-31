use nannou::prelude::*;
mod point;
use crate::point::Circle;
use rand::Rng;
use std::time::Instant;

struct Model {
    _window: window::Id,
    point_vec: Vec<Circle>,
    last_time: Instant,
    closest_point: usize,
    once: bool,
}

const POINT_NUMB: u32 = 50;

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();
    let mut points = vec![];
    let mut rng = rand::thread_rng();
    for i in 0..POINT_NUMB {
        let mass = rng.gen_range(10..100) as f32;
        let new_point = Circle {
            pos: vec2(
                rng.gen_range(-600..600) as f32,
                rng.gen_range(-400..400) as f32,
            ),
            prev_vel: Vec2::ZERO,
            velocity: Vec2::ZERO,//vec2(rng.gen_range(-100..100) as f32,rng.gen_range(-100..100) as f32,),
            kinetic_energy: 0.0,
            potential_energy: 0.0,
            force: Vec2::ZERO,
            is_kinematic: false,
            mass,
            radius: mass.sqrt(),
            id: i,
        };
        points.push(new_point);
    }

    let last_time = Instant::now();
    let once = false;
    let closest_point:usize = 0;
    Model {
        _window,
        point_vec: points,
        last_time,
        once,
        closest_point,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {

    let now = Instant::now();
    let dt = now.duration_since(model.last_time).secs();
    model.last_time = Instant::now();
    
    
    let mut clone_points = model.point_vec.clone();
    // for planet in &mut model.point_vec {
    // }
    for i in 0..model.point_vec.len() {
        Circle::add_gravity_to_force(&mut model.point_vec[i], &clone_points);
        let mut col = model.point_vec[i].check_collisions(&clone_points, true);

        if !col.is_empty() && col.len() > 0 {
            for j in &mut col {
                let new_vel = model.point_vec[i].calculate_new_velocity_after_col(*j);
                // apply velocity to point
                // make them not touch each other
                for _ in 0..100{
                    let local_col = model.point_vec[i].check_collision(*j, true);
                    let local_new_vel = match local_col {
                        Some(x) => model.point_vec[i].calculate_correction(&x),
                        None => Vec2::ZERO,
                    };
                    if local_col.is_some() {
                        model.point_vec[i].pos += local_new_vel/2.0;
                        local_col.unwrap().pos -= local_new_vel/2.0;
                    } else {
                        break;
                    }
                }
                model.point_vec[i].velocity = new_vel.0;
                j.velocity = new_vel.1;
                model.point_vec[i].update(dt);
                j.update(dt);
            }
        }
        model.point_vec[i].update(dt);
        // clone_points[i] = model.point_vec[i].clone();
    }
    if app.mouse.buttons.left().is_down() && !model.once {
        let mut closest_point_i: usize = 0;
        model.once = true;
        let mut closest: f32 = f32::MAX;
        for point in 0..model.point_vec.len() {
            let dist = app.mouse.position().distance(model.point_vec[point].pos);

            if dist < closest {
                closest = dist;
                closest_point_i = point.clone();
            }
        }
        model.closest_point = closest_point_i;
    } else if model.once && app.mouse.buttons.left().is_up() {
        model.once = false;
    }
    if app.mouse.buttons.left().is_down() {
        model.point_vec[model.closest_point].force = -100.0 * (-app.mouse.position() + (model.point_vec[model.closest_point].pos));
        model.point_vec[model.closest_point].update(dt);
        model.point_vec[model.closest_point].velocity /= 1.1;
        model.point_vec[model.closest_point].update(dt);
    }
    calculate_total_energy(&model);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    for i in &model.point_vec {
        draw.ellipse().radius(i.radius).color(BLACK).xy(i.pos);
    }

    draw.background().color(WHITE);

    draw.to_frame(app, &frame).unwrap();
}

fn calculate_total_energy(model: &Model) {
    let mut total_kinetic_energy = 0.0;
    let mut total_potential_energy = 0.0;
    for i in &model.point_vec {
        total_kinetic_energy += i.kinetic_energy;
        total_potential_energy += i.potential_energy;

        // -G * M * m / r,
        // total_potential_energy += 0.1 *
    }
    println!(
        "total kinetic energy {} total potential energy: {}, total energy: {}",
        total_kinetic_energy, total_potential_energy, total_kinetic_energy + total_potential_energy
    );
}
