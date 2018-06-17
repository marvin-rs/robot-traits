//! Traits used for adding information into the system

use Physical;

pub trait DistanceSensor: Physical {
    type Output;
    fn poll_distance(&mut self) -> Self::Output;
}

pub trait PhotoSensor: Physical {
    type Output;
    fn poll_light(&mut self) -> Self::Output;
}
