//! Elements of the C0 set.
//!
//! These control functions are represented in 7-bit codes by bit combinations from `00/00` to `01/15`.
//!
//! The 3-character escape sequence designating and invoking this C0 set is `ESC 02/01 04/00`,
//! see [`ANNOUNCER_SEQUENCE`].
//!
//! It is assumed that even with no invoked C0 set, the control character `ESCAPE` ([`ESC`]) is always available, and is
//! represented by bit combination `01/11`.
//!
//! ## Usage
//!
//! You can use the Elements of the C0 set inside normal strings, format them with the `format!()` macro, or print
//! them with the `print!()` and `println!()` macros.
//!
//! For example, designate the C0 set, then ring the bell.
//!
//! ```
//! use ansi_control_codes::c0::{ANNOUNCER_SEQUENCE, BEL};
//! println!("{}{}", ANNOUNCER_SEQUENCE, BEL);
//! ```
//!
//! ## Overview of the C0 Set
//!
//! | Row Number | Column `00` | Column `01` |
//! | ---------: | :---------: | :---------: |
//! |       `00` |   [`NUL`]   |   [`DLE`]   |
//! |       `01` |   [`SOH`]   |   [`DC1`]   |
//! |       `02` |   [`STX`]   |   [`DC2`]   |
//! |       `03` |   [`ETX`]   |   [`DC3`]   |
//! |       `04` |   [`EOT`]   |   [`DC4`]   |
//! |       `05` |   [`ENQ`]   |   [`NAK`]   |
//! |       `06` |   [`ACK`]   |   [`SYN`]   |
//! |       `07` |   [`BEL`]   |   [`ETB`]   |
//! |       `08` |   [`BS`]    |   [`CAN`]   |
//! |       `09` |   [`HT`]    |   [`EM`]    |
//! |       `10` |   [`LF`]    |   [`SUB`]   |
//! |       `11` |   [`VT`]    |   [`ESC`]   |
//! |       `12` |   [`FF`]    |   [`IS4`]   |
//! |       `13` |   [`CR`]    |   [`IS3`]   |
//! |       `14` |   [`SO`]    |   [`IS2`]   |
//! |       `15` |   [`SI`]    |   [`IS1`]   |
use crate::ControlFunction;

macro_rules! c0 {
    ($xx:literal/$yy:literal) => {
        ControlFunction::new_c0(ascii!($xx / $yy))
    };
}

/// Announcer Sequence for Control Set C0.
///
/// Designate the C0 set of control functions as the active set of control functions.
///
/// ## Note 1
///
/// The use of this escape sequence implies that all control function of this C0 set must be implemented.
///
/// ## Note 2
///
/// It is assumed that even with no invoked C0 set, the control character ESCAPE (`ESC`) is available, and is
/// represented by the bit combination `01/11`.
pub const ANNOUNCER_SEQUENCE: &str = ascii!(01 / 11, 02 / 01, 04 / 00);

/// Acknowledge.
///
/// `ACK` is transmitted by a receiver as an affirmative response to the sender.
///
/// The use of `ACK` is defined in [ISO 1745][iso-1745].
///
/// [iso-1745]: https://www.ecma-international.org/wp-content/uploads/ECMA-16_2nd_edition_june_1973.pdf
pub const ACK: ControlFunction = c0!(00 / 06);

/// Bell.
///
/// `BEL` is used when there is a need to call for attention; it may control alarm or attention devices.
pub const BEL: ControlFunction = c0!(00 / 07);

/// Backspace.
///
/// `BS` causes the active data position to be moved one character position in the data component in the direction
/// opposite to that of the implicit movement.
///
/// The direction of the implicit movement depends on the parameter value of Select Implicit Movement Direction
/// ([`SIMD`][crate::control_sequences::SIMD]).
pub const BS: ControlFunction = c0!(00 / 08);

/// Cancel.
///
/// `CAN` is used to indicate that the data preceding it in the data stream is in error. As a result, this data shall be
/// ignored. The specific meaning of this control function shall be defined for each application and/or between sender
/// and recipient.
pub const CAN: ControlFunction = c0!(01 / 08);

/// Carriage Return.
///
/// The effect of `CR` depends on the setting of the DEVICE COMPONENT SELECT MODE ([`DCSM`][crate::modes::DCSM]) and
/// on the parameter value of SELECT IMPLICIT MOVEMENT DIRECTION ([`SIMD`][crate::control_sequences::SIMD]).
///
/// If the DEVICE COMPONENT SELECT MODE ([`DCSM`][crate::modes::DCSM]) is set to PRESENTATION and with the parameter
/// value of [`SIMD`][crate::control_sequences::SIMD] equal to
/// [`Normal`][crate::control_sequences::MovementDirection::Normal], `CR` causes the active presentation position to be
/// moved to the line home position of the same line in the presentation component. The line home position is
/// established by the parameter value of SET LINE HOME ([`SLH`][crate::control_sequences::SLH]).
///
/// With a parameter value of [`SIMD`][crate::control_sequences::SIMD] equal to
/// [`Opposite`][crate::control_sequences::MovementDirection::Opposite], `CR` causes the active presentation position to
/// be moved to the line limit position of the same line in the presentation component. The line limit position is
/// established by the parameter value of SET LINE LIMIT ([`SLL`][crate::control_sequences::SLL]).
///
/// If the DEVICE COMPONENT SELECT MODE ([`DCSM`][crate::modes::DCSM]) is set to DATA and with a parameter of
/// [`SIMD`][crate::control_sequences::SIMD] equal to [`Normal`][crate::control_sequences::MovementDirection::Normal],
/// `CR` causes the active data position to be moved to the line home position of the same line in the data component.
/// The line home position is established by the parameter value of SET LINE HOME
/// ([`SLH`][crate::control_sequences::SLH]).
///
/// With a parameter value of [`SIMD`][crate::control_sequences::SIMD] equal to
/// [`Opposite`][crate::control_sequences::MovementDirection::Opposite], `CR` causes the active data position to be
/// moved to the line limit position of the same line in the data component. The line limit position is established by
/// the parameter value of SET LINE LIMIT ([`SLL`][crate::control_sequences::SLL]).
pub const CR: ControlFunction = c0!(00 / 13);

/// Device Control One.
///
/// `DC1` is primarily intended for tuning on or starting an ancillary device. If it is not required for this purpose,
/// it may be used to restore a device to the basic mode of operation (see also [`DC2`] and [`DC3`]), or any other
/// device control function not provided by other DCs.
///
/// ## Note
///
/// When used for data flow control, `DC1` is sometimes called `X-ON`.
pub const DC1: ControlFunction = c0!(01 / 01);

/// Device Control Two.
///
/// `DC2` is primarily intended for tuning on or starting an ancillary device. If it is not required for this purpose,
/// it may be used to set a device to a special mode of operation (in which case [`DC1`] is used to restore the device
/// to the basic mode), or for any other device control function not provided by other DCs.
pub const DC2: ControlFunction = c0!(01 / 02);

/// Device Control Three.
///
/// `DC3` is primarily intended for turning off or stopping an ancillary device. This function may be a secondary level
/// stop, for example wait, pause, stand-by or halt (in which case [`DC1`] is used to restore normal operation). If it
/// is not required for this purpose, it may be used for any other device control function not provided by other DCs.
pub const DC3: ControlFunction = c0!(01 / 03);

/// Device Control Four.
///
/// `DC4` is primarily intended for turning off, stopping or interrupting an ancillary device. If it is not required for
/// this purpose, it may be used for any other device control function not provided by other DCs.
pub const DC4: ControlFunction = c0!(01 / 04);

/// Data Link Escape.
///
/// `DLE` is used exclusively to provide supplementary transmission control functions.
///
/// The use of `DLE` is defined in [ISO 1745][iso-1745].
///
/// [iso-1745]: https://www.ecma-international.org/wp-content/uploads/ECMA-16_2nd_edition_june_1973.pdf
pub const DLE: ControlFunction = c0!(01 / 00);

/// End Of Medium.
///
/// `EM` is used to identify the physical end of a medium, or the end of the used portion of a medium, or the end of the
/// wanted portion of data recorded on a medium.
pub const EM: ControlFunction = c0!(01 / 09);

/// Enquiry.
///
/// `ENQ` is transmitted by a sender as a request for a response from a receiver.
///
/// The use of `ENQ` is defined in [ISO 1745][iso-1745].
///
/// [iso-1745]: https://www.ecma-international.org/wp-content/uploads/ECMA-16_2nd_edition_june_1973.pdf
pub const ENQ: ControlFunction = c0!(00 / 05);

/// End Of Transmission.
///
/// `EOT` is used to indicate the conclusion of the transmission of one or more texts.
///
/// The use of `EOT` is defined in [ISO 1745][iso-1745].
///
/// [iso-1745]: https://www.ecma-international.org/wp-content/uploads/ECMA-16_2nd_edition_june_1973.pdf
pub const EOT: ControlFunction = c0!(00 / 04);

/// Escape.
///
/// `ESC` is used for code extension purposes. It causes the meanings of a limited number of bit combinations following
/// it in the data stream to be changed.
///
/// The use of `ESC` is defined in Standard [ECMA-35][ecma-35].
///
/// [ecma-35]: https://www.ecma-international.org/wp-content/uploads/ECMA-35_6th_edition_december_1994.pdf
pub const ESC: ControlFunction = c0!(01 / 11);

/// End Of Transmission Block.
///
/// `ETB` is used to indicate the end of a block of data where the data are divided into such blocks for transmission
/// purposes.
///
/// The use of `ETB` is defined in [ISO 1745][iso-1745].
///
/// [iso-1745]: https://www.ecma-international.org/wp-content/uploads/ECMA-16_2nd_edition_june_1973.pdf
pub const ETB: ControlFunction = c0!(01 / 07);

/// End Of Text.
///
/// `ETX` is used to indicate the end of a text.
///
/// The use of `ETX` is defined in [ISO 1745][iso-1745].
///
/// [iso-1745]: https://www.ecma-international.org/wp-content/uploads/ECMA-16_2nd_edition_june_1973.pdf
pub const ETX: ControlFunction = c0!(00 / 03);

/// Form Feed.
///
/// `FF` causes the active presentation position to be moved to the corresponding character position of the line at the
/// page home position of the next form or page in the presentation component. The page home position is established by
/// the parameter value of SET PAGE HOME ([`SPH`][crate::control_sequences::SPH]).
pub const FF: ControlFunction = c0!(00 / 12);

/// Character Tabulation.
///
/// `HT` causes the active presentation position to be moved to the following character tabulation stop in the
/// presentation component.
///
/// In addition, if that following character tabulation stop has been set by TABULATION ALIGN CENTRE
/// ([`TAC`][crate::control_sequences::TAC]), TABULATION ALIGN LEADING EDGE ([`TALE`][crate::control_sequences::TALE]),
/// TABULATION ALIGN TRAILING EDGE ([`TATE`][crate::control_sequences::TATE]) or TABULATION CENTRED ON CHARACTER
/// ([`TCC`][crate::control_sequences::TCC]), `HT` indicates the beginning of a string of text which is to be positioned
/// within a line according to the properties of that tabulation stop. The end of the string is indicated by the next
/// occurrence of `HT` or CARRIAGE RETURN ([`CR`]) or NEXT LINE ([`NEL`][crate::c1::NEL]) in the data stream.
pub const HT: ControlFunction = c0!(00 / 09);

/// Information Separator One (US - Unit Separator).
///
/// `IS1` is used to separate and qualify data logically; its specific meaning has to be defined for each application.
/// If this control function is used in hierarchical order, it may delimit a data item called a unit.
pub const IS1: ControlFunction = c0!(01 / 15);

/// Information Separator Two (RS - Record Separator).
///
/// `IS2` is used to separate and qualify data logically; its specific meaning has to be defined for each application.
/// If this control function is used in hierarchical order, it may delimit a data item called a record.
pub const IS2: ControlFunction = c0!(01 / 14);

/// Information Separator Three (GS - Group Separator).
///
/// `IS3` is used to separate and qualify data logically; its specific meaning has to be defined for each application.
/// If this control function is used in hierarchical order, it may delimit a data item called a group.
pub const IS3: ControlFunction = c0!(01 / 13);

/// Information Separator Four (FS - File Separator).
///
/// `IS4` is used to separate and qualify data logically; its specific meaning has to be defined for each application.
/// If this control function is used in hierarchical order, it may delimit a data item called a file.
pub const IS4: ControlFunction = c0!(01 / 12);

/// Line Feed.
///
/// If the DEVICE COMPONENT SELECT MODE ([`DCSM`][crate::modes::DCSM]) is set to PRESENTATION, `LF` causes the active
/// presentation position to be moved to the corresponding character position of the following line in the presentation
/// component.
///
/// If the DEVICE COMPONENT SELECT MODE ([`DCSM`][crate::modes::DCSM]) is set to DATA, `LF` causes the active data
/// position to be moved to the corresponding character position of the following line in the data component.
pub const LF: ControlFunction = c0!(00 / 10);

/// Locking-Shift Zero.
///
/// `LS0` is used for code extension purposes. It causes the meanings of the bit combinations following it in the data
/// stream to be changed.
///
/// The use of `LS0` is defined in Standard ECMA-35.
///
/// ## Note
///
/// `LS0` is used in 8-bit environments only; in 7-bit environments SHIFT-IN ([`SI`]) is used instead.
pub const LS0: ControlFunction = c0!(00 / 15);

/// Locking-Shift One.
///
/// `LS1` is used for code extension purposes. It causes the meanings of the bit combinations following it in the data
/// stream to be changed.
///
/// The use of `LS1` is defined in Standard [ECMA-35][ecma-35].
///
/// ## Note
///
/// `LS1` is used in 8-bit environments only; in 7-bit environments SHIFT-OUT ([`SO`]) is used instead.
///
/// [ecma-35]: https://www.ecma-international.org/wp-content/uploads/ECMA-35_6th_edition_december_1994.pdf
pub const LS1: ControlFunction = c0!(00 / 14);

/// Negative Acknowledge.
///
/// `NAK` is transmitted by a receiver as a negative response to the sender.
///
/// The use of `NAK` is defined in [ISO 1745][iso-1745].
///
/// [iso-1745]: https://www.ecma-international.org/wp-content/uploads/ECMA-16_2nd_edition_june_1973.pdf
pub const NAK: ControlFunction = c0!(01 / 05);

/// Null.
///
/// `NUL` is used for media-fill or time-fill. `NUL` characters may be inserted into, or removed from, a data stream
/// without affecting information content of that stream, but such action may affect the information layout and/or the
/// control of equipment.
pub const NUL: ControlFunction = c0!(00 / 00);

/// Shift-In.
///
/// `SI` is used for code extension purposes. It causes the meanings of the bit combinations following it in the data
/// stream to be changed.
///
/// The use of `SI` is defined in Standard [ECMA-35][ecma-35].
///
/// ## Note
///
/// `SI` is used in 7-bit environments only; in 8-bit environments LOCKING-SHIFT ZERO ([`LS0`]) is used instead.
///
/// [ecma-35]: https://www.ecma-international.org/wp-content/uploads/ECMA-35_6th_edition_december_1994.pdf
pub const SI: ControlFunction = c0!(00 / 15);

/// Shift-Out.
///
/// `SO` is used for code extension purposes. It causes the meanings of the bit combinations following it in the data
/// stream to be changed.
///
/// The use of `SI` is defined in Standard [ECMA-35][ecma-35].
///
/// ## Note
///
/// `SO` is used in 7-bit environments only; in 8-bit environments LOCKING-SHIFT ONE ([`LS1`]) is used instead.
///
/// [ecma-35]: https://www.ecma-international.org/wp-content/uploads/ECMA-35_6th_edition_december_1994.pdf
pub const SO: ControlFunction = c0!(00 / 14);

/// Start of Heading.
///
/// `SOH` is used to indicate the beginning of a heading.
///
/// The use of `SOH` is defined in [ISO 1745][iso-1745].
///
/// [iso-1745]: https://www.ecma-international.org/wp-content/uploads/ECMA-16_2nd_edition_june_1973.pdf
pub const SOH: ControlFunction = c0!(00 / 01);

/// Start of Text.
///
/// `STX` is used to indicate the beginning of a text and the ned of a heading.
///
/// The use of `STX` is defined in [ISO 1745][iso-1745].
///
/// [iso-1745]: https://www.ecma-international.org/wp-content/uploads/ECMA-16_2nd_edition_june_1973.pdf
pub const STX: ControlFunction = c0!(00 / 02);

/// Substitute.
///
/// `SUB` is used in the place of a character that has been found to be invalid or in error. `SUB` is intended to be
/// introduced by automatic means.
pub const SUB: ControlFunction = c0!(01 / 10);

/// Synchronous Idle.
///
/// `SYN` is used by a synchronous transmission system in the absence of any other character (idle condition) to provide
/// a signal from which synchronism may be achieved or retained between data terminal equipment.
///
/// The use of `SYN` is defined in [ISO 1745][iso-1745].
///
/// [iso-1745]: https://www.ecma-international.org/wp-content/uploads/ECMA-16_2nd_edition_june_1973.pdf
pub const SYN: ControlFunction = c0!(01 / 06);

/// Line Tabulation.
///
/// `VT` causes the active presentation position to be moved in the presentation component to the corresponding
/// character position on the line at which the following line tabulation stop is set.
pub const VT: ControlFunction = c0!(00 / 11);
