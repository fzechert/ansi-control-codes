/// Device Modes.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    /// Guarded Area Transfer Mode `GATM`.
    GuardedAreaTransferMode = 1,

    /// Keyboard Action Mode `KAM`.
    KeyboardActionMode,

    /// Control Presentation Mode `CRM`.
    ControlPresentationMode,

    /// Insertion Replacement Mode `IRM`.
    InsertionReplacementMode,

    /// Status Report Transfer Mode `SRTM`.
    StatusReportTransferMode,

    /// Erasure Mode `ERM`.
    ErasureMode,

    /// Line Editing Mode `VEM`.
    LineEditingMode,

    /// Bi-directional support mode `BDSM`.
    BiDirectionalSupportMode,

    /// Device Component Select Mode `DCSM`.
    DeviceComponentSelectMode,

    /// Character Editing Mode `HEM`.
    CharacterEditingMode,

    /// Positioning Unit Mode `PUM`.
    PositioningUnitMode,

    /// Send/Receive Mode `SRM`.
    SendReceiveMode,

    /// Format Effector Action Mode `FEAM`.
    FormatEffectorActionMode,

    /// Format Effector Transfer Mode `FETM`.
    FormatEffectorTransferMode,

    /// Multiple Area Transfer Mode `MATM`.
    MultipleAreaTransferMode,

    /// Transfer Termination Mode `TTM`.
    TransferTerminationMode,

    /// Selected Area Transfer Mode `SATM`.
    SelectedAreaTransferMode,

    /// Tabulation Stop Mode `TSM`.
    TabulationStopMode,

    /// Graphic Rendition Combination Mode `GRCM`.
    GraphicRenditionCombinationMode = 21,

    /// Zero Default Mode `ZDM`.
    ZeroDefaultMode,
}

/// Guarded Area Transfer Mode `GATM`.
pub const GATM: Mode = Mode::GuardedAreaTransferMode;

/// Keyboard Action Mode `KAM`.
pub const KAM: Mode = Mode::KeyboardActionMode;

/// Control Presentation Mode `CRM`.
pub const CRM: Mode = Mode::ControlPresentationMode;

/// Insertion Replacement Mode `IRM`.
pub const IRM: Mode = Mode::InsertionReplacementMode;

/// Status Report Transfer Mode `SRTM`.
pub const SRTM: Mode = Mode::StatusReportTransferMode;

/// Erasure Mode `ERM`.
pub const ERM: Mode = Mode::ErasureMode;

/// Line Editing Mode `VEM`.
pub const VEM: Mode = Mode::LineEditingMode;

/// Bi-directional support mode `BDSM`.
pub const BDSM: Mode = Mode::BiDirectionalSupportMode;

/// Device Component Select Mode `DCSM`.
pub const DCSM: Mode = Mode::DeviceComponentSelectMode;

/// Character Editing Mode `HEM`.
pub const HEM: Mode = Mode::CharacterEditingMode;

/// Positioning Unit Mode `PUM`.
pub const PUM: Mode = Mode::PositioningUnitMode;

/// Send/Receive Mode `SRM`.
pub const SRM: Mode = Mode::SendReceiveMode;

/// Format Effector Action Mode `FEAM`.
pub const FEAM: Mode = Mode::FormatEffectorActionMode;

/// Format Effector Transfer Mode `FETM`.
pub const FETM: Mode = Mode::FormatEffectorTransferMode;

/// Multiple Area Transfer Mode `MATM`.
pub const MATM: Mode = Mode::MultipleAreaTransferMode;

/// Transfer Termination Mode `TTM`.
pub const TTM: Mode = Mode::TransferTerminationMode;

/// Selected Area Transfer Mode `SATM`.
pub const SATM: Mode = Mode::SelectedAreaTransferMode;

/// Tabulation Stop Mode `TSM`.
pub const TSM: Mode = Mode::TabulationStopMode;

/// Graphic Rendition Combination Mode `GRCM`.
pub const GRCM: Mode = Mode::GraphicRenditionCombinationMode;

/// Zero Default Mode `ZDM`.
pub const ZDM: Mode = Mode::ZeroDefaultMode;
