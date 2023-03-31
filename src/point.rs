use std::ops::Mul;

use nannou::prelude::*;

// use crate::GRAVITY;

#[derive(Debug, Clone, Copy)]
pub struct Circle {
    pub pos: Vec2,
    pub prev_vel: Vec2,
    pub velocity: Vec2,
    pub force: Vec2,
    pub is_kinematic: bool,
    pub mass: f32,
    pub potential_energy: f32,
    pub kinetic_energy: f32,
    pub radius: f32,
    pub id: u32,
}

impl Circle {
    pub fn update(&mut self, dt: f64) {
        if self.is_kinematic || self.force.is_nan() {
            return;
        }
        self.verlet_integration(dt);
        self.kinetic_energy = 0.5 * self.mass * self.velocity.length() * self.velocity.length();
    }
    pub fn new() -> Circle {
        Circle {
            pos: vec2(0.0, 0.0),
            prev_vel: vec2(1.0, 0.0),
            velocity: vec2(0.0, 0.0),
            force: vec2(0.0, 0.0),
            kinetic_energy: 0.0,
            potential_energy: 0.0,
            is_kinematic: false,
            mass: 1.0,
            radius: 1.0,
            id: 0,
        }
    }
    fn verlet_integration(&mut self, dt: f64) {
        let prev_pos = self.velocity;
        let future_acceleration = self.velocity - self.prev_vel;
        // https://en.wikipedia.org/wiki/Verlet_integration#Algorithmic_representation
        self.pos = self.pos + self.velocity * dt as f32 + self.force * (dt * dt * 0.5) as f32;
        self.velocity = (self.velocity + (self.force + future_acceleration) * (dt * 0.5) as f32);
        self.prev_vel = prev_pos;
    }
    pub fn air_drag(&mut self) {
        self.force = self.force.mul(0.99);
        self.velocity = self.velocity.mul(0.99);
    }
    pub fn gravity(&mut self, dt: f64) {
        self.force.y = -100.0;
    }
}

impl Circle {
    fn gravity_to_place(&self, p2: Circle, g_const: f32) -> Vec2 {
        // ((m1*m2)/r^2)*G
        let distance_x = p2.pos.x - self.pos.x;
        let distance_y = p2.pos.y - self.pos.y;
        let distance = (distance_x * distance_x + distance_y * distance_y).sqrt();
        // straight light force
        let force = g_const * self.mass * p2.mass / distance * distance;

        // angle theta TBH im not sure why we need trigonometry for this but alright..
        let theta = (distance_y).atan2(distance_x);

        let force_x = theta.cos() * force;
        let force_y = theta.sin() * force;

        return vec2(force_x, force_y) / self.mass;
    }
    pub fn add_gravity_to_force(point: &mut Circle, planets: &Vec<Circle>) {
        let mut sum = Vec2::ZERO;
        let mut potential_energy = 0.0;
        for planet in planets {
            if planet.id == point.id {
                //point.gravity_to_place(*planet, 0.1).is_nan()){
                continue;
            }
            potential_energy += point.mass * point.gravity_to_place(*planet, 0.1).length() * point.pos.distance(planet.pos);
            sum += point.gravity_to_place(*planet, 0.1);
            // println!("{:?}", planet);
        }
        point.potential_energy = potential_energy / 10.0;
        point.force = sum;
    }
}
impl Circle{
    pub fn check_collisions(&self, planets: &Vec<Circle>, edge: bool) -> Vec<Circle>{
        // (R0 - R1)^2 <= (x0 - x1)^2 + (y0 - y1)^2 <= (R0 + R1)^2
        let mut circle_vec:Vec<Circle> = vec![];
        for planet in planets{
            if planet.id == self.id {continue;}
            let radius_diff =  (self.radius - planet.radius) * (self.radius - planet.radius);

            let radius_add = (self.radius + planet.radius) * (self.radius + planet.radius);
            let dist_sq = (self.pos.x - planet.pos.x) * (self.pos.x - planet.pos.x) + (self.pos.y - planet.pos.y) * (self.pos.y - planet.pos.y);
            if (radius_diff <= dist_sq && dist_sq <= radius_add && edge) || (radius_diff < dist_sq && dist_sq < radius_add && !edge) {
                circle_vec.push(*planet);
            }
        }
        return circle_vec;
    }
    pub fn check_collision(&self, planet: Circle, edge: bool) -> Option<Circle>{
        // (R0 - R1)^2 <= (x0 - x1)^2 + (y0 - y1)^2 <= (R0 + R1)^2
        if planet.id == self.id {return None};
        let radius_diff = (self.radius - planet.radius) * (self.radius - planet.radius);
        let radius_add = (self.radius + planet.radius) * (self.radius + planet.radius);
        let dist_sq = (self.pos.x - planet.pos.x) * (self.pos.x - planet.pos.x) + (self.pos.y - planet.pos.y) * (self.pos.y - planet.pos.y);
        if (radius_diff <= dist_sq && dist_sq <= radius_add && edge) || (radius_diff < dist_sq && dist_sq < radius_add && !edge) {
            return Some(planet)
        }
        
        None
    }

    

    pub fn calculate_new_velocity_after_col(&mut self, planet: Circle) -> (Vec2, Vec2){
        let normal_dir = (self.pos - planet.pos).normalize();
        let tangent = vec2(-normal_dir.y, normal_dir.x);


        let v1n = normal_dir.dot(self.velocity);
        let v1t = tangent.dot(self.velocity);
        let v2n = normal_dir.dot(planet.velocity);
        let v2t = tangent.dot(planet.velocity);
        let first_force = (v1n * (self.mass - planet.mass) + 2.0 * planet.mass * v2n) / (self.mass + planet.mass);
        let second_force = (v2n * (planet.mass - self.mass) + 2.0 * self.mass * v1n) / (self.mass + planet.mass);
        let first_vel = first_force * normal_dir + v1t * tangent;
        let second_vel = second_force * normal_dir + v2t * tangent;


        return  (first_vel, second_vel);
    }
    pub fn calculate_correction(&self, planet: &Circle) -> Vec2{
        let normal_dir = (self.pos - planet.pos).normalize();
        let overlap = (self.radius + planet.radius) - (planet.pos - self.pos).length();
        let correction = overlap * normal_dir;
        correction
    }
}
