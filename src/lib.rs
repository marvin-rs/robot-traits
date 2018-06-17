//! This crate defined a collection of robot related traits to allow
//! interoperability between generic inputs and outputs.
//!
//! Many of the traits were originally borrowed from the python gpiozero
//! library although I expect them to diverge over time.
//! 
//! **Note:** This is still a work in progress and the API should not be considered stable until the
//! `1.0` release.

pub mod actuators;
pub mod sensors;

pub trait Physical {
    type Position;
    fn get_position(&self) -> Self::Position;
}