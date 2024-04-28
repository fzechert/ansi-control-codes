//! Elements of the C1 set.
//!
//! These control functions are represented in 7-bit codes by escape sequences of the form `ESC Fe`, where `ESC` is
//! represented by bit combination `01/11` and `Fe` is represented by a bit combination from `04/00` to `05/15`.
//!
//! The unallocated bit combinations are reserved for future standardization and shall not be used.
//!
//! The 3-character escape sequence designating and invoking this c1 set is `ESC 02/06 04/00` and `ESC 02/02 F`.
//! see [`ANNOUNCER_SEQUENCE`], and [`ALTERNATIVE_ANNOUNCER_SEQUENCE`].
//!
//! ## Usage
//!
//! You can use the Elements of the C1 set inside normal strings, format them with the `format!()` macro, or print
//! them with the `print!()` and `println!()` macros.
//!
//! For example, designate the C1 set, then set a character tabulation stop.
//!
//! ```
//! use ansi_control_codes::c1::{ANNOUNCER_SEQUENCE, HTS};
//! println!("{}{}", ANNOUNCER_SEQUENCE, HTS);
//! ```
//!
//! ## Overview of the C1 Set
//!
//! | Row Number | Column `04` | Column `05` |
//! | ---------: | :---------: | :---------: |
//! |       `00` |     --      |   [`DCS`]   |
//! |       `01` |     --      |   [`PU1`]   |
//! |       `02` |   [`BPH`]   |   [`PU2`]   |
//! |       `03` |   [`NBH`]   |   [`STS`]   |
//! |       `04` |     --      |   [`CCH`]   |
//! |       `05` |   [`NEL`]   |   [`MW`]    |
//! |       `06` |   [`SSA`]   |   [`SPA`]   |
//! |       `07` |   [`ESA`]   |   [`EPA`]   |
//! |       `08` |   [`HTS`]   |   [`SOS`]   |
//! |       `09` |   [`HTJ`]   |     --      |
//! |       `10` |   [`VTS`]   |   [`SCI`]   |
//! |       `11` |   [`PLD`]   |   [`CSI`]   |
//! |       `12` |   [`PLU`]   |    [`ST`]   |
//! |       `13` |    [`RI`]   |   [`OSC`]   |
//! |       `14` |   [`SS2`]   |    [`PM`]   |
//! |       `15` |   [`SS3`]   |   [`APC`]   |

use crate::ControlFunction;

macro_rules! c1 {
    ($xx:literal/$yy:literal) => {
        ControlFunction::new_c1(ascii!($xx / $yy))
    };
}

/// Announcer Sequence for C1.
///
/// Designate the C1 set of control functions as the active set of control functions.
///
/// ## Note
///
/// The use of this escape sequence implies that all control function of this C1 set must be implemented.
pub const ANNOUNCER_SEQUENCE: &str = ascii!(01 / 11, 02 / 06, 04 / 00);

/// Alternative Announcer Sequence for C1.
///
/// Designate the C1 set of control functions as the active set of control functions.
///
/// ## Note
///
/// The use of this escape sequence implies that all control function of this C1 set must be implemented.
pub const ALTERNATIVE_ANNOUNCER_SEQUENCE: &str = ascii!(01 / 11, 02 / 02, 04 / 06);

/// Application Program Command.
///
/// `APC` is used as the opening delimiter of a control string for application program use. The command string following
/// may consist of bit combinations in the range `00/08` to `00/13` and `02/00` to `07/14`. The control string is closed
/// by the terminating delimiter String Terminator ([`ST`]). The interpretation of the command string depends on the
/// relevant application program.
pub const APC: ControlFunction = c1!(05 / 15);

/// Break Permitted Here.
///
/// `BPH` is used to indicate a point where a line break may occur when text is formatted. `BPH` may occur between two
/// graphic characters, either or both of which may be `SPACE`.
pub const BPH: ControlFunction = c1!(04 / 02);

/// Cancel Character.
///
/// `CCH` is used to indicate that both the preceding graphic character in the data stream (represented by one or more
/// bit combinations), including `SPACE`, and the control function `CCH` itself are to be ignored for further
/// interpretation of the data stream.
///
/// If the character preceding `CCH` in the data stream is a control function (represented by one or more bit
/// combinations), the effect of `CCH` is not defined.
pub const CCH: ControlFunction = c1!(05 / 04);

/// Control Sequence Introducer.
///
/// `CSI` is used as the first character of a control sequence. See [control_sequences][crate::control_sequences].
pub const CSI: ControlFunction = c1!(05 / 11);

/// Device Control String.
///
/// `DCS` is used as the opening delimiter of a control string for device control use. The command string following may
/// consist of bit combinations in the range `00/08` to `00/13` and `02/00` to `07/14`. The control string is closed by
/// the terminating delimiter STRING TERMINATOR ([`ST`]).
///
/// The command string represents either one or more commands for the receiving device, or one or more status reports
/// from the sending device. The purpose and the format of the command string are specified by the most recent
/// occurrence of IDENTIFY DEVICE CONTROL STRING ([`IDCS`][crate::control_sequences::IDCS]), if any, or depend on the
/// sending and/or the receiving device.
pub const DCS: ControlFunction = c1!(05 / 00);

/// End Of Guarded Area.
///
/// `EPA` is used to indicate that the active presentation position is the last of a string of character positions in
/// the presentation component, the contents of which are protected against manual alteration, are guarded against
/// transmission or transfer, depending on the setting of GUARDED AREA TRANSFER MODE ([`GATM`][crate::modes::GATM]),
/// and may be protected against erasure, depending on the setting of the ERASURE MODE ([`ERM`][crate::modes::ERM]).
/// The beginning of this string is indicated by START OF GUARDED AREA ([`SPA`]).
///
/// ## Note
///
/// The control functions for area definition ([`DAQ`][crate::control_sequences::DAQ], [`EPA`], [`ESA`], [`SPA`],
/// [`SSA`]) should not be used within an [`SRS`][crate::control_sequences::SRS] string or an
/// [`SDS`][crate::control_sequences::SDS] string.
pub const EPA: ControlFunction = c1!(05 / 07);

/// End Of Selected Area.
///
/// `ESA` is used to indicate that the active presentation position is the last of a string of character positions in
/// the presentation component, the contents of which are eligible to be transmitted in the form of a data stream or
/// transferred to an auxiliary input/output device. The beginning of the string is indicated by START OF SELECTED
/// AREA ([`SSA`])
///
/// ## Note
///
/// The control functions for area definition ([`DAQ`][crate::control_sequences::DAQ], [`EPA`], [`ESA`], [`SPA`],
/// [`SSA`]) should not be used within an [`SRS`][crate::control_sequences::SRS] string or an
/// [`SDS`][crate::control_sequences::SDS] string.
pub const ESA: ControlFunction = c1!(04 / 07);

/// Character Tabulation With Justification.
///
/// `HTJ` causes the contents of the active field (the field in the presentation component that contains the active
/// presentation position) to be shifted forward so that it ends at the character position preceding the following
/// character tabulation stop. The active presentation position is moved to that following character tabulation stop.
/// The character positions which precede the beginning of the shifted string are put into the erased state.
pub const HTJ: ControlFunction = c1!(04 / 09);

/// Character Tabulation Set.
///
/// `HTS` causes a character tabulation stop to be set at the active presentation position in the presentation
/// component.
///
/// The number of liens affected depends on the setting of the TABULATION STOP MODE ([`TSM`][crate::modes::TSM]).
pub const HTS: ControlFunction = c1!(04 / 08);

/// Message Waiting.
///
/// `MW` is used to set a message waiting indicator in the receiving device. An appropriate acknowledgement to the
/// receipt of `MW` may be given by using DEVICE STATUS REPORT ([`DSR`][crate::control_sequences::DSR]).
pub const MW: ControlFunction = c1!(05 / 05);

/// No Break Here.
///
/// `NBH` is used to indicate a point where a line break shall not occur when text is formatted. `NBH` may occur between
/// two graphic characters either or both of which may be `SPACE`.
pub const NBH: ControlFunction = c1!(04 / 03);

/// Next Line.
///
/// The effect of `NEL` depends on the setting of the DEVICE COMPONENT SELECT MODE ([`DCSM`][crate::modes::DCSM]) and on
/// the parameter value of SELECT IMPLICIT MOVEMENT DIRECTION ([`SIMD`][crate::control_sequences::SIMD]).
///
/// If the DEVICE COMPONENT SELECT MODE ([`DCSM`][crate::modes::DCSM]) is set to PRESENTATION and with a parameter value
/// of [`SIMD`][crate::control_sequences::SIMD] equal to
/// [`Normal`][crate::control_sequences::MovementDirection::Normal], `NEL` causes the active presentation position to be
/// moved to the line home position of the following line in the presentation component. The line home position is
/// established by the parameter value of SET LINE HOME ([`SLH`][crate::control_sequences::SLH]).
///
/// With a parameter value of [`SIMD`][crate::control_sequences::SIMD] equal to
/// [`Opposite`][crate::control_sequences::MovementDirection::Opposite], `NEL` causes the active presentation position
/// to be moved to the line limit position of the following line in the presentation component. The line limit position
/// is established by the parameter value of SET LINE LIMIT ([`SLL`][crate::control_sequences::SLL]).
///
/// If the DEVICE COMPONENT SELECT MODE ([`DCSM`][crate::modes::DCSM]) is set to DATA and with a parameter value of
/// [`SIMD`][crate::control_sequences::SIMD] equal to [`Normal`][crate::control_sequences::MovementDirection::Normal],
/// `NEL` causes the active data position to be moved to the line home position of the following line in the data
/// component. The line home position is established by the parameter value of SET LINE HOME
/// ([`SLH`][crate::control_sequences::SLH]).
///
/// With a parameter value of [`SIMD`][crate::control_sequences::SIMD] equal to
/// [`Opposite`][crate::control_sequences::MovementDirection::Opposite], `NEL` causes the active data position to be
/// moved to the line limit position of the following line in the data component. The line limit position is established
/// by the parameter value of SET LINE LIMIT ([`SLL`][crate::control_sequences::SLL]).
pub const NEL: ControlFunction = c1!(04 / 05);

/// Operating System Command
///
/// `OSC` is used as the opening delimiter of a control string for operating system use. The command string following
/// may consist of a sequence of bit combinations in the range `00/08` to `00/13` and `02/00` to `07/14`. The control
/// string is closed by the terminating delimiter STRING TERMINATOR ([`ST`]). The interpretation of the command string
/// depends on the relevant operating system.
pub const OSC: ControlFunction = c1!(05 / 13);

/// Partial Line Forward.
///
/// `PLD` causes the active presentation position to be moved in the presentation component to the corresponding
/// position of an imaginary line with a partial offset in the direction of the line progression. This offset should be
/// sufficient either to image following characters as subscripts until the first following occurrence of PARTIAL LINE
/// BACKWARD ([`PLU`]) in the data stream, or, if preceding characters were imaged as superscripts, to restore imaging
/// of following characters to the active line (the line that contains the active presentation position).
///
/// Any interactions between `PLD` and format effectors other than [`PLU`] are not defined.
pub const PLD: ControlFunction = c1!(04 / 11);

/// Partial Line Backwards.
///
/// `PLU` causes the active presentation position to be moved in the presentation component to the corresponding
/// position of an imaginary line with a partial offset in the direction opposite to that of the line progression. This
/// offset should be sufficient either to image following characters as superscripts until the first following
/// occurrence of PARTIAL LINE FORWARD ([`PLD`]) in the data stream, or, if preceding characters were imaged as
/// subscripts, to restore imaging of following characters to the active line (the line that contains the active
/// presentation position).
///
/// Any interactions between `PLU` and format effectors other than [`PLD`] are not defined.
pub const PLU: ControlFunction = c1!(04 / 12);

/// Privacy Message.
///
/// `PM` is used as the opening delimiter of a control string for privacy message use. The command string following may
/// consist of a sequence of bit combinations in the range `00/08` to `00/13` and `02/00` to `07/14`. The control string
/// is closed by the terminating delimiter STRING TERMINATOR ([`ST`]). The interpretation of the command string depends
/// on the relevant privacy discipline.
pub const PM: ControlFunction = c1!(05 / 14);

/// Private Use One.
///
/// `PU1` is reserved for a function without standardized meaning for private use as required, subject to the prior
/// agreement between the sender and the recipient of the data.
pub const PU1: ControlFunction = c1!(05 / 01);

/// Private Use Two.
///
/// `PU2` is reserved for a function without standardized meaning for private use as required, subject to the prior
/// agreement between the sender and the recipient of the data.
pub const PU2: ControlFunction = c1!(05 / 02);

/// Reverse Line Feed.
///
/// If the DEVICE COMPONENT SELECT MODE ([`DCSM`][crate::modes::DCSM]) is set to PRESENTATION, `RI` causes the active
/// presentation position to be moved in the presentation component to the corresponding character position of the
/// preceding line feed.
///
/// If the DEVICE COMPONENT SELECT MODE ([`DCSM`][crate::modes::DCSM]) is set to DATA, `RI` causes the active data
/// position to be moved in the data component to the corresponding character position of the preceding line.
pub const RI: ControlFunction = c1!(04 / 13);

/// Single Character Introducer.
///
/// `SCI` and the bit combination following it are used to represent a control function or a graphic character. The bit
/// combination following `SCI` must be from `00/08` to `00/13` or `02/00` to `07/14`. The use of `SCI` is reserved
/// for future standardization.
pub const SCI: ControlFunction = c1!(05 / 10);

/// Start of String.
///
/// `SOS` is used as the opening delimiter of a control string. The character string following may consist of any bit
/// combination, except those representing `SOS` or STRING TERMINATOR ([`ST`]). The control string is closed by the
/// terminating delimiter STRING TERMINATOR ([`ST`]). The interpretation of the character string depends on the
/// application.
pub const SOS: ControlFunction = c1!(05 / 08);

/// Start of Guarded Area.
///
/// `SPA` is used to indicate that the active presentation position is the first of a string of character positions in
/// the presentation component, the contents of which are protected against manual alteration, are guarded against
/// transmission or transfer, depending on the setting of the GUARDED AREA TRANSFER MODE ([`GATM`][crate::modes::GATM])
/// and may be protected against erasure, depending on the setting of the ERASURE MODE ([`ERM`][crate::modes::ERM]).
/// The end of this string is indicated by END OF GUARDED AREA (`EPA`).
///
/// ## Note
///
/// The control functions for area definition ([`DAQ`][crate::control_sequences::DAQ], [`EPA`], [`ESA`], [`SPA`],
/// [`SSA`]) should not be used within an [`SRS`][crate::control_sequences::SRS] string or an
/// [`SDS`][crate::control_sequences::SDS] string.
pub const SPA: ControlFunction = c1!(05 / 06);

/// Start of Selected Area.
///
/// `SSA` is used to indicate that the active presentation position is the first of a string of character positions in
/// the presentation component, the contents of which are eligible to be transmitted in the form of a data stream or
/// transferred to an auxiliary input/output device.
///
/// The end of this string is indicated by END OF SELECTED AREA ([`ESA`]). The string of characters actually transmitted
/// or transferred depends on the setting of the GUARDED AREA TRANSFER MODE ([`GATM`][crate::modes::GATM]) and on any
/// guarded areas established by DEFINE AREA QUALIFICATION ([`DAQ`][crate::control_sequences::DAQ]), or by START OF
/// GUARDED AREA ([`SPA`]) and END OF GUARDED AREA ([`EPA`]).
///
/// ## Note
///
/// The control functions for area definition ([`DAQ`][crate::control_sequences::DAQ], [`EPA`], [`ESA`], [`SPA`],
/// [`SSA`]) should not be used within an [`SRS`][crate::control_sequences::SRS] string or an
/// [`SDS`][crate::control_sequences::SDS] string.
pub const SSA: ControlFunction = c1!(04 / 06);

/// Single-Shift Two.
///
/// `SS2` is used for code extension purposes. It causes the meanings of the bit combinations following it in the data
/// stream to be changed.
///
/// The use of `SS2` is defined in Standard [ECMA-35][ecma-35].
///
/// [ecma-35]: https://www.ecma-international.org/wp-content/uploads/ECMA-35_6th_edition_december_1994.pdf
pub const SS2: ControlFunction = c1!(04 / 14);

/// Single-Shift Three.
///
/// `SS3` is used for code extension purposes. It causes the meanings of the bit combinations following it in the data
/// stream to be changed.
///
/// The use of `SS3` is defined in Standard [ECMA-35][ecma-35].
///
/// [ecma-35]: https://www.ecma-international.org/wp-content/uploads/ECMA-35_6th_edition_december_1994.pdf
pub const SS3: ControlFunction = c1!(04 / 15);

/// String Terminator.
///
/// `ST` is used as the closing delimiter of a control string opened by APPLICATION PROGRAM COMMAND ([`APC`]), DEVICE
/// CONTROL STRING ([`DCS`]), OPERATING SYSTEM COMMAND ([`OSC`]), PRIVACY MESSAGE ([`PM`]), or START OF STRING
/// ([`SOS`]).
pub const ST: ControlFunction = c1!(05 / 12);

/// Set Transmit State.
///
/// `STS` is used to establish the transmit state in the receiving device. In this state the transmission of data from
/// the device is possible.
///
/// The actual initiation of transmission of data is performed by a data communication or input/output interface control
/// procedure which is outside the scope of this Standard.
///
/// The transmit state is established either by `STS` appearing in the received data stream or by the operation of an
/// appropriate key on a keyboard.
pub const STS: ControlFunction = c1!(05 / 03);

/// Line Tabulation Set.
///
/// `VTS` causes a line tabulation stop to be set at the active line (the line that contains the active presentation
/// position).
pub const VTS: ControlFunction = c1!(04 / 10);
