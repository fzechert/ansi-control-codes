//! # ANSI Escape Code Library
//!
//! ANSI escape sequences are a standard for in-band signalling to control cursor location, color, font styling, and
//! other options on video text terminals and terminal emulators.
//!
//! This library contains all ANSI Escape Codes that are defined in the [ISO 6429 Standard][iso-6429]. ISO 6429 is
//! the international standard that followed from the efforts of aligning the european [ECMA-48 Standard][ecma-48] and
//! the american [ANSI X3.64 Standard][ansi-x364].
//!
//! ## Notation
//!
//! In the [ECMA-48 Standard][ecma-48] a convention has been adopted to assist the reader of the Standard.
//!
//! Capital letters are used to refer to a specific control function, mode, mode setting, or graphic character in order
//! to avoid confusion, for example, between the concept "space", and the character `SPACE`.
//!
//! As is intended by the [ECMA-48 Standard][ecma-48], this convention and all acronyms of modes, and control functions
//! are retained in this library, where rust permits.
//!
//! A character from the [ASCII table][ascii-table] is represented in the form `xx/yy`, where `xx` represents the column
//! number `00` to `07` in a 7-bit code table, and `yy` represents the row number `00` to `15`.
//!
//! ## Low-Level Control Functions
//!
//! The control functions of this library are sorted into several modules. You will find the low-level control functions
//! in the modules [c0], [c1], [control_sequences], [independent_control_functions], and [control_strings].
//!
//! The control functions can be put into normal strings. For example, to ring the bell:
//!
//! ```
//! use ansi::c0::BEL;
//! print!("{}", BEL);
//! ```
//!
//! Or to move the cursor to line 5, column 13:
//!
//! ```
//! use ansi::control_sequences::CUP;
//! print!("{}", CUP(Some(5), Some(13)));
//! ```
//!
//! It might be necessary in some circumstances to announce the active set of control sequences before they can be used.
//! This is possible by invoking one of the announcer sequences.
//!
//! ```
//! use ansi::c1::{ANNOUNCER_SEQUENCE, NEL};
//! // announce the C1 control function set, then move to the next line.
//! print!("{}{}", ANNOUNCER_SEQUENCE, NEL);
//! ```
//!
//! ## Source Material
//!
//! The second, and newer, editions of the [ECMA-48 Standard][ecma-48] are based on the text of the
//! [ISO 6429 Standard][iso-6429] and are technically identical with it. Since the [ISO 6429 Standard][iso-6429] is not
//! freely available on the internet, this implementation is based on the publicly available documents of the
//! [ECMA-48 Standard][ecma-48]. In particular on edition 5 of the [ECMA-48 Standard][ecma-48], which is identical to
//! the third edition of [ISO-6429][iso-6429].
//!
//! The [ANSI X3.64 Standard][ansi-x364] has been withdrawn by ANSI in 1994 in favour of the international standard.
//!
//! You can read more about the history of the standards on [Wikipedia: ANSI escape code][wikipedia-ansi].
//!
//! [ansi-x364]: https://nvlpubs.nist.gov/nistpubs/Legacy/FIPS/fipspub86.pdf
//! [ascii-table]: https://en.wikipedia.org/wiki/ASCII#/media/File:USASCII_code_chart.png
//! [ecma-48]: https://www.ecma-international.org/publications-and-standards/standards/ecma-48/
//! [iso-6429]: https://www.iso.org/standard/12782.html
//! [wikipedia-ansi]: https://en.wikipedia.org/wiki/ANSI_escape_code
#![allow(dead_code)]

use std::{fmt, str};

/// Converts the ascii table notation `xx/yy` into a rust string.
///
/// A character from the [ASCII table][ascii-table] is represented in the form `xx/yy`, where `xx` represents the column
/// number `00` to `07` in a 7-bit code table, and `yy` represents the row number `00` to `15`.
///
/// The macro can be used to convert a single code point into a str, or to convert a sequence of them.
///
/// ```ignore
/// let a: &'static str = ascii!(06 / 01);
/// let abc: &'static str = ascii!(06 / 01, 06 / 02, 06 / 03);
/// ```
///
/// ## Safety
///
/// This macro converts the given `xx/yy` combination into a ascii code by the formula `(xx << 4) + yy`.
/// The result is passed to the unsafe function std::str::from_utf8_unchecked.
///
/// This will result in an unsafe calculation, if the values for xx and yy are out of range. Valid ranges are:
///
/// - `xx: [0,7]`
/// - `yy: [0,15]`
///
/// Since this macro is not public and only used by the library itself, it is assumed to be used only within safe
/// bounds, and therefore considered safe.
///
/// [ascii-table]: https://en.wikipedia.org/wiki/ASCII#/media/File:USASCII_code_chart.png
macro_rules! ascii {
    ($($xx:literal/$yy:literal), *) => {
        unsafe { std::str::from_utf8_unchecked(&[$(($xx << 4) + $yy),*]) }
    };
}

/// The different types of control functions.
enum ControlFunctionType {
    /// Elements of the C0 set.
    ///
    /// C0 control functions are represented in 7-bit codes by bit combinations from `00/00` to `01/15`.
    ///
    /// The control functions of the C0 set are defined in the module [c0].
    C0,

    /// Elements of the C1 set.
    ///
    /// C1 control functions are represented in 7-bit codes by 2-character escape sequences of the form `ESC Fe`,
    /// where `ESC` is represented by bit combination `01/11`, and `Fe` is represented by a bit combination from
    /// `04/00` to `05/15`.
    ///
    /// The control functions of the C1 set are defined in the module [c1].
    C1,

    /// Control Sequences.
    ///
    /// Control sequences are strings of bit combinations starting with the control function
    /// `CONTROL SEQUENCE INTRODUCER` ([`CSI`][c1::CSI]), followed by one or more bit combinations representing
    /// parameters, if any, and by one ore more bit combinations identifying the control function. The control function
    /// `CSI` itself is an element of the [c1] set.
    ///
    /// The control sequences are defined in the module [control_sequences].
    ControlSequence,

    /// Independent Control Functions.
    ///
    /// Independent control functions are represented in 7-bit codes by 2-character escape sequences of the form
    /// `ESC Fs`, where `ESC` is represented by bit combination `01/11`, and `Fs` is represented by a bit combination
    /// from `06/00` to `07/14`.
    ///
    /// The independent control functions are defined in the module [independent_control_functions].
    IndependentControlFunction,

    /// Control Strings.
    ///
    /// A control string is a string of bit combinations which may occur in the data stream as a logical entity for
    /// control purposes. A control string consists of an opening delimiter, a command string or character string, and
    /// a terminating delimiter, the String Terminator ([`ST`][c1::ST]).
    ///
    /// The control strings are defined in the module [control_strings].
    ControlString,
}

impl fmt::Debug for ControlFunctionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ControlFunctionType::C0 => write!(f, "C0"),
            ControlFunctionType::C1 => write!(f, "C1"),
            ControlFunctionType::ControlSequence => write!(f, "Control Sequence"),
            ControlFunctionType::IndependentControlFunction => {
                write!(f, "Independent Control Function")
            }
            ControlFunctionType::ControlString => write!(f, "Control String"),
        }
    }
}

/// An ansi control function defined in [ECMA-48][ecma-48].
///
/// [ecma-48]: https://www.ecma-international.org/publications-and-standards/standards/ecma-48/
pub struct ControlFunction {
    /// The type of the control function.
    function_type: ControlFunctionType,

    /// The byte or byte combination identifying the control function.
    value: &'static str,

    /// An arbitrary number of arguments for this control function.
    parameters: Vec<String>,
}

impl ControlFunction {
    /// Creates a new control function of type [`C0`][ControlFunctionType::C0].
    ///
    /// `C0` control functions do not accept any parameters.
    const fn new_c0(value: &'static str) -> Self {
        ControlFunction {
            function_type: ControlFunctionType::C0,
            value,
            parameters: vec![],
        }
    }

    /// Creates a new control function of type [`C1`][ControlFunctionType::C1].
    ///
    /// `C1` control functions do not accept any parameters.
    const fn new_c1(value: &'static str) -> Self {
        ControlFunction {
            function_type: ControlFunctionType::C1,
            value,
            parameters: vec![],
        }
    }

    /// Creates a new control function of type [`ControlSequence`][ControlFunctionType::ControlSequence].
    const fn new_sequence(value: &'static str, parameters: Vec<String>) -> Self {
        ControlFunction {
            function_type: ControlFunctionType::ControlSequence,
            value,
            parameters,
        }
    }

    /// Creates a new control function of type
    /// [`IndependentControlFunction`][ControlFunctionType::IndependentControlFunction].
    const fn new_independent_control_function(value: &'static str) -> Self {
        ControlFunction {
            function_type: ControlFunctionType::IndependentControlFunction,
            value,
            parameters: vec![],
        }
    }

    fn format_parameters(&self) -> String {
        self.parameters.join(ascii!(03 / 11))
    }
}

impl fmt::Display for ControlFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.function_type {
            ControlFunctionType::C0 => {
                write!(f, "{}", self.value)
            }
            ControlFunctionType::C1 => {
                write!(f, "{}{}", c0::ESC, self.value)
            }
            ControlFunctionType::ControlSequence => {
                let parameters = self.format_parameters();
                write!(f, "{}{}{}", c1::CSI, parameters, self.value)
            }
            ControlFunctionType::IndependentControlFunction => {
                write!(f, "{}{}", c0::ESC, self.value)
            }
            ControlFunctionType::ControlString => {
                write!(f, "{}", self.value)
            }
        }
    }
}

impl fmt::Debug for ControlFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let function: String = self
            .value
            .as_bytes()
            .into_iter()
            .map(|b| format!("{:02}/{:02}", b >> 4, (b & 0xF)))
            .collect();

        f.debug_struct("ControlFunction")
            .field("function_type", &self.function_type)
            .field("function", &function)
            .field("parameters", &self.parameters)
            .finish()
    }
}

pub mod c0;
pub mod c1;
pub mod control_sequences;
pub mod control_strings;
pub mod independent_control_functions;
pub mod modes;
