extern crate robot_traits;

use robot_traits::Physical;
use robot_traits::sensors::PhotoSensor;
use robot_traits::actuators::Robot;

trait Simulation {
    fn tick(&mut self);
}

#[derive(Debug, Copy, Clone, Default)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug, Copy, Clone, Default)]
struct Vector {
    x: f32,
    y: f32,
}

struct RelativePoint<'a> {
    point: &'a Point,
    offset: Point,
}

fn distance(p1: &Point, p2: &Point) -> f32 {
    ((p1.x - p2.x) * (p1.x - p2.x) + (p1.y - p2.y) * (p1.y - p2.y)).sqrt()
}

#[derive(Debug)]
struct SimulatedWorld {
    light_source: Point,
}

struct SimulatedPhotoSensor<'a> {
    world: &'a SimulatedWorld,
    position: Point,
    light_value: f32,
}

impl<'a> SimulatedPhotoSensor<'a> {
    fn new(world: &'a SimulatedWorld) -> Self {
        let position = Point::default();
        let light_value = 0.1;
        Self { world, position, light_value }
    }
}

impl<'a> PhotoSensor for SimulatedPhotoSensor<'a> {
    type Output = f32;
    fn poll_light(&mut self) -> f32 {
        self.light_value
        // distance(&self.position, &self.world.light_source)
    }
}

impl<'a> Physical for SimulatedPhotoSensor<'a> {
    type Position = Point;

    fn get_position(&self) -> Point {
        self.position
    }
}

impl<'a> Simulation for SimulatedPhotoSensor<'a> {
    fn tick(&mut self) {
        self.light_value += 0.1;
    }
}

#[derive(Debug, Default)]
struct Pose {
    point: Point,
    rotation: Vector,
}

#[derive(Debug, Default)]
struct Velocity {
    linear: f32,
    angular: f32,
}

#[derive(Debug, Default)]
struct SimulatedRobot {
    pose: Pose,
    velocity: Velocity,
}

impl SimulatedRobot {
    fn new() -> Self {
        Self {
            pose: Pose {
                point: Point {
                    x: 0.0,
                    y: 0.0,
                },
                rotation: Vector {
                    x: 1.0,
                    y: 0.0,
                }
            },
            velocity: Velocity::default(),
        }
    }
}

impl Robot for SimulatedRobot {
    fn forward(&mut self, speed: f32) {
        self.velocity.linear = speed;
    }

    fn turn(&mut self, speed: f32) {
        self.velocity.angular = speed;
    }

    fn stop(&mut self) {
        self.velocity.linear = 0.0;
        self.velocity.angular = 0.0;
    }
}

impl Physical for SimulatedRobot {
    type Position = Point;

    fn get_position(&self) -> Point {
        self.pose.point
    }
}

impl Simulation for SimulatedRobot {
    fn tick(&mut self) {
        self.pose.point.x += self.pose.rotation.x * self.velocity.linear;
        self.pose.point.y += self.pose.rotation.y * self.velocity.linear;
        self.pose.rotation.x = self.velocity.angular.cos() * self.pose.rotation.x - self.velocity.angular.sin() * self.pose.rotation.y;
        self.pose.rotation.y = self.velocity.angular.sin() * self.pose.rotation.x + self.velocity.angular.cos() * self.pose.rotation.y;
    }
}



fn main() {
    use std::thread::sleep;
    use std::time::Duration;

    let world = SimulatedWorld {light_source: Point{x: 5.0, y: 6.0}};
    let mut robot = SimulatedRobot::new();
    let mut sensor = SimulatedPhotoSensor::new(&world);

    loop {
        if sensor.poll_light() > 10.0 {
            robot.turn(1.0);
        } else {
            robot.forward(1.0);
        }

        robot.tick();
        sensor.tick();
        println!("{:#?}", robot);
        sleep(Duration::from_millis(100));
    }
}