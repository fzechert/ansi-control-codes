//! Control Strings.
//!
//! A control string is a string of bit combinations which may occur in the data stream as a logical entity for
//! control purposes. A control string consists of an opening delimiter, a command string or a character string,
//! and a terminating delimiter, the STRING TERMINATOR ([`ST`]).
//!
//! A command string is a sequence of bit combinations in the range `00/08` to `00/13` and `02/00` to `07/14`.
//!
//! A character string is a sequence of any bit combination, except those representing START OF STRING
//! ([`SOS`]) or STRING TERMINATOR ([`ST`]).
//!
//! The low-level ansi control codes for control strings are defined in the module [`c1`][crate::c1].
//!
//! - APPLICATION PROGRAM COMMAND ([`APC`])
//! - DEVICE CONTROL STRING ([`DCS`])
//! - OPERATING SYSTEM COMMAND ([`OSC`])
//! - PRIVACY MESSAGE ([`PM`])
//! - START OF STRING ([`SOS`])
//! - STRING TERMINATOR ([`ST`])
//!
//! This module contains higher level functions to invoke these low-level ansi control codes.
//!
//! ## Usage
//!
//! Invoke one of the available functions to create new control strings.
//!
//! For example, create a new operating system command to halt the operating system (this is operating system specific
//! and will most likely not work on your operating system).
//!
//! ```no_run
//! use ansi_control_codes::control_strings::operating_system_command;
//! let halt_command = operating_system_command("HALT");
//! println!("{}", halt_command);
//! ```

use crate::c1::{APC, DCS, OSC, PM, SOS, ST};

/// Creates a new Application Program Command.
///
/// The given command string will be prefixed with [`APC`] and suffixed with [`ST`].
///
/// The interpretation of the command string depends on the relevant application program.
pub fn application_program_command(command_string: &str) -> String {
    format!("{}{}{}", APC, command_string, ST)
}

/// Creates a new Device Control String.
///
/// The given control string will be prefixed with [`DCS`] and suffixed with [`ST`].
///
/// The command string represents either one or more commands for the receiving device, or one or more status reports
/// from the sending device. The purpose and the format of the command string are specified by the most recent
/// occurrence of IDENTIFY DEVICE CONTROL STRING ([`IDCS`][crate::control_sequences::IDCS]), if any, or depend on the
/// sending and/or the receiving device.
pub fn device_control_string(control_string: &str) -> String {
    format!("{}{}{}", DCS, control_string, ST)
}

/// Creates a new Operating System Command.
///
/// The given system command will be prefixed with [`OSC`] and suffixed with [`ST`].
///
/// The interpretation of the command string depends on the relevant operating system.
pub fn operating_system_command(system_command: &str) -> String {
    format!("{}{}{}", OSC, system_command, ST)
}

/// Creates a new Privacy Message.
///
/// The given message will be prefixed with [`PM`] and suffixed with [`ST`].
///
/// The interpretation of the message depends on the relevant privacy discipline.
pub fn privacy_message(message: &str) -> String {
    format!("{}{}{}", PM, message, ST)
}

/// Creates a new Control String.
///
/// The given control string will be prefixed with [`SOS`] and suffixed with [`ST`].
///
/// The interpretation of the character string depends on the application.
pub fn control_string(control_string: &str) -> String {
    format!("{}{}{}", SOS, control_string, ST)
}
