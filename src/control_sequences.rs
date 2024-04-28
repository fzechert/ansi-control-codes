//! Control Sequences.
//!
//! A control sequence is a string of bit combinations starting with the control function Control Sequence Introducer
//! (`CSI`), followed by one or more bit combinations representing parameters, if any, and by one or more bit
//! combinations identifying the control function. The control function `CSI` itself is an element of the
//! [C1][crate::c1] set.
//!
//! ## Usage
//!
//! You can use the control sequences inside normal strings, format them with the `format!()` macro, or print
//! them with the `print!()` and `println!()` macros.
//!
//! For example, move the cursor to line 5, character 13:
//!
//! ```
//! use ansi_control_codes::control_sequences::CUP;
//! print!("{}", CUP(Some(5), Some(13)));
//! ```
//!
//! ## Overview of the Control Sequences
//!
//! ### Without Intermediate Bytes
//!
//! | Row Number | Column `04` | Column `05` | Column `06` |
//! | ---------: | :---------: | :---------: | :---------: |
//! |       `00` |   [`ICH`]   |   [`DCH`]   |   [`HPA`]   |
//! |       `01` |   [`CUU`]   |   [`SEE`]   |   [`HPR`]   |
//! |       `02` |   [`CUD`]   |   [`CPR`]   |   [`REP`]   |
//! |       `03` |   [`CUF`]   |   [`SU`]    |   [`DA`]    |
//! |       `04` |   [`CUB`]   |   [`SD`]    |   [`VPA`]   |
//! |       `05` |   [`CNL`]   |   [`NP`]    |   [`VPR`]   |
//! |       `06` |   [`CPL`]   |   [`PP`]    |   [`HVP`]   |
//! |       `07` |   [`CHA`]   |   [`CTC`]   |   [`TBC`]   |
//! |       `08` |   [`CUP`]   |   [`ECH`]   |   [`SM`]    |
//! |       `09` |   [`CHT`]   |   [`CVT`]   |   [`MC`]    |
//! |       `10` |   [`ED`]    |   [`CBT`]   |   [`HPB`]   |
//! |       `11` |   [`EL`]    |   [`SRS`]   |   [`VPB`]   |
//! |       `12` |   [`IL`]    |   [`PTX`]   |   [`RM`]    |
//! |       `13` |   [`DL`]    |   [`SDS`]   |   [`SGR`]   |
//! |       `14` |   [`EF`]    |   [`SIMD`]  |   [`DSR`]   |
//! |       `15` |   [`EA`]    |      --     |   [`DAQ`]   |
//!
//! ### With Intermediate Bytes `02/00`
//!
//! | Row Number | Column `04` | Column `05` | Column `06` |
//! | ---------: | :---------: | :---------: | :---------: |
//! |       `00` |   [`SL`]    |   [`PPA`]   |  [`TATE`]   |
//! |       `01` |   [`SR`]    |   [`PPR`]   |  [`TALE`]   |
//! |       `02` |   [`GSM`]   |   [`PPB`]   |   [`TAC`]   |
//! |       `03` |   [`GSS`]   |   [`SPD`]   |   [`TCC`]   |
//! |       `04` |   [`FNT`]   |   [`DTA`]   |   [`TSR`]   |
//! |       `05` |   [`TSS`]   |   [`SLH`]   |   [`SCO`]   |
//! |       `06` |   [`JFY`]   |   [`SLL`]   |  [`SRCS`]   |
//! |       `07` |   [`SPI`]   |   [`FNK`]   |   [`SCS`]   |
//! |       `08` |  [`QUAD`]   |  [`SPQR`]   |   [`SLS`]   |
//! |       `09` |   [`SSU`]   |   [`SEF`]   |     --      |
//! |       `10` |   [`PFS`]   |   [`PEC`]   |     --      |
//! |       `11` |   [`SHS`]   |   [`SSW`]   |   [`SCP`]   |
//! |       `12` |   [`SVS`]   |  [`SACS`]   |     --      |
//! |       `13` |   [`IGS`]   |  [`SAPV`]   |     --      |
//! |       `14` |     --      |  [`STAB`]   |     --      |
//! |       `15` |  [`IDCS`]   |   [`GCC`]   |     --      |
//!
//! ## Note
//!
//! As intended by the standard, notation of control functions is kept identical to the definitions in the standard.
//! This means that functions in this rust module will not follow the standard snake_case rust naming convention, but
//! instead follow the ECMA standard. This is intended.
#![allow(non_snake_case)]

use crate::{modes::Mode, ControlFunction};

macro_rules! sequence {
    // numeric control sequence with no intermediate byte and no default value
    ($xx:literal / $yy:literal, numeric $param:ident) => {
        ControlFunction::new_sequence(ascii!($xx / $yy), vec![$param.to_string()])
    };
    // numeric control sequence with no intermediate byte and default value
    ($xx:literal / $yy:literal, numeric $param:ident, default $default:literal) => {
        ControlFunction::new_sequence(
            ascii!($xx / $yy),
            vec![$param.unwrap_or($default).to_string()],
        )
    };
    // numeric control sequence with no intermediate byte, two parameters and default values
    ($xx:literal / $yy:literal, numeric $param1:ident, default $default1:literal, numeric $param2:ident, default $default2:literal) => {
        ControlFunction::new_sequence(
            ascii!($xx / $yy),
            vec![
                $param1.unwrap_or($default1).to_string(),
                $param2.unwrap_or($default2).to_string(),
            ],
        )
    };
    // selective control sequence with no intermediate byte and default value
    ($xx:literal / $yy:literal, selective default $param:ident) => {
        ControlFunction::new_sequence(
            ascii!($xx / $yy),
            vec![($param.unwrap_or_default() as u32).to_string()],
        )
    };
    // selective control sequence with intermediate byte and default value
    ($xx1:literal / $yy1:literal, $xx2:literal / $yy2:literal, selective default $param:ident) => {
        ControlFunction::new_sequence(
            ascii!($xx1 / $yy1, $xx2 / $yy2),
            vec![($param.unwrap_or_default() as u32).to_string()],
        )
    };
    // selective control sequence with intermediate byte and two default value
    ($xx1:literal / $yy1:literal, $xx2:literal / $yy2:literal, selective default $param1:ident, selective default $param2:ident) => {
        ControlFunction::new_sequence(
            ascii!($xx1 / $yy1, $xx2 / $yy2),
            vec![
                ($param1.unwrap_or_default() as u32).to_string(),
                ($param2.unwrap_or_default() as u32).to_string(),
            ],
        )
    };
    // numeric control sequence with intermediate byte, one parameters, and no default value
    ($xx1:literal / $yy1:literal, $xx2:literal / $yy2:literal, numeric $param:ident) => {
        ControlFunction::new_sequence(ascii!($xx1 / $yy1, $xx2 / $yy2), vec![$param.to_string()])
    };
    // numeric control sequence with intermediate byte, one parameters, and default value
    ($xx1:literal / $yy1:literal, $xx2:literal / $yy2:literal, numeric $param:ident, default $default:literal) => {
        ControlFunction::new_sequence(
            ascii!($xx1 / $yy1, $xx2 / $yy2),
            vec![$param.unwrap_or($default).to_string()],
        )
    };
    // numeric control sequence with intermediate byte, two parameters, and no default value
    ($xx1:literal / $yy1:literal, $xx2:literal / $yy2:literal, numeric $param1:ident, numeric $param2:ident) => {
        ControlFunction::new_sequence(
            ascii!($xx1 / $yy1, $xx2 / $yy2),
            vec![$param1.to_string(), $param2.to_string()],
        )
    };
    // numeric control sequence with intermediate byte, two parameters, and default values
    ($xx1:literal / $yy1:literal, $xx2:literal / $yy2:literal, numeric $param1:ident, default $default1:literal, numeric $param2:ident, default $default2:literal) => {
        ControlFunction::new_sequence(
            ascii!($xx1 / $yy1, $xx2 / $yy2),
            vec![
                $param1.unwrap_or($default1).to_string(),
                $param2.unwrap_or($default2).to_string(),
            ],
        )
    };
    // control sequence with variadic number of selective arguments
    ($xx:literal / $yy: literal, variadic selective $vector:expr) => {
        ControlFunction::new_sequence(
            ascii!($xx / $yy),
            $vector.iter().map(|e| (*e as u32).to_string()).collect(),
        )
    };
}

/// Cursor Backward Tabulation.
///
/// `CBT` causes the active presentation position to be moved to the character position corresponding to the `n`-th
/// preceding character tabulation stop in the presentation component, according to the character path.
///
/// Default value for `n` is `1`.
pub fn CBT(n: Option<u32>) -> ControlFunction<'static> {
    sequence!(05 / 10, numeric n, default 1)
}

/// Cursor Character Absolute.
///
/// `CHA` causes the active presentation position to be moved to character position `n` in the active line in the
/// presentation component.
///
/// Default value for `n` is `1`.
pub fn CHA(n: Option<u32>) -> ControlFunction<'static> {
    sequence!(04 / 07, numeric n, default 1)
}

/// Cursor Forward Tabulation.
///
/// `CHT` causes the active presentation position to be moved to the character position corresponding to the `n`-th
/// following character tabulation stop in the presentation component, according to the character path.
///
/// Default value for `n` is `1`.
pub fn CHT(n: Option<u32>) -> ControlFunction<'static> {
    sequence!(04 / 09, numeric n, default 1)
}

/// Cursor Next Line.
///
/// `CNL` causes the active presentation position to be moved to the first character position of the `n`-th following
/// line in the presentation component.
///
/// Default value for `n` is `1`.
pub fn CNL(n: Option<u32>) -> ControlFunction<'static> {
    sequence!(04 / 05, numeric n, default 1)
}

/// Cursor Preceding Line.
///
/// `CPL` causes the active presentation position to be moved to the first character position of the `n`-th preceding
/// line in the presentation component.
///
/// Default value for `n` is `1`.
pub fn CPL(n: Option<u32>) -> ControlFunction<'static> {
    sequence!(04 / 06, numeric n, default 1)
}

/// Active Position Report.
///
/// If the DEVICE COMPONENT SELECT MODE ([`DCSM`][crate::modes::DCSM]) is set to PRESENTATION, `CPR` is used to report
/// the active presentation position of the sending device as residing in the presentation component at the `n`-th line
/// position according to the line progression and at the `m`-th character position according to the character path.
///
/// If the DEVICE COMPONENT SELECT MODE ([`DCSM`][crate::modes::DCSM]) is set to DATA, `CPR` is used to report the
/// active data position of the sending device as residing in the data component at the `n`-th line position according
/// to the line progression and at the `m`-th character position according to the character progression.
///
/// `CPR` may be solicited by a DEVICE STATUS REPORT ([`DSR`]) or be sent unsolicited.
///
/// Default value for `n` and `m` is `1`.
pub fn CPR(n: Option<u32>, m: Option<u32>) -> ControlFunction<'static> {
    sequence!(05 / 02, numeric n, default 1, numeric m, default 1)
}

/// Valid parameter values to the function [`CTC`].
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum TabulationControl {
    /// A character tabulation stop is set at the active presentation position.
    #[default]
    SetCharacterTabulationStop = 0,

    /// A line tabulation stop is set at the active line (the line that contains the active presentation position).
    SetLineTabulationStop,

    /// The character tabulation stop at the active presentation position is cleared.
    ClearCharacterTabulationStop,

    /// The line tabulation stop at the active line is cleared.
    ClearLineTabulationStop,

    /// All character tabulation stops in the active line are cleared.
    ClearCharacterTabulationStopsInLine,

    /// All character tabulation stops are cleared.
    ClearAllCharacterTabulationStops,

    /// All line tabulation stops are cleared.
    ClearAllLineTabulationStops,
}

/// Cursor Tabulation Control.
///
/// `CTC` causes one or more tabulation stops to be set or cleared in the presentation component, depending on the
/// parameter value.
///
/// Default value for `s` is [`TabulationControl::SetCharacterTabulationStop`].
pub fn CTC(s: Option<TabulationControl>) -> ControlFunction<'static> {
    sequence!(05 / 07, selective default s)
}

/// Cursor Left.
///
/// `CUB` causes the active presentation position to be moved leftwards in the presentation component by `n` character
/// positions, if the character path is horizontal, or by `n` line positions if the character path is vertical.
///
/// Default value for `n` is `1`.
pub fn CUB(n: Option<u32>) -> ControlFunction<'static> {
    sequence!(04 / 04, numeric n, default 1)
}

/// Cursor Down.
///
/// `CUD` causes the active presentation position to be moved downwards in the presentation component by `n` line
/// positions, if the character path is horizontal, or by `n` character positions if the character path is vertical.
///
/// Default value for `n` is `1`.
pub fn CUD(n: Option<u32>) -> ControlFunction<'static> {
    sequence!(04 / 02, numeric n, default 1)
}

/// Cursor Right.
///
/// `CUF` causes the active presentation position to be moved rightwards in the presentation component by `n` character
/// positions, if the character path is horizontal, or by `n` line positions if the character path is vertical.
///
/// Default value for `n` is `1`.
pub fn CUF(n: Option<u32>) -> ControlFunction<'static> {
    sequence!(04 / 03, numeric n, default 1)
}

/// Cursor Position.
///
/// `CUP` causes the active presentation position to be moved in the presentation component ot the `n`-th line position
/// according to the line progression, and to the `m`-th character position according to the character path.
///
/// Default value for `n` and `m` is `1`.
pub fn CUP(n: Option<u32>, m: Option<u32>) -> ControlFunction<'static> {
    sequence!(04 / 08, numeric n, default 1, numeric m, default 1)
}

/// Cursor Up.
///
/// `CUU` causes the active presentation position to be moved upwards in the presentation component by `n` line
/// positions, if the character path is horizontal, or by `n` character positions if the character path is vertical.
///
/// Default value for `n` is `1`.
pub fn CUU(n: Option<u32>) -> ControlFunction<'static> {
    sequence!(04 / 01, numeric n, default 1)
}

/// Cursor Line Tabulation.
///
/// `CVT` causes the active presentation position to be moved to the character position of the line corresponding to
/// the `n`-th following line tabulation stop in the presentation component.
///
/// Default value for `n` is `1`.
pub fn CVT(n: Option<u32>) -> ControlFunction<'static> {
    sequence!(05 / 09, numeric n, default 1)
}

/// Valid parameter values to the function [`DA`].
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum DeviceAttributes {
    /// Request identifying device attributes from a device.
    #[default]
    Request,

    /// Device attributes identification code.
    Identify(u32),
}

/// Device Attributes.
///
/// With a parameter [`DeviceAttributes::Identify`] (not equal to 0), `DA` is used to identify the device which sends
/// the `DA`. The parameter value is a device type identification code according to a register which is to be
/// established.
///
/// If the parameter value is [`DeviceAttributes::Request`], `DA` is used to request an identifying `DA` from a device.
///
/// Default value for `s` is [`DeviceAttributes::Request`].
pub fn DA(s: Option<DeviceAttributes>) -> ControlFunction<'static> {
    let v = match s {
        Some(DeviceAttributes::Request) => 0,
        Some(DeviceAttributes::Identify(x)) => x,
        None => 0,
    };
    sequence!(06 / 03, numeric v)
}

/// Valid parameter values to the function [`DAQ`].
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum AreaQualification {
    /// Unprotected and unguarded.
    #[default]
    UnprotectedUnguarded = 0,

    /// Protected and guarded.
    ProtectedGuarded,

    /// Graphic character input.
    GraphicCharacterInput,

    /// Numeric input.
    NumericInput,

    /// Alphabetic input.
    AlphabeticInput,

    /// Input aligned on the last character position of the qualified area.
    InputAlignedRight,

    /// Fill with ZEROs.
    FillZeros,

    /// Set a character tabulation stop at the active presentation position (the first character position of the
    /// qualified area) to indicate the beginning of a field.
    SetCharacterTabulationStop,

    /// Protected and unguarded
    ProtectedUnguarded,

    /// Fill with SPACEs
    FillSpaces,

    /// Input aligned on the first character position of the qualified area.
    InputAlignedLeft,

    /// The order of the character positions in the input field is reserved, i.e. the last position in each line becomes
    /// the first and vice versa; input begins at the new first position.
    Reversed,
}

/// Define Area Qualification.
///
/// `DAQ` is used to indicate that the active presentation position in the presentation component is the first character
/// position of a qualified area. The last character position of the qualified area is the character position in the
/// presentation component immediately preceding the first character position of the following qualified area.
///
/// The control function operates independently of the setting of the TABULATION STOP MODE ([`TSM`][crate::modes::TSM]).
/// The character tabulation stop set by parameter value [`AreaQualification::SetCharacterTabulationStop`] applies to
/// the active line only.
///
/// The default value for `s` is [`AreaQualification::UnprotectedUnguarded].
///
/// ## Note
///
/// The control functions for area definitions ([`DAQ`], [`EPA`][crate::c1::EPA], [`ESA`][crate::c1::ESA],
/// [`SPA`][crate::c1::SPA], [`SSA`][crate::c1::SSA]) should not be used within an [`SRS`] string or an [`SDS`] string.
pub fn DAQ(s: Option<AreaQualification>) -> ControlFunction<'static> {
    sequence!(06 / 15, selective default s)
}

/// Delete Character.
///
/// If the DEVICE COMPONENT SELECT MODE ([`DSCM`][crate::modes::DCSM]) is set to PRESENTATION, `DCH` causes the contents
/// of the active presentation position and, depending on the setting of the CHARACTER EDITING MODE
/// ([`HEM`][crate::modes::HEM]), the contents of the `n-1` preceding or following character positions to be removed
/// from the presentation component. The resulting gap is closed by shifting the contents of the adjacent character
/// positions towards the active presentation position. At the other end of the shifted part, `n` character positions
/// are put into the erased state.
///
/// The extent of the shifted part is established by SELECT EDITING EXTEND ([`SEE`]).
///
/// The effect of `DCH` on the start or end of a selected area, the start or end of a qualified area, or a tabulation
/// stop in the shifted part is undefined.
///
/// If the DEVICE COMPONENT SELECT MODE ([`DCSM`][crate::modes::DCSM]) is set to DATA, `DCH` causes the contents of the
/// active data position and, depending on the setting of the CHARACTER EDITING MODE ([`HEM`][crate::modes::HEM]), the
/// contents of the `n-1` preceding or following character positions to be removed from the data component. The
/// resulting gap is closed by shifting the contents of the adjacent character positions towards the active data
/// position. At the other end of the shifted part, `n` character positions are put into the erased state.
///
/// Default value for `n` is `1`.
pub fn DCH(n: Option<u32>) -> ControlFunction<'static> {
    sequence!(05 / 00, numeric n, default 1)
}

/// Delete Line.
///
/// If the DEVICE COMPONENT SELECT MODE ([`DCSM`][crate::modes::DCSM]) is set to PRESENTATION, `DL` causes the contents
/// of the active line (the line that contains the active presentation position) and, depending on the setting of the
/// LINE EDITING MODE ([`VEM`][crate::modes::VEM]), the contents of the `n-1` preceding or following lines to be removed
/// from the presentation component. The resulting gap is closed by shifting the contents of a number of adjacent lines
/// towards the active line. At the other end of the shifted part, `n` lines are put into the erased state.
///
/// The active presentation position is moved to the line home position in the active line. The line home position is
/// established by the parameter value of SET LINE HOME ([`SLH`]). If the TABULATION STOP MODE
/// ([`TSM`][crate::modes::TSM]) is set to SINGLE, character tabulation stops are cleared in the lines that are put into
/// the erased state.
///
/// The extent of the shifted part is established by SELECT EDITING EXTEND ([`SEE`]).
///
/// Any occurrences of the start or end of a selected area, the start or end of a qualified area, or a tabulation stop
/// in the shifted part, are also shifted.
///
/// If the DEVICE COMPONENT SELECT MODE ([`DCSM`][crate::modes::DCSM]) is set to DATA, `DL` causes the contents of the
/// active line (the line that contains the active data position) and, depending on the setting of the LINE EDITING MODE
/// ([`VEM`][crate::modes::VEM]), the contents of the `n-1` preceding or following lines to be removed from the data
/// component. The resulting gap is closed by shifting the contents of a number of adjacent lines towards the active
/// line. At the other end of the shifted part, `n` lines are put into the erased state. The active data position is
/// moved to the lines home position in the active line. The line home position is established by the parameter value of
/// SET LINE HOME ([`SLH`]).
///
/// The default value for `n` is `1`.
pub fn DL(n: Option<u32>) -> ControlFunction<'static> {
    sequence!(04 / 13, numeric n, default 1)
}

/// Valid parameter values to the function [`DSR`].
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum DeviceStatusReport {
    /// The device is ready, no malfunction detected
    #[default]
    Ready = 0,

    /// The device is busy, another [`DSR`] must be requested later.
    BusyRepeat,

    /// The device is busy, another [`DSR`] will be sent later.
    BusyLater,

    /// Some malfunction detected, another [`DSR`] must be requested later.
    MalfunctionRepeat,

    /// Some malfunction detected, another [`DSR`] will be sent later.
    MalfunctionLater,

    /// A device status report is requested.
    RequestDeviceStatusReport,

    /// A report of the active presentation position or of the active data position in the form of ACTIVE POSITION
    /// REPORT ([`CPR`]) is requested.
    RequestActivePositionReport,
}

/// Device Status Report.
///
/// `DSR` is used either to report the status of the sending device or to request a status report from the receiving
/// device, depending on the parameter value.
///
/// `DSR` with parameter value [`DeviceStatusReport::Ready`], [`DeviceStatusReport::BusyRepeat`],
/// [`DeviceStatusReport::BusyLater`], [`DeviceStatusReport::MalfunctionRepeat`], or
/// [`DeviceStatusReport::MalfunctionLater`] may be sent either unsolicited or as a response to a request such as a
/// `DSR` with a parameter value [`DeviceStatusReport::RequestDeviceStatusReport`] or MESSAGE WAITING
/// ([`MW`][crate::c1::MW]).
///
/// The default value for `s` is [`DeviceStatusReport::Ready`].
pub fn DSR(s: Option<DeviceStatusReport>) -> ControlFunction<'static> {
    sequence!(06 / 14, selective default s)
}

/// Dimension Text Area.
///
/// `DTA` is used to establish the dimensions of the text area for subsequent pages.
///
/// The established dimensions remain in effect until the next occurrence of `DTA` in the data stream.
///
/// - `n` specifies the dimension in the direction perpendicular to the line orientation.
/// - `m` specifies the dimension in the direction parallel to the line orientation.
///
/// The unit in which the parameter value is expressed is that established by the parameter value of SELECT SIZE UNIT
/// (`SSU`).
pub fn DTA(n: u32, m: u32) -> ControlFunction<'static> {
    sequence!(02 / 00, 05 / 04, numeric n, numeric m)
}

/// Valid parameter values to the function [`EA`].
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum EraseArea {
    /// Erase from the active position until the end of the qualified area.
    #[default]
    ActivePositionToEnd = 0,

    /// Erase from the beginning of the qualified area until (including) the active position.
    BeginToActivePosition,

    /// Erase all contents from the beginning of the qualified area until the end of the qualified area.
    BeginToEnd,
}

/// Erase in Area.
///
/// If the DEVICE COMPONENT SELECT MODE ([`DCSM`][crate::modes::DCSM]) is set to PRESENTATION, `EA` causes some or all
/// character positions in the active qualified area (the qualified area in the presentation component which contains
/// the active presentation position) to be put into the erased state, depending on the parameter value.
///
/// If the DEVICE COMPONENT SELECT MODE ([`DCSM`][crate::modes::DCSM]) is set to DATA, `EA` causes some or all character
/// positions in the active qualified area (the qualified area in the data component which contains the active data
/// position) to be put into the erased state, depending ont he parameter value.
///
/// Whether the character positions of protected areas are put into the erased state, or the character positions of
/// unprotected areas only, depends on the setting of ERASURE MODE ([`ERM`][crate::modes::ERM]).
///
/// The default value of `s` is [`EraseArea::ActivePositionToEnd`].
pub fn EA(s: Option<EraseArea>) -> ControlFunction<'static> {
    sequence!(04 / 15, selective default s)
}

/// Erase Character.
///
/// If the DEVICE COMPONENT SELECT MODE ([`DCSM`][crate::modes::DCSM]) is set to PRESENTATION, `ECH` causes the active
/// presentation position and the `n-1` following character positions in the presentation component to be put into the
/// erased state.
///
/// If the DEVICE COMPONENT SELECT MODE ([`DCSM`][crate::modes::DCSM]) is set to DATA, `ECH` causes the active data
/// position and the `n-1` following character positions in the data component to be put into the erased state.
///
/// Whether the character positions of protected areas are put into the erased state, or the character positions of
/// unprotected areas only, depends on the setting of the ERASURE MODE ([`ERM`][crate::modes::ERM]).
///
/// The default value for `n` is `1`.
pub fn ECH(n: Option<u32>) -> ControlFunction<'static> {
    sequence!(05 / 08, numeric n, default 1)
}

/// Valid parameter values to the function [`ED`].
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum ErasePage {
    /// Erase from the active position until the end of the page.
    #[default]
    ActivePositionToEnd = 0,

    /// Erase from the beginning of the page until (including) the active position.
    BeginToActivePosition,

    /// Erase all contents from the beginning of the page until the end of the page.
    BeginToEnd,
}

/// Erase In Page.
///
/// If the DEVICE COMPONENT SELECT MODE ([`DCSM`][crate::modes::DCSM]) is set to PRESENTATION, `ED` causes some or all
/// character positions of the active page (the page which contains the active presentation position in the presentation
/// component) to be/ put into the erased state, depending on the parameter value.
///
/// If the DEVICE COMPONENT SELECT MODE ([`DCSM`][crate::modes::DCSM]) is set to DATA, `ED` causes some or all character
/// positions of the active page (the page which contains the active data position in the data component) to be put into
/// the erased state, depending on the parameter value.
///
/// Whether the character positions of protected areas are put into the erased state, or the character positions of
/// unprotected areas only, depends on the setting of the ERASURE MODE ([`ERM`][crate::modes::ERM]).
///
/// The default value of `s` is [`ErasePage::ActivePositionToEnd`].
pub fn ED(s: Option<ErasePage>) -> ControlFunction<'static> {
    sequence!(04 / 10, selective default s)
}

/// Valid parameter values to the function [`EF`].
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum EraseField {
    /// Erase from the active position until the end of the field.
    #[default]
    ActivePositionToEnd = 0,

    /// Erase from the beginning of the field until (including) the active position.
    BeginToActivePosition,

    /// Erase all contents from the beginning of the field until the end of the field.
    BeginToEnd,
}

/// Erase In Field.
///
/// If the DEVICE COMPONENT SELECT MODE ([`DCSM`][crate::modes::DCSM]) is set to PRESENTATION, `EF` causes some or all
/// character positions of the active field (the field which contains the active presentation position in the
/// presentation component) to be put into the erased state, depending on the parameter value.
///
/// If the DEVICE COMPONENT SELECT MODE ([`DCSM`][crate::modes::DCSM]) is set to DATA, `EF` causes some or all character
/// positions of the active field (the field which contains the active data position in the data component) to be put
/// into the erased state, depending on the parameter value.
///
/// Whether the character positions of protected areas are put into the erased state, or the character positions of
/// unprotected areas only, depends on the setting of the ERASURE MODE ([`ERM`][crate::modes::ERM]).
///
/// The default value for `s` is [`EraseField::ActivePositionToEnd`].
pub fn EF(s: Option<EraseField>) -> ControlFunction<'static> {
    sequence!(04 / 14, selective default s)
}

/// Valid parameter values to the function [`EL`].
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum EraseLine {
    /// Erase from the active position until the end of the line.
    #[default]
    ActivePositionToEnd = 0,

    /// Erase from the beginning of the line until (including) the active position.
    BeginToActivePosition,

    /// Erase all contents from the beginning of the line until the end of the line.
    BeginToEnd,
}

/// Erase In Line.
///
/// If the DEVICE COMPONENT SELECT MODE ([`DCSM`][crate::modes::DCSM]) is set to PRESENTATION, `EL` causes some or all
/// character positions of the active line (the line which contains the active presentation position in the presentation
/// component) to be put into the erased state, depending on the parameter value.
///
/// If the DEVICE COMPONENT SELECT MODE ([`DCSM`][crate::modes::DCSM]) is set to DATA, `EL` causes some or all character
/// positions of the active line (the line which contains the active data position in the data component) to be put into
/// the erased state, depending on the parameter value.
///
/// Whether the character positions of protected areas are put into the erased state, or the character positions of
/// unprotected areas only, depends on the setting of the ERASURE MODE ([`ERM`][crate::modes::ERM]).
///
/// The default value for `s` is [`EraseLine::ActivePositionToEnd`].
pub fn EL(s: Option<EraseLine>) -> ControlFunction<'static> {
    sequence!(04 / 11, selective default s)
}

/// Function Key.
///
/// `FNK` is a control function in which the parameter value identifies the function key which has been operated.
pub fn FNK(n: u32) -> ControlFunction<'static> {
    sequence!(02 / 00, 05 / 07, numeric n)
}

/// Valid parameter values to the function [`FNT`].
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum Font {
    /// Primary font.
    #[default]
    Primary = 0,

    /// First alternative font.
    Alternative1,

    /// Second alternative font.
    Alternative2,

    /// Third alternative font.
    Alternative3,

    /// Forth alternative font.
    Alternative4,

    /// Fifth alternative font.
    Alternative5,

    /// Sixth alternative font.
    Alternative6,

    /// Seventh alternative font.
    Alternative7,

    /// Eighth alternative font.
    Alternative8,

    /// Ninth alternative font.
    Alternative9,
}

/// Font Selection.
///
/// `FNT` is used to identify the character font to be selected as primary or alternative font by subsequent occurrences
/// of SELECT GRAPHIC RENDITION ([`SGR`]) in the data stream.
///
/// - `s` specifies the primary or alternative font concerned.
/// - `t` identifies the character font according to a register which is to be established.
///
/// The default value for `s` is [`Font::Primary`], and for `t` is `0`.
pub fn FNT(s: Option<Font>, t: Option<u32>) -> ControlFunction<'static> {
    let a = match s {
        Some(font) => font as u32,
        None => (Font::default()) as u32,
    };
    let b = t.unwrap_or(0);
    sequence!(02 / 00, 04 / 04, numeric a, numeric b)
}

/// Valid parameter values to the function [`GCC`].
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum GraphicCharacterCombination {
    /// Combine the following two graphic characters into a single graphic symbol.
    #[default]
    CombineTwo = 0,

    /// Combine all following graphic characters, until the occurrence of
    /// [`GraphicCharacterCombination::EndOfCombination`] into a single graphic symbol.
    StartOfCombination,

    /// Combine all preceding graphic characters, starting from [`GraphicCharacterCombination::StartOfCombination`],
    /// into a single graphic symbol.
    EndOfCombination,
}

/// Graphic Character Combination
///
/// `GCC` is used to indicate that two or more graphic characters are to be imaged as one single graphic symbol.
///
/// The default value of `s` is [`GraphicCharacterCombination::CombineTwo`].
///
/// ## Note
///
/// `GCC` does not explicitly specify the relative sizes or placements of the component parts of a composite graphic
/// symbol. In the simplest case, two components may be "half-width" and side-by-side. For example in Japanese text a
/// pair of characters may be presented side-by-side, and occupy the space of a normal-size Kanji character.
pub fn GCC(s: Option<GraphicCharacterCombination>) -> ControlFunction<'static> {
    sequence!(02 / 00, 05 / 15, selective default s)
}

/// Graphic Size Modification.
///
/// `GSM` is used to modify for subsequent text the height and / or the width of all primary and alternative fonts
/// identified by FONT SELECT ([`FNT`]) and established by GRAPHIC SIZE SELECTION ([`GSS`]). The established values
/// remain in effect until the next occurrence of `GSM` or [`GSS`] in the data stream.
///
/// - `h` specifies the height as a percentage of the height established by [`GSS`].
/// - `w` specifies the width as a percentage of the width established by [`GSS`].
///
/// The default value for `h`, and `w` is `100`.
pub fn GSM(h: Option<u32>, w: Option<u32>) -> ControlFunction<'static> {
    sequence!(02 / 00, 04 / 02, numeric h, default 100, numeric w, default 100)
}

/// Graphic Size Selection.
///
/// `GSS` is used to establish for subsequent texts the height and the width of all primary and alternative fonts
/// identified by FONT SELECT ([`FNT`]). The established values remain in effect until the next occurrence of `GSS` in the
/// data stream.
///
/// - `n` specifies the height, the width is implicitly defined by the height.
///
/// The unit in which the parameter value is expressed is that established by the parameter value of SELECT SIZE UNIT
/// ([`SSU`]).
pub fn GSS(n: u32) -> ControlFunction<'static> {
    sequence!(02 / 00, 04/03, numeric n)
}

/// Character Position Absolute.
///
/// `HPA` causes the active data position to be moved to character position `n` in the active line (the line in the data
/// component that contains the active data position).
///
/// The default value for `n` is `1`.
pub fn HPA(n: Option<u32>) -> ControlFunction<'static> {
    sequence!(06 / 00, numeric n, default 1)
}

/// Character Position Backward.
///
/// `HPB` causes the active data position to be moved by `n` character positions in the data component in the direction
/// opposite to that of the character progression.
///
/// The default value for `n` is `1`.
pub fn HPB(n: Option<u32>) -> ControlFunction<'static> {
    sequence!(06 / 10, numeric n, default 1)
}

/// Character Position Forward.
///
/// `HPR` causes the active data position to be moved by `n` character positions in the data component in the direction
/// of the character progression.
///
/// The default value for `n` is `1`.
pub fn HPR(n: Option<u32>) -> ControlFunction<'static> {
    sequence!(06 / 01, numeric n, default 1)
}

/// Character And Line Position.
///
/// `HVP` causes the active data position to be moved in the data component to the `n`-th line position according to the
/// line progression and to the `m`-th character position according to the character progression.
///
/// The default value for `n` and `m` is `1`.
pub fn HVP(n: Option<u32>, m: Option<u32>) -> ControlFunction<'static> {
    sequence!(06 / 06, numeric n, default 1, numeric m, default 1)
}

/// Insert Character.
///
/// If the DEVICE COMPONENT SELECT MODE ([`DCSM`][crate::modes::DCSM]) is set to PRESENTATION, `ICH` is used to prepare
/// the insertion of `n` characters, by putting into the erased state the active presentation position and, depending on
/// the setting of the CHARACTER EDITING MODE ([`HEM`][crate::modes::HEM]), the `n-1` preceding or following character
/// positions in the presentation component. The previous contents of the active presentation position and an adjacent
/// string of character positions are shifted away from the active presentation position. The contents of `n` character
/// positions at the other end of the shifted part are removed. The active presentation position is moved to the line
/// home position in the active line. The line home position is established by the parameter value of SET LINE HOME
/// ([`SLH`]).
///
/// The extent of the shifted part is established by SELECT EDITING EXTENT ([`SEE`]).
///
/// The effect of `ICH` on the start or end of a selected area, the start or end of a qualified area, or a tabulation
/// stop in the shifted part is undefined.
///
/// If the DEVICE COMPONENT SELECT MODE ([`DCSM`][crate::modes::DCSM]) is set to DATA, `ICH` is used to prepare the
/// insertion of `n` characters, by putting into the erased state the active data position and, depending on the setting
/// of the CHARACTER EDITING MODE ([`HEM`][crate::modes::HEM]), the `n-1` preceding or following character positions in
/// the data component. The previous contents of the active data position and and adjacent string of character positions
/// are shifted away from the active data position. The contents of `n` character positions at the other end of the
/// shifted part are removed. The active data position is moved to the line home position in the active line. The line
/// home position is/ established by the parameter value of SET LINE HOME ([`SLH`]).
///
/// The default value for `n` is `1`.
pub fn ICH(n: Option<u32>) -> ControlFunction<'static> {
    sequence!(04 / 00, numeric n, default 1)
}

/// Valid parameter values to the function [`IDCS`].
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum IdentifyDeviceControlString {
    /// Reserved for use with the DIAGNOSTIC state of the STATUS REPORT TRANSFER MODE.
    Diagnostic,

    /// Reserved for Dynamically Redefinable Character Sets according to Standard [ECMA-35][ecma-35].
    ///
    /// [ecma-35]: https://www.ecma-international.org/wp-content/uploads/ECMA-35_6th_edition_december_1994.pdf
    DynamicallyRedefinableCharacterSet,

    /// Private command string.
    Private(u32),
}

/// Identify Device Control String.
///
/// `IDCS` is used to specify the purpose and format of the command string of subsequent DEVICE CONTROL STRINGS
/// ([`DCS`][crate::c1::DCS]). The specified purpose and format remain in effect until the next occurrence of `IDCS`
/// in the data stream.
///
/// The format and interpretation of the command string corresponding to the parameter `s` are to be defined in
/// appropriate standards. If this control function is used to identify a private command string, a private parameter
/// value shall be used [`IdentifyDeviceControlString::Private`].
pub fn IDCS(s: IdentifyDeviceControlString) -> ControlFunction<'static> {
    let v = match s {
        IdentifyDeviceControlString::Private(i) => i,
        IdentifyDeviceControlString::Diagnostic => 1,
        IdentifyDeviceControlString::DynamicallyRedefinableCharacterSet => 2,
    };

    sequence!(02 / 00, 04 / 15, numeric v)
}

/// Identify Graphic Subrepertoire.
///
/// `IGS` is used to indicate that a repertoire of the graphic characters of ISO/IEC 10367 is used in the subsequent
/// text.
///
/// The parameter value of `IGS` identifies a graphic character repertoire registered in accordance with ISO/IEC 7350.
pub fn IGS(n: u32) -> ControlFunction<'static> {
    sequence!(02 / 00, 04 / 13, numeric n)
}

/// Insert Line.
///
/// If the DEVICE COMPONENT SELECT MODE ([`DCSM`][crate::modes::DCSM]) is set to PRESENTATION, `IL` is used to prepare
/// the insertion of `n` lines, by putting into the erased state in the presentation component the active line
/// (the line that contains the active presentation position) and, depending on the setting of the LINE EDITING MODE
/// ([`VEM`][crate::modes::VEM]), the `n-1` preceding or following lines. The previous contents of the active line and
/// of adjacent lines are shifted away from the active line. The contents of `n` lines at the other end of the shifted
/// part are removed. The active presentation position is moved to the line home position in the active line. The line
/// home position is established by the parameter value of SET LINE HOME ([`SLH`]).
///
/// The extent of the shifted part is established by SELECT EDITING EXTENT ([`SEE`]).
///
/// Any occurrence of the start or end of a selected area, the start or end of a qualified area, or a tabulation stop in
/// the shifted part, are also shifted.
///
/// If the TABULATION STOP MODE ([`TSM`][crate::modes::TSM]) is set to SINGLE, character tabulation stops are cleared in
/// the lines that are put into the erased state.
///
/// If the DEVICE COMPONENT SELECT MODE ([`DCSM`][crate::modes::DCSM]) is set to DATA, `IL` is used to prepare the
/// insertion of `n` lines, by putting into the erased state in the data component the active line (the line that
/// contains the active data position) and, depending on the setting of the LINE EDITING MODE
/// ([`VEM`][crate::modes::VEM]), the `n-1` preceding or following lines. The previous contents of the active line and
/// of adjacent lines are shifted away from the active line. The contents of `n` lines at the other end of the shifted
/// part are removed. The active data position is moved to the line home position in the active line. The line home
/// position is established by the parameter value of SET LINE HOME ([`SLH`]).
///
/// The default value for `n` is `1`.
pub fn IL(n: Option<u32>) -> ControlFunction<'static> {
    sequence!(04 / 12, numeric n, default 1)
}

/// Valid parameter values to the function [`JFY`].
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum Justification {
    /// No justification, end of justification of preceding text.
    #[default]
    None = 0,

    /// Word fill.
    WordFill,

    /// Word space.
    WordSpace,

    /// Letter space.
    LetterSpace,

    /// Hyphenation.
    Hyphenation,

    /// Flush to line home position margin.
    Left,

    /// Centre between line home position and line limit position margins.
    Centre,

    /// Flush to line limit position margin.
    Right,

    /// Italian hyphenation.
    ItalianHyphenation,
}

/// Justify.
///
/// `JFY` is used to indicate the beginning of a string of graphic characters in the presentation component that are to
/// be justified according to the layout specified by the parameter value.
///
/// The end of the string to be justified is indicated by the next occurrence of `JFY` in the data stream.
///
/// The line home position is established by the parameter value of SET LINE HOME ([`SLH`]). The line limit position is
/// established by the parameter value of SET LINE LIMIT ([`SLL`]).
///
/// The default value of `s` is [`Justification::None`].
pub fn JFY(s: Option<Justification>) -> ControlFunction<'static> {
    sequence!(02 / 00, 04 / 06, selective default s)
}

/// Valid parameter values to the function [`MC`].
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum MediaCopy {
    /// Initiate transfer to a primary auxiliary device.
    #[default]
    BeginTransferToPrimary = 0,

    /// Initiate transfer from a primary auxiliary device.
    BeginTransferFromPrimary,

    /// Initiate transfer to a secondary auxiliary device.
    BeginTransferToSecondary,

    /// Initiate transfer from a secondary auxiliary device.
    BeginTransferFromSecondary,

    /// Stop relay to a primary auxiliary device.
    StopRelayPrimary,

    /// Start relay to a primary auxiliary device.
    StartRelayPrimary,

    /// Stop relay to a secondary auxiliary device.
    StopRelaySecondary,

    /// Start relay to a secondary auxiliary device.
    StartRelaySecondary,
}

/// Media Copy.
///
/// `MC` is used either to initiate a transfer of data from or to an auxiliary input/output device or to enable or
/// disable the relay of the received data stream to an auxiliary input/output device, depending on the parameter value.
///
/// The default value for `s` is [`MediaCopy::BeginTransferToPrimary`].
pub fn MC(s: Option<MediaCopy>) -> ControlFunction<'static> {
    sequence!(06 / 09, selective default s)
}

/// Next Page.
///
/// `NP` causes the `n`-th following page in the presentation component to be displayed.
///
/// The effect of this control function on the active presentation position is not defined.
///
/// The default value for `n` is `1`.
pub fn NP(n: Option<u32>) -> ControlFunction<'static> {
    sequence!(05 / 05, numeric n, default 1)
}

/// Valid parameter values to the function [`PEC`].
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum PresentationExpandContract {
    /// Normal, as specified by [`SCS`], [`SHS`] or [`SPI`].
    #[default]
    Normal = 0,

    /// Expanded, multiplied by a factor not greater than `2`.
    Expanded,

    /// Condensed, multiplied by a factor not less than `0.5`.
    Condensed,
}

/// Presentation Expand or Contract.
///
/// `PEC` is used to establish the spacing and the extent of the graphic characters for subsequent text. The spacing is
/// specified in the line as multiples of the spacing established by the most recent occurrence of SET CHARACTER SPACING
/// ([`SCS`]) or of SELECT CHARACTER SPACING ([`SHS`]) or of SPACING INCREMENT ([`SPI`]) in the data stream. The extent
/// of the characters is implicitly established by these control functions. The established spacing and the extent
/// remain in effect until the next occurrence of `PEC`, of [`SCS`], of [`SHS`] or of [`SPI`] in the data stream.
///
/// The default value for `s` is [`PresentationExpandContract::Normal`].
pub fn PEC(s: Option<PresentationExpandContract>) -> ControlFunction<'static> {
    sequence!(02/00, 05/10, selective default s)
}

/// Valid parameter values to the function [`PFS`].
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum PageFormat {
    /// Tall basic text communication format.
    #[default]
    TallBasicText = 0,

    /// Wide basic text communication format.
    WideBasicText,

    ///Tall basic A4 format.
    TallBasicA4,

    /// Wide basic A4 format.
    WideBasicA4,

    /// Tall North American letter format.
    TallLetter,

    /// Wide North American letter format.
    WideLetter,

    /// Tall extended A4 format.
    TallExtendedA4,

    /// Wide extended A4 format.
    WideExtendedA4,

    /// Tall North American legal format.
    TallLegal,

    /// Wide North American legal format.
    WideLegal,

    /// A4 short lines format.
    A4ShortLines,

    /// A4 long lines format.
    A4LongLines,

    /// B5 short lines format.
    B5ShortLines,

    /// B5 long lines format
    B5LongLines,

    /// B4 short lines format.
    B4ShortLines,

    /// B4 long lines format
    B4LongLines,
}

/// Page Format Selection
///
/// `PFS` is used to establish the available area for the imaging of pages of text based on paper size. The pages are
/// introduced by the subsequent occurrence of FORM FEED ([`FF`][crate::c0::FF]) in the data stream.
///
/// The established image area remains in effect until the next occurrence of `PFS` in the data stream.
///
/// The page home position is established by the parameter value of SET PAGE HOME ([`SPH`]), the page limit position is
/// established by the parameter value of SET PAGE LIMIT ([`SPL`]).
///
/// The default value for `s` is [`PageFormat::TallBasicText`].
pub fn PFS(s: Option<PageFormat>) -> ControlFunction<'static> {
    sequence!(02/00, 04/10, selective default s)
}

/// Preceding Page.
///
/// `PP` causes the `n`-th preceding page in the presentation component to be displayed. The effect of this control
/// function on the active presentation position is not defined.
///
/// The default for `n` is `1`.
pub fn PP(n: Option<u32>) -> ControlFunction<'static> {
    sequence!(05 / 06, numeric n, default 1)
}

/// Page Position Absolute.
///
/// `PPA` causes the active data position to be moved in the data component to the corresponding character position on
/// the `n-th` page.
///
/// The default for `n` is `1`.
pub fn PPA(n: Option<u32>) -> ControlFunction<'static> {
    sequence!(02 / 00, 05 / 00, numeric n, default 1)
}

/// Page Position Backward.
///
/// `PPB` causes the active data position to be moved in the data component to the corresponding character position on
/// the `n-th` page.
///
/// The default value for `n` is `1`.
pub fn PPB(n: Option<u32>) -> ControlFunction<'static> {
    sequence!(02 / 00, 05 / 02, numeric n, default 1)
}

/// Page Position Forward.
///
/// `PPR` causes the active data position to be moved in the data component to the corresponding character position on
/// the `n`-th following page.
///
/// The default value for `n` is `1`.
pub fn PPR(n: Option<u32>) -> ControlFunction<'static> {
    sequence!(02 / 00, 05 / 01, numeric n, default 1)
}

/// Valid parameter values to the function [`PTX`].
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum ParallelText {
    /// End of parallel texts.
    #[default]
    End = 0,

    /// Beginning of a string of principal parallel text.
    BeginPrincipal,

    /// Beginning of a string of supplementary parallel text.
    BeginSupplementary,

    /// Beginning of a string of supplementary Japanese phonetic annotation
    BeginJapanesePhonetic,

    /// Beginning of a string of supplementary Chinese phonetic annotation
    BeginChinesePhonetic,

    /// End of a string of supplementary phonetic annotations
    EndPhonetic,
}

/// Parallel Texts.
///
/// `PTX` is used to delimit strings of graphic characters that are communicated one after another in the data stream,
/// but that are intended to be presented in parallel with one another, usually in adjacent lines.
///
/// `PTX` with a parameter value of [`ParallelText::BeginPrincipal`] indicates the beginning of the string of principal
/// text intended to be presented in parallel with one or more strings of supplementary text.
///
/// `PTX` with a parameter value of [`ParallelText::BeginSupplementary`], [`ParallelText::BeginJapanesePhonetic`], or
/// [`ParallelText::BeginChinesePhonetic`] indicates the beginning of a string of supplementary text that is intended to
/// be presented in parallel with either a string of principal text or the immediately preceding string of supplementary
/// text, if any; at the same time it indicates the end of the preceding string of principal text or of the immediately
/// preceding string of supplementary text, if any. The end of a string of supplementary text is indicated by a
/// subsequent occurrence of `PTX` with a parameter other than [`ParallelText::BeginPrincipal`].
///
/// `PTX` with a parameter value of [`ParallelText::End`] indicates the end of the strings of text intended to be
/// presented in parallel with one another.
///
/// The default value for `s` is [`ParallelText::End`].
///
/// ## Note
///
/// `PTX` does not explicitly specify the relative placement of the strings of principal and supplementary parallel
/// texts, or the relative sizes of graphic characters in the strings of parallel text. A string of supplementary text
/// is normally presented in a line adjacent to the line containing the string of principal text, or adjacent to the
/// line containing the immediately preceding string of supplementary text, if any. The first graphic character of the
/// string of principal text and the first graphic character of a string of supplementary text are normally presented
/// in the same position of their respective lines. However, a string of supplementary text longer (when presented)
/// than the associated string of principal text may be centred on that string. In the case of long strings of text,
/// such as paragraphs in different languages, the strings may be presented in successive lines in parallel columns,
/// with their beginnings aligned with one another and the shorter of the paragraphs followed by an appropriate amount
/// of "white space".
///
/// Japanese phonetic annotation typically consists of a few half-size or smaller Kana characters which indicate the
/// pronunciation or interpretation of one ore more Kanji characters and are presented above those Kanji characters if
/// the character path is horizontal, or to the right of them if the character path is vertical.
///
/// Chines phonetic annotation typically consists of a few Pinyin characters which indicate the pronunciation of one
/// or more Hanzi characters and are presented above those Hanzi characters. Alternatively, the Pinyin characters may
/// be presented in the same line as the Hanzi characters and following the respective Hanzi characters. The Pinyin
/// characters will then be presented within enclosing paris of parentheses.
pub fn PTX(s: Option<ParallelText>) -> ControlFunction<'static> {
    sequence!(05 / 12, selective default s)
}

/// Valid parameter values to the function [`QUAD`].
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum Alignment {
    /// Flush to line home position margin.
    #[default]
    LineHome = 0,

    /// Flush to line home position margin and fill with leader.
    LineHomeLeader,

    /// Centre between line home position and line limit position margins.
    Centre,

    /// Center between line home position and line limit position margins and fill with leader.
    CentreLeader,

    /// Flush to line limit position margin.
    LineLimit,

    /// Flush to line limit position margin and fill with leader.
    LineLimitLeader,

    /// Flush to both margins
    Justify,
}

/// Quad.
///
/// `QUAD` is used to indicate the end of a string of graphic characters that are to be positioned on a single line
/// according to the layout specified by the parameter value, see [`Alignment`].
///
/// The beginning of the string to be positioned is indicated by the preceding occurrence in the data stream of either
/// `QUAD` or one of the following formator functions: FORM FEED ([`FF`][crate::c0::FF]), CHARACTER AND LINE POSITION
/// ([`HVP`]), LINE FEED ([`LF`][crate::c0::LF]), NEXT LINE ([`NEL`][crate::c1::NEL]), PAGE POSITION ABSOLUTE
/// ([`PPA`]), PAGE POSITION BACKWARD ([`PPB`]), PAGE POSITION FORWARD ([`PPR`]), REVERSE LINE FEED
/// ([`RI`][crate::c1::RI]), LINE POSITION ABSOLUTE ([`VPA`]), LINE POSITION BACKWARD ([`VPB`]), LINE POSITION
/// FORWARD ([`VPR`]), or LINE TABULATION ([`VT`][crate::c0::VT]).
///
/// The line home position is established by the parameter value of SET LINE HOME ([`SLH`]). The line limit position is
/// established by the parameter value of SET LINE LIMIT ([`SLL`]).
///
/// The default value for `s` is [`Alignment::LineHome`].
pub fn QUAD(s: Option<Alignment>) -> ControlFunction<'static> {
    sequence!(02 / 00, 04 / 08, selective default s)
}

/// Repeat.
///
/// `REP` is used to indicate that the preceding character in the data stream, if it is a graphic character (presented
/// by one or more bit combinations) including SPACE, is to be repeated `n` times. If the character preceding `REP` is
/// a control function or part of a control function, the effect of `REP` is not defined.
///
/// The default value for `n` is `1`.
pub fn REP(n: Option<u32>) -> ControlFunction<'static> {
    sequence!(06 / 02, numeric n, default 1)
}

/// Reset Mode.
///
/// `RM` causes the modes of the receiving device to be reset as specified by the parameter values.
pub fn RM(v: Vec<Mode>) -> ControlFunction<'static> {
    sequence!(06 / 12, variadic selective v)
}

/// Set Additional Character Representation.
///
/// `SACS` is used to establish extra inter-character escapement for subsequent text. The established extra escapement
/// remains in effect until the next occurrence of `SACS` or of SET REDUCED CHARACTER SEPARATION ([`SRCS`]) in the data
/// stream or until it is reset to the default value by a subsequent occurrence of CARRIAGE RETURN LINE FEED
/// ([`CR`][crate::c0::CR] [`LF`][crate::c0::LF]) or of NEXT LINE ([`NEL`][crate::c1::NEL]) in the data stream.
///
/// `n` specifies the number of units by which the inter-character escapement is enlarged.
///
/// The unit in which the parameter value is expressed is that established by the parameter value of SELECT SIZE UNIT
/// ([`SSU`]).
///
/// The default value for `n` is 0.
pub fn SACS(n: Option<u32>) -> ControlFunction<'static> {
    sequence!(02 / 00, 05 / 12, numeric n, default 0)
}

/// Valid parameter values to the function [`SAPV`].
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum PresentationVariant {
    /// Default presentation (implementation-defined); cancels the effect of any preceding occurrence of [`SAPV`] in the
    /// data stream.
    #[default]
    Default = 0,

    /// The decimal digits are presented by means of the graphic symbols used in the Latin script.
    LatinDecimals,

    /// The decimal digits are presented by means of the graphic symbols used in the Arabic script, i.e. the Hindi
    /// symbols.
    ArabicDecimals,

    /// When the direction of the character path is right-to-left, each of the graphic characters in the graphic
    /// character set(s) in use which is one of a left/right handed pair (parentheses, square brackets, curly brackets,
    /// greater-than/less-than signs, etc.) is presented as "mirrored", i.e. as the other member of the pair. For
    /// example, the coded graphic character given the name LEFT PARENTHESIS is presented as RIGHT PARENTHESIS, and
    /// vice versa.
    MirrorPairs,

    /// When the direction of the character path is right-to-left, all graphic characters which represent operators and
    /// delimiters in mathematical formulae and which are not symmetrical about a vertical axis are presented as
    /// mirrored about that vertical axis.
    MirrorFormulae,

    /// The following graphic character is presented in its isolated form.
    Isolated,

    /// The following graphic character is presented in its initial form.
    Initial,

    /// The following graphic character is presented in its medial form.
    Medial,

    /// The following graphic character is presented in its final form.
    Final,

    /// Where the bit combination `02/14` is intended to represent a decimal mark in a decimal number it shall be
    /// represented by means of the graphic symbol FULL STOP.
    DecimalFullStop,

    /// Where the bit combination `02/14` is intended to represent a decimal mark in a decimal number it shall be
    /// represented by means of the graphic symbol COMMA.
    DecimalComma,

    /// Vowels are presented above or below the preceding character.
    VowelAboveOrBelow,

    /// Vowels are presented after the preceding character.
    VowelAfterPreceding,

    /// Contextual shape determination of Arabic scripts, including the LAM-ALEPH ligature but excluding all other
    /// Arabic ligatures.
    ContextualShapeArabicScriptWithLamAleph,

    /// Contextual shape determination of Arabic scripts, excluding all Arabic ligatures.
    ContextualShapeArabicScript,

    /// Cancels the effect of [`PresentationVariant::MirrorPairs`] and [`PresentationVariant::MirrorFormulae`].
    NoMirroring,

    /// Vowels are not presented.
    NoVowels,

    /// When the string direction is right-to-left, the italicized characters are slanted to the left, when the string
    /// direction is left-to-right, the italicized characters are slanted to the right.
    SlantFollowsStringDirection,

    /// Contextual shape determination of Arabic scripts is not used, the graphic characters - including the digits -
    /// are presented in the form they are stored (pass-through).
    NoContextualShapeArabicScript,

    /// Contextual shape determination of Arabic scripts i not used, the graphic characters - excluding the digits -
    /// are presented in the form they are stored (pass-through).
    NoContextualShapeArabicScriptExceptDigits,

    /// The graphic symbols used to present the decimal digits are device dependent.
    DeviceDependentDecimalDigits,

    /// Establishes the effect of parameter values [`PresentationVariant::Isolated`], [`PresentationVariant::Initial`],
    /// [`PresentationVariant::Medial`], and [`PresentationVariant::Final`] for the following graphic characters until
    /// cancelled.
    PersistCharacterForm,

    /// Cancels the effect of parameter value [`PresentationVariant::PersistCharacterForm`], i.e. re-establishes the
    /// effect of parameter values [`PresentationVariant::Isolated`], [`PresentationVariant::Initial`],
    /// [`PresentationVariant::Medial`], and [`PresentationVariant::Final`] for the next single graphic character only.
    DesistCharacterForm,
}

/// Select Alternative Presentation Variants.
///
/// `SAPV` is used to specify one or more variants for the presentation of subsequent text.
///
/// The default value for `s` is [`PresentationVariant::Default`].
pub fn SAPV(s: Option<PresentationVariant>) -> ControlFunction<'static> {
    sequence!(02 / 00, 05 / 13, selective default s)
}

/// Valid parameter values to the function [`SCO`].
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum CharacterOrientation {
    /// Rotate by 0°, normal orientation.
    #[default]
    Normal = 0,

    /// Rotate by 45°.
    Rotate45,

    /// Rotate by 90°.
    Rotate90,

    /// Rotate by 135°.
    Rotate135,

    /// Rotate by 180°.
    Rotate180,

    /// Rotate by 225°.
    Rotate225,

    /// Rotate by 270°.
    Rotate270,

    /// Rotate by 315°.
    Rotate315,
}

/// Select Character Orientation.
///
/// `SCO` is used to establish the amount of rotation of the graphic characters following in the data stream. The
/// established value remains in effect until the next occurrence of `SCO` in the data stream.
///
/// Rotation is positive, i.e. counter-clockwise and applies to the normal presentation of the graphic characters along
/// the character path. The centre of rotation of the affected graphic characters is not default.
///
/// The default value for `s` is [`CharacterOrientation::Normal`].
pub fn SCO(s: Option<CharacterOrientation>) -> ControlFunction<'static> {
    sequence!(02 / 00, 06 / 05, selective default s)
}

/// Valid parameter values to the function [`SCP`].
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CharacterPath {
    /// Left-to-right (in the case of horizontal line orientation), or top-to-bottom (in the case of vertical line
    /// orientation).
    LefToRight = 1,

    /// Right-to-left (in the case of horizontal line orientation), or bottom-to-top (in the case of vertical line
    /// orientation).
    RightToLeft,
}

/// Valid parameter values to the function [`SCP`].
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CharacterPathScope {
    /// Undefined, implementation specific.
    ///
    /// ## Note
    ///
    /// This may also permit the effect to take place after the next occurrence of [`CR`][crate::c0::CR],
    /// [`NEL`][crate::c1::NEL] or any control function which initiates an absolute movement of the active presentation
    /// position or the active data position.
    Undefined = 0,

    /// The content of the active line in the presentation component (the line that contains the active presentation
    /// position) is updated to correspond to the content of the active line in the data component (the line that
    /// contains the active data position) according to the newly established character path characteristics in the
    /// presentation component; the active data position is moved to the first character position in the active line
    /// in the data component, the active presentation position in the presentation component is updated accordingly.
    InPresentationComponent,

    /// The content of the active line in the data component (the line that contains the active data position) is
    /// updated to correspond to the content of the active line in the presentation component (the line that contains
    /// the active presentation position) according to the newly established character path characteristics of the
    /// presentation component; the active presentation position is moved to the first character position in the active
    /// line in the presentation component, the active data position in the data component is updated accordingly.
    InDataComponent,
}

/// Select Character Path.
///
/// `SCP` is used to select the character path, relative to the line orientation, for the active line (the line that
/// contains the active presentation position) and subsequent lines in the presentation component. It is also used to
/// update the content of the active line in the presentation component and the content of the active line (the line
/// that contains the active data position) in the data component. This takes effect immediately.
pub fn SCP(s: CharacterPath, t: CharacterPathScope) -> ControlFunction<'static> {
    let (n, m) = ((s as u32), (t as u32));
    sequence!(02 / 00, 06 / 11, numeric n, numeric m)
}

/// Set Character Spacing.
///
/// `SCS` is used to establish the character spacing for subsequent text. The established spacing remains in effect
/// until the next occurrence of `SCS`, or of SELECT CHARACTER SPACING ([`SHS`]) or of SPACING INCREMENT ([`SPI`]) in
/// the data stream.
///
/// `n` specifies the character spacing.
///
/// The unit in which the parameter value is expressed is that established by the parameter value of SELECT SIZE UNIT
/// ([`SSU`]).
pub fn SCS(n: u32) -> ControlFunction<'static> {
    sequence!(02 / 00, 06 / 07, numeric n)
}

/// Scroll Down.
///
/// `SD` causes the data in the presentation component to be moved by `n` line positions if the line orientation is
/// horizontal, or by `n` character positions if the line orientation is vertical, such that the data appear to
/// move down.
///
/// The active presentation position is not affected by this control function.
///
/// The default value for `n` is `1`.
pub fn SD(n: Option<u32>) -> ControlFunction<'static> {
    sequence!(05 / 04, numeric n, default 1)
}

/// Valid parameter values to the function [`SDS`].
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum StringDirection {
    /// End of a directed string; re-establish the previous direction.
    #[default]
    End = 0,

    /// Start of a directed string, establish the direction left-to-right.
    StartLeftToRight,

    /// Start of a directed string, establish the direction right-to-left.
    StartRightToLeft,
}

/// Start Directed String.
///
/// `SDS` is used to establish in the data component the beginning and end of a string of characters as well as the
/// direction of the string. This direction may be different from that currently established. The indicated string
/// follows the preceding text. The established character progression is not affected.
///
/// The beginning of a directed string is indicated by `SDS` with a parameter value not equal to
/// [`StringDirection::End`]. A directed string may contain one or more nested strings. These nested strings may be
/// directed strings the beginning of which are indicated by `SDS` with a parameter value not equal to
/// [`StringDirection::End`], or reversed stings the beginning of which are indicated by START REVERSED STRING ([`SRS`])
/// with a parameter value of [`ReversedString::Start`]. Every beginning of such a string invokes the next deeper level
/// of nesting.
///
/// The standard does not define the location of the active data position within any such nested string.
///
/// The end of a directed string is indicated by `SDS` with a parameter value of [`StringDirection::End`]. Every such
/// end of such a string re-establishes the next higher level of nesting (the one in effect prior to the string just
/// ended). The direction is re-established to that in effect prior to the string just ended. The active data position
/// is moved to the character position following the characters of the string just ended.
///
/// The default value of `s` is [`StringDirection::End`].
///
/// ## Note 1
///
/// The effect of receiving a [`CVT`], [`HT`][crate::c0::HT], [`SCP`], [`SPD`], or [`VT`][crate::c0::VT] control
/// function within an `SDS` string is not defined.
///
/// ## Note 2
///
/// The control functions for area definitions ([`DAQ`], [`EPA`][crate::c1::EPA], [`SPA`][crate::c1::SPA],
/// [`SSA`][crate::c1::SPA]) should not be used within an `SDS` string.
pub fn SDS(s: Option<StringDirection>) -> ControlFunction<'static> {
    sequence!(05 / 13, selective default s)
}

/// Valid parameter values to the function [`SEE`].
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum EditingExtend {
    /// The shifted part is limited to the active page in the presentation component.
    #[default]
    ActivePage = 0,

    /// The shifted part is limited to the active line in the presentation component.
    ActiveLine,

    /// The shifted part is limited to the active field in the presentation component.
    ActiveField,

    /// The shifted part is limited to the active qualified area.
    QualifiedArea,

    /// The shifted part consists of the relevant part of the entire presentation component.
    All,
}

/// Select Editing Extent.
///
/// `SEE` is used to establish the editing extent for subsequent character or line line insertion or deletion. The
/// established extent remains in effect until the next occurrence of `SEE` in the data stream. The editing extend
/// depends on the parameter value.
///
/// The default value for `s` is [`EditingExtend::ActivePage`].
pub fn SEE(s: Option<EditingExtend>) -> ControlFunction<'static> {
    sequence!(05 / 01, selective default s)
}

/// Valid parameter values to the function [`SEF`].
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum Load {
    /// Eject sheet, no new sheet loaded
    #[default]
    None,

    /// Eject sheet and load another from the given bin.
    Bin(u32),
}

/// Valid parameter values to the function [`SEF`].
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum Stack {
    /// Eject sheet, no stacker specified.
    #[default]
    None,

    /// Eject sheet into the specified stacker.
    Stacker(u32),
}

/// Sheet Eject And Feed.
///
/// `SEF` causes a sheet of paper to be ejected from a printing device into a specified output stacker and another
/// sheet to be loaded into the printing device from a specified paper bin.
///
/// The default value for `l` is [`Load::None`].  
/// The default value for `s` is [`Stack::None`].
pub fn SEF(l: Option<Load>, s: Option<Stack>) -> ControlFunction<'static> {
    let n = match l.unwrap_or_default() {
        Load::None => 0,
        Load::Bin(bin) => bin,
    };
    let m = match s.unwrap_or_default() {
        Stack::None => 0,
        Stack::Stacker(stacker) => stacker,
    };
    sequence!(02 / 00, 05 / 09, numeric n, numeric m)
}

/// Valid parameter values to the function [`SGR`].
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum GraphicRendition {
    /// Default rendition (implementation-defined), cancels the effect of any preceding occurrence of [`SGR`] in the
    /// data stream regardless of the setting of the GRAPHIC RENDITION COMBINATION MODE ([`GRCM`][crate::modes::GRCM]).
    #[default]
    Default = 0,

    /// Bold or increased intensity.
    HighIntensity,

    /// Faint, decreased intensity or second color.
    LowIntensity,

    /// Italicized.
    Italicized,

    /// Singly underlined.
    Underlined,

    /// Slowly blinking (less than 150 per minute).
    SlowlyBlinking,

    /// Rapidly blinking (more than 150 per minute).
    RapidlyBlinking,

    /// Negative image.
    Negative,

    /// Concealed characters.
    Concealed,

    /// Crossed-out (characters still legible but marked as to be deleted).
    CrossedOut,

    /// Primary (default) font.
    PrimaryFont,

    /// First alternative font.
    FirstAlternativeFont,

    /// Second alternative font.
    SecondAlternativeFont,

    /// Third alternative font.
    ThirdAlternativeFont,

    /// Forth alternative font.
    ForthAlternativeFont,

    /// Fifth alternative font.
    FifthAlternativeFont,

    /// Sixth alternative font.
    SixthAlternativeFont,

    /// Seventh alternative font.
    SeventhAlternativeFont,

    /// Eighth alternative font.
    EighthAlternativeFont,

    /// Ninth alternative font.
    NinthAlternativeFont,

    /// Fraktur (Gothic).
    Fraktur,

    /// Doubly underlined.
    DoublyUnderlined,

    /// Normal colour or normal intensity (neither bold nor faint).
    NormalIntensity,

    /// Not italicized, not fraktur
    NormalStyle,

    /// Not underlined (neither singly nor doubly).
    NotUnderlined,

    /// Steady (not blinking).
    NotBlinking,

    /// Positive Image.
    Positive = 27,

    /// Revealed characters.
    Revealed,

    /// Not crossed out.
    NotCrossedOut,

    /// Black display.
    BlackForeground,

    /// Red display.
    RedForeground,

    /// Green display.
    GreenForeground,

    /// Yellow display.
    YellowForeground,

    /// Blue display.
    BlueForeground,

    /// Magenta display.
    MagentaForeground,

    /// Cyan display.
    CyanForeground,

    /// White display.
    WhiteForeground,

    /// Default display color (implementation specific).
    DefaultForeground = 39,

    /// Black background.
    BlackBackground,

    /// Red background.
    RedBackground,

    /// Green Background.
    GreenBackground,

    /// Yellow background.
    YellowBackground,

    /// Blue background.
    BlueBackground,

    /// Magenta background.
    MagentaBackground,

    /// Cyan background.
    CyanBackground,

    /// White background.
    WhiteBackground,

    /// Default background color (implementation specific).
    DefaultBackground = 49,

    /// Framed.
    Framed = 51,

    /// Encircled.
    Encircled,

    /// Overlined.
    Overlined,

    /// Not framed, not encircled.
    NotFramed,

    /// Not overlined,
    NotOverlined,

    /// Ideogram underline or right side line.
    IdeogramUnderline = 60,

    /// Ideogram double underline or double line on the right side.
    IdeogramDoubleUnderline,

    /// Ideogram stress marking.
    IdeogramStressMarking,

    /// Cancel Ideogram rendition settings.
    CancelIdeogramRendition,
}

/// Select Graphic Rendition.
///
/// `SGR` is used to establish one or more graphic rendition aspects for subsequent text. The established aspects remain
/// in effect until the next occurrence of `SGR` in the data stream, depending on the setting of the GRAPHIC RENDITION
/// COMBINATION MODE ([`GRCM`][crate::modes::GRCM]). Each graphic rendition aspect is specified by a parameter value of
/// [`GraphicRendition`].
///
/// The default value for `s` is [`GraphicRendition::Default`].
///
/// ## Note
///
/// The usable combinations of parameter values are determined by the implementation.
pub fn SGR(s: Option<Vec<GraphicRendition>>) -> ControlFunction<'static> {
    let g = s.unwrap_or(vec![Default::default()]);
    sequence!(06 / 13, variadic selective g)
}

/// Valid parameter values to the function [`SHS`].
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum CharacterSpacing {
    /// 10 characters per 25.4 mm.
    #[default]
    TenCharacters = 0,

    /// 12 characters per 25.4 mm.
    TwelveCharacters,

    /// 15 characters per 25.4 mm.
    FifteenCharacters,

    /// 6 characters per 25.4 mm.
    SixCharacters,

    /// 3 characters per 25.4 mm.
    ThreeCharacters,

    /// 9 characters per 25.4 mm.
    NineCharacters,

    /// 4 characters per 25.4 mm.
    FourCharacters,
}

/// Select Character Spacing.
///
/// `SHS` is used to establish the character spacing for subsequent text. The established spacing remains in effect
/// until the next occurrence of `SHS` or of SET CHARACTER SPACING ([`SCS`]) or of SPACING INCREMENT ([`SPI`]) in the
/// data stream.
///
/// The default value for `s` is [`CharacterSpacing::TenCharacters`].
pub fn SHS(s: Option<CharacterSpacing>) -> ControlFunction<'static> {
    sequence!(02 / 00, 04 / 11, selective default s)
}

/// Valid parameter values to the function [`SIMD`].
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum MovementDirection {
    /// The direction of implicit movement is the same as that of the character progression.
    #[default]
    Normal = 0,

    /// The direction of implicit movement is opposite to that of the character progression.
    Opposite,
}

/// Select Implicit Movement Direction.
///
/// `SIMD` is used to select the direction of implicit movement of the data position relative to the character
/// progression. The direction selected remains in effect until the next occurrence of [`SIMD`].
///
/// The default value of `s` is [`MovementDirection::Normal`].
pub fn SIMD(s: Option<MovementDirection>) -> ControlFunction<'static> {
    sequence!(05 / 14, selective default s)
}

/// Scroll Left.
///
/// `SL` causes the data in the presentation component to be moved by `n` character positions if the line orientation
/// is horizontal, or by `n` line positions if the line orientation is vertical, such that the data appear to move
/// to the left.
///
/// The active presentation position is not affected by this control function.
///
/// The default value for `n` is `1`.
pub fn SL(n: Option<u32>) -> ControlFunction<'static> {
    sequence!(02 / 00, 04 / 00, numeric n, default 1)
}

/// Set Line Home.
///
/// If the DEVICE COMPONENT SELECT MODE ([`DCSM`][crate::modes::DCSM]) is set to PRESENTATION, `SLH` is used to
/// establish at character position `n` in the active line (the line that contains the active presentation position) and
/// lines of subsequent text in the presentation component the position to which the active presentation position will
/// be moved by subsequent occurrences of CARRIAGE RETURN ([`CR`][crate::c0::CR]), DELETE LINE ([`DL`]), INSERT LINE
/// ([`IL`]) or NEXT LINE ([`NEL`][crate::c1::NEL]) in the data stream. In the case of a device without data component,
/// it is also the position ahead of which no implicit movement of the active presentation position shall occur.
///
/// If the DEVICE COMPONENT SELECT MODE ([`DCSM`][crate::modes::DCSM]) is set to DATA, `SLH` is used to establish at
/// character position `n` in the active line (the line that contains the active data position) and lines of subsequent
/// text in the data component the position to which the active data position will be moved by subsequent occurrences of
/// CARRIAGE RETURN ([`CR`][crate::c0::CR]), DELETE LINE ([`DL`]), INSERT LINE ([`IL`]) or NEXT LINE
/// ([`NEL`][crate::c1::NEL]) in the data stream. It is also the position ahead of which no implicit movement of the
/// active data position shall occur.
///
/// The established position is called the line home position and remains in effect until the next occurrence of `SLH`
/// in the data stream.
pub fn SLH(n: u32) -> ControlFunction<'static> {
    sequence!(02 / 00, 05 / 05, numeric n)
}

/// Set Line Limit.
///
/// If the DEVICE COMPONENT SELECT MODE ([`DCSM`][crate::modes::DCSM]) is set to PRESENTATION, `SLL` is used to
/// establish at character position `n` in the active line (the line that contains the active presentation position) and
/// lines of subsequent text in the presentation component the position to which the active presentation position will
/// be moved by subsequent occurrences of CARRIAGE RETURN ([`CR`][crate::c0::CR]), or NEXT LINE
/// ([`NEL`][crate::c1::NEL]) in the data stream if the parameter value of SELECT IMPLICIT MOVEMENT DIRECTION ([`SIMD`])
/// is equal to [`MovementDirection::Opposite`]. In the case of a device without data component, it is also the position
/// beyond which no implicit movement of the active presentation position shall occur.
///
/// If the DEVICE COMPONENT SELECT MODE ([`DCSM`][crate::modes::DCSM]) is set to DATA, `SLL` is used to establish at
/// character position `n` in the active line (the line that contains the active data position) and lines of subsequent
/// text in the data component the position beyond which no implicit movement of the active data position shall occur.
/// It is also the position in the data component to which the active data position will be moved by subsequent
/// occurrences of [`CR`][crate::c0::CR] or [`NEL`][crate::c1::NEL] in the data stream, if the parameter value of
/// SELECT IMPLICIT MOVEMENT DIRECTION ([`SIMD`]) is equal to [`MovementDirection::Opposite`].
///
/// The established position is called the line limit position and remains in effect until the next occurrence of `SLL`
/// in the data stream.
pub fn SLL(n: u32) -> ControlFunction<'static> {
    sequence!(02 / 00, 05 / 06, numeric n)
}

/// Set Line Spacing.
///
/// `SLS` is used to establish the line spacing for subsequent text. The established spacing remains in effect until the
/// next occurrence of `SLS` or of SELECT LINE SPACING ([`SVS`]) or of SPACING INCREMENT ([`SPI`]) in the data stream.
///
/// `n` specifies the line spacing.
///
/// The unit in which the parameter value is expressed is that established by the parameter value of SELECT SIZE UNIT
/// ([`SSU`]).
pub fn SLS(n: u32) -> ControlFunction<'static> {
    sequence!(02 / 00, 06 / 08, numeric n)
}

/// Set Mode.
///
/// `SM` causes the modes of the receiving device to be set as specified by the parameter values.
pub fn SM(s: Vec<Mode>) -> ControlFunction<'static> {
    sequence!(06 / 08, variadic selective s)
}

/// Valid parameter values to the function [`SPD`].
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum PresentationDirection {
    /// Horizontal line orientation, top-to-bottom line progression, left-to-right character path.
    #[default]
    HorizontalLinesTopToBottomLeftToRight = 0,

    /// Vertical line orientation, right-to-left line progression, top-to-bottom character path.
    VerticalLinesRightToLeftTopToBottom,

    /// Vertical line orientation, left-to-right line progression, top-to-bottom character path.
    VerticalLinesLeftToRightTopToBottom,

    /// Horizontal line orientation, top-to-bottom line progression, right-to-left character path.
    HorizontalLinesTopToBottomRightToLeft,

    /// Vertical line orientation, left-to-right line progression, bottom-to-top character path.
    VerticalLinesLeftToRightBottomToTop,

    /// Horizontal line orientation, bottom-to-top line progression, right-to-left character path.
    HorizontalLinesBottomToTopRightToLeft,

    /// Horizontal line orientation, bottom-to-top line progression, left-to-right character path.
    HorizontalLinesBottomToTopLefToRight,

    /// Vertical line orientation, right to left line progression, bottom-to-top character path.
    VerticalLinesRightToLeftBottomToTop,
}

/// Valid parameter values to the function [`SPD`].
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum PresentationDirectionScope {
    /// Undefined, implementation specific.
    ///
    /// ## Note
    ///
    /// This may also permit the effect to take place after the next occurrence of [`CR`][crate::c0::CR],
    /// [`NEL`][crate::c1::NEL] or any control function which initiates an absolute movement of the active presentation
    /// position or the active data position.
    #[default]
    Undefined = 0,

    /// The content of the presentation component is updated to correspond to the content of the data component
    /// according to the newly established characteristics of the presentation component; the active data position is
    /// moved to the first character position in the first line in the data component, the active presentation position
    /// in the presentation component is updated accordingly.
    InPresentationComponent,

    /// The content of the data component is updated to correspond to the content of the presentation component
    /// according to the newly established characteristics of the presentation component; the active presentation
    /// position is moved to the first character position in the first line in the presentation component, the active
    /// data position in the data component is updated accordingly.
    InDataComponent,
}

/// Select Presentation Directions.
///
/// `SPD` is used to select the line orientation, the line progression, and the character path in the presentation
/// component. It is also used to update the content of the presentation component and the content of the data
/// component. This takes effect immediately.
///
/// `s` specifies the line orientation, the line progression and the character path.  
/// `t` specifies the effect on the content of the presentation component and the content of the data component.
///
/// The default value for `s` is [`PresentationDirection::HorizontalLinesTopToBottomLeftToRight`].  
/// The default value for `t` is [`PresentationDirectionScope::Undefined`].
pub fn SPD(
    s: Option<PresentationDirection>,
    t: Option<PresentationDirectionScope>,
) -> ControlFunction<'static> {
    sequence!(02 / 00, 05 / 03, selective default s, selective default t)
}

/// Set Page Home.
///
/// If the DEVICE COMPONENT SELECT MODE ([`DCSM`][crate::modes::DCSM]) is set to PRESENTATION, `SPH` is used to
/// establish at line position `n` in the active page (the page that contains the active presentation position) and
/// subsequent pages in the presentation component the position to which the active presentation position will be moved
/// by subsequent occurrences of FORM FEED ([`FF`][crate::c0::FF]) in the data stream. In the case of a device without
/// data component, it is also the position ahead of which no implicit movement of the active presentation position
/// shall occur.
///
/// If the DEVICE COMPONENT SELECT MODE ([`DCSM`][crate::modes::DCSM]) is set to DATA, `SPH` is used to establish at
/// line position `n` in the active page (the page that contains the active data position) and subsequent pages in the
/// data component the position to which the active data position will be moved by subsequent occurrences of FORM FEED
/// ([`FF`][crate::c0::FF]) in the data stream. It is also the position ahead of which no implicit movement of the
/// active presentation position shall occur.
///
/// The established position is called the page home position and remains in effect until the next occurrence of `SPH`
/// in the data stream.
pub fn SPH(n: u32) -> ControlFunction<'static> {
    sequence!(02 / 00, 06 / 09, numeric n)
}

/// Spacing Increment.
///
/// `SPI` is used to establish the line spacing and the character spacing for subsequent text. The established line
/// spacing remains in effect until the next occurrence of `SPI` or of SET LINE SPACING ([`SLS`]) or of SELECT LINE
/// SPACING ([`SVS`]) in the data stream. The established character spacing remains in effect until the next occurrence
/// of SET CHARACTER SPACING ([`SCS`]) or of SELECT CHARACTER SPACING ([`SHS`]) in the data stream.
///
/// `l` specifies the line spacing.  
/// `c` specifies the character spacing.
///
/// The unit in which the parameter values are expressed is that established by the parameter value of SELECT SIZE UNIT
/// ([`SSU`]).
pub fn SPI(l: u32, c: u32) -> ControlFunction<'static> {
    sequence!(02 / 00, 04 / 07, numeric l, numeric c)
}

/// Set Page Limit.
///
/// If the DEVICE COMPONENT SELECT MODE ([`DCSM`][crate::modes::DCSM]) is set to PRESENTATION, `SPL` is used to
/// establish at line position `n` in the active page (the page that contains the active presentation position) and
/// pages of subsequent text in the presentation component the position beyond which the active presentation position
/// can normally not be moved. In the case of a device without data component, it is also the position beyond which no
/// implicit movement of the active presentation position shall occur.
///
/// If the DEVICE COMPONENT SELECT MODE ([`DCSM`][crate::modes::DCSM]) is set to DATA, `SPL` is used to establish at
/// line position `n` in the active page (the page that contains the active data position) and pages of subsequent text
/// in the data component the position beyond which no implicit movement of the active data position shall occur.
///
/// The established position is called the page limit position and remains in effect until the next occurrence of `SPL`
/// in the data stream.
pub fn SPL(n: u32) -> ControlFunction<'static> {
    sequence!(02 / 00, 06 / 10, numeric n)
}

/// Valid parameter values to the function [`SPQR`].
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum PrintQuality {
    /// Highest available print quality, low print speed
    #[default]
    HighQualityLowSpeed = 0,

    /// Medium print quality, medium print speed.
    MediumQualityMediumSpeed,

    /// Draft print quality, highest available print speed
    LowQualityHighSpeed,
}

/// Select Print Quality and Rapidity.
///
/// `SPQR` is used to select the relative print quality and the print speed for devices where the output quality and
/// speed of are inversely related. The selected values remain in effect until the next occurrence of `SPQR` in the
/// data stream.
///
/// The default value of `s` is [`PrintQuality::HighQualityLowSpeed`].
pub fn SPQR(s: Option<PrintQuality>) -> ControlFunction<'static> {
    sequence!(02 / 00, 05 / 08, selective default s)
}

/// Scroll Right.
///
/// `SR` causes the data in the presentation component to be moved by `n` character positions if the line orientation is
/// horizontal, or by `n` line positions if the line orientation is vertical, such that the data appear to move to the
/// right.
///
/// The active presentation position is not affected by this control function.
///
/// The default value for `n` is `1`.
pub fn SR(n: Option<u32>) -> ControlFunction<'static> {
    sequence!(02 / 00, 05 / 08, numeric n, default 1)
}

/// Set Reduced Character Separation.
///
/// `SRCS` is used to establish reduced inter-character escapement for subsequent text. The established reduced
/// escapement remains in effect until the next occurrence of `SRCS` or of SET ADDITIONAL CHARACTER SEPARATION
/// ([`SACS`]) in the data stream or until it is reset to the default value by a subsequent occurrence of
/// CARRIAGE RETURN/LINE FEED ([`CR`][crate::c0::CR]/[`LF`][crate::c0::LF]) or of NEXT LINE ([`NEL`][crate::c1::NEL])
/// in the data stream.
///
/// `n` specifies the number of units by which the inter-character escapement is reduced.
///
/// The unit in which the parameter values are expressed is that established by the parameter value of SELECT SIZE UNIT
/// ([`SSU`]).
///
/// The default value of `n` is `0`.
pub fn SRCS(n: Option<u32>) -> ControlFunction<'static> {
    sequence!(02 / 00, 06 / 06, numeric n, default 0)
}

/// Valid parameter values to the function [`SRS`].
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum ReversedString {
    /// End of a reversed string; re-establish the previous direction.
    #[default]
    End = 0,

    /// Beginning of a reversed string; reverse the direction
    Start,
}

/// Start Reversed String.
///
/// `SRS` is used to establish in the data component the beginning and the end of a string of characters as well as
/// the direction of the string. This direction is opposite to that currently established. The indicated string follows
/// the preceding text. The established character progression is not affected.
///
/// The beginning of a reversed string is indicated by `SRS` with a parameter value of [`ReversedString::Start`].
/// A reversed string may contain one or more nested strings. These nested strings may be reversed strings the
/// beginnings of which are indicated by `SRS` with a parameter value of [`ReversedString::Start`], or directed strings
/// the beginnings of which are indicated by START DIRECTED STRING ([`SDS`]) with a parameter value not equal to
/// [`StringDirection::End`]. Every beginning of such a string invokes the next deeper level of nesting.
///
/// This Standard does not define the location of the active data position within any such nested string.
///
/// The end of a reversed string is indicated by `SRS` with a parameter value of [`ReversedString::End`]. Every end of
/// such a string re-establishes the next higher level of nesting (the one in effect prior to the string just ended).
/// The direction is re-established to that in effect prior to the string just ended. The active data position is moved
/// to the character position following the characters of the string just ended.
///
/// Default value of `s` is [`ReversedString::End`].
///
/// ## Note 1
///
/// The effect of receiving a [`CVT`], [`HT`][crate::c0::HT], [`SCP`], [`SPD`], or [`VT`][crate::c0::VT] control
/// function within an `SRS` string is not defined.
///
/// ## Note 2
///
/// The control functions for area definition ([`DAQ`], [`EPA`][crate::c1::EPA], [`ESA`][crate::c1::ESA],
/// [`SPA`][crate::c1::SPA], [`SSA`][crate::c1::SSA]) should not be used within an `SRS` string.
pub fn SRS(s: Option<ReversedString>) -> ControlFunction<'static> {
    sequence!(05 / 11, selective default s)
}

/// Valid parameter values to the function [`SSU`].
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum SizeUnit {
    /// The dimension of this unit are device-dependent.
    #[default]
    Character = 0,

    /// Millimetre.
    Millimetre,

    /// Computer Decipoint (0.03528 mm - 1/720 of 25.4 mm).
    ComputerDecipoint,

    /// Decidot (0.03759 mm - 10/266 mm).
    Decidot,

    /// Mil (0.0254 mm - 1/1000 of 25.4 mm).
    Mil,

    /// Basic Measuring Unit (BMU) (0.02117 mm - 1/1200 of 25.4 mm).
    BasicMeasuringUnit,

    /// Micrometer (0.001 mm).
    Micrometer,

    /// Pixel. The smallest increment that can be specified in a device.
    Pixel,

    /// Decipoint (0.03514mm - 35/996 mm).
    Decipoint,
}

/// Select Size Unit.
///
/// `SSU` is used to establish the unit in which the numeric parameters of certain control functions are expressed. The
/// established unit remains in effect until the next occurrence of `SSU` in the data stream.
///
/// Default value of `s` is [`SizeUnit::Character`].
pub fn SSU(s: Option<SizeUnit>) -> ControlFunction<'static> {
    sequence!(02 / 00, 04 / 09, selective default s)
}

/// Set Space Width.
///
/// `SSW` is used to establish for subsequent text the character escapement associated with the character `SPACE`. The
/// established escapement remains in effect until the next occurrence of `SSW` in the data stream or until it is reset
/// to the default value by a subsequent occurrence of CARRIAGE RETURN/LINE FEED
/// ([`CR`][crate::c0::CR]/[`LF`][crate::c0::LF]), CARRIAGE RETURN/FORM FEED
/// ([`CR`][crate::c0::CR]/[`FF`][crate::c0::FF]), or of NEXT LINE ([`NEL`][crate::c1::NEL]) in the data stream.
///
/// `n` specifies the escapement.
///
/// The unit in which the parameter value is expressed is that established by the parameter value of SELECT SIZE UNIT
/// ([`SSU`]).
///
/// The default character escapement of SPACE is specified by the most recent occurrence of SET CHARACTER SPACING
/// ([`SCS`]) or of SELECT CHARACTER SPACING ([`SHS`]) or of SELECT SPACING INCREMENT ([`SPI`]) in the data stream if
/// the current font has constant spacing, or is specified by the nominal width of the character `SPACE` in the current
/// font if that font has proportional spacing.
pub fn SSW(n: u32) -> ControlFunction<'static> {
    sequence!(02 / 00, 05 / 11, numeric n)
}

/// Selective Tabulation.
///
/// `STAB` causes subsequent text in the presentation component to be aligned according to the position and the
/// properties of a tabulation stop which is selected from a list according to the value of the parameter `s`.
///
/// The use of this control function and means of specifying a list of tabulation stop to be referenced by the control
/// function are specified in other standards, for example ISO 8613-6.
pub fn STAB(s: u32) -> ControlFunction<'static> {
    sequence!(02 / 00, 05 / 14, numeric s)
}

/// Scroll Up.
///
/// `SU` causes the data in the presentation component to be moved by `n` line positions if the line operation is
/// horizontal, or by `n` character positions if the line orientation is vertical, such that the dat data appear to move
/// up.
///
/// The active presentation position is not affected by this control function.
///
/// The default value for `n` is `1`.
pub fn SU(n: Option<u32>) -> ControlFunction<'static> {
    sequence!(05 / 03, numeric n, default 1)
}

/// Valid parameter values to the function [`SLS`].
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum LineSpacing {
    /// Six lines per 25.4 mm.
    #[default]
    SixLinesPer25 = 0,

    /// Four lines per 25.4 mm.
    FourLinesPer25,

    /// Three lines per 25.4 mm.
    ThreeLinesPer25,

    /// Twelve lines per 25.4 mm.
    TwelveLinesPer25,

    /// Eight lines per 25.4 mm.
    EightLinesPer25,

    /// Six lines per 30 mm.
    SixLinesPer30,

    /// Four lines per 30 mm.
    FourLinesPer30,

    /// Three lines per 30 mm.
    ThreeLinesPer30,

    /// Twelve lines per 30 mm.
    TwelveLinesPer30,

    /// Two lines per 25.4 mm.
    TwoLinesPer25,
}

/// Select Line Spacing.
///
/// `SVS` is used to establish the line spacing for subsequent text. The established spacing remains in effect until the
/// next occurrence of `SVS` or of SET LINE SPACING ([`SLS`]) or of SPACING INCREMENT ([`SPI`]) in the data stream.
///
/// The default value for `s` is [`LineSpacing::SixLinesPer25`].
pub fn SVS(s: Option<LineSpacing>) -> ControlFunction<'static> {
    sequence!(02 / 00, 04 / 12, selective default s)
}

/// Tabulation Aligned Centred.
///
/// `TAC` causes a character tabulation stop calling for centring to be set at character position `n` in the active
/// line (the line that contains the active presentation position) and lines of subsequent text in the presentation
/// component. `TAC` causes the replacement of any tabulation stop previously set at that character position, but does
/// not affect other tabulation stops.
///
/// A text string centred upon a tabulation stop set by `TAC` will be positioned so that the (trailing edge of the)
/// first graphic character and the (leading edge of the) last graphic character are at approximately equal distances
/// from the tabulation stop.
pub fn TAC(n: u32) -> ControlFunction<'static> {
    sequence!(02 / 00, 06 / 02, numeric n)
}

/// Tabulation Aligned Leading Edge.
///
/// `TALE` causes a character tabulation stop calling for leading edge alignment to be set at character position `n` in
/// the active line (the line that contains the active presentation position) and lines of subsequent text in the
/// presentation component. `TALE` causes the replacement of any tabulation stop previously set at that character
/// position, but does not affect other tabulation stops.
///
/// A text string aligned with a tabulation stop set by `TALE` will be positioned so that the (leading edge of the) last
/// graphic character of the string is placed at the tabulation stop.
pub fn TALE(n: u32) -> ControlFunction<'static> {
    sequence!(02 / 00, 06 / 01, numeric n)
}

/// Tabulation Aligned Trailing Edge.
///
/// `TATE` causes a character tabulation stop calling for trailing edge alignment to be set at character position `n`
/// in the active line (the line that contains the active presentation position) and lines of subsequent text in the
/// presentation component. `TATE` causes the replacement of any tabulation stop previously set at the character
/// position, but does not affect other tabulation stops.
///
/// A text string aligned with a tabulation stop set by `TATE` will be positioned so that the (trailing edge of the)
/// first graphic character of the string is placed at the tabulation stop.
pub fn TATE(n: u32) -> ControlFunction<'static> {
    sequence!(02 / 00, 06 / 00, numeric n)
}

/// Valid parameter values to the function [`TBC`].
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum ClearTabulation {
    /// Clear the character tabulation stop at the active presentation position.
    #[default]
    CharacterTabulationStopActivePosition = 0,

    /// Clear the line tabulation stop at the active line.
    LineTabulationStopActiveLine,

    /// Clear all character tabulation stops at the active line.
    AllCharacterTabulationStopsActiveLine,

    /// Clear all character tabulation stops.
    AllCharacterTabulationStops,

    /// Clear all line tabulation stops.
    AllLineTabulationStops,

    /// Clear all tabulation stops.
    AllTabulationStops,
}

/// Tabulation Clear.
///
/// `TBC` causes one or more tabulation stops in the presentation component to be cleared, depending on the parameter
/// value `s`.
///
/// The default value for `s` is [`ClearTabulation::CharacterTabulationStopActivePosition`].
pub fn TBC(s: Option<ClearTabulation>) -> ControlFunction<'static> {
    sequence!(06 / 07, selective default s)
}

/// Tabulation Centred on Character.
///
/// `TCC` causes a character tabulation stop calling for alignment of a target graphic character to be set at character
/// position `n` in the active line (the line that contains the active presentation position) and lines of subsequent
/// text in the presentation component, and the target character about which centring is to be performed is specified
/// by parameter `m`. `TCC` causes the replacement of any tabulation stop previously set at that character position, but
/// does not affect other tabulation stops.
///
/// The positioning of a text string aligned with a tabulation stop set by `TCC` will be determined by the first
/// occurrence in the string of the target graphic character; that character will be centred upon the tabulation stop.
/// If the target character does not occur within the string, then the trailing edge of the first character of the
/// string will be positioned at the tabulation stop.
///
/// The value of `m` indicates the code table position (binary value) of the target character in the currently invoked
/// code. For a 7-bit code, the permissible range of values is `32` to `127`; for an 8-bit code, the permissible range
/// of values is `32` to `127` and `160` to `255`.
///
/// The default value of `m` is `32`.
pub fn TCC(n: u32, m: Option<u32>) -> ControlFunction<'static> {
    let k = m.unwrap_or(32);
    sequence!(02 / 00, 06 / 03, numeric n, numeric k)
}

/// Tabulation Stop Remove.
///
/// `TSR` causes any character tabulation stop at character position `n` in the active line (the line that contains the
/// active presentation position) and lines of subsequent text in the presentation component to be cleared, but does
/// not affect other tabulation stops.
///
pub fn TSR(n: u32) -> ControlFunction<'static> {
    sequence!(02 / 00, 06 /04, numeric n)
}

/// Thin Space Specification.
///
/// `TSS` is used to establish the width of a thin space for subsequent text. The established width remains in effect
/// until the next occurrence of `TSS` in the data stream.
///
/// `n` specifies the width of the thin space.
///
/// The unit in which the parameter value is expressed is that established by the parameter value of SELECT SIZE UNIT
/// ([`SSU`]).
pub fn TSS(n: u32) -> ControlFunction<'static> {
    sequence!(02 / 00, 04 / 05, numeric n)
}

/// Line Position Absolute.
///
/// `VPA` causes the active data position to be moved to line position `n` in the data component in a direction
/// parallel to the line progression.
///
/// The default value for `n` is `1`.
pub fn VPA(n: Option<u32>) -> ControlFunction<'static> {
    sequence!(06 / 04, numeric n, default 1)
}

/// Line Position Backward.
///
/// `VPB` causes the active data position to be moved by `n` line positions in the data component in a direction
/// opposite to that of the line progression.
///
/// The default value for `n` is `1`.
pub fn VPB(n: Option<u32>) -> ControlFunction<'static> {
    sequence!(06 / 11, numeric n, default 1)
}

/// Line Position Forward.
///
/// `VPR` causes the active data position to be moved `n` line positions in the data component in a direction parallel
/// to the line progression.
///
/// The default value for `n` is `1`.
pub fn VPR(n: Option<u32>) -> ControlFunction<'static> {
    sequence!(06 / 05, numeric n, default 1)
}
