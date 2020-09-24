/// Gilrs bindings for the `Thrustmaster Throttle - HOTAS Warthog` device
/// Provided is an enumeration of events that can be found on the module
///
/// the `codes` module provides a mechanism for acquiring the linux event bindings
/// such that the gilrs gamepad state interface can be used.


include!(concat!(env!("OUT_DIR"), "/thrustmaster.rs"));