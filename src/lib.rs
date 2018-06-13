//! This crate defined a collection of robot related traits to allow
//! interoperability between generic inputs and outputs.
//!
//! Many of the traits were originally borrowed from the python gpiozero
//! library although I expect them to diverge over time.
//! 
//! **Note:** This is still a work in progress and the API should not be considered stable until the
//! `1.0` release.

/// Represents a single motor. All commands are assumed to
/// supercede the previous one, i.e. they are *not* additive.
///
/// All `speed` fields are in arbitrary units of "power" between 0.0 and 1.0
pub trait Motor {
    /// Drives the motor at the required speed.
    /// A negative speed will cause the motor to turn backwards.
    ///
    /// `speed` should be in the range [-1.0, 1.0]
    fn start(&mut self, speed: f32);

    /// Stop the motor.
    fn stop(&mut self);
}

/// Represents a differential drive robot. All commands are assumed to
/// supercede the previous one, i.e. they are *not* additive.
///
/// All `speed` fields are in arbitrary units of "power" between 0.0 and 1.0
pub trait Robot {
    /// Drive the robot forward by running both motors.
    /// A negative speed will drive the robot backwards.
    ///
    /// `speed` should be in the range [-1.0, 1.0]
    fn forward(&mut self, speed: f32);

    /// A positive speed makes the robot turn right (clockwise).
    /// A negative speed makes the robot turn left (counter-clockwise).
    ///
    /// `speed` should be in the range [-1.0, 1.0]
    fn turn(&mut self, speed: f32);

    /// Stop the robot.
    fn stop(&mut self);
}

/// Represents a single LED that can be turned on or off
pub trait Led {
    fn led_on(&mut self);
    fn led_off(&mut self);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
