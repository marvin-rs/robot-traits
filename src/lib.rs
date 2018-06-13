//! This crate defined a collection of robot related traits to allow
//! interoperability between generic inputs and outputs.
//! 
//! Many of the traits were originally borrowed from the python gpiozero
//! library although I expect them to diverge over time.


/// Represents a single motor. All commands are assumed to
/// supercede the previous one, i.e. they are *not* additive.
/// 
/// All `speed` fields are in arbitrary units of "power" between 0.0 and 1.0
/// 
/// https://gpiozero.readthedocs.io/en/stable/api_output.html#motor
pub trait Motor {
    /// Drive the motor backwards.
    ///
    /// speed (float) – The speed at which the motor should turn. Can be any
    /// value between 0 (stopped) and the default 1 (maximum speed) if pwm
    ///  was True when the class was constructed (and only 0 or 1 if not).
    fn backward(speed: f32);

    /// Drive the motor forwards.
    ///
    /// speed (float) – The speed at which the motor should turn. Can be any
    /// value between 0 (stopped) and the default 1 (maximum speed) if pwm
    /// was True when the class was constructed (and only 0 or 1 if not).
    fn forward(speed: f32);

    /// Reverse the current direction of the motor. If the motor is currently
    /// idle this does nothing. Otherwise, the motor’s direction will be
    /// reversed at the current speed.
    fn reverse();

    /// Stop the motor.
    fn stop();
}

/// Represents a differential drive robot. All commands are assumed to
/// supercede the previous one, i.e. they are *not* additive.
/// 
/// All `speed` fields are in arbitrary units of "power" between 0.0 and 1.0
/// 
/// Derived https://gpiozero.readthedocs.io/en/stable/api_boards.html#robot
pub trait Robot {
    /// Drive the robot backward by running both motors backward.
    fn backward(&mut self, speed: f32);

    /// Drive the robot forward by running both motors forward.
    fn forward(&mut self, speed: f32);

    /// Make the robot turn left by running the right motor forward and left
    /// motor backward.
    fn turn_left(&mut self, speed: f32);

    /// Make the robot turn right by running the left motor forward and right
    /// motor backward.
    fn turn_right(&mut self, speed: f32);

    /// Reverse the robot's current motor directions. If the robot is currently
    /// running full speed forward, it will run full speed backward. If the
    /// robot is turning left at half-speed, it will turn right at half-speed.
    /// If the robot is currently stopped it will remain stopped.
    fn reverse(&mut self);

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
