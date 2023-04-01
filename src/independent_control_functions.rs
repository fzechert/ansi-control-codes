//! Independent Control Functions.
//!
//! These control functions are represented in 7-bit codes by 2-character escape sequences of the form `ESC Fs`, where
//! `ESC` is represented by bit combination `01/11` and `Fs` is represented by a bit combination from `06/00` to
//! `07/14`.
//!
//! ## Usage
//!
//! You can use the independent control functions inside normal strings, format them with the `format!()` macro, or
//! print them with the `print!()` and `println!()` macros.
//!
//! For example, to disable manual input:
//!
//! ```
//! use ansi_control_codes::independent_control_functions::DMI;
//! println!("{}", DMI);
//! ```
//!
//! ## Overview of the Independent Control Functions
//!
//! | Row Number | Column `06` | Column `07` |
//! | ---------: | :---------: | :---------: |
//! |       `00` |   [`DMI`]   |     --      |
//! |       `01` |   [`INT`]   |     --      |
//! |       `02` |   [`EMI`]   |     --      |
//! |       `03` |   [`RIS`]   |     --      |
//! |       `04` |   [`CMD`]   |     --      |
//! |       `05` |     --      |     --      |
//! |       `06` |     --      |     --      |
//! |       `07` |     --      |     --      |
//! |       `08` |     --      |     --      |
//! |       `09` |     --      |     --      |
//! |       `10` |     --      |     --      |
//! |       `11` |     --      |     --      |
//! |       `12` |     --      |  [`LS3R`]   |
//! |       `13` |     --      |  [`LS2R`]   |
//! |       `14` |   [`LS2`]   |  [`LS1R`]   |
//! |       `15` |   [`LS3`]   |     --      |
//!  
//! ## Note
//!
//! `ESC Fs` sequences are registered in the ISO International Register of Coded Character Sets to be Used with Escape
//! Sequences, which is maintained by the Registration Authority for ISO 2375.

use crate::ControlFunction;

macro_rules! independent {
    ($xx:literal/$yy:literal) => {
        ControlFunction::new_independent_control_function(ascii!($xx / $yy))
    };
}

/// Coding Method Delimiter.
///
/// `CMD` is used as the delimiter of a string of data coded according to Standard ECMA-35, and to switch to a general
/// level of control.
///
/// The use of `CMD` is not mandatory if the higher level protocol defines means of delimiting the string, for instance,
/// by specifying the length of the string.
pub const CMD: ControlFunction = independent!(06 / 04);

/// Disable Manual Input
///
/// `DMI` causes the manual input facilities of a device to be disabled.
pub const DMI: ControlFunction = independent!(06 / 00);

/// Enable Manual Input.
///
/// `EMI` is used to enable the manual input facilities of a device.
pub const EMI: ControlFunction = independent!(06 / 02);

/// Interrupt.
///
/// `INT` is used to indicate to the receiving device that the current process is to be interrupted and an agreed
/// procedure is to be initiated. This control function is applicable to either direction of transmission.
pub const INT: ControlFunction = independent!(06 / 01);

/// Locking-Shift One Right.
///
/// `LS1R` is used for code extension purposes. It causes the meaning of the bit combinations following it in the data
/// stream to be changed.
///
/// The use of `LS1R` is defined in Standard [ECMA-35][ecma-35].
///
/// [ecma-35]: https://www.ecma-international.org/wp-content/uploads/ECMA-35_6th_edition_december_1994.pdf
pub const LS1R: ControlFunction = independent!(07 / 14);

/// Locking-Shift Two.
///
/// `LS2` is used for code extension purposes. It causes the meaning of the bit combinations following it in the data
/// stream to be changed.
///
/// The use of `LS2` is defined in Standard [ECMA-35][ecma-35].
///
/// [ecma-35]: https://www.ecma-international.org/wp-content/uploads/ECMA-35_6th_edition_december_1994.pdf
pub const LS2: ControlFunction = independent!(06 / 14);

/// Locking-Shift Two Right.
///
/// `LS2R` is used for code extension purposes. It causes the meaning of the bit combinations following it in the data
/// stream to be changed.
///
/// The use of `LS2R` is defined in Standard [ECMA-35][ecma-35].
///
/// [ecma-35]: https://www.ecma-international.org/wp-content/uploads/ECMA-35_6th_edition_december_1994.pdf
pub const LS2R: ControlFunction = independent!(07 / 13);

/// Locking-Shift Three.
///
/// `LS3` is used for code extension purposes. It causes the meaning of the bit combinations following it in the data
/// stream to be changed.
///
/// The use of `LS3` is defined in Standard [ECMA-35][ecma-35].
///
/// [ecma-35]: https://www.ecma-international.org/wp-content/uploads/ECMA-35_6th_edition_december_1994.pdf
pub const LS3: ControlFunction = independent!(06 / 15);

/// Locking-Shift Three Right.
///
/// `LS3R` is used for code extension purposes. It causes the meaning of the bit combinations following it in the data
/// stream to be changed.
///
/// The use of `LS3R` is defined in Standard [ECMA-35][ecma-35].
///
/// [ecma-35]: https://www.ecma-international.org/wp-content/uploads/ECMA-35_6th_edition_december_1994.pdf
pub const LS3R: ControlFunction = independent!(07 / 12);

/// Reset to Initial State.
///
/// `RIS` causes a device to be reset to its initial state, i.e. the state it has after it is made operational. This
/// may imply, if applicable: clear tabulation stops, remove qualified areas, reset graphic rendition, put all character
/// positions into the erased state, move the active presentation position to the first position of the first line in
/// the presentation component, move the active data position to the first character position of the first line in the
/// data component, set the modes into the reset state, etc..
pub const RIS: ControlFunction = independent!(06 / 03);
