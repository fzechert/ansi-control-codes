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
//! to avoid confusion, for example, between the concept `space`, and the character `SPACE`.
//!
//! As is intended by the [ECMA-48 Standard][ecma-48], this convention and all acronyms of modes, and control functions
//! are retained in this library, where rust permits.
//!
//! A character from the [ASCII table][ascii-table] is represented in the form `xx/yy`, where `xx` represents the column
//! number `00` to `07` in a 7-bit code table, and `yy` represents the row number `00` to `15`.
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

use std::str;

/// Convert the ascii table notation `xx/yy` into a rust string.
///
/// A character from the [ASCII table][ascii-table] is represented in the form `xx/yy`, where `xx` represents the column
/// number `00` to `07` in a 7-bit code table, and `yy` represents the row number `00` to `15`.
///
/// The macro can be used to convert a single code point into a str, or to convert a sequence of them.
///
/// ```
/// let a: &'static str = ascii!(06 / 01);
/// let abc: &'static str = ascii!(06 / 01, 06 / 02, 06 / 03);
/// ```
///
/// ## Safeness
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
/// bounds.
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
    /// C0 control functions are represented in 7-bit codes by bit combinations from 00/00 to 01/15.
    C0,

    /// Elements of the C1 set.
    ///
    /// C1 control functions are represented in 7-bit codes by 2-character escape sequences of the form `ESC Fe`,
    /// where `ESC` is represented by bit combination `01/11`, and `Fe` is represented by a bit combination from
    /// `04/00` to `05/15`.
    C1,

    /// Control Sequences.
    ///
    /// Control sequences are strings of bit combinations starting with the control function Control Function Introducer
    /// (`CSI`), followed by one or more bit combinations representing parameters, if any, and by one ore more bit
    /// combinations identifying the control function. The control function `CSI` itself is an element of the independent_control_function set.
    ControlSequence,

    /// Independent Control Functions.
    ///
    /// Independent control functions are represented in 7-bit codes by 2-character escape sequences of the form
    /// `ESC Fs`, where `ESC` is represented by bit combination `01/11`, and `Fs` is represented by a bit combination
    /// from `06/00` to `07/14`
    IndependentControlFunction,

    /// Control Strings.
    ///
    /// A control string is a string of bit combinations which may occur in the data stream as a logical entity for
    /// control purposes. A control string consists of an opening delimiter, a command string or character string, and
    /// a terminating delimiter, the String Terminator (`ST`).
    ControlString,
}

/// A control function.
pub struct ControlFunction {
    function_type: ControlFunctionType,
    value: &'static str,
    parameters: Vec<String>,
}

impl ControlFunction {
    /// Creates a new control function of type [C0][ControlFunctionType::C0].
    ///
    /// C0 control functions do not accept any parameters.
    const fn new_c0(value: &'static str) -> Self {
        ControlFunction {
            function_type: ControlFunctionType::C0,
            value,
            parameters: vec![],
        }
    }

    /// Creates a new control function of type [C1][ControlFunctionType::C1].
    ///
    /// independent_control_function control functions do not accept any parameters.
    const fn new_c1(value: &'static str) -> Self {
        ControlFunction {
            function_type: ControlFunctionType::C1,
            value,
            parameters: vec![],
        }
    }

    /// Creates a new control function of type [ControlSequence][ControlFunctionType::ControlSequence].
    const fn new_sequence(value: &'static str, parameters: Vec<String>) -> Self {
        ControlFunction {
            function_type: ControlFunctionType::ControlSequence,
            value,
            parameters,
        }
    }

    /// Creates a new control function of type [IndependentControlFunction][ControlFunctionType::IndependentControlFunction].
    const fn new_independent_control_function(value: &'static str) -> Self {
        ControlFunction {
            function_type: ControlFunctionType::IndependentControlFunction,
            value,
            parameters: vec![],
        }
    }
}

pub mod c0;
pub mod c1;
pub mod control_sequences;
pub mod control_strings;
pub mod independent_control_functions;
pub mod modes;
