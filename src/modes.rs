//! Modes.
//!
//! The [ECMA-48][ecma-48] standard is intended to be applicable to a very large range of devices, in which there are
//! variations. Some of these variations have been formalized in the form of modes. They deal with the way in which a
//! device transmits, receives, processes, or images data. Each mode has two states. The reset state, and the set state.
//!
//! The states of the modes may be established explicitly in the data stream by the control functions SET MODE
//! ([`SM`]) and RESET MODE ([`RM`]) or may be established
//! by agreement between sender and recipient. In an implementation, some or all of the modes have one state only.
//!
//! To ensure data compatibility and ease of interchange with a variety of equipment the use of modes is deprecated. If
//! modes have to be implemented for backward compatibility it is recommended that the reset state of the modes be the
//! initial state. Otherwise, explicit agreements will have to be negotiated between sender and recipient, to the
//! detriment of "blind" interchange.
//!
//! ## Usage
//!
//! Two possibilities exist to use modes.
//!
//! ### Directly invoking `SM` or `RM` Control Functions
//!
//! You can pass modes to the arguments of the control functions SET MODE ([`SM`]) and
//! RESET MODE ([`RM`]).
//!
//! ```
//! use ansi_control_codes::control_sequences;
//! use ansi_control_codes::modes;
//!
//! // set the device component select mode to PRESENTATION.
//! print!("{}", control_sequences::SM(vec![modes::DCSM]));
//! // set the device component select mode to DATA.
//! print!("{}", control_sequences::RM(vec![modes::Mode::DeviceComponentSelectMode]));
//! ```
//!
//! ### Setting or Resetting Modes
//!
//! You can invoke the set and reset functions of a mode instead.
//!
//! ```
//! use ansi_control_codes::modes;
//!
//! // set the device component select mode to PRESENTATION.
//! print!("{}", modes::DCSM.set());
//! // set the device component select mode to DATA.
//! print!("{}", modes::Mode::DeviceComponentSelectMode.reset());
//! ```
//!
//! [ecma-48]: https://www.ecma-international.org/publications-and-standards/standards/ecma-48/

use crate::ControlFunction;

/// Device Modes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    /// Guarded Area Transfer Mode `GATM`.
    ///
    /// ## Reset: Guard
    ///
    /// Only the contents of unguarded areas in an eligible area are transmitted or transferred.
    ///
    /// ## Set: All
    ///
    /// The contents of guarded as well as of unguarded areas in an eligible area are transmitted or transferred.
    ///
    /// ## Note
    ///
    /// No control functions are affected.
    GuardedAreaTransferMode = 1,

    /// Keyboard Action Mode `KAM`.
    ///
    /// ## Reset: Enabled
    ///
    /// All or part of the manual input facilities are enabled to be used.
    ///
    /// ## Set: Disabled
    ///
    /// All or part of the manual input facilities are disabled.
    ///
    /// ## Note
    ///
    /// No control functions are affected.
    KeyboardActionMode,

    /// Control Presentation Mode `CRM`.
    ///
    /// ## Reset: Control
    ///
    /// All control functions are performed as defined; the way formator functions are processed depends on the setting
    /// of the FORMAT EFFECTOR ACTION MODE ([`FEAM`]). A device may choose to image the graphical representations of
    /// control functions in addition to performing them.
    ///
    /// ## Set: Graphic
    ///
    /// All control functions, except RESET MODE ([`RM`]), are treated as graphic characters. A device may choose to
    /// perform some control functions in addition to storing them and imaging their graphical representations.
    ControlPresentationMode,

    /// Insertion Replacement Mode `IRM`.
    ///
    /// ## Reset: Replace
    ///
    /// The graphic symbol of a graphic character or of a control function, for which a graphical representation is
    /// required, replaces (or, depending upon the implementation, is combined with) the graphic symbol imaged at the
    /// active presentation position.
    ///
    /// ## Set: Insert
    ///
    /// The graphic symbol of a graphic character or of a control function, for which a graphical representation is
    /// required, is inserted at the active presentation position.
    ///
    /// ## Note
    ///
    /// Only control functions for which a graphical representation is required are affected.
    InsertionReplacementMode,

    /// Status Report Transfer Mode `SRTM`.
    ///
    /// ## Reset: Normal
    ///
    /// Status reports in the form of DEVICE CONTROL STRINGs ([`DCS`][crate::c1::DCS]) are not generated automatically.
    ///
    /// ## Set: Diagnostic
    ///
    /// Status reports in the form of DEVICE CONTROL STRINGs ([`DCS`][crate::c1::DCS]) are included in every data stream
    /// transmitted or transferred.
    ///
    /// ## Note:
    ///
    /// No control functions are affected.
    StatusReportTransferMode,

    /// Erasure Mode `ERM`.
    ///
    /// ## Reset: Protect
    ///
    /// Only the contents of unprotected areas are affected by an erasure control function.
    ///
    /// ## Set: All
    ///
    /// The contents of protected as well as of unprotected areas are affected by an erasure control function.
    ///
    /// ## Note
    ///
    /// Control functions affected are: [`EA`][crate::control_sequences::EA], [`ECH`][crate::control_sequences::ECH],
    /// [`ED`][crate::control_sequences::ED], [`EF`][crate::control_sequences::EF],
    /// [`EL`][crate::control_sequences::EL].
    ErasureMode,

    /// Line Editing Mode `VEM`.
    ///
    /// ## Reset: Following
    ///
    /// If the DEVICE COMPONENT SELECT MODE ([`DCSM`]) is set to PRESENTATION, a line insertion causes the contents of
    /// the active line (the line that contains the active presentation position) and of the following lines in the
    /// presentation component to be shifted in the direction of the line progression; a line deletion causes the
    /// contents of the lines following the active line to be shifted in the direction opposite to that of the line
    /// progression.
    ///
    /// If the DEVICE COMPONENT SELECT MODE ([`DCSM`]) is set to DATA, a line insertion causes the contents of the
    /// active line (the line that contains the active data position) and of the following lines in the data component
    /// to be shifted in the direction of the line progression; a line deletion causes the contents of the lines
    /// following the active line to be shifted in the direction opposite to that of the line progression.
    ///
    /// ## Set: Preceding
    ///
    /// If the DEVICE COMPONENT SELECT MODE ([`DCSM`]) is set to PRESENTATION, a line insertion causes the contents of
    /// the active line (the line that contains the active presentation position) and of the preceding lines to be
    /// shifted in the direction opposite to that of the line progression; a line deletion causes the contents of the
    /// lines preceding the active line to be shifted in the direction of the line progression.
    ///
    /// If the DEVICE COMPONENT SELECT MODE ([`DCSM`]) is set to DATA, a line insertion causes the contents of the
    /// active line (the line that contains the active data position) and of the preceding lines to be shifted in the
    /// direction opposite to that of the line progression; a line deletion causes the contents of the lines preceding
    /// the active line to be shifted in the direction of the line progression.
    ///
    /// ## Note
    ///
    /// Control functions affected are: [`DL`][crate::control_sequences::DL], [`IL`][crate::control_sequences::IL].
    LineEditingMode,

    /// Bi-directional support mode `BDSM`.
    ///
    /// ## Reset: Explicit
    ///
    /// Control functions are performed in the data component or in the presentation component, depending on the setting
    /// of the DEVICE COMPONENT SELECT MODE ([`DeviceComponentSelectMode`][Mode::DeviceComponentSelectMode]).
    ///
    /// ## Set: Implicit
    ///
    /// Control functions are performed in the data component. All bi-directional aspects of data are handled by the
    /// device itself.
    BiDirectionalSupportMode,

    /// Device Component Select Mode `DCSM`.
    ///
    /// ## Reset: Presentation
    ///
    /// Certain control functions are performed in the presentation component. The active presentation position (or the
    /// active line, where applicable) in the presentation component is the reference position against which the
    /// relevant control functions are performed.
    ///
    /// ## Set: Data
    ///
    /// Certain control functions are performed in the data component. The active data position (or the active line,
    /// where applicable) in the data component is the reference position against which the relevant control functions
    /// are performed.
    ///
    /// ## Note
    ///
    /// Control functions affected are: [`CPR`][crate::control_sequences::CPR], [`CR`][crate::c0::CR],
    /// [`DCH`][crate::control_sequences::DCH], [`DL`][crate::control_sequences::DL],
    /// [`EA`][crate::control_sequences::EA], [`ECH`][crate::control_sequences::ECH],
    /// [`ED`][crate::control_sequences::ED], [`EF`][crate::control_sequences::EF],
    /// [`EL`][crate::control_sequences::EF], [`ICH`][crate::control_sequences::ICH],
    /// [`IL`][crate::control_sequences::IL], [`LF`][crate::c0::LF], [`NEL`][crate::c1::NEL], [`RI`][crate::c1::RI],
    /// [`SLH`][crate::control_sequences::SLH], [`SLL`][crate::control_sequences::SLL],
    /// [`SPH`][crate::control_sequences::SPH], [`SPL`][crate::control_sequences::SPH].
    DeviceComponentSelectMode,

    /// Character Editing Mode `HEM`.
    ///
    /// ## Reset: Following
    ///
    /// If the DEVICE COMPONENT SELECT MODE ([`DCSM`]) is set to PRESENTATION, a character insertion causes the contents
    /// of the active presentation position and of the following character positions in the presentation component to be
    /// shifted in the direction of the character path; a character deletion causes the contents of the character
    /// positions following the active presentation position to be shifted in the direction opposite to that of the
    /// character path
    ///
    /// If the DEVICE COMPONENT SELECT MODE ([`DCSM`]) is set to DATA, a character insertion causes the contents of the
    /// active data position and of the following character positions in the data component to be shifted in the
    /// direction of the character progression; a character deletion causes the contents of the character positions
    /// following the active data position to be shifted in the direction opposite to that of the character progression.
    ///
    /// ## Set: Preceding
    ///
    /// If the DEVICE COMPONENT SELECT MODE ([`DCSM`]) is set to PRESENTATION, a character insertion causes the contents
    /// of the active presentation position and of the following character positions in the presentation component to be
    /// shifted in the direction opposite to that of the character path; a character deletion causes the contents of the
    /// character positions following the active presentation position to be shifted in the direction of the character
    /// path.
    ///
    /// If the DEVICE COMPONENT SELECT MODE ([`DCSM`]) is set to DATA, a character insertion causes the contents of the
    /// active data position and of preceding character positions in the data component to be shifted in the direction
    /// opposite to that of the character progression; a character deletion causes the contents of the character
    /// positions preceding the active data position to be shifted in the direction of the character progression.
    ///
    /// ## Note
    ///
    /// Control functions affected are: [`DCH`][crate::control_sequences::DCH], [`ICH`][crate::control_sequences::ICH].
    CharacterEditingMode,

    /// Positioning Unit Mode `PUM`.
    ///
    /// ## Reset: Character
    ///
    /// The unit for numeric parameters of the positioning format effectors is one character position.
    ///
    /// ## Set: Size
    ///
    /// The unit for numeric parameters of the positioning format effectors is that established by the parameter value
    /// of SELECT SIZE UNIT ([`SSU`][crate::control_sequences::SSU]).
    ///
    /// ## Note 1
    ///
    /// Control functions affected are: [`CUB`][crate::control_sequences::CUB], [`CUD`][crate::control_sequences::CUD],
    /// [`CUF`][crate::control_sequences::CUF], [`CUU`][crate::control_sequences::CUU],
    /// [`HPA`][crate::control_sequences::HPA], [`HPB`][crate::control_sequences::HPB],
    /// [`HPR`][crate::control_sequences::HPR], [`HVP`][crate::control_sequences::HVP],
    /// [`SLH`][crate::control_sequences::SLH], [`SLL`][crate::control_sequences::SLL],
    /// [`SSU`][crate::control_sequences::SSU], [`VPA`][crate::control_sequences::VPA],
    /// [`VPB`][crate::control_sequences::VPB], [`VPR`][crate::control_sequences::VPR].
    ///
    /// ## Note 2
    ///
    /// As the default parameter value of the control function SELECT SIZE UNIT (SSU) is CHARACTER, this mode is
    /// redundant and should no longer be used.
    ///
    /// # Note 3
    ///
    /// The use of the POSITIONING UNIT MODE ([`PUM`]) is deprecated.
    PositioningUnitMode,

    /// Send/Receive Mode `SRM`.
    ///
    /// ## Reset: Monitor
    ///
    /// Data which are locally entered are immediately imaged.
    ///
    /// ## Set: Simultaneous
    ///
    /// Local input facilities are logically disconnected from the output mechanism; only data which are sent to the
    /// device are imaged.
    ///
    /// ## Note
    ///
    /// No control functions are affected.
    SendReceiveMode,

    /// Format Effector Action Mode `FEAM`.
    ///
    /// ## Reset: Execute
    ///
    /// Formator functions are performed immediately and may be stored in addition to being performed.
    ///
    /// ## Set: Store
    ///
    /// Formator functions are stored but not performed. In this case, the specified action is intended to be performed
    /// by another device when the associated data are transmitted or transferred.
    ///
    /// ## Note
    ///
    /// Control functions affected are: [`BPH`][crate::c1::BPH], [`BS`][crate::c0::BS], [`CR`][crate::c0::CR],
    /// [`DTA`][crate::control_sequences::DTA], [`FF`][crate::c0::FF], [`FNT`][crate::control_sequences::FNT],
    /// [`GCC`][crate::control_sequences::GCC], [`GSM`][crate::control_sequences::GSM],
    /// [`GSS`][crate::control_sequences::GSS], [`HPA`][crate::control_sequences::HPA],
    /// [`HPB`][crate::control_sequences::HPB], [`HPR`][crate::control_sequences::HPR],
    /// [`HT`][crate::c0::HT], [`HTJ`][crate::c1::HTJ], [`HTS`][crate::c1::HTS], [`HVP`][crate::control_sequences::HVP],
    /// [`JFY`][crate::control_sequences::JFY], [`NEL`][crate::c1::NEL], [`PEC`][crate::control_sequences::PEC],
    /// [`PFS`][crate::control_sequences::PFS], [`PLD`][crate::c1::PLD], [`PLU`][crate::c1::PLU],
    /// [`PPA`][crate::control_sequences::PPA], [`PPB`][crate::control_sequences::PPB],
    /// [`PPR`][crate::control_sequences::PPR], [`PTX`][crate::control_sequences::PTX],
    /// [`QUAD`][crate::control_sequences::QUAD], [`RI`][crate::c1::RI], [`SACS`][crate::control_sequences::SACS],
    /// [`SAPV`][crate::control_sequences::SAPV], [`SCO`][crate::control_sequences::SCO],
    /// [`SCS`][crate::control_sequences::SCS], [`SGR`][crate::control_sequences::SGR],
    /// [`SHS`][crate::control_sequences::SHS], [`SLH`][crate::control_sequences::SLH],
    /// [`SLL`][crate::control_sequences::SLL], [`SLS`][crate::control_sequences::SLS],
    /// [`SPD`][crate::control_sequences::SPD], [`SPI`][crate::control_sequences::SPI],
    /// [`SPQR`][crate::control_sequences::SPQR], [`SRCS`][crate::control_sequences::SRCS],
    /// [`SRS`][crate::control_sequences::SRS], [`SSU`][crate::control_sequences::SSU],
    /// [`SSW`][crate::control_sequences::SSW], [`STAB`][crate::control_sequences::STAB],
    /// [`SVS`][crate::control_sequences::SVS], [`TAC`][crate::control_sequences::TAC],
    /// [`TALE`][crate::control_sequences::TALE], [`TATE`][crate::control_sequences::TATE],
    /// [`TBC`][crate::control_sequences::TBC], [`TCC`][crate::control_sequences::TCC],
    /// [`TSS`][crate::control_sequences::TSS], [`VPA`][crate::control_sequences::VPA],
    /// [`VPB`][crate::control_sequences::VPB], [`VPR`][crate::control_sequences::VPR],
    /// [`VTS`][crate::c1::VTS].
    FormatEffectorActionMode,

    /// Format Effector Transfer Mode `FETM`.
    ///
    /// ## Reset: Insert
    ///
    /// Formator functions may be inserted in a data stream to be transmitted or in data to be transferred to an
    /// auxiliary input/output device.
    ///
    /// ## Set: Exclude
    ///
    /// No formator functions other than those received while the FORMAT EFFECTOR ACTION MODE [`FEAM`] is set to
    /// STORE are included in a transmitted data stream or in data transferred to an auxiliary input/output device.
    ///
    /// ## Note
    ///
    /// No control functions are affected.
    FormatEffectorTransferMode,

    /// Multiple Area Transfer Mode `MATM`.
    ///
    /// ## Reset: Single
    ///
    /// Only the contents of the selected area which contains the active presentation position are eligible to be
    /// transmitted or transferred
    ///
    /// ## Set: Multiple
    ///
    /// The contents of all selected areas are eligible to be transmitted or transferred.
    ///
    /// ## Note
    ///
    /// No control functions are affected.
    MultipleAreaTransferMode,

    /// Transfer Termination Mode `TTM`.
    ///
    /// ## Reset: Cursor
    ///
    /// Only the contents of the character positions preceding the active presentation position in the presentation
    /// component are eligible to be transmitted or transferred.
    ///
    /// ## Set: All
    ///
    /// The contents of character positions preceding, following, and at the active presentation position are eligible
    /// to be transmitted or transferred.
    ///
    /// ## Note
    ///
    /// No control functions are affected.
    TransferTerminationMode,

    /// Selected Area Transfer Mode `SATM`.
    ///
    /// ## Reset: Select
    ///
    /// Only the contents of selected areas are eligible to be transmitted or transferred.
    ///
    /// ## Set: All
    ///
    /// The contents of all character positions, irrespective of any explicitly defined selected areas, are eligible to
    /// be transmitted or transferred.
    ///
    /// ## Note
    ///
    /// No control functions are affected.
    SelectedAreaTransferMode,

    /// Tabulation Stop Mode `TSM`.
    ///
    /// ## Reset: Multiple
    ///
    /// Character tabulation stops in the presentation component are set or cleared in the active line (the line that
    /// contains the active presentation position) and in the corresponding character positions of the preceding lines
    /// and of the following lines.
    ///
    /// ## Set: Single
    ///
    /// Character tabulation stops in the presentation component are set or cleared in the active line only.
    ///
    /// ## Note
    ///
    /// Control functions affected are: [`CTC`][crate::control_sequences::CTC], [`DL`][crate::control_sequences::DL],
    /// [`HTS`][crate::c1::HTS], [`IL`][crate::control_sequences::IL], [`TBC`][crate::control_sequences::TBC].
    TabulationStopMode,

    /// Graphic Rendition Combination Mode `GRCM`.
    ///
    /// ## Reset: Replacing
    ///
    /// Each occurrence of the control function SELECT GRAPHIC RENDITION ([`SGR`][crate::control_sequences::SGR])
    /// cancels the effect of any preceding occurrence. Any graphic rendition aspects that are to remain unchanged after
    /// an occurrence of [`SGR`][crate::control_sequences::SGR] have to be re-specified by that
    /// [`SGR`][crate::control_sequences::SGR].
    ///
    /// ## Set: Cumulative
    ///
    /// Each occurrence of the control function SELECT GRAPHIC RENDITION ([`SGR`][crate::control_sequences::SGR]) causes
    /// only those graphic rendition aspects to be changed that are specified by that
    /// [`SGR`][crate::control_sequences::SGR]. All other graphic rendition aspects remain unchanged.
    ///
    /// ## Note
    ///
    /// Control function affected is [`SGR`][crate::control_sequences::SGR].
    GraphicRenditionCombinationMode = 21,

    /// Zero Default Mode `ZDM`.
    ///
    /// ## Reset: Zero
    ///
    /// A parameter value of 0 of a control function means the number `0`.
    ///
    /// ## Set: Default
    ///
    /// A parameter value of `0` represents a default parameter value which may be different from `0`.
    ///
    /// ## Note 1
    ///
    /// This mode was provided for implementations of the first edition of this Standard which specified that "an empty
    /// parameter sub-string or a parameter sub-string which consists of bit combinations `03/00` only represents a
    /// default value which depends on the control function".
    ///
    /// For numeric parameters which are expressed in units established by the parameter value of SELECT SIZE UNIT
    /// ([`SSU`][crate::control_sequences::SSU]) the value `0` could then be specified. For numeric parameters which are
    /// effectively repeat counts, a `0` parameter value corresponded to a "no-op". In either instance, non-negative
    /// computed numeric parameter values might have been used without treating `0` as a special (unusable) case.
    ///
    /// Where an explicit parameter value was not used, implementers were urged to omit a parameter value (use an empty
    /// parameter sub-string) to imply a default parameter value.
    ///
    /// Control functions affected are: [`CBT`][crate::control_sequences::CBT], [`CHA`][crate::control_sequences::CHA],
    /// [`CHT`][crate::control_sequences::CHT], [`CNL`][crate::control_sequences::CNL],
    /// [`CPL`][crate::control_sequences::CPL], [`CPR`][crate::control_sequences::CPR],
    /// [`CUB`][crate::control_sequences::CUB], [`CUD`][crate::control_sequences::CUD],
    /// [`CUF`][crate::control_sequences::CUF], [`CUP`][crate::control_sequences::CUP],
    /// [`CUU`][crate::control_sequences::CUU], [`CVT`][crate::control_sequences::CVT],
    /// [`DCH`][crate::control_sequences::DCH], [`DL`][crate::control_sequences::DL],
    /// [`ECH`][crate::control_sequences::ECH], [`GSM`][crate::control_sequences::GSM],
    /// [`HPA`][crate::control_sequences::HPA], [`HPB`][crate::control_sequences::HPB],
    /// [`HPR`][crate::control_sequences::HPR], [`HVP`][crate::control_sequences::HVP],
    /// [`ICH`][crate::control_sequences::ICH], [`IL`][crate::control_sequences::IL],
    /// [`NP`][crate::control_sequences::NP], [`PP`][crate::control_sequences::PP],
    /// [`PPA`][crate::control_sequences::PPA], [`PPB`][crate::control_sequences::PPB],
    /// [`PPR`][crate::control_sequences::PPR], [`REP`][crate::control_sequences::REP],
    /// [`SD`][crate::control_sequences::SD], [`SL`][crate::control_sequences::SL],
    /// [`SR`][crate::control_sequences::SR], [`SU`][crate::control_sequences::SU],
    /// [`TCC`][crate::control_sequences::TCC], [`VPA`][crate::control_sequences::VPA],
    /// [`VPB`][crate::control_sequences::VPB], [`VPR`][crate::control_sequences::VPR].
    ///
    /// ## Note 2
    ///
    /// Since the publication of the first edition of this Standard in 1976 almost 15 years have expired. The use of
    /// this mode should no longer be required because the definition of default parameter values has been changed.
    ///
    /// # Note 3
    ///
    /// The use of the ZERO DEFAULT MODE ([`ZDM`]) is deprecated.
    ZeroDefaultMode,
}

use crate::control_sequences::{RM, SM};
impl Mode {
    /// Set the mode.
    pub fn set(self) -> ControlFunction<'static> {
        SM(vec![self])
    }

    /// Reset the mode.
    pub fn reset(self) -> ControlFunction<'static> {
        RM(vec![self])
    }
}

/// Guarded Area Transfer Mode `GATM`.
///
/// See [`Mode::GuardedAreaTransferMode`].
pub const GATM: Mode = Mode::GuardedAreaTransferMode;

/// Keyboard Action Mode `KAM`.
///
/// See [`Mode::KeyboardActionMode`].
pub const KAM: Mode = Mode::KeyboardActionMode;

/// Control Presentation Mode `CRM`.
///
/// See [`Mode::ControlPresentationMode`].
pub const CRM: Mode = Mode::ControlPresentationMode;

/// Insertion Replacement Mode `IRM`.
///
/// See [`Mode::InsertionReplacementMode`].
pub const IRM: Mode = Mode::InsertionReplacementMode;

/// Status Report Transfer Mode `SRTM`.
///
/// See [`Mode::StatusReportTransferMode`].
pub const SRTM: Mode = Mode::StatusReportTransferMode;

/// Erasure Mode `ERM`.
///
/// See [`Mode::ErasureMode`].
pub const ERM: Mode = Mode::ErasureMode;

/// Line Editing Mode `VEM`.
///
/// See [`Mode::LineEditingMode`].
pub const VEM: Mode = Mode::LineEditingMode;

/// Bi-directional support mode `BDSM`.
///
/// See [`Mode::BiDirectionalSupportMode`].
pub const BDSM: Mode = Mode::BiDirectionalSupportMode;

/// Device Component Select Mode `DCSM`.
///
/// See [`Mode::DeviceComponentSelectMode`].
pub const DCSM: Mode = Mode::DeviceComponentSelectMode;

/// Character Editing Mode `HEM`.
///
/// See [`Mode::CharacterEditingMode`].
pub const HEM: Mode = Mode::CharacterEditingMode;

/// Positioning Unit Mode `PUM`.
///
/// See [`Mode::PositioningUnitMode`].
pub const PUM: Mode = Mode::PositioningUnitMode;

/// Send/Receive Mode `SRM`.
///
/// See [`Mode::SendReceiveMode`].
pub const SRM: Mode = Mode::SendReceiveMode;

/// Format Effector Action Mode `FEAM`.
///
/// See [`Mode::FormatEffectorActionMode`].
pub const FEAM: Mode = Mode::FormatEffectorActionMode;

/// Format Effector Transfer Mode `FETM`.
///
/// See [`Mode::FormatEffectorTransferMode`].
pub const FETM: Mode = Mode::FormatEffectorTransferMode;

/// Multiple Area Transfer Mode `MATM`.
///
/// See [`Mode::MultipleAreaTransferMode`].
pub const MATM: Mode = Mode::MultipleAreaTransferMode;

/// Transfer Termination Mode `TTM`.
///
/// See [`Mode::TransferTerminationMode`].
pub const TTM: Mode = Mode::TransferTerminationMode;

/// Selected Area Transfer Mode `SATM`.
///
/// See [`Mode::SelectedAreaTransferMode`].
pub const SATM: Mode = Mode::SelectedAreaTransferMode;

/// Tabulation Stop Mode `TSM`.
///
/// See [`Mode::TabulationStopMode`].
pub const TSM: Mode = Mode::TabulationStopMode;

/// Graphic Rendition Combination Mode `GRCM`.
///
/// See [`Mode::GraphicRenditionCombinationMode`].
pub const GRCM: Mode = Mode::GraphicRenditionCombinationMode;

/// Zero Default Mode `ZDM`.
///
/// See [`Mode::ZeroDefaultMode`].
pub const ZDM: Mode = Mode::ZeroDefaultMode;
