extern crate robot_traits;
extern crate nalgebra as na;

use robot_traits::Physical;
use robot_traits::sensors::PhotoSensor;
use robot_traits::actuators::Robot;

use na::{Point2, Vector2, Rotation2};

type Point = Point2<f32>;
type Vector = Vector2<f32>;

trait Simulation {
    fn tick(&mut self);
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
        let position = Point::origin();
        let light_value = 0.1;
        Self { world, position, light_value }
    }
}

impl<'a> PhotoSensor for SimulatedPhotoSensor<'a> {
    type Output = f32;
    fn poll_light(&mut self) -> f32 {
        self.light_value
        // na::distance(&self.position, &self.world.light_source)
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

#[derive(Debug)]
struct Pose {
    point: Point,
    rotation: Vector,
}

#[derive(Debug, Default)]
struct Velocity {
    linear: f32,
    angular: f32,
}

#[derive(Debug)]
struct SimulatedRobot {
    pose: Pose,
    velocity: Velocity,
}

impl SimulatedRobot {
    fn new() -> Self {
        Self {
            pose: Pose {
                point: Point::origin(),
                rotation: Vector::new(1.0, 0.0),
            },
            velocity: Velocity::default(),
        }
    }
}

impl Robot for SimulatedRobot {
    fn forward(&mut self, speed: f32) {
        self.velocity.linear = speed;
        self.velocity.angular = 0.0;
    }

    fn turn(&mut self, speed: f32) {
        self.velocity.linear = 0.0;
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
        self.pose.point += self.pose.rotation * self.velocity.linear;
        let rot = Rotation2::new(self.velocity.angular);
        self.pose.rotation = rot * self.pose.rotation;
    }
}



fn main() {
    use std::thread::sleep;
    use std::time::Duration;

    let world = SimulatedWorld {light_source: Point::new(5.0, 6.0)};
    let mut robot = SimulatedRobot::new();
    let mut sensor = SimulatedPhotoSensor::new(&world);

    loop {
        if sensor.poll_light() % 10.0 > 5.0  {
            robot.turn(0.1);
        } else {
            robot.forward(0.1);
        }

        robot.tick();
        sensor.tick();
        print!("{}[2J", 27 as char); // clear screen
        println!("{:#?}", robot);
        sleep(Duration::from_millis(100));
    }
}