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
//! in the modules [c0], [c1], [control_sequences], [independent_control_functions]
//!
//! The control functions can be put into normal strings. For example, to ring the bell:
//!
//! ```
//! use ansi_control_codes::c0::BEL;
//! println!("Let's ring the bell {}", BEL);
//! ```
//!
//! Or to move the cursor to line 5, column 13:
//!
//! ```
//! use ansi_control_codes::control_sequences::CUP;
//! print!("{}", CUP(5.into(), 13.into()));
//! ```
//!
//! It might be necessary in some circumstances to announce the active set of control sequences before they can be used.
//! This is possible by invoking one of the announcer sequences.
//!
//! ```
//! use ansi_control_codes::c1::{ANNOUNCER_SEQUENCE, NEL};
//! // announce the C1 control function set, then move to the next line.
//! print!("{}{}", ANNOUNCER_SEQUENCE, NEL);
//! ```
//!
//! ## Categories of control functions
//!
//! Most control functions are categorized into different groups. They can be accessed from the module
//! [categories].
//!
//! ```
//! use ansi_control_codes::categories::format_effectors::{CR, LF};
//! println!("line1{}{}line2", CR, LF);
//! ```
//!
//! ## High-Level Functions
//!
//! For your convenience and ease-of-use of the ansi control codes, some functionality is exposed in wrapper functions.
//! See the following module documentations for a more in-depth introduction to these functions.
//!
//! - Working with control strings in module [control_strings].
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

#![allow(clippy::zero_prefixed_literal)]
use std::{error::Error, fmt, str};

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

/// Possible errors when specifying a custom control funciton.
///
/// It is possible to define custom control functions, so called private-use or experimental functions.
/// These private-use functions still need to follow some rules. If they violate the rules, one of these
/// variants is returned as an error.
#[derive(Debug)]
pub enum InvalidControlFunction {
    /// All control function values must be valid ASCII.
    InvalidAsciiError,
    /// All control functions must have a one- or two-byte function identifier.
    InvalidFunctionValueError,
    /// If the function has an intermediate byte, it must by `02 / 00`. All other intermediate bytes are invalid.
    InvalidIntermediateByteError,
    /// All private-use functions must be in the range `07 / 00` to `07 / 15`.
    InvalidPrivateUseError,
}

impl fmt::Display for InvalidControlFunction {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InvalidControlFunction::InvalidAsciiError => {
                write!(formatter, "Control function must be valid ASCII")
            }
            InvalidControlFunction::InvalidFunctionValueError => write!(
                formatter,
                "Control function must have one- or two-byte identifier"
            ),
            InvalidControlFunction::InvalidIntermediateByteError => {
                write!(formatter, "Intermediate byte must be 02/00")
            }
            InvalidControlFunction::InvalidPrivateUseError => write!(
                formatter,
                "Private use functions are only allowed in range 07/00 to 07/15"
            ),
        }
    }
}

impl Error for InvalidControlFunction {}

/// The different types of control functions.
///
#[derive(Clone, Copy, PartialEq, Eq)]
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
        }
    }
}

/// An ansi control function defined in [ECMA-48][ecma-48].
///
/// This struct implements the `PartialEq` trait for String-like types (all types that implement `AsRef<str>`).
/// It can be used to compare ControlFunctions with string-like values using `==` or `!=` functions.
///
/// Example:
/// ```
/// use ansi_control_codes::c0;
///
/// let some_string = String::from("\u{001B}");
/// if (c0::ESC == some_string) {
///     println!("ESC!")
/// }
/// ```
///
/// [ecma-48]: https://www.ecma-international.org/publications-and-standards/standards/ecma-48/
#[derive(PartialEq, Eq)]
pub struct ControlFunction<'a> {
    /// The type of the control function.
    function_type: ControlFunctionType,

    /// The byte or byte combination identifying the control function.
    value: &'a str,

    /// An arbitrary number of arguments for this control function.
    parameters: Vec<String>,
}

impl<'a> ControlFunction<'a> {
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
    const fn new_sequence(value: &'a str, parameters: Vec<String>) -> Self {
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

    /// Creates a new control function of type [`ControlSequence`][ControlFunctionType::ControlSequence] with a
    /// function that is declared as private use. These functions are not standardized and their function is unknown.
    /// Yet, the standard allows these functions to exist for experimental use.
    ///
    /// If the specified value lies outside of the valid private use area, this function will return Err.
    pub fn private_use(
        value: &'a str,
        parameters: Vec<String>,
    ) -> Result<Self, InvalidControlFunction> {
        if !value.is_ascii() {
            return Err(InvalidControlFunction::InvalidAsciiError);
        }
        if value.as_bytes().len() > 2 {
            return Err(InvalidControlFunction::InvalidFunctionValueError);
        }
        let function_value = if value.as_bytes().len() == 2 {
            if &value[0..1] != ascii!(02 / 00) {
                return Err(InvalidControlFunction::InvalidIntermediateByteError);
            }
            &value[1..2]
        } else {
            &value[0..1]
        };

        if function_value.as_bytes()[0] >> 4 != 7 {
            return Err(InvalidControlFunction::InvalidPrivateUseError);
        }

        Ok(ControlFunction {
            function_type: ControlFunctionType::ControlSequence,
            value,
            parameters,
        })
    }

    fn format_parameters(&self) -> String {
        self.parameters.join(ascii!(03 / 11))
    }
}

impl<'a> fmt::Display for ControlFunction<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.function_type {
            ControlFunctionType::C0 => {
                write!(f, "{}", self.value)
            }
            ControlFunctionType::C1 | ControlFunctionType::IndependentControlFunction => {
                write!(f, "{}{}", c0::ESC, self.value)
            }
            ControlFunctionType::ControlSequence => {
                let parameters = self.format_parameters();
                write!(f, "{}{}{}", c1::CSI, parameters, self.value)
            }
        }
    }
}

impl<'a> fmt::Debug for ControlFunction<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let function: String = self
            .value
            .as_bytes()
            .iter()
            .map(|b| format!("{:02}/{:02}", b >> 4, (b & 0xF)))
            .collect::<Vec<_>>()
            .join(" ");

        f.debug_struct("ControlFunction")
            .field("function_type", &self.function_type)
            .field("function", &function)
            .field("parameters", &self.parameters)
            .finish()
    }
}

impl<'a> From<ControlFunction<'a>> for String {
    fn from(control_function: ControlFunction) -> Self {
        format!("{}", control_function)
    }
}

impl<'a, T> PartialEq<T> for ControlFunction<'a>
where
    T: AsRef<str>,
{
    // comparison for control sequences must be done on the evaluated sequence.
    #[allow(clippy::cmp_owned)]
    fn eq(&self, other: &T) -> bool {
        let other_str = other.as_ref();

        match self.function_type {
            ControlFunctionType::C0 => self.value == other_str,
            ControlFunctionType::C1 | ControlFunctionType::IndependentControlFunction => {
                if other_str.len() != 2 {
                    return false;
                }
                other_str[0..1] == *c0::ESC.value && other_str[1..2] == *self.value
            }
            ControlFunctionType::ControlSequence => self.to_string() == other_str,
        }
    }
}

impl<'a> PartialEq<ControlFunction<'a>> for &str {
    fn eq(&self, other: &ControlFunction) -> bool {
        other == self
    }
}

impl<'a> PartialEq<ControlFunction<'a>> for String {
    fn eq(&self, other: &ControlFunction) -> bool {
        other == self
    }
}

pub mod c0;
pub mod c1;
pub mod categories;
pub mod control_sequences;
pub mod control_strings;
pub mod independent_control_functions;
pub mod modes;

#[cfg(feature = "parser")]
pub mod parser;

#[cfg(test)]
mod tests {
    use crate::c0::{BEL, ESC};
    use crate::c1::CSI;
    use crate::control_sequences::CNL;
    use crate::independent_control_functions::INT;
    use crate::ControlFunctionType;

    /// Test the debug format of [`ControlFunctionType`].
    #[test]
    fn debug_control_function_type() {
        assert_eq!(format!("{:?}", ControlFunctionType::C0), "C0");
        assert_eq!(format!("{:?}", ControlFunctionType::C1), "C1");
        assert_eq!(
            format!("{:?}", ControlFunctionType::ControlSequence),
            "Control Sequence"
        );
        assert_eq!(
            format!("{:?}", ControlFunctionType::IndependentControlFunction),
            "Independent Control Function"
        );
    }

    /// Test the debug format of [`ControlFunction`][crate::ControlFunction].
    #[test]
    fn debug_control_function() {
        assert_eq!(
            format!("{:?}", BEL),
            "ControlFunction { function_type: C0, function: \"00/07\", parameters: [] }"
        );

        assert_eq!(
            format!("{:?}", crate::control_sequences::CUP(None, Some(10))),
            "ControlFunction { function_type: Control Sequence, function: \"04/08\", parameters: [\"1\", \"10\"] }"
        );
    }

    #[test]
    fn string_equality_c0() {
        let esc_control = ESC;
        let esc_str = "\u{001B}";
        let esc_string = String::from(esc_str);

        assert_eq!(
            esc_control, esc_control,
            "Asserting equality between same control codes failed"
        );
        assert_eq!(
            esc_control, esc_str,
            "Asserting equality between control code and string slice failed"
        );
        assert_eq!(
            esc_control, esc_string,
            "Asserting equality between control code and string failed"
        );

        assert!(
            esc_control == esc_str,
            "Failed to compare control code and string slice"
        );
        assert!(
            esc_control == esc_string,
            "Failed to compare control code and string"
        );

        // this should fail, as ESC and BEL are not equal
        assert_ne!(
            esc_control, "\u{0007}",
            "Different control codes should not be equal"
        );
    }

    #[test]
    fn string_equality_c1() {
        let csi_control = CSI;
        let csi_str = "\u{001B}[";
        let csi_string = String::from(csi_str);

        assert_eq!(
            csi_control, csi_control,
            "Asserting equality between same control codes failed"
        );
        assert_eq!(
            csi_control, csi_str,
            "Asserting equality between control code and string slice failed"
        );
        assert_eq!(
            csi_control, csi_string,
            "Asserting equality between control code and string failed"
        );

        assert!(
            csi_control == csi_str,
            "Failed to compare control code and string slice"
        );
        assert!(
            csi_control == csi_string,
            "Failed to compare control code and string"
        );

        // this should fail, as CSI and OSC are not equal
        assert_ne!(
            csi_control, "\u{001B}]",
            "Different control codes should not be equal"
        );
    }

    #[test]
    fn string_equality_control_sequence() {
        let cnl_control = CNL(4.into());
        let cnl_str = "\u{001B}[4E";
        let cnl_string = String::from(cnl_str);

        assert_eq!(
            cnl_control, cnl_control,
            "Asserting equality between same control codes failed"
        );
        assert_eq!(
            cnl_control, cnl_str,
            "Asserting equality between control code and string slice failed"
        );
        assert_eq!(
            cnl_control, cnl_string,
            "Asserting equality between control code and string failed"
        );

        assert!(
            cnl_control == cnl_str,
            "Failed to compare control code and string slice"
        );
        assert!(
            cnl_control == cnl_string,
            "Failed to compare control code and string"
        );

        // this should fail, as CNL for 4 lines and CNL for 3 lines should differ
        assert_ne!(
            cnl_control, "\u{001B}[3E",
            "Different control codes should not be equal"
        );
    }

    #[test]
    fn string_equality_independent_control_functions() {
        let icf_control = INT;
        let icf_str = "\u{001B}a";
        let icf_string = String::from(icf_str);

        assert_eq!(
            icf_control, icf_control,
            "Asserting equality between same control codes failed"
        );
        assert_eq!(
            icf_control, icf_str,
            "Asserting equality between control code and string slice failed"
        );
        assert_eq!(
            icf_control, icf_string,
            "Asserting equality between control code and string failed"
        );

        assert!(
            icf_control == icf_str,
            "Failed to compare control code and string slice"
        );
        assert!(
            icf_control == icf_string,
            "Failed to compare control code and string"
        );

        // this should fail, as Interrupt and Enable Manual Input are different
        assert_ne!(
            icf_control, "\u{001B}b",
            "Different control codes should not be equal"
        );
    }
}
