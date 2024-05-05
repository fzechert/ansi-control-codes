//! # Explanations of ansi-control-codes
//!
//! This module provides functionality to explain and inspect any ansi-control-code.
//!
//! Enable this module by using the feature `explain`
//!
//! ```text
//! cargo add ansi-control-codes --features explain
//! ```
//!
//! ## Names of Control Codes
//!
//! The functions [`short_name`][Explain::short_name] and [`long_name`][Explain::long_name] of the trait [`Explain`]
//! provide access to a control function's name. Short names are the abbreviated names of control functions, whereas
//! long names are the human readable equivalents.
//!
//! ```
//! use ansi_control_codes::categories::format_effectors::CR;
//! use ansi_control_codes::explain::Explain;
//! println!("short name: {}, long name: {}", CR.short_name().unwrap(), CR.long_name());
//! // this will print "short name: CR, long name: Carriage Return"
//! ```
//!
//! Short names of control functions are available for all control functions except for private-use control codes.
//!
//! ## Descriptions of Control Codes
//!
//! The functions [`short_description`][Explain::short_description] and [`long_description`][Explain::long_description]
//! of the trait [`Explain`] provide access to a control function's short and long descriptions. Not all control
//! functions have long descriptions, in which case the `long_description` returns the same description text as the
//! `short_description` functions.
//!
//! ```
//! use ansi_control_codes::categories::format_effectors::CR;
//! use ansi_control_codes::explain::Explain;
//! println!("short description: {}, long description: {}", CR.short_description(), CR.long_description());
//! ```

use std::{convert::Infallible, str::FromStr};

use crate::{control_sequences::*, modes::Mode, ControlFunction, ControlFunctionType};

macro_rules! param {
    ($self:ident, $index:literal, $default:literal) => {
        get_param(&$self.parameters, $index, $default)
    };
    ($self:ident, ordinal $index:literal, $default:literal) => {
        ordinal_indicator(get_param(&$self.parameters, $index, $default))
    };
}

macro_rules! explain_selection {
    ($selection:ident, $self:ident, $index:literal) => {
        $selection::from_str(
            $self
                .parameters
                .get($index)
                .map(&String::as_ref)
                .unwrap_or(""),
        )
        .expect("Reached infallible code.")
        .explain()
    };
}

#[derive(Debug)]
enum Function {
    // C0
    ACK,
    BEL,
    BS,
    CAN,
    CR,
    DC1,
    DC2,
    DC3,
    DC4,
    DLE,
    EM,
    ENQ,
    EOT,
    ESC,
    ETB,
    ETX,
    FF,
    HT,
    IS1,
    IS2,
    IS3,
    IS4,
    LF,
    LS0,
    LS1,
    NAK,
    NUL,
    SOH,
    STX,
    SUB,
    SYN,
    VT,
    // C1
    APC,
    BPH,
    CCH,
    CSI,
    DCS,
    EPA,
    ESA,
    HTJ,
    HTS,
    MW,
    NBH,
    NEL,
    OSC,
    PLD,
    PLU,
    PM,
    PU1,
    PU2,
    RI,
    SCI,
    SOS,
    SPA,
    SSA,
    SS2,
    SS3,
    ST,
    STS,
    VTS,
    // Independent Control Functions
    CMD,
    DMI,
    EMI,
    INT,
    LS1R,
    LS2,
    LS2R,
    LS3,
    LS3R,
    RIS,
    // Control Sequences
    CBT,
    CHA,
    CHT,
    CNL,
    CPL,
    CPR,
    CTC,
    CUB,
    CUD,
    CUF,
    CUP,
    CUU,
    CVT,
    DA,
    DAQ,
    DCH,
    DL,
    DSR,
    DTA,
    EA,
    ECH,
    ED,
    EF,
    EL,
    FNK,
    FNT,
    GCC,
    GSM,
    GSS,
    HPA,
    HPB,
    HPR,
    HVP,
    ICH,
    IDCS,
    IGS,
    IL,
    JFY,
    MC,
    NP,
    PEC,
    PFS,
    PP,
    PPA,
    PPB,
    PPR,
    PTX,
    QUAD,
    REP,
    RM,
    SACS,
    SAPV,
    SCO,
    SCP,
    SCS,
    SD,
    SDS,
    SEE,
    SEF,
    SGR,
    SHS,
    SIMD,
    SL,
    SLH,
    SLL,
    SLS,
    SM,
    SPD,
    SPH,
    SPI,
    SPL,
    SPQR,
    SR,
    SRCS,
    SRS,
    SSU,
    SSW,
    STAB,
    SU,
    SVS,
    TAC,
    TALE,
    TATE,
    TBC,
    TCC,
    TSR,
    TSS,
    VPA,
    VPB,
    VPR,
    PRIVATE,
}

fn function(control_function: &ControlFunction<'_>) -> Function {
    match control_function.function_type {
        ControlFunctionType::C0 => {
            // C0 control functions are always 1 byte long
            let byte = control_function.value.as_bytes()[0];

            match byte {
                0 => Function::NUL,
                1 => Function::SOH,
                2 => Function::STX,
                3 => Function::ETX,
                4 => Function::EOT,
                5 => Function::ENQ,
                6 => Function::ACK,
                7 => Function::BEL,
                8 => Function::BS,
                9 => Function::HT,
                10 => Function::LF,
                11 => Function::VT,
                12 => Function::FF,
                13 => Function::CR,
                14 => Function::LS1,
                15 => Function::LS0,
                16 => Function::DLE,
                17 => Function::DC1,
                18 => Function::DC2,
                19 => Function::DC3,
                20 => Function::DC4,
                21 => Function::NAK,
                22 => Function::SYN,
                23 => Function::ETB,
                24 => Function::CAN,
                25 => Function::EM,
                26 => Function::SUB,
                27 => Function::ESC,
                28 => Function::IS4,
                29 => Function::IS3,
                30 => Function::IS2,
                31 => Function::IS1,
                _ => {
                    unreachable!("No C0 control function exists outside of above range")
                }
            }
        }
        ControlFunctionType::C1 => {
            // C1 control functions are always 1 byte long
            let byte = control_function.value.as_bytes()[0];

            match byte {
                66 => Function::BPH,
                67 => Function::NBH,
                69 => Function::NEL,
                70 => Function::SSA,
                71 => Function::ESA,
                72 => Function::HTS,
                73 => Function::HTJ,
                74 => Function::VTS,
                75 => Function::PLD,
                76 => Function::PLU,
                77 => Function::RI,
                78 => Function::SS2,
                79 => Function::SS3,
                80 => Function::DCS,
                81 => Function::PU1,
                82 => Function::PU2,
                83 => Function::STS,
                84 => Function::CCH,
                85 => Function::MW,
                86 => Function::SPA,
                87 => Function::EPA,
                88 => Function::SOS,
                90 => Function::SCI,
                91 => Function::CSI,
                92 => Function::ST,
                93 => Function::OSC,
                94 => Function::PM,
                95 => Function::APC,
                _ => {
                    unreachable!("No C1 control function exists outside of above range")
                }
            }
        }
        ControlFunctionType::IndependentControlFunction => {
            // Independent control functions are always 1 byte long
            let byte = control_function.value.as_bytes()[0];

            match byte {
                96 => Function::DMI,
                97 => Function::INT,
                98 => Function::EMI,
                99 => Function::RIS,
                100 => Function::CMD,
                110 => Function::LS2,
                111 => Function::LS3,
                124 => Function::LS3R,
                125 => Function::LS2R,
                126 => Function::LS1R,
                _ => {
                    unreachable!("No independent control function exists outside of above range")
                }
            }
        }
        ControlFunctionType::ControlSequence => {
            let bytes = control_function.value.as_bytes();
            if bytes.len() == 1 {
                // control sequence with no intermediate byte
                let byte = bytes[0];
                return match byte {
                    64 => Function::ICH,
                    65 => Function::CUU,
                    66 => Function::CUD,
                    67 => Function::CUF,
                    68 => Function::CUB,
                    69 => Function::CNL,
                    70 => Function::CPL,
                    71 => Function::CHA,
                    72 => Function::CUP,
                    73 => Function::CHT,
                    74 => Function::ED,
                    75 => Function::EL,
                    76 => Function::IL,
                    77 => Function::DL,
                    78 => Function::EF,
                    79 => Function::EA,
                    80 => Function::DCH,
                    81 => Function::SEE,
                    82 => Function::CPR,
                    83 => Function::SU,
                    84 => Function::SD,
                    85 => Function::NP,
                    86 => Function::PP,
                    87 => Function::CTC,
                    88 => Function::ECH,
                    89 => Function::CVT,
                    90 => Function::CBT,
                    91 => Function::SRS,
                    92 => Function::PTX,
                    93 => Function::SDS,
                    94 => Function::SIMD,
                    96 => Function::HPA,
                    97 => Function::HPR,
                    98 => Function::REP,
                    99 => Function::DA,
                    100 => Function::VPA,
                    101 => Function::VPR,
                    102 => Function::HVP,
                    103 => Function::TBC,
                    104 => Function::SM,
                    105 => Function::MC,
                    106 => Function::HPB,
                    107 => Function::VPB,
                    108 => Function::RM,
                    109 => Function::SGR,
                    110 => Function::DSR,
                    111 => Function::DAQ,
                    112..=127 => Function::PRIVATE,
                    _ => {
                        unreachable!("No valid control sequence exist outside of above range")
                    }
                };
            }
            if bytes.len() == 2 {
                // control sequence with intermediate byte
                let byte = bytes[1];
                return match byte {
                    64 => Function::SL,
                    65 => Function::SR,
                    66 => Function::GSM,
                    67 => Function::GSS,
                    68 => Function::FNT,
                    69 => Function::TSS,
                    70 => Function::JFY,
                    71 => Function::SPI,
                    72 => Function::QUAD,
                    73 => Function::SSU,
                    74 => Function::PFS,
                    75 => Function::SHS,
                    76 => Function::SVS,
                    77 => Function::IGS,
                    79 => Function::IDCS,
                    80 => Function::PPA,
                    81 => Function::PPR,
                    82 => Function::PPB,
                    83 => Function::SPD,
                    84 => Function::DTA,
                    85 => Function::SLH,
                    86 => Function::SLL,
                    87 => Function::FNK,
                    88 => Function::SPQR,
                    89 => Function::SEF,
                    90 => Function::PEC,
                    91 => Function::SSW,
                    92 => Function::SACS,
                    93 => Function::SAPV,
                    94 => Function::STAB,
                    95 => Function::GCC,
                    96 => Function::TATE,
                    97 => Function::TALE,
                    98 => Function::TAC,
                    99 => Function::TCC,
                    100 => Function::TSR,
                    101 => Function::SCO,
                    102 => Function::SRCS,
                    103 => Function::SCS,
                    104 => Function::SLS,
                    105 => Function::SPH,
                    106 => Function::SPL,
                    107 => Function::SCP,
                    112..=127 => Function::PRIVATE,
                    _ => {
                        unreachable!("No valid control sequence exist outside of above range")
                    }
                };
            }
            unreachable!("No valid control sequence exist outside of above range")
        }
    }
}

fn ordinal_indicator(numeric_value: String) -> String {
    numeric_value
        .parse::<u64>()
        .map(|value| match value % 10 {
            0 | 4..=9 => numeric_value.clone(),
            1 => format!("{}st", value),
            2 => format!("{}nd", value),
            3 => format!("{}rd", value),
            _ => {
                unreachable!("This is not reachable, all possible values of modulo 10 are covered.")
            }
        })
        .unwrap_or_else(|_| numeric_value)
}

fn get_param(parameters: &Vec<String>, index: usize, default_value: u64) -> String {
    parameters
        .get(index)
        .map(|value| value.to_owned())
        .unwrap_or_else(|| format!("{default_value}"))
}

trait ExplainSelection {
    fn explain(&self) -> String;
}

trait ExplainMode {
    fn name(&self) -> String;
    fn explain_reset(&self) -> String;
    fn explain_set(&self) -> String;
}

/// Explanation of an ansi-control-code.
pub trait Explain {
    /// Returns the short name (abbreviation) of this control function, e.g. `CR`, `LF`.
    ///
    /// An abbreviated name is available for all ansi-escape-codes, except for those in the private use area.
    fn short_name(&self) -> Option<&'static str>;

    /// Returns the name of this control function, e.g. `Carriage Return`, `Line Feed`.
    fn long_name(&self) -> &'static str;

    /// Returns the short description of what this function does.
    fn short_description(&self) -> String;

    /// Returns a long description of what this function does.
    ///
    /// Not all control functions have a long description, in which case this will return the
    /// same as `short_description()`.
    fn long_description(&self) -> String;
}

impl Explain for ControlFunction<'_> {
    fn short_name(&self) -> Option<&'static str> {
        match function(&self) {
            Function::ACK => Some("ACK"),
            Function::BEL => Some("BEL"),
            Function::BS => Some("BS"),
            Function::CAN => Some("CAN"),
            Function::CR => Some("CR"),
            Function::DC1 => Some("DC1"),
            Function::DC2 => Some("DC2"),
            Function::DC3 => Some("DC3"),
            Function::DC4 => Some("DC4"),
            Function::DLE => Some("DLE"),
            Function::EM => Some("EM"),
            Function::ENQ => Some("ENQ"),
            Function::EOT => Some("EOT"),
            Function::ESC => Some("ESC"),
            Function::ETB => Some("ETB"),
            Function::ETX => Some("ETX"),
            Function::FF => Some("FF"),
            Function::HT => Some("HT"),
            Function::IS1 => Some("IS1"),
            Function::IS2 => Some("IS2"),
            Function::IS3 => Some("IS3"),
            Function::IS4 => Some("IS4"),
            Function::LF => Some("LF"),
            Function::LS0 => Some("LS0"),
            Function::LS1 => Some("LS1"),
            Function::NAK => Some("NAK"),
            Function::NUL => Some("NUL"),
            Function::SOH => Some("SOH"),
            Function::STX => Some("STX"),
            Function::SUB => Some("SUB"),
            Function::SYN => Some("SYN"),
            Function::VT => Some("VT"),
            Function::APC => Some("APC"),
            Function::BPH => Some("BPH"),
            Function::CCH => Some("CCH"),
            Function::CSI => Some("CSI"),
            Function::DCS => Some("DCS"),
            Function::EPA => Some("EPA"),
            Function::ESA => Some("ESA"),
            Function::HTJ => Some("HTJ"),
            Function::HTS => Some("HTS"),
            Function::MW => Some("MW"),
            Function::NBH => Some("NBH"),
            Function::NEL => Some("NEL"),
            Function::OSC => Some("OSC"),
            Function::PLD => Some("PLD"),
            Function::PLU => Some("PLU"),
            Function::PM => Some("PM"),
            Function::PU1 => Some("PU1"),
            Function::PU2 => Some("PU2"),
            Function::RI => Some("RI"),
            Function::SCI => Some("SCI"),
            Function::SOS => Some("SOS"),
            Function::SPA => Some("SPA"),
            Function::SSA => Some("SSA"),
            Function::SS2 => Some("SS2"),
            Function::SS3 => Some("SS3"),
            Function::ST => Some("ST"),
            Function::STS => Some("STS"),
            Function::VTS => Some("VTS"),
            Function::CMD => Some("CMD"),
            Function::DMI => Some("DMI"),
            Function::EMI => Some("EMI"),
            Function::INT => Some("INT"),
            Function::LS1R => Some("LS1R"),
            Function::LS2 => Some("LS2"),
            Function::LS2R => Some("LS2R"),
            Function::LS3 => Some("LS3"),
            Function::LS3R => Some("LS3R"),
            Function::RIS => Some("RIS"),
            Function::CBT => Some("CBT"),
            Function::CHA => Some("CHA"),
            Function::CHT => Some("CHT"),
            Function::CNL => Some("CNL"),
            Function::CPL => Some("CPL"),
            Function::CPR => Some("CPR"),
            Function::CTC => Some("CTC"),
            Function::CUB => Some("CUB"),
            Function::CUD => Some("CUD"),
            Function::CUF => Some("CUF"),
            Function::CUP => Some("CUP"),
            Function::CUU => Some("CUU"),
            Function::CVT => Some("CVT"),
            Function::DA => Some("DA"),
            Function::DAQ => Some("DAQ"),
            Function::DCH => Some("DCH"),
            Function::DL => Some("DL"),
            Function::DSR => Some("DSR"),
            Function::DTA => Some("DTA"),
            Function::EA => Some("EA"),
            Function::ECH => Some("ECH"),
            Function::ED => Some("ED"),
            Function::EF => Some("EF"),
            Function::EL => Some("EL"),
            Function::FNK => Some("FNK"),
            Function::FNT => Some("FNT"),
            Function::GCC => Some("GCC"),
            Function::GSM => Some("GSM"),
            Function::GSS => Some("GSS"),
            Function::HPA => Some("HPA"),
            Function::HPB => Some("HPB"),
            Function::HPR => Some("HPR"),
            Function::HVP => Some("HVP"),
            Function::ICH => Some("ICH"),
            Function::IDCS => Some("IDCS"),
            Function::IGS => Some("IGS"),
            Function::IL => Some("IL"),
            Function::JFY => Some("JFY"),
            Function::MC => Some("MC"),
            Function::NP => Some("NP"),
            Function::PEC => Some("PEC"),
            Function::PFS => Some("PFS"),
            Function::PP => Some("PP"),
            Function::PPA => Some("PPA"),
            Function::PPB => Some("PPB"),
            Function::PPR => Some("PPR"),
            Function::PTX => Some("PTX"),
            Function::QUAD => Some("QUAD"),
            Function::REP => Some("REP"),
            Function::RM => Some("RM"),
            Function::SACS => Some("SACS"),
            Function::SAPV => Some("SAPV"),
            Function::SCO => Some("SCO"),
            Function::SCP => Some("SCP"),
            Function::SCS => Some("SCS"),
            Function::SD => Some("SD"),
            Function::SDS => Some("SDS"),
            Function::SEE => Some("SEE"),
            Function::SEF => Some("SEF"),
            Function::SGR => Some("SGR"),
            Function::SHS => Some("SHS"),
            Function::SIMD => Some("SIMD"),
            Function::SL => Some("SL"),
            Function::SLH => Some("SLH"),
            Function::SLL => Some("SLL"),
            Function::SLS => Some("SLS"),
            Function::SM => Some("SM"),
            Function::SPD => Some("SPD"),
            Function::SPI => Some("SPI"),
            Function::SPL => Some("SPL"),
            Function::SPH => Some("SPH"),
            Function::SPQR => Some("SPQR"),
            Function::SR => Some("SR"),
            Function::SRCS => Some("SRCS"),
            Function::SRS => Some("SRS"),
            Function::SSU => Some("SSU"),
            Function::SSW => Some("SSW"),
            Function::STAB => Some("STAB"),
            Function::SU => Some("SU"),
            Function::SVS => Some("SVS"),
            Function::TAC => Some("TAC"),
            Function::TALE => Some("TALE"),
            Function::TATE => Some("TATE"),
            Function::TBC => Some("TBC"),
            Function::TCC => Some("TCC"),
            Function::TSR => Some("TSR"),
            Function::TSS => Some("TSS"),
            Function::VPA => Some("VPA"),
            Function::VPB => Some("VPB"),
            Function::VPR => Some("VPR"),
            Function::PRIVATE => None,
        }
    }

    fn long_name(&self) -> &'static str {
        match function(&self) {
            Function::ACK => "Acknowledge",
            Function::BEL => "Bell",
            Function::BS => "Backspace",
            Function::CAN => "Cancel",
            Function::CR => "Carriage Return",
            Function::DC1 => "Device Control One",
            Function::DC2 => "Device Control Two",
            Function::DC3 => "Device Control Three",
            Function::DC4 => "Device Control Four",
            Function::DLE => "Data Link Escape",
            Function::EM => "End of Medium",
            Function::ENQ => "Enquiry",
            Function::EOT => "End of Transmission",
            Function::ESC => "Escape",
            Function::ETB => "End of Transmission Block",
            Function::ETX => "End of Text",
            Function::FF => "Form Feed",
            Function::HT => "Character Tabulation",
            Function::IS1 => "Information Separator One (US - Unit Separator)",
            Function::IS2 => "Information Separator Two (RS - Record Separator)",
            Function::IS3 => "Information Separator Three (GS - Group Separator)",
            Function::IS4 => "Information Separator Four (FS - File Separator)",
            Function::LF => "Line Feed",
            Function::LS0 => "Locking-Shift Zero (Shift-In)",
            Function::LS1 => "Locking-Shift One (Shift-Out)",
            Function::NAK => "Negative Acknowledge",
            Function::NUL => "Null",
            Function::SOH => "Start of Heading",
            Function::STX => "Start of Text",
            Function::SUB => "Substitute",
            Function::SYN => "Synchronous Idle",
            Function::VT => "Line Tabulation",
            Function::APC => "Application Program Command",
            Function::BPH => "Break Permitted Here",
            Function::CCH => "Cancel Character",
            Function::CSI => "Control Sequence Introducer",
            Function::DCS => "Device Control String",
            Function::EPA => "End of Guarded Area",
            Function::ESA => "End of Selected Area",
            Function::HTJ => "Character Tabulation With Justification",
            Function::HTS => "Character Tabulation Set",
            Function::MW => "Message Waiting",
            Function::NBH => "No Break Here",
            Function::NEL => "Next Line",
            Function::OSC => "Operating System Command",
            Function::PLD => "Partial Line Forward",
            Function::PLU => "Partial Line Backwards",
            Function::PM => "Privacy Message",
            Function::PU1 => "Private Use One",
            Function::PU2 => "Private Use Two",
            Function::RI => "Reverse Line Feed",
            Function::SCI => "Single Character Introducer",
            Function::SOS => "Start of String",
            Function::SPA => "Start of Guarded Area",
            Function::SSA => "Start of Selected Area",
            Function::SS2 => "Single-Shift Two",
            Function::SS3 => "Single-Shift Three",
            Function::ST => "String Terminator",
            Function::STS => "Set Transmit State",
            Function::VTS => "Line Tabulation Set",
            Function::CMD => "Coding Method Delimiter",
            Function::DMI => "Disable Manual Input",
            Function::EMI => "Enable Manual Input",
            Function::INT => "Interrupt",
            Function::LS1R => "Locking-Shift One Right",
            Function::LS2 => "Locking-Shift Two",
            Function::LS2R => "Locking-Shift Two Right",
            Function::LS3 => "Locking-Shift Three",
            Function::LS3R => "Locking-Shift Three Right",
            Function::RIS => "Reset to Initial State",
            Function::CBT => "Cursor Backwards Tabulation",
            Function::CHA => "Cursor Character Absolute",
            Function::CHT => "Cursor Forward Tabulation",
            Function::CNL => "Cursors Next Line",
            Function::CPL => "Cursor Preceding Line",
            Function::CPR => "Active Position Report",
            Function::CTC => "Cursor Tabulation Control",
            Function::CUB => "Cursor Left",
            Function::CUD => "Cursor Down",
            Function::CUF => "Cursor Right",
            Function::CUP => "Cursor Position",
            Function::CUU => "Cursor Up",
            Function::CVT => "Cursor Line Tabulation",
            Function::DA => "Device Attributes",
            Function::DAQ => "Define Area Qualification",
            Function::DCH => "Delete Character",
            Function::DL => "Delete Line",
            Function::DSR => "Device Status Report",
            Function::DTA => "Dimension Text Area",
            Function::EA => "Erase Area",
            Function::ECH => "Erase Character",
            Function::ED => "Erase in Page",
            Function::EF => "Erase in Field",
            Function::EL => "Erase in Line",
            Function::FNK => "Function Key",
            Function::FNT => "Font Selection",
            Function::GCC => "Graphic Character Combination",
            Function::GSM => "Graphic Size Modification",
            Function::GSS => "Graphic Size Selection",
            Function::HPA => "Character Position Absolute",
            Function::HPB => "Character Position Backwards",
            Function::HPR => "Character Position Forward",
            Function::HVP => "Character and Line Position",
            Function::ICH => "Insert Character",
            Function::IDCS => "Identify Device Control String",
            Function::IGS => "Identify Graphic Subrepertoire",
            Function::IL => "Insert Line",
            Function::JFY => "Justify",
            Function::MC => "Media Copy",
            Function::NP => "Next Page",
            Function::PEC => "Presentation Expand or Contract",
            Function::PFS => "Page Format Selection",
            Function::PP => "Preceding Page",
            Function::PPA => "Page Position Absolute",
            Function::PPB => "Page Position Backwards",
            Function::PPR => "Page Position Forward",
            Function::PTX => "Parallel Texts",
            Function::QUAD => "Quad",
            Function::REP => "Repeat",
            Function::RM => "Reset Mode",
            Function::SACS => "Set Additional Character Representation",
            Function::SAPV => "Select Alternative Presentation Variants",
            Function::SCO => "Select Character Orientation",
            Function::SCP => "Select Character Path",
            Function::SCS => "Set Character Spacing",
            Function::SD => "Scroll Down",
            Function::SDS => "Start Directed String",
            Function::SEE => "Select Editing Extent",
            Function::SEF => "Sheet Eject and Feed",
            Function::SGR => "Select Graphic Rendition",
            Function::SHS => "Select Character Spacing",
            Function::SIMD => "Select Implicit Movement Direction",
            Function::SL => "Scroll Left",
            Function::SLH => "Set Line Home",
            Function::SLL => "Set Line Limit",
            Function::SLS => "Set Line Spacing",
            Function::SM => "Set Mode",
            Function::SPD => "Select Presentation Direction",
            Function::SPH => "Set Page Home",
            Function::SPI => "Spacing Increment",
            Function::SPL => "Set Page Limit",
            Function::SPQR => "Select Page Quality and Rapidity",
            Function::SR => "Scroll Right",
            Function::SRCS => "Set Reduced Character Separation",
            Function::SRS => "Start Reversed String",
            Function::SSU => "Select Size Unit",
            Function::SSW => "Set Space Width",
            Function::STAB => "Selective Tabulation",
            Function::SU => "Scroll Up",
            Function::SVS => "Select Line Spacing",
            Function::TAC => "Tabulation Aligned Centred",
            Function::TALE => "Tabulation Aligned Leading Edge",
            Function::TATE => "Tabulation Aligned Trailing Edge",
            Function::TBC => "Tabulation Clear",
            Function::TCC => "Tabulation Centred on Character",
            Function::TSR => "Tabulation Stop Remove",
            Function::TSS => "Thin Space Specification",
            Function::VPA => "Line Position Absolute",
            Function::VPB => "Line Position Backwards",
            Function::VPR => "Line Position Forward",
            Function::PRIVATE => "Private Use / Experimental Use",
        }
    }

    fn short_description(&self) -> String {
        match function(&self) {
            Function::ACK => {
                String::from("Transmitted by a receiver as an affirmative response to the sender.")
            }
            Function::BEL => String::from("Calls for attention."),
            Function::BS => {
                String::from("Causes the active data position to be moved one character backwards.")
            }
            Function::CAN => String::from("Indicate that the preceding data is in error."),
            Function::CR => String::from("Move to the beginning of the line."),
            Function::DC1 => {
                String::from("Primarily intended for turning on or starting an ancillary device.")
            }
            Function::DC2 => {
                String::from("Primarily intended for turning on or starting an ancillary device.")
            }
            Function::DC3 => {
                String::from("Primarily intended for turning off or stopping an ancillary device.")
            }
            Function::DC4 => String::from(
                "Primarily intended for turning off, stopping, or interrupting an ancillary device."
            ),
            Function::DLE => String::from("Used exclusively to provide supplementary transmission control functions."),
            Function::EM => String::from("Identifies the physical end of a medium."),
            Function::ENQ => String::from("Transmitted by a sender as a request for a response from a receiver."),
            Function::EOT => String::from("Indicates the conclusion of the transmission of one or more texts."),
            Function::ESC => String::from("Used for code extension purposes."),
            Function::ETB => String::from(
                concat!(
                    "Indicates the end of a block of data, where the data are divided into such blocks for ",
                    "transmission purposes."
                )
            ),
            Function::ETX => String::from("Indicates the end of a text."),
            Function::FF => String::from(
                "Causes the active presentation position to be moved to the line home position of the next line."
            ),
            Function::HT => String::from(
                concat!(
                    "Causes the active presentation position to be moved to the following character tabulation stop ",
                    "in the presentation component."
                )
            ),
            Function::IS1 => String::from("Separates and qualifies data logically."),
            Function::IS2 => String::from("Separates and qualifies data logically."),
            Function::IS3 => String::from("Separates and qualifies data logically."),
            Function::IS4 => String::from("Separates and qualifies data logically."),
            Function::LF => String::from("Move to following line."),
            Function::LS0 => String::from("Used for code extension purposes."),
            Function::LS1 => String::from("Used for code extension purposes."),
            Function::NAK => String::from("Transmitted by a receiver as a negative response to the sender."),
            Function::NUL => String::from("Used for media-fill or time-fill."),
            Function::SOH => String::from("Indicates the beginning of a heading."),
            Function::STX => String::from("Indicates the beginning of a text and the end of a heading."),
            Function::SUB => String::from(
                "Used in the place of a character that has been found to be invalid or in error"
            ),
            Function::SYN => String::from(
                "Used by a synchronous transmission system in the absence of any other character."
            ),
            Function::VT => String::from("Move to the next line that has a line tabulation stop."),
            Function::APC => String::from("Opening delimiter of a control string for application program use."),
            Function::BPH => String::from("A break may occur here when text is formatted."),
            Function::CCH => String::from(
                concat!(
                    "Indicates that both the preceding graphic character in the data stream, and this character ",
                    "should be ignored."
                )
            ),
            Function::CSI => String::from("Used as the first character of a longer control sequence."),
            Function::DCS => String::from("Opening delimiter of a control string for device control use."),
            Function::EPA => String::from("End of an area that protects its content against unwanted alteration."),
            Function::ESA => String::from(
                "End of an area selected for transferring or transmitting to an ancillary input/output device."
            ),
            Function::HTJ => String::from(
                concat!(
                    "Shift the contents of the active field forward, so that it ends in before of the next character ",
                    "tabulation stop."
                )
            ),
            Function::HTS => String::from("Set a character tabulation stop at the current position."),
            Function::MW => String::from("Sets a message waiting indicator in the receiving device."),
            Function::NBH => String::from("A line break shall not occur here when the text is formatted."),
            Function::NEL => String::from("Move to the next line."),
            Function::OSC => String::from("Opening delimiter of a control string for operating system use."),
            Function::PLD => String::from(
                "Move to an imaginary line with a partial offset downwards of the current line."
            ),
            Function::PLU => String::from(
                "Move to an imaginary line with a partial offset upwards of the current line."
            ),
            Function::PM => String::from("Opening delimiter of a control string for privacy message use."),
            Function::PU1 => String::from(
                "Reserved for function without standardized meaning, for private use as required."
            ),
            Function::PU2 => String::from(
                "Reserved for function without standardized meaning, for private use as required."
            ),
            Function::RI => String::from("Move to the preceding line."),
            Function::SCI => String::from(
                "This character and the following one represent a control function or a graphic character."
            ),
            Function::SOS => String::from("Opening delimiter of a control String."),
            Function::SPA => String::from("
                First position of a string that is guarded against manual alteration, transmission, transferor deletion."
            ),
            Function::SSA => String::from(
                concat!(
                    "First position of a string that is eligible to be transmitted or transferred to an ancillary ",
                    "input/output device."
                )
            ),
            Function::SS2 => String::from(
                concat!(
                    "Used for code extension purposes. Changes the meaning of the bit combinations following it in ",
                    "the data stream."
                )
            ),
            Function::SS3 => String::from(
                concat!(
                    "Used for code extension purposes. Changes the meaning of the bit combinations following it in ",
                    "the data stream."
                )
            ),
            Function::ST => String::from("Closing delimiter of a control string opened by APC, DCS, OSC, PM or SOS."),
            Function::STS => String::from(
                concat!(
                    "Establish the transmit state in the receiving device. In this state the transmission of data ",
                    "from the device is possible."
                )
            ),
            Function::VTS => String::from("Set a line tabulation stop at the active line."),
            Function::CMD => String::from("Delimits a string of data coded according to standard ECMA-35."),
            Function::DMI => String::from("Causes the manual input facilities of a device to be disabled."),
            Function::EMI => String::from("Causes the manual input facilities of a device to be enabled."),
            Function::INT => String::from(
                concat!(
                    "Indicate to the receiving device that the current process is to be interrupted and an agreed ",
                    "procedure is to be initiated."
                )
            ),
            Function::LS1R => String::from(
                "Used for code extension purposes. Changes the meaning of the following characters in the data stream."
            ),
            Function::LS2 => String::from(
                "Used for code extension purposes. Changes the meaning of the following characters in the data stream."
            ),
            Function::LS2R => String::from(
                "Used for code extension purposes. Changes the meaning of the following characters in the data stream."
            ),
            Function::LS3 => String::from(
                "Used for code extension purposes. Changes the meaning of the following characters in the data stream."
            ),
            Function::LS3R => String::from(
                "Used for code extension purposes. Changes the meaning of the following characters in the data stream."
            ),
            Function::RIS => String::from("Causes a device to be reset to its initial state."),
            Function::CBT => format!(
                "Causes the active position to be moved backwards by {} tabulation stops.", 
                param!(self, 0, 1)
            ),
            Function::CHA => format!(
                "Causes the active position to be set to character position {} in the active line",
                param!(self, 0, 1)
            ),
            Function::CHT => format!(
                "Causes the active position to be moved forward by {} tabulation stops.",
                param!(self, 0, 1)
            ),
            Function::CNL => format!(
                "Causes the active position to be moved to the first character of the {} following line.",
                param!(self, ordinal 0, 1)
            ),
            Function::CPL => format!(
                concat!(
                    "Causes the active position to be moved to the first character of the {} preceding line."
                ),
                param!(self, ordinal 0, 1)
            ),
            Function::CPR => format!(
                concat!(
                    "The active position is reported to be in line {} at character position {}."
                ),
                param!(self, 0, 1), param!(self, 1, 1)
            ),
            Function::CTC => explain_selection!(TabulationControl, self, 0),
            Function::CUB => format!(
                "Move the active position {} characters to the left.",
                param!(self, 0, 1)
            ),
            Function::CUD => format!(
                "Move the active position {} lines downwards.",
                param!(self, 0, 1)
            ),
            Function::CUF => format!(
                "Move the active position {} characters to the right.",
                param!(self, 0, 1)
            ),
            Function::CUP => format!(
                "Move the active position to line {} and character {}.",
                param!(self, 0, 1),
                param!(self, 1, 1),
            ),
            Function::CUU => format!(
                "Move the active position {} lines upwards.",
                param!(self, 0, 1)
            ),
            Function::CVT => format!(
                "Causes the active position to the {} following line tabulation stop.",
                param!(self, ordinal 0, 1)
            ),
            Function::DA => explain_selection!(DeviceAttributes, self, 0),
            Function::DAQ => format!(
                "The active position is the first position of a qualified area. This area {}.",
                explain_selection!(AreaQualification, self, 0),
            ),
            Function::DCH => format!(
                "Delete {} characters, starting from the active position to the left.",
                param!(self, 0, 1)
            ),
            Function::DL => format!(
                "Delete {} lines",
                param!(self, 0, 1)
            ),
            Function::DSR => explain_selection!(DeviceStatusReport, self, 0),
            Function::DTA => format!(
                concat!(
                    "Establishes the dimension of the text area for subsequent pages. Dimension perpendicular to the ",
                    "line orientation: {}. Dimension parallel to the line orientation: {}."
                ),
                param!(self, 0, 0),
                param!(self, 1, 0)
            ),
            Function::EA => format!("This {}.", explain_selection!(EraseArea, self, 0)),
            Function::ECH => format!(
                concat!(
                    "Erase {} characters from the active position to the right."
                ),
                param!(self, 0, 1)
            ),
            Function::ED => format!("This {}.", explain_selection!(ErasePage, self, 0)),
            Function::EF => format!("This {}.", explain_selection!(EraseField, self, 0)),
            Function::EL => format!("This {}.", explain_selection!(EraseLine, self, 0)),
            Function::FNK => format!("Function Key number {} has been pressed.",
                param!(self, 0, 1)
            ),
            Function::FNT => format!(
                concat!(
                    "Indicates that the {} should be set to font {} and be accessible as {} from here on."
                ),
                explain_selection!(Font, self, 0),
                param!(self, 1, 0),
                explain_selection!(Font, self, 0)
            ),
            Function::GCC => explain_selection!(GraphicCharacterCombination, self, 0),
            Function::GSM => format!(
                "Modify the text height and / or width of all fonts to {}% height and  {}% width.",
                param!(self, 0, 100),
                param!(self, 1, 100)
            ),
            Function::GSS => format!(
                "Modify the text height of all fonts to {}. The width is implicitly defined by the height.",
                param!(self, 0, 0)
            ),
            Function::HPA => format!(
                "Move the active data position to character position {} in the active line.",
                param!(self, 0, 1)
            ),
            Function::HPB => format!(
                "Move the active data position backwards by {} characters.",
                param!(self, 0, 1)
            ),
            Function::HPR => format!(
                "Move the active data position forward by {} characters.",
                param!(self, 0, 1)
            ),
            Function::HVP => format!(
                "Move the active data position to the {} line and {} character.",
                param!(self, ordinal 0, 1),
                param!(self, ordinal 1, 1)
            ),
            Function::ICH => format!(
                "Prepare the insertion of {} characters.",
                param!(self, 0, 1)
            ),
            Function::IDCS => explain_selection!(IdentifyDeviceControlString, self, 0),
            Function::IGS => format!(
                "The graphic subrepertoire {} is used in the subsequent text.",
                param!(self, 0, 0)
            ),
            Function::IL => format!(
                "Prepare the insertion of {} liens.",
                param!(self, 0, 1)
            ),
            Function::JFY => explain_selection!(Justification, self, 0),
            Function::MC => explain_selection!(MediaCopy, self, 0),
            Function::NP => format!(
                "Display the {} following page in the presentation component.",
                param!(self, ordinal 0, 1)
            ),
            Function::PEC => format!(
                concat!(
                    "Display the following graphic characters with spacing and extent in {}."
                ),
                explain_selection!(PresentationExpandContract, self, 0)
            ),
            Function::PFS => explain_selection!(PageFormat, self, 0),
            Function::PP => format!(
                "Display the {} preceding page in the presentation component.",
                param!(self, ordinal 0, 1)
            ),
            Function::PPA => format!(
                "Causes the active data position to be moved to the corresponding character position on page {}.",
                param!(self, 0, 1)
            ),
            Function::PPB => format!(
                concat!(
                    "Causes the active data position to be moved to the corresponding character position on the {} ",
                    "previous pages."
                ),
                param!(self, ordinal 0, 1)
            ),
            Function::PPR => format!(
                concat!(
                    "Causes the active data position to be moved to the corresponding character position on the {} ",
                    "following pages."
                ),
                param!(self, ordinal 0, 1)
            ),
            Function::PTX => explain_selection!(ParallelText, self, 0),
            Function::QUAD => format!(
                "Indicates the end of a string of graphic characters that are to be positioned on a single line {}.",
                explain_selection!(Alignment, self, 0)
            ),
            Function::REP => format!(
                "Repeat the previous graphic character {} times.",
                param!(self, 0, 1)
            ),
            Function::RM => format!(
                "Reset the following Modes: {}",
                self.parameters.iter().map(|value| {
                    value.parse::<Mode>().expect("Expect only valid Modes").name()
                }).fold(String::new(), |mut modes, mode| {
                    modes.push_str(", ");
                    modes.push_str(&mode);
                    modes
                })
            ),
            Function::SACS => format!(
                "Enlarge inter-character escapement by {} units.",
                param!(self, 0, 0)
            ),
            Function::SAPV => format!(
                "Select an alternative presentation variant for the subsequent text. {}",
                explain_selection!(PresentationVariant, self, 0)
            ),
            Function::SCO => format!(
                "Establishes the amount of rotation of graphic characters following. {}",
                explain_selection!(CharacterOrientation, self, 0)
            ),
            Function::SCP => format!(
                "Change the character path. {} {}",
                explain_selection!(CharacterPath, self, 0),
                explain_selection!(CharacterPathScope, self, 1)
            ),
            Function::SCS => format!(
                "Character are spaced by {} units",
                param!(self, 0, 0)
            ),
            Function::SD => format!(
                concat!(
                    "Scroll down by {} lines."
                ),
                param!(self, 0, 1)
            ),
            Function::SDS => explain_selection!(StringDirection, self, 0),
            Function::SEE => format!(
                "When character or line insertions or deletions require content to be shifted, {}.",
                explain_selection!(EditingExtend, self, 0)
            ),
            Function::SEF => format!(
                "{} {}",
                explain_selection!(Load, self, 0),
                explain_selection!(Stack, self, 1)
            ),
            Function::SGR => format!(
                "Change the representation of following text. {}.",
                self.parameters.iter().map(|value| {
                    value.parse::<GraphicRendition>().expect("Expect only valid Graphic Renditions").explain()
                }).fold(String::new(), |mut renditions, rendition| {
                    renditions.push_str(", ");
                    renditions.push_str(&rendition);
                    renditions
                })
            ),
            Function::SHS => explain_selection!(CharacterSpacing, self, 0),
            Function::SIMD => explain_selection!(MovementDirection, self, 0),
            Function::SL => format!(
                "Scroll left by {} characters",
                param!(self, 0, 1)
            ),
            Function::SLH => format!(
                "Set the line home position to line {} for the active and following lines.",
                param!(self, 0, 0)
            ),
            Function::SLL => format!(
                "Set the line limit position to character position {} for the active and following lines.",
                param!(self, 0, 0)
            ),
            Function::SLS => format!(
                "Set the line spacing to {}, expressed in the unit established by 'Select Size Unit' (SSU).",
                param!(self, 0, 0)
            ),
            Function::SM => format!(
                "Set the following Modes: {}",
                self.parameters.iter().map(|value| {
                    value.parse::<Mode>().expect("Expect only valid Modes").name()
                }).fold(String::new(), |mut modes, mode| {
                    modes.push_str(", ");
                    modes.push_str(&mode);
                    modes
                })
            ),
            Function::SPD => format!(
                "In {}, set the presentation direction to {}.",
                explain_selection!(PresentationDirectionScope, self, 1),
                explain_selection!(PresentationDirection, self, 0)
            ),
            Function::SPH => format!(
                "Set the page home position to line position {}.",
                param!(self, 0, 0)
            ),
            Function::SPI => format!(
                concat!(
                    "Establish the spacing increment to {} line spacing and {} character spacing, expressed in the ",
                    "unit established by 'Select Size Unit' (SSU)."
                ),
                param!(self, 0, 0),
                param!(self, 1, 0)
            ),
            Function::SPL => format!(
                "Set the page limit position to line {} for the active and following lines.",
                param!(self, 0, 0)
            ),
            Function::SPQR => explain_selection!(PrintQuality, self, 0),
            Function::SR => format!(
                "Scroll right by {} characters.",
                param!(self, 0, 1)
            ),
            Function::SRCS => format!(
                "Establish reduced inter-character escapement by {} units for subsequent text.",
                param!(self, 0, 0)
            ),
            Function::SRS => explain_selection!(ReversedString, self, 0),
            Function::SSU => format!(
                "The size unit for operation is expressed as {}",
                explain_selection!(SizeUnit, self, 0)
            ),
            Function::SSW => format!(
                "Set the escapement of space to {} units.",
                param!(self, 0, 0)
            ),
            Function::STAB => format!(
                concat!(
                    "Causes subsequent text in the presentation component to be aligned according to the position and ",
                    "properties of a tabulation stop which is selected from a list according to the value of the ",
                    "parameter: {}."
                ),
                param!(self, 0, 0)
            ),
            Function::SU => format!(
                "Scroll up by {} lines.",
                param!(self, 0, 1)
            ),
            Function::SVS => explain_selection!(LineSpacing, self, 0),
            Function::TAC => format!(
                concat!(
                    "Causes a character tabulation stop calling for centring to be set at character position {} in ",
                    "the active line."
                ),
                param!(self, 0, 0)
            ),
            Function::TALE => format!(
                concat!(
                    "Causes a character tabulation stop calling for leading edge alignment to be set at character ",
                    "position {} in the active line."
                ),
                param!(self, 0, 0)
            ),
            Function::TATE => format!(
                concat!(
                    "Causes a character tabulation stop calling for trailing edge alignment to be set at character ",
                    "position {} in the active line."
                ),
                param!(self, 0, 0)
            ),
            Function::TBC => explain_selection!(ClearTabulation, self, 0),
            Function::TCC => format!(
                concat!(
                    "Causes a character tabulation stop calling for alignment of a target graphic character {} to be ",
                    "set at character position {} in the active line."
                ),
                param!(self, 1, 32),
                param!(self, 0, 0)
            ),
            Function::TSR => format!(
                concat!(
                    "Causes any character tabulation stop at character position {} in the active line and subsequent ",
                    "lines to be cleared."
                ),
                param!(self, 0, 0)
            ),
            Function::TSS => format!(
                "Establish the width of a thin space for subsequent text to be {} units.",
                param!(self, 0, 0)
            ),
            Function::VPA => format!(
                concat!(
                    "Causes the active data position to be moved to line position {} in the data component in a ",
                    "direction parallel to the line progression."
                ),
                param!(self, 0, 1)
            ),
            Function::VPB => format!(
                concat!(
                    "Causes the active data position to be moved by {} line positions in the data component in a ",
                    "direction opposite of that of the line progression."
                ),
                param!(self, 0, 1)
            ),
            Function::VPR => format!(
                concat!(
                    "Causes the active data position to be moved {} line positions in the data component in a ",
                    "direction parallel of the line progression."
                ),
                param!(self, 0, 1)
            ),
            Function::PRIVATE => String::from("Reserved for private use / not standardized."),
        }
    }

    fn long_description(&self) -> String {
        match function(&self) {
            Function::BEL => String::from(
                "Calls for the attention of the user by controlling an alarm or attention device.",
            ),
            Function::BS => String::from(
                concat!(
                    "Causes the active data position to be moved one character position in the direction opposite to ",
                    "that of the implicit character movement. The direction of the implicit movement depends on the ",
                    "parameter value of 'Select Implicit Movement Direction' (SIMD)."
                )
            ),
            Function::CAN => String::from(
                concat!(
                    "Indicates that the data preceding it is in error. As a result, this data shall be ignored. ",
                    "The specific meaning of this control function shall be defined for each application and/or ",
                    "between sender and recipient."
                )
            ),
            Function::CR => String::from(
                concat!(
                    "Move the cursor to the beginning of the line. The exact meaning depends on the setting of ",
                    "'Device Component Select Mode' (DCSM) and on the parameter value of 'Select Implicit Movement ",
                    "Direction' (SIMD).",
                    "\n",
                    "\n",
                    "If the DCSM is set to 'Presentation' and SIMD is set to 'Normal', it ",
                    "causes the active presentation position to be moved to the line home position of the same line ",
                    "in the presentation component. The line home position is established by the parameter value of ",
                    "'Set Line Home' SLH.",
                    "\n",
                    "With SIMD set to 'Opposite', it causes the active presentation position ",
                    "to be moved to the line limit position of the same line in the presentation component. ",
                    "The line limit position is established by the parameter value of 'Set Line Limit' (SLL).",
                    "\n",
                    "\n",
                    "If the DCSM is set to 'Data' and SIMD is set to 'Normal', it causes the active data position to ",
                    "be moved to the line home position of the same line in the data component. The line home ",
                    "position is established by the parameter value of 'Set Line Home' (SLH)",
                    "\n",
                    "With SIMD set to 'Opposite', it causes the active data position to be moved to the line limit ",
                    "position of the same line in the data component. The line limit position position is established ",
                    "by the parameter value of 'Set Line Limit' (SLL)."
                )
            ),
            Function::DC1 => String::from(
                concat!(
                    "Primarily intended for turning on or starting an ancillary device. If it is not required for ",
                    "this purpose, it may be used to restore a device to the basic mode of operation. When used for ",
                    "data flow control, it is also sometimes called X-ON."
                )
            ),
            Function::DC2 => String::from(
                concat!(
                    "Primarily intended for turning on or starting an ancillary device. If it is not required for ",
                    "this purpose, it may be used to set a device to a special mode of operation (in which case DC1 ",
                    "is used to restore the mode of operation to the normal mode), or for any other device control ",
                    "function not provided by other DCs."
                )
            ),
            Function::DC3 => String::from(
                concat!(
                    "Primarily intended for turning off or stopping an ancillary device. This function may be a ",
                    "secondary level stop, for example wait, pause, stand-by, or halt (in which case DC1 is used to ",
                    "restore normal operation). If it is not required for this purpose, it may be used for any other ",
                    "device control function not provided by other DCs."
                )
            ),
            Function::DC4 => String::from(
                concat!(
                    "Primarily intended for turning off, stopping, or interrupting an ancillary device. If it is not ",
                    "required for this purpose, it may be used for any other device control function not provided by ",
                    "other DCs."
                )
            ),
            Function::EM => String::from(
                concat!(
                    "Identifies the physical end of a medium, or the end of the used portion of a medium, or the end ",
                    "of the wanted portion of data recorded on a medium."
                )
            ),
            Function::ESC => String::from(
                concat!(
                    "Used for code extension purposes. It causes the meanings of a limited number of bit combinations ",
                    "following it in the data stream to be changed."
                )
            ),
            Function::FF => String::from(
                concat!(
                    "Causes the active presentation position to be moved to the corresponding character position of ",
                    "the line at the page home position of the next form or page in the presentation component. The ",
                    "page home position is established by the parameter value of 'Set Page Home' (SPH)."
                )
            ),
            Function::HT => String::from(
                concat!(
                    "Causes the active presentation position to be moved to the following character tabulation stop ",
                    "in the presentation component. In addition, if that following character tabulation stop has been ",
                    "set by 'Tabulation Align Center' (TAC), 'Tabulation Align Leading Edge' (TALE), or 'Tabulation ",
                    "Centred On Character' (TACE), it causes the beginning of a string of text which is to be ",
                    "positioned within a line according to the properties of that tabulation stop. The end of the ",
                    "string is indicated by the next occurrence of HT, CR, or NEL in the data stream."
                )
            ),
            Function::IS1 => String::from(
                concat!(
                    "Separates and qualifies data logically, its specific meaning has to be defined for each ",
                    "application. If this control function is used in hierarchical order, it may delimit a data item ",
                    "called a unit."
                )
            ),
            Function::IS2 => String::from(
                concat!(
                    "Separates and qualifies data logically, its specific meaning has to be defined for each ",
                    "application. If this control function is used in hierarchical order, it may delimit a data item ",
                    "called a record."
                )
            ),
            Function::IS3 => String::from(
                concat!(
                    "Separates and qualifies data logically, its specific meaning has to be defined for each ",
                    "application. If this control function is used in hierarchical order, it may delimit a data item ",
                    "called a group."
                )
            ),
            Function::IS4 => String::from(
                concat!(
                    "Separates and qualifies data logically, its specific meaning has to be defined for each ",
                    "application. If this control function is used in hierarchical order, it may delimit a data item ",
                    "called a file."
                )
            ),
            Function::LF => String::from(
                concat!(
                    "If the 'Device Component Select Mode' is set to 'Presentation', it causes the active ", 
                    "presentation position to be moved to the corresponding character position of the following line ",
                    "in the presentation component.",
                    "\n",
                    "\n",
                    "If the 'Device Component Select Mode' is set to 'Data', it causes the active data position to be ",
                    "moved to the corresponding character position of the following line in the data component."
                )
            ),
            Function::LS0 => String::from(
                concat!(
                    "Used for code extension purposes. It causes the meanings of the bit combinations following it in ",
                    "the data stream to be changed."
                )
            ),
            Function::LS1 => String::from(
                concat!(
                    "Used for code extension purposes. It causes the meanings of the bit combinations following it in ",
                    "the data stream to be changed."
                )
            ),
            Function::NUL => String::from(
                concat!(
                    "Used for media-fill or time-fill. NUL characters may be inserted into, or removed from, a data ",
                    "stream without affecting information content of that stream, but such action may affect the ",
                    "information layout and/or the control of equipment."
                )
            ),
            Function::SYN => String::from(
                concat!(
                    "Used by a synchronous transmission system in the absence of any other character (idle condition) ",
                    "to provide a signal from which synchronism may be achieved or retained between data terminal ",
                    "equipment."
                )
            ),
            Function::VT => String::from(
                concat!(
                    "Causes the active presentation position to be moved in the presentation component to the ",
                    "corresponding character position on th e line at which the following line tabulation stop is ",
                    "set."
                )
            ),
            Function::APC => String::from(
                concat!(
                    "Used as the opening delimiter of a control string for application program use. The command ",
                    "string following may consist of bit combinations in the range 00/08 to 00/13 and 02/00 to 07/14. ",
                    "The control string is closed by the terminating delimiter 'String Terminator' (ST). The ",
                    "interpretation of the command string depends on the relevant application program."
                )
            ),
            Function::CCH => String::from(
                concat!(
                    "Indicates that both the preceding graphic character in the data stream (represented by one or ",
                    "more bit combinations), including 'Space', and the control function itself are to be ignored ",
                    "for further interpretation in the data stream.",
                    "\n",
                    "\n",
                    "If the character preceding CCH in the data stream is a control function (represented by one or ",
                    "more bit combinations), the effect of CCH is not defined."
                )
            ),
            Function::DCS => String::from(
                concat!(
                    "Used as the opening delimiter of a control string for device control use. The command string ", 
                    "following may consist of bit combinations in the range 00/08 to 00/13 and 02/00 to 07/14. The ",
                    "control string is closed by the terminating delimiter 'String Terminator' (ST)."
                )
            ),
            Function::EPA => String::from(
                concat!(
                    "Indicates that the active presentation position is the last of a string of character positions ",
                    "in the presentation component, the contents of which are protected against manual alteration, ",
                    "are guarded against transmission or transfer, depending on the settings of 'Guarded Area ",
                    "Transfer Mode' (GATM), and may be protected against erasure, depending on the setting of ",
                    "'Erasure Mode' (ERM). The beginning of this string is indicated by 'Start of Guarded Area' (SPA)."
                )
            ),
            Function::ESA => String::from(
                concat!(
                    "Indicates that the active presentation position is the last of a string of character positions ",
                    "in the presentation component, the contents of which are eligible to be transmitted in the form ",
                    "of a data stream or transferred to an auxiliary input/output device. The beginning of the string ",
                    "is indicated by 'Start of Selected Area' (SSA)"
                )
            ),
            Function::HTJ => String::from(
                concat!(
                    "Causes the contents of the active field (the field in the presentation component that contains ",
                    "active presentation position) to be shifted forwarded, so that it ends at the character position ",
                    "preceding the following character tabulation stop. The active presentation position is moved to ",
                    "that following character tabulation stop. The character position which precede the beginning of ",
                    "the shifted string are put into the erased state."
                )
            ),
            Function::HTS => String::from(
                concat!(
                    "Causes a character tabulation stop to be set at the active presentation position in the ",
                    "presentation component. The number of lines affected depends on the setting of the ",
                    "'Tabulation Stop Mode' (TSM)."
                )
            ),
            Function::MW => String::from(
                concat!(
                    "Sets a message waiting indicated in the receiving device. An appropriate acknowledgement to the ",
                    "receipt of MW may be given by using 'Device Status Report' (DSR)."
                )
            ),
            Function::NBH => String::from(
                concat!(
                    "Indicates a point where a line break shall not occur when text is formatted. This may occur ",
                    "between two graphic characters, either or both which may be 'Space'."
                )
            ),
            Function::NEL => String::from(
                concat!(
                    "The effect of NEL depends on the setting of the 'Device Component Select Mode' (DCSM) and the ",
                    "parameter value of 'Select Implicit Movement Direction' (SIMD).",
                    "\n",
                    "\n",
                    "If DCSM is set to 'Presentation' and SIMD equal to 'Normal', it causes the active presentation ",
                    "position to be moved to the line home position of the following line in the presentation ",
                    "component. The line home position may be established by the parameter of 'Set Line Home' (SLH). ",
                    "\n",
                    "With SIMD equal to 'Opposite', it causes the active presentation position to be moved to the ",
                    "line limit position of the following line in the presentation component. The line limit position ",
                    "may be established by the parameter of 'Set Line Limit' (SLL).",
                    "\n",
                    "\n",
                    "If DCSM is set to 'Data' and SIMD equal to 'Normal', it causes the active data ",
                    "position to be moved to the line home position of the following line in the data ",
                    "component. The line home position may be established by the parameter of 'Set Line Home' (SLH). ",
                    "\n",
                    "With SIMD equal to 'Opposite', it causes the active data position to be moved to the ",
                    "line limit position of the following line in the data component. The line limit position ",
                    "may be established by the parameter of 'Set Line Limit' (SLL)."
                )
            ),
            Function::OSC => String::from(
                concat!(
                    "Opening delimiter of a control string for operating system use. The command string following may ",
                    "consist of a sequence of bit combinations in the range 00/08 to 00/13 and 02/00 to 07/14. The ",
                    "control string is closed by the terminating delimiter 'String Terminator' (ST). The ",
                    "interpretation of the command string depends on the relevant operating system."
                )
            ),
            Function::PLD => String::from(
                concat!(
                    "Move the active presentation position in the presentation component to the corresponding ",
                    "position of an imaginary line with a partial offset in the direction of line progression. This ",
                    "offset should be sufficient either to image following characters as subscripts until the first ",
                    "following occurrence of 'Partial Line Backwards' (PLU) in the data stream, or, if preceding ",
                    "characters were imaged as superscripts, to restore imaging of following characters to the active ",
                    "line."
                )
            ),
            Function::PLU => String::from(
                concat!(
                    "Move the active presentation position in the presentation component to the corresponding ",
                    "position of an imaginary line with a partial offset in the direction opposite of line ",
                    "progression. This offset should be sufficient either to image following characters as ",
                    "superscripts until the first following occurrence of 'Partial Line Forward' (PLD) in the data ",
                    "stream, or, if preceding characters were imaged as subscripts, to restore imaging of following ",
                    "characters to the active line."
                )
            ),
            Function::PM => String::from(
                concat!(
                    "Indicates the beginning of a control string privacy message use. The command string following ",
                    "may consist of bit combination sin the range 00/08 to 00/13 and 02/00 to 07/14. The control ",
                    "string is closed by the terminating delimiter 'String Terminator' (ST). The interpretation", 
                    "of the command string depends on the relevant privacy discipline."
                )
            ),
            Function::RI => String::from(
                concat!(
                    "If the 'Device Component Select Mode' (DCSM) is set to 'Presentation', it causes the active ",
                    "presentation position to be moved in the presentation component to the corresponding character ",
                    "position of the preceding line.",
                    "\n",
                    "\n",
                    "If the 'Device Component Select Mode' (DCSM) is set to 'Data', it causes the active ",
                    "data position to be moved in the data component to the corresponding character ",
                    "position of the preceding line."
                )
            ),
            Function::SCI => String::from(
                concat!(
                    "This and the bit combination following it are used to represent a control function or a graphic ",
                    "character. The bit combination following SCI must be from 00/08 to 00/13 or 02/00 to 07/14. The ",
                    "use of SCI is reserved for future standardization."
                )
            ),
            Function::SOS => String::from(
                concat!(
                    "Used as the opening delimiter of a control string. The character string following may consist of ",
                    "any bit combinations, except those representing SOS or 'String Terminator' (ST). The control ",
                    "string is closed by the terminating delimiter 'String Terminator' (ST). The interpretation of the ",
                    "character string depends on the application."
                )
            ),
            Function::SPA => String::from(
                concat!(
                    "Used to indicate that the active presentation position is the first of a string of character ",
                    "positions in the presentation component, the contents of which are protected against manual ",
                    "alteration, are guarded against transmission or transfer, depending on the setting of 'Guarded ",
                    "Area Transfer Mode' (GATM), and may be protected against erasure, depending on the setting of ",
                    "the 'Erasure Mode' (ERM). The end of this string is indicated by 'End of Guarded Area' (EPA)."
                )
            ),
            Function::SSA => String::from(
                concat!(
                    "Indicates that the active presentation position is the first of a string of character positions ",
                    "in the presentation component, the contents of which are eligible to be transmitted in the form ",
                    "of a data stream or transferred to an ancillary input/output device. The end of this string is ",
                    "indicated by 'End of Selected Area' (ESA). ",
                    "\n",
                    "\n",
                    "The string of character actually transmitted or transferred depends on the setting of 'Guarded ",
                    "Area Transfer mode' (GATM) and on any guarded areas established by 'Define Area Qualification' ",
                    "(DAQ), or by 'Start of Guarded Area' (SPA) and 'End of Guarded Area' (EPA)."
                )
            ),
            Function::STS => String::from(
                concat!(
                    "Used to establish the transmit state in the receiving device. In this state the transmission of ",
                    "data from the device is possible. The actual initiation of transmission of data is performed by ",
                    "a data communication or input/output interface control procedure, which is outside of the scope ",
                    "of this Standard.",
                    "\n",
                    "\n",
                    "The transmit state is established either by this appearing in the received data stream, or by ",
                    "the operation of an appropriate key on a keyboard."
                )
            ),
            Function::CMD => String::from(
                concat!(
                    "Delimits a string of data coded according to standard ECMA-35, and to switch to a general level ",
                    "of control. The use of this is not mandatory if the higher level protocol defines means of ",
                    "delimiting the string, for instance by specifying the length of the string."
                )
            ),
            Function::RIS => String::from(
                concat!(
                    "Reset the receiving device to its initial state, i.e. the state it has after it is made ",
                    "operational. This may imply, if applicable: clear tabulation stops, remove qualified areas, ",
                    "reset graphic rendition, put all character positions into the erased state, move the active ",
                    "presentation position to the first position of the first line in the presentation component, ",
                    "move the active data position to the first character position in the first line in the data ",
                    "component, set the modes into the reset state, etc.."
                )
            ),
            Function::CBT => format!(
                concat!(
                    "Causes the active presentation position to be moved to the character position corresponding ",
                    "to the {} preceding character tabulation stop in the presentation component, according to ",
                    "the character path.",

                ),
                param!(self, ordinal 0, 1)
            ),
            Function::CHA => format!(
                concat!(
                    "Causes the active presentation position to be moved to character position {} in the active line ",
                    "in the presentation component"
                ),
                param!(self, 0, 1)
            ),
            Function::CHT => format!(
                concat!(
                    "Causes the active presentation position to be moved to the character position corresponding to ",
                    "the {} following character tabulation stop in the presentation component, according to the ",
                    "character path."
                ),
                param!(self, ordinal 0, 1)
            ),
            Function::CNL => format!(
                concat!(
                    "Causes the active presentation position to be moved to the first character position of the {} ",
                    "following line in the presentation component."
                ),
                param!(self, ordinal 0, 1)
            ),
            Function::CPL => format!(
                concat!(
                    "Causes the active presentation position to be moved to the first character position of the {} ",
                    "preceding line in the presentation component."
                ),
                param!(self, ordinal 0, 1)
            ),
            Function::CPR => format!(
                concat!(
                    "If the 'Device Component Select Mode' (DCSM) is set to 'Presentation', reports the active ",
                    "presentation position of the sending device as residing in the presentation component at the {} ",
                    "line position according to the line progress and at the {} character position according to the ",
                    "character path.",
                    "\n\n",
                    "If the 'Device Component Select Mode' (DCSM) is set to 'Data', reports the active data position ",
                    "of the sending device as residing in the data component at the {} line position according to the ",
                    "line progression and at the {} character position according to the character progression.",
                    "\n\n",
                    "CPR may be solicited by a 'Device Status Report' (DSR) or be sent unsolicited."
                ),
                param!(self, ordinal 0, 1),
                param!(self, ordinal 1, 1),
                param!(self, ordinal 0, 1),
                param!(self, ordinal 1, 1),
            ),
            Function::CUB => format!(
                concat!(
                    "Causes the active presentation position to be moved leftwards in the presentation component by ",
                    "{} character positions, if the character path is horizontal, or by {} line positions, if the ",
                    "character path is vertical."
                ),
                param!(self, 0, 1),
                param!(self, 0, 1)
            ),
            Function::CUD => format!(
                concat!(
                    "Causes the active presentation position to be moved downwards in the presentation component by ",
                    "{} line positions, if the character path is horizontal, or by {} character positions, if the ",
                    "character path is vertical."
                ),
                param!(self, 0, 1),
                param!(self, 0, 1)
            ),
            Function::CUF => format!(
                concat!(
                    "Causes the active presentation position to be moved rightwards in the presentation component by ",
                    "{} character positions, if the character path is horizontal, or by {} line positions, if the ",
                    "character path is vertical."
                ),
                param!(self, 0, 1),
                param!(self, 0, 1)
            ),
            Function::CUP => format!(
                concat!(
                    "Causes the active presentation position to be moved in the presentation component to the {} line ",
                    "position according to the line progression, and to the {} character position according to the ",
                    "character path.",
                ),
                param!(self, ordinal 0, 1),
                param!(self, ordinal 1, 1)
            ),
            Function::CUU => format!(
                concat!(
                    "Causes the active presentation position to be moved upwards in the presentation component by {} ",
                    "line positions, if the character path is horizontal, or by {} character positions, if the ",
                    "character path is vertical."
                ),
                param!(self, 0, 1),
                param!(self, 0, 1)
            ),
            Function::CVT => format!(
                concat!(
                    "Causes the active presentation position to be moved to the character position of the line ",
                    "corresponding to the {} following line tabulation stop in the presentation component."
                ),
                param!(self, ordinal 0, 1)
            ),
            Function::DAQ => format!(
                concat!(
                    "This is used to indicate that the active presentation position in the presentation component is ",
                    "the first character position of a qualified area. The last character position of the qualified ",
                    "area is the character position in the presentation component immediately preceding the first ",
                    "character position of the following qualified area. This area {}."
                ),
                explain_selection!(AreaQualification, self, 0)
            ),
            Function::DCH => format!(
                concat!(
                    "If the 'Device Component Select Mode' (DSCM) is set to 'Presentation', it causes the contents ",
                    "of the active presentation position and, depending on the setting of 'Character Editing Mode' ",
                    "(HEM), the contents of the preceding or following character positions to be removed from the ",
                    "presentation component. The resulting gap of {} characters is closed by shifting the contents of ",
                    "the adjacent character positions towards the active presentation position. At the other end of ",
                    "the shifter part {} character positions are put into the erased state.",
                    "\n\n",
                    "The extend of the shifted part is established by 'Select Editing Extend' (SEE).",
                    "\n\n",
                    "The effect of this on the start or end of a selected area, the start or end of a qualified area, ",
                    "or a tabulation stop in the shifted part is undefined.",
                    "\n\n",
                    "If the 'Device Component Select Mode' (DCSM) is set to 'Data', it causes the contents of the ",
                    "active data position and, depending on the setting of 'Character Editing Mode' (HEM), the ",
                    "contents of the preceding or following character positions to be removed from the data ",
                    "component. The resulting gap of {} characters is closed by shifting the contents of the adjacent ",
                    "character positions towards the active data position. At the other end of the shifted part, {} ",
                    "character positions are put into the erased state."
                ),
                param!(self, 0, 1),
                param!(self, 0, 1),
                param!(self, 0, 1),
                param!(self, 0, 1)
            ),
            Function::DL => format!(
                concat!(
                    "If the 'Device Component Select Mode' (DSCM) is set to 'Presentation', it causes the contents of ",
                    "the active line (the line that contains the active presentation position) and, depending on the ",
                    "setting of the 'Line Editing Mode' (VEM), the contents of the preceding or following lines to be ",
                    "removed from the presentation component. The resulting gap of {} lines is closed by shifting the ",
                    "contents of a number of adjacent lines towards the active line. At the end of the shifted part, ",
                    "{} lines are put into the erased state. The active presentation position is moved to the line ",
                    "home position in the active line. The line home position is established by the parameter value ",
                    "of 'Set Line Home' (SLH). If the 'Tabulation Stop Mode' (TSM) is set to 'Single', character ",
                    "tabulation stops are cleared in the lines that are put into the erased state.",
                    "\n\n",
                    "The extend of the shifted part is established by 'Select Editing Extend' (SEE).",
                    "\n\n",
                    "Any occurrences of the start or end of a selected area, the start or end of a qualified area, or ",
                    "a tabulation stop in the shifted part, are also shifted.",
                    "\n\n",
                    "If the 'Device Component Select Mode (DCSM) is set to 'Data', it causes the contents of the ",
                    "active line (the line that contains the active data position) and, depending on the settings of ",
                    "the 'Line Editing Mode' (VEM), the contents of the preceding or following lines to be removed ",
                    "from the data component. The resulting gap of {} lines is closed by shifting the contents of a ",
                    "number of adjacent lines towards the active line. At the other end of the shifted part, {} lines ",
                    "are put into the erased state. The active data position is moved to the line home position in ",
                    "the active line. The line home position is established by the parameter value of 'Set Line Home' ",
                    "(SLH)."
                ),
                param!(self, 0, 1),
                param!(self, 0, 1),
                param!(self, 0, 1),
                param!(self, 0, 1)
            ),
            Function::DTA => format!(
                concat!(
                    "Establishes the dimension of the text area for subsequent pages. The established dimensions ",
                    "remain in effect until the next occurrence of DTA in the data stream. The new dimension is ",
                    "specified to be {} in the direction perpendicular to the line orientation and {} parallel to the ",
                    "line orientation. The unit in which the value is expressed is that established by the parameter ",
                    "value of 'Select Size Unit' (SSU)."
                ),
                param!(self, 0, 0),
                param!(self, 1, 0)
            ),
            Function::EA => format!(
                concat!(
                    "If the 'Device Component Select Mode' (DCSM) is set to 'Presentation', {} in the presentation ",
                    "component. The contents of the removed area are put into the erased state.",
                    "\n\n",
                    "If the 'Device Component Select Mode' (DCSM) is set to 'Data', {} in the data component. The ",
                    "contents of the removed area are put into the erased state.",
                    "\n\n",
                    "Whether the character positions of protected areas are put into the erased state, or the ",
                    "character positions of unprotected areas only, depends on the settings of 'Erasure Mode' (ERM)."
                ),
                explain_selection!(EraseArea, self, 0),
                explain_selection!(EraseArea, self, 0)
            ),
            Function::ECH => format!(
                concat!(
                    "If the 'Device Component Select Mode' (DCSM) is set to 'Presentation', it causes the active ",
                    "presentation position and the following character positions in the presentation component to be ",
                    "put into the erased state. {} characters will be erased.",
                    "\n\n",
                    "If the 'Device Component Select Mode' (DCSM) is set to 'Data', it causes the active data position ",
                    "and the following character positions in the data component to be put into the erased state. {} ",
                    "characters will be erased.",
                    "\n\n",
                    "Whether the character positions of protected areas are put into the erased state, or the ",
                    "character positions of unprotected areas only, depends on the settings of 'Erasure Mode' (ERM)."
                ),
                param!(self, 0, 1),
                param!(self, 0, 1)
            ),
            Function::ED => format!(
                concat!(
                    "If the 'Device Component Select Mode' (DCSM) is set to 'Presentation', {} in the presentation ",
                    "component. The contents of the removed page are put into the erased state.",
                    "\n\n",
                    "If the 'Device Component Select Mode' (DCSM) is set to 'Data', {} in the data component. The ",
                    "contents of the removed page are put into the erased state.",
                    "\n\n",
                    "Whether the character positions of protected areas are put into the erased state, or the ",
                    "character positions of unprotected areas only, depends on the settings of 'Erasure Mode' (ERM)."
                ),
                explain_selection!(EraseArea, self, 0),
                explain_selection!(EraseArea, self, 0)
            ),
            Function::EF => format!(
                concat!(
                    "If the 'Device Component Select Mode' (DCSM) is set to 'Presentation', {} in the presentation ",
                    "component. The contents of the removed field are put into the erased state.",
                    "\n\n",
                    "If the 'Device Component Select Mode' (DCSM) is set to 'Data', {} in the data component. The ",
                    "contents of the removed field are put into the erased state.",
                    "\n\n",
                    "Whether the character positions of protected areas are put into the erased state, or the ",
                    "character positions of unprotected areas only, depends on the settings of 'Erasure Mode' (ERM)."
                ),
                explain_selection!(EraseArea, self, 0),
                explain_selection!(EraseArea, self, 0)
            ),
            Function::EL => format!(
                concat!(
                    "If the 'Device Component Select Mode' (DCSM) is set to 'Presentation', {} in the presentation ",
                    "component. The contents of the removed line are put into the erased state.",
                    "\n\n",
                    "If the 'Device Component Select Mode' (DCSM) is set to 'Data', {} in the data component. The ",
                    "contents of the removed line are put into the erased state.",
                    "\n\n",
                    "Whether the character positions of protected areas are put into the erased state, or the ",
                    "character positions of unprotected areas only, depends on the settings of 'Erasure Mode' (ERM)."
                ),
                explain_selection!(EraseArea, self, 0),
                explain_selection!(EraseArea, self, 0)
            ),
            Function::FNT => format!(
                concat!(
                    "{}\n\n",
                    "The active Font might be switched in the following data stream by 'Select Graphic Rendition (SGR)."
                ),
                self.short_description()
            ),
            Function::GSM => format!(
                concat!(
                    "Used to modify the text height and / or width of the subsequent text for all primary and ",
                    "alternatives fonts and established 'Graphic Size Select' (GSS). The established values remain in ",
                    "effect until the next occurrence of GSM or GSS in the data stream. The new size is set to to {}% ",
                    "height and {}% width."
                ),
                param!(self, 0, 100),
                param!(self, 1, 100)
            ),
            Function::GSS => format!(
                concat!(
                    "Used to establish the height for the subsequent text for all primary and alternative fonts. The ",
                    "established value remains in effect until the next occurrence of GSS in the data stream. The new ",
                    "height is set to {} with a unit established by 'Select Size Unit' (SSU)."
                ),
                param!(self, 0, 0)
            ),
            Function::HPA => format!(
                concat!(
                    "Causes the active data position to be moved to the character position {} in the active line (the ",
                    "line in the data component that contains the active data position)"
                ),
                param!(self, 0, 1)
            ),
            Function::HPB => format!(
                concat!(
                    "Causes the active data position to be moved by {} character positions in the data component in ",
                    "the direction opposite to that of the character progression."
                ),
                param!(self, 0, 1)
            ),
            Function::HPR => format!(
                concat!(
                    "Causes the active data position to be moved by {} character positions in the data component in ",
                    "the direction of character progression."
                ),
                param!(self, 0, 1)
            ),
            Function::HVP => format!(
                concat!(
                    "Causes the active data position to be moved in the data component to the {} line position ",
                    "according to the line progression and to the {} character position according to the character ",
                    "position."
                ),
                param!(self, ordinal 0, 1),
                param!(self, ordinal 1, 1)
            ),
            Function::ICH => format!(
                concat!(
                    "If the 'Device Component Select Mode' (DCSM) is set to 'Presentation', this is used to prepare ",
                    "the insertion of {} characters, by putting into the erased state the active presentation ",
                    "position and, depending on the setting of the Character Editing Mode (HEM), the preceding or ",
                    "following character positions in the presentation component. The previous contents of the active ",
                    "presentation position and an adjacent string of character positions are shifted away from the ",
                    "active presentation position. The contents of {} character positions at the other end of the ",
                    "shifted part are removed. The active presentation position is moved to the line home position in ",
                    "the active line. The line home position is established by the parameter value of 'Set Line Home' ",
                    "(SLH).",
                    "\n\n",
                    "The extent of the shifted part is established by Select Editing Extend (SEE).",
                    "\n\n",
                    "The effect of this on the start or end of a selected area, the start or end of a qualified area, ",
                    "or a tabulation stop in the shifted part is undefined.",
                    "\n\n",
                    "If the 'Device Component Select Mode' (DCSM) is set to 'Data', this is used to prepare the ",
                    "insertion of {} characters, by putting into the erased state the active data position and, ",
                    "depending on the setting of the Character Editing Mode (HEM), the preceding or following ",
                    "character positions in the data component. The previous contents of the active data position and ",
                    "and adjacent string of character positions are shifted away from the active data position. ",
                    "The contents of {} character positions at the other end of the shifted part are removed. The ",
                    "active data position is moved to the line home position in the active line. The line ",
                    "home position is established by the parameter value of Set Line Home (SLH)."
                ),
                param!(self, ordinal 0, 1),
                param!(self, ordinal 0, 1),
                param!(self, ordinal 0, 1),
                param!(self, ordinal 0, 1)
            ),
            Function::IGS => format!(
                concat!(
                    "Indicates that the graphic subrepertoire {} is used in the subsequent text according to the ",
                    "graphic characters of ISO/IEC 10367. The graphic subrepertoire {} is registered in accordance ",
                    "with ISO/IEC 7350"
                ),
                param!(self, 0, 0),
                param!(self, 0, 0)
            ),
            Function::IL => format!(
                concat!(
                    "If the 'Device Component Select Mode' (DCSM) is set to 'Presentation', this is used to prepare ",
                    "the insertion of {} lines, by putting into the erased state in the presentation component the ",
                    "active line (the line that contains the active presentation position) and, depending on the ",
                    "setting of the 'Line Editing Mode' (VEM), the preceding or following lines. The previous contents ",
                    "of the active line and of adjacent lines are shifted away from the active line. The contents of ",
                    "{} lines at the other end of the shifted part are removed. The active presentation position is ",
                    "moved to the line home position in the active line. The line home position is established by the ",
                    "parameter value of 'Set Line Home' (SLH).",
                    "\n\n",
                    "The extent of the shifted part is established by 'Select Editing Extent' (SEE).",
                    "\n\n",
                    "Any occurrence of the start or end of a selected area, the start or end of a qualified area, or ",
                    "a tabulation stop in the shifted part, are also shifted.",
                    "\n\n",
                    "If the 'Tabulation Stop Mode' (TSM) is set to 'Single', character tabulation stops are cleared ",
                    "in the lines that are put into the erased state.",
                    "\n\n",
                    "If the 'Device Component Select Mode' (DCSM) is set to 'Data', this is used to prepare the ",
                    "insertion of {} lines, by putting into the erased state in the data component the active line ",
                    "(the line that contains the active data position) and, depending on the setting of the 'Line ",
                    "Editing Mode' (VEM), the preceding or following lines. The previous contents of the active line ",
                    "and of adjacent lines are shifted away from the active line. The contents of {} lines at the ",
                    "other end of the shifted part are removed. The active data position is moved to the line home ",
                    "position in the active line. The line home position is established by the parameter value of ",
                    "'Set Line Home' (SLH)."
                ),
                param!(self, 0, 1),
                param!(self, 0, 1),
                param!(self, 0, 1),
                param!(self, 0, 1)
            ),
            Function::JFY => format!(
                concat!(
                    "Indicates the beginning of a string of graphic characters in the presentation component that are ",
                    "to be justified according to the layout specified: {}"
                ),
                self.short_description()
            ),
            Function::PEC => format!(
                concat!(
                    "Establish the spacing and the extent of graphic characters for subsequent text. {}",
                    "\n\n",
                    "The spacing is specified in the line as multiples of the spacing established by the most recent ",
                    "occurrence of 'Set Character Spacing' (SCS), of 'Select Character Spacing' (SHS), or of 'Spacing ",
                    "Increment' (SPI) in the data stream. The extent of characters is implicitly established by these ",
                    "control functions. The established spacing and extent remain in effect until the next occurrence ",
                    "of PEC. "
                ),
                self.short_description()
            ),
            Function::PFS => format!(
                concat!(
                    "Establish the available area for the imaging of pages of text based on paper size. {}",
                    "\n\n",
                    "The pages are introduced by the subsequent occurrences of 'Form Feed' (FF) in the data stream. ",
                    "The established area stays into effect until the next occurrence.",
                    "\n\n",
                    "The page home position is established by 'Set Page Home' (SPH), the page limit position is ",
                    "established by 'Set Page Limit' (SPL)."
                ),
                self.short_description()
            ),
            Function::PTX => format!(
                concat!(
                    "Used to delimit strings of graphic characters that are communicated one after another in the ",
                    "data stream, but that are intended to be presented in parallel with another one, usually in ",
                    "adjacent lines.",
                    "\n\n",
                    "{}"
                ),
                self.short_description()
            ),
            Function::QUAD => format!(
                concat!(
                    "Indicates the end of a string of graphic characters that are to be positioned on a single line ",
                    "{}.\n\n",
                    "The beginning of the string to be positioned is indicated by the preceding occurrence in the data ",
                    "stream of either another QUAD, or one of the following formator functions: FF, LF, NEL, RI, VT, ",
                    "HVP, HPA, PPB, PPR, VPA, VPB.",
                    "\n\n",
                    "The line home position is established by the parameter value of 'Set Line Home' (SLH). The line ",
                    "limit position is established by the parameter value of 'Set Line Home' (SLH)."
                ),
                self.short_description()
            ),
            Function::REP => format!(
                concat!(
                    "Used to indicate that the preceding character in the data stream, if it is a graphic character, ",
                    "including 'Space', is to be repeated {} times. If the preceding character is a control function ",
                    "or part of a control function, the effect is undefined."
                ),
                param!(self, 0, 1)
            ),
            Function::RM =>
                self.parameters.iter().map(|value| {
                    value.parse::<Mode>().expect("Expect only valid Modes").explain_reset()
                }).fold(String::new(), |mut modes, mode| {
                    modes.push_str(", ");
                    modes.push_str(&mode);
                    modes
                }
            ),
            Function::SACS => format!(
                concat!(
                    "Used to establish extra inter-character escapement for subsequent text. The established extra ",
                    "escapement remains in effect until the next occurrence of SACS or of 'Set Reduced Character ",
                    "Separation' (SRCS) in the data stream or until it is reset to the default value by a subsequent ",
                    "occurrence of 'Carriage Return Line Feed' (CR LF) or of 'Next Line' (NEL) in the data stream.",
                    "\n\n",
                    "The inter-character escapement is enlarged by {} units",
                    "\n\n",
                    "the unit in which the parameter value is expressed is that established by the parameter value of ",
                    "'Select Size Unit' (SSU)."
                ),
                param!(self, 0, 0)
            ),
            Function::SCS => format!(
                concat!(
                    "Establishes the character spacing for subsequent text. The established spacing remains in effect ",
                    "until the next occurrence, or of 'Select Character Spacing' (SHS) or of 'Spacing Increment' ",
                    "(SPI) in the data stream.\n\nCharacters are spaced by {} units.",
                    "\n\n",
                    "The units in which the value is expressed is that established by the parameter value of 'Select ",
                    "Size Unit' (SSU)."
                ),
                param!(self, 0, 0)
            ),
            Function::SD => format!(
                concat!(
                    "Causes the data in the presentation component to be moved by {} line positions if the line ",
                    "orientation is horizontal, or by {} character positions if the line orientation is vertical, ",
                    "such that the data appear to move down.",
                    "\n\n",
                    "The active presentation position is not affected by this function."
                ),
                param!(self, 0, 1),
                param!(self, 0, 1)
            ),
            Function::SDS => format!(
                concat!(
                    "Establishes in the data component the beginning and end of a string of characters, as well as ",
                    "the direction of the string. This direction may be different from that currently established. ",
                    "The indicated string follows the preceding text. The established character progression is not ",
                    "affected. {}"
                ),
                self.short_description()
            ),
            Function::SEE => format!(
                concat!(
                    "Used to establish the editing extend for subsequent character or line insertion or deletion. The ",
                    "established context remains in effect until the next occurrence of SEE in the data stream. {}"
                ),
                self.short_description()
            ),
            Function::SEF => format!(
                concat!(
                    "Causes a sheet of paper to be ejected from a printing device into a specified output stacker an ",
                    "another sheet to be loaded into the printing device from a specified paper bin. {} {}"
                ),
                explain_selection!(Load, self, 0),
                explain_selection!(Stack, self, 1)
            ),
            Function::SGR => format!(
                concat!(
                    "Establishes one or more graphic rendition aspects for subsequent text. The established aspects ",
                    "remain in effect until the next occurrence, depending on the setting of the 'Graphic Rendition ",
                    "Combination Mode' (GRCM).\n\n{}"
                ),
                self.parameters.iter().map(|value| {
                    value.parse::<GraphicRendition>().expect("Expect only valid Graphic Renditions").explain()
                }).fold(String::new(), |mut renditions, rendition| {
                    renditions.push_str(", ");
                    renditions.push_str(&rendition);
                    renditions
                })
            ),
            Function::SHS => format!(
                concat!(
                    "Used to establish the character spacing for subsequent text. {} The established spacing remains ",
                    "in effect until the next occurrence of SHS or of 'Set Character Spacing' (SHS) or of 'Spacing ",
                    "Increment' (SPI)."
                ),
                self.short_description()
            ),
            Function::SIMD => format!(
                concat!(
                    "Used to select the direction of implicit movement of the data position relative to the character ",
                    "position. Remains in effect until the next occurrence of SIMD. {}"
                ),
                self.short_description()
            ),
            Function::SL => format!(
                concat!(
                    "Causes the data in the presentation component to be moved by {} character positions if the line ",
                    "orientation is horizontal, or by {} line positions if the line orientation is vertical, such ",
                    "that the data appear to move to the left. The active presentation position is not affected by ",
                    "this control function."
                ),
                param!(self, 0, 1),
                param!(self, 0, 1)
            ),
            Function::SLH => format!(
                concat!(
                    "If the 'Device Component Select Mode' (DCSM) is set to 'Presentation', this is used to establish ",
                    "at character position {} in the active line (the line that contains the active presentation ",
                    "position) and lines of subsequent text in the presentation component, the position to which the ",
                    "active presentation position will be moved by subsequent occurrences of 'Carriage Return' (CR), ",
                    "'Delete Line' (DL), 'Insert Line' (IL) or 'Next Line' (NEL) in the data stream. In the case of a ",
                    "device without a data component, it is also the position ahead of which no implicit movement of ",
                    "the active presentation position shall occur.",
                    "\n\n",
                    "If the 'Device Component Select Mode' (DCSM) is set to 'Data', this is used to establish at ",
                    "character position {} in the active line (the line that contains the active data position) and ",
                    "lines of subsequent text in the data component, the position to which the active data position ",
                    "will be moved by subsequent occurrences of 'Carriage Return' (CR), 'Delete Line' (DL), 'Insert ",
                    "Line' (IL), or 'Next Line' (NEL) in the data stream. It is also the position ahead of which no ",
                    "implicit movement of the active data position shall occur.",
                    "\n\n",
                    "The established position is called the line home position and remains in effect until the next ",
                    "occurrence of SLH in the data stream."
                ),
                param!(self, 0, 0),
                param!(self, 0, 0)
            ),
            Function::SLL => format!(
                concat!(
                    "If the 'Device Component Select Mode' (DCSM) is set to 'Presentation', this is used to establish ",
                    "at character position {} in the active line (the line that contains the active presentation ",
                    "position) and lines of subsequent text in the presentation component, the position to which the ",
                    "active presentation position will be moved by subsequent occurrences of 'Carriage Return' (CR) ",
                    "or 'Next Line' (NEL) in the data stream, if the parameter value of 'Select Implicit Movement ",
                    "Direction' (SIMD) is equal to 'Opposite'. In the case of a device without data component, it is ",
                    "also the position beyond which no implicit movement of the active presentation position shall ",
                    "occur.",
                    "\n\n",
                    "If the 'Device Component Select Mode' (DSCM) is set to 'Data', this is used to establish at ",
                    "character position {} in the active line (the line that contains the active data position) and ",
                    "lines of subsequent text in the data component, the position beyond which no implicit movement ",
                    "of the active data position shall occur. It is also the position in the data component to which ",
                    "the active data position will be moved by subsequent occurrences of 'Carriage Return' (CR) or ",
                    "'Next Line' (NEL) in the data stream, if the parameter value of 'Select Implicit Movement ",
                    "Direction' (SIMD) is equal to 'Opposite'.",
                    "\n\n",
                    "The established position is called the line limit position and remains in effect until the next ",
                    "occurrence of SLL in the data stream."
                ),
                param!(self, 0, 0),
                param!(self, 0, 0)
            ),
            Function::SLS => format!(
                concat!(
                    "Establishes the line spacing for subsequent text. The established spacing remains in effect ",
                    "until the next occurrence of SLS or of 'Select Line Spacing' (SVS) in the data stream. {}"
                ),
                self.short_description()
            ),
            Function::SM =>
                self.parameters.iter().map(|value| {
                    value.parse::<Mode>().expect("Expect only valid Modes").explain_set()
                }).fold(String::new(), |mut modes, mode| {
                    modes.push_str(", ");
                    modes.push_str(&mode);
                    modes
                }
            ),
            Function::SPH => format!(
                concat!(
                    "If the 'Device Component Select Mode' (DCSM) is set to 'Presentation', this is used to establish ",
                    "at line position {} in the active page (the page that contains the active presentation position) ",
                    "and subsequent pages in the presentation component, the position to which the active ",
                    "presentation position will be moved by subsequent occurrences of 'Form Feed' (FF) in the data ",
                    "stream. In the case of a device without data component, it is also the position ahead of which ",
                    "no implicit movement of the active presentation position shall occur.",
                    "\n\n",
                    "If the 'Device Component Select Mode' (DCSM) is set to 'Data', this is used to establish at line ",
                    "position {} in the active page (the page that contains the active data position) and subsequent ",
                    "pages in the data component, the position to which the active data position will be moved by ",
                    "subsequent occurrences of 'Form Feed' (FF) in the data stream. It is also the position ahead of ",
                    "which no implicit movement of the active presentation position shall occur.",
                    "\n\n",
                    "The established position is called the page home position and remains in effect until the next ",
                    "occurrence of SPH in the data stream."
                ),
                param!(self, 0, 0),
                param!(self, 0, 0)
            ),
            Function::SPI => format!(
                concat!(
                    "Used to establish the line spacing and the character spacing for subsequent text. The ",
                    "established line spacing remains in effect until the next occurrence of SPI or 'Set Line ",
                    "Spacing' (SLS) or of 'Select Line Spacing' (SVS) in the data stream. The established character ",
                    "spacing remains in effect until the next occurrence of 'Set Character Spacing' (SCS) or of ",
                    "'Select Character Spacing' (SHS) in the data stream.",
                    "\n\n",
                    "Line spacing is set to {}, character spacing is set to {}, expressed in the unit that is ",
                    "established by 'Select Size Unit' (SSU)."

                ),
                param!(self, 0, 0),
                param!(self, 1, 0)
            ),
            Function::SPL => format!(
                concat!(
                    "If the 'Device Component Select Mode' (DCSM) is set to 'Presentation', this is used to establish ",
                    "at line position {} in the active page (the page that contains the active presentation position) ",
                    "and pages of subsequent text in the presentation component, the position beyond which the active ",
                    "presentation position can normally not be moved. In the case of a device without data component, ",
                    "it is also the position beyond which no implicit movement of the active presentation position ",
                    "shall occur.",
                    "\n\n",
                    "If the 'Device Component Select Mode' (DCSM) is set to 'Data', this is used to establish at line ",
                    "position {} in the active page (the page that contains the active data position) and pages of ",
                    "subsequent text in the data component, the position beyond which no implicit movement of the ",
                    "active data position shall occur.",
                    "\n\n",
                    "The established position is called the page limit position and remains in effect until the next ",
                    "occurrence of SPL in the data stream."
                ),
                param!(self, 0, 0),
                param!(self, 0, 0)
            ),
            Function::SPQR => format!(
                concat!(
                    "Select the relative print quality and print speed for devices where the output quality and ",
                    "speed are inversely related. The selected value will remain in effect until the next ",
                    "occurrence of SPQR. {}"
                ),
                self.short_description()
            ),
            Function::SR => format!(
                concat!(
                    "Causes the data in the presentation component to be moved by {} character positions if the ",
                    "line orientation is horizontal, or by {} line positions if the line orientation is ",
                    "vertical, such that the data appear to be moved to the right.",
                    "\n\n",
                    "The active presentation position is not affected by this control function."
                ),
                param!(self, 0, 1),
                param!(self, 0, 1)
            ),
            Function::SRCS => format!(
                concat!(
                    "Used to establish reduced inter-character escapement by {} units. The established reduced ",
                    "escapement remains in effect until the next occurrence of SRCS or of 'Set Additional ",
                    "Character Separation' (SACS) in the data stream or until it is reset to the default value ",
                    "by a subsequent occurrence of 'Carriage Return/Line Feed' (CRLF) or of 'Next Line' (NEL) in ",
                    "the data stream.",
                    "\n\n",
                    "The unit in which the escapement is reduced is that established by 'Select Size Unit' (SSU)."
                ),
                param!(self, 0, 0)
            ),
            Function::SRS => format!(
                concat!(
                    "Used to establish in the data component the beginning and the end of a string of ",
                    "characters as well as the direction of this string. This direction is opposite to that ",
                    "currently established. The indicated string follows the preceding text. The established ",
                    "character progression is not affected. {}"
                ),
                self.short_description()
            ),
            Function::SSU => format!(
                concat!(
                    "Used to establish the unit in which the numeric parameters of certain control functions ",
                    "are expressed. The establish unit remains in effect until the next occurrence of SSU in ",
                    "the data stream. {}"
                ),
                self.short_description()
            ),
            Function::SSW => format!(
                concat!(
                    "Used to establish for subsequent text the character escapement associated with the ",
                    "character 'SPACE'. The established escapement remains in effect until the next occurrence ",
                    "of SSW in the data stream or until it is reset to the default value by a subsequent ",
                    "occurrence of 'Carriage Return/Line Feed' (CRLF), 'Carriage Return/Form Feed' (CRFF), or ",
                    "'Next Line' (NEL) in the data stream.",
                    "\n\n",
                    "{}",
                    "\n\n",
                    "The unit in which the value is expressed is defined by 'Select Size Unit' (SSU).",
                    "\n\n",
                    "The default character escapement of 'SPACE' is specified by the most recent occurrence of ",
                    "'Set Character Spacing' (SCS) or of 'Select Character Spacing' (SHS) or of 'Select Spacing ",
                    "Increment' (SPI) in the data stream if the current font has constant spacing, or is ",
                    "specified by the normal width of the character 'SPACE' in the current font if that font ",
                    "has proportional spacing."
                ),
                self.short_description()
            ),
            Function::STAB => format!(
                concat!(
                    "{} The use of this control function and means of specifying a list of tabulation stop to ",
                    "be referenced by the control function are specified in other standards, for example ISO ",
                    "8613-6."
                ),
                self.short_description()
            ),
            Function::SU => format!(
                concat!(
                    "Causes the data in the presentation component to be moved by {} line positions, if the line ",
                    "operation is horizontal, or by {} character positions, if the line orientation is vertical, ",
                    "such that the data appear to move up. The active presentation position is not affected by ",
                    "this control function."
                ),
                param!(self, 0, 1),
                param!(self, 0, 1)
            ),
            Function::SVS => format!(
                concat!(
                    "Used to establish the line spacing for subsequent text. {} The established spacing remains ",
                    "in effect until the next occurrence of SVS or of 'Set Line Spacing' (SLS) or of 'Spacing ",
                    "Increment' (SPI) in the data stream."
                ),
                explain_selection!(LineSpacing, self, 0)
            ),
            Function::TAC => format!(
                concat!(
                    "Causes a character tabulation stop calling for centring to be set at character position {} ",
                    "in the active line (the line that contains the active presentation position) and lines of ",
                    "subsequent text in the presentation component. TAC causes the replacement of any ",
                    "tabulation stop previously set at that character position, but does not affect other ",
                    "tabulation stops.",
                    "\n\n",
                    "A text string centred upon a tabulation stop set by TAC will be positioned so that the ",
                    "(trailing edge of the) first graphic character and the (leading edge of the) last graphic ",
                    "character are at approximately equal distances from the tabulation stop."
                ),
                param!(self, 0, 0)
            ),
            Function::TALE => format!(
                concat!(
                    "Causes a character tabulation stop calling for leading edge alignment to be set at ",
                    "character position {} in the active line (the line that contains the active presentation ",
                    "position) and lines of subsequent text in the presentation component. TALE causes the ",
                    "replacement of any tabulation stop previously set at that character position, but does not ",
                    "affect other tabulation stops.",
                    "\n\n",
                    "A text string aligned with a tabulation stop set by TALE will be positioned so that the ",
                    "(leading edge of the) last graphic character of the string is placed at the tabulation stop."
                ),
                param!(self, 0, 0)
            ),
            Function::TATE => format!(
                concat!(
                    "Causes a character tabulation stop calling for trailing edge alignment to be set at ",
                    "character position {} in the active line (the line that contains the active presentation ",
                    "position) and lines of subsequent text in the presentation component. TATE causes the ",
                    "replacement of any tabulation stop previously set at the character position, but does not ",
                    "affect other tabulation stops.",
                    "\n\n",
                    "A text string aligned with a tabulation stop set by TATE will be positioned so that the ",
                    "(trailing edge of the) first graphic character of the string is placed at the tabulation ",
                    "stop."
                ),
                param!(self, 0, 0)
            ),
            Function::TCC => format!(
                concat!(
                    "Causes a character tabulation stop calling for alignment of a target graphic character {} ",
                    "to be set at character position {} in the active line (the line that contains the active ",
                    "presentation position) and lines of subsequent text in the presentation component. TCC ",
                    "causes the replacement of any tabulation stop previously set at that character position, ",
                    "but does not affect other tabulation stops.",
                    "\n\n",
                    "The positioning of a text string aligned with a tabulation stop set by TCC will be ",
                    "determined by the first occurrence in the string of the target graphic character; that ",
                    "character will be centred upon the tabulation stop. If the target character does not occur ",
                    "within the string, then the trailing edge of the first character of the string will be ",
                    "positioned at the tabulation stop.",
                    "\n\n",
                    "The value of {} indicates the code table position (binary value) of the target character ",
                    "in the currently invoked code. For a 7-bit code, the permissible range of values is 32 ",
                    "to 127; for an 8-bit code, the permissible range of values is 32 to 127 and 160 to 255."
                ),
                param!(self, 1, 32),
                param!(self, 0, 0),
                param!(self, 1, 32)
            ),
            Function::TSS => format!(
                concat!(
                    "Used to establish the width of a thin space for subsequent text to be {} units. The ",
                    "established width remains in effect until the next occurrence of TSS in the data stream.",
                    "\n\n",
                    "The unit in which the parameter is expressed is that established by the value of 'Select ",
                    "Size Unit' (SSU)."
                ),
                param!(self, 0, 0)
            ),
            _ => self.short_description(),
        }
    }
}

impl FromStr for TabulationControl {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "1" => Self::SetLineTabulationStop,
            "2" => Self::ClearCharacterTabulationStop,
            "3" => Self::ClearLineTabulationStop,
            "4" => Self::ClearCharacterTabulationStopsInLine,
            "5" => Self::ClearAllCharacterTabulationStops,
            "6" => Self::ClearLineTabulationStop,
            _ => Self::SetCharacterTabulationStop,
        })
    }
}

impl ExplainSelection for TabulationControl {
    fn explain(&self) -> String {
        match self {
            Self::SetCharacterTabulationStop => {
                String::from("Set a character tabulation at the active position.")
            }
            Self::SetLineTabulationStop => {
                String::from("Set a line tabulation stop at the active line.")
            }
            Self::ClearCharacterTabulationStop => {
                String::from("Clear the character tabulation stop at the active position.")
            }
            Self::ClearLineTabulationStop => {
                String::from("Clear the line tabulation stop at the active line.")
            }
            Self::ClearCharacterTabulationStopsInLine => {
                String::from("Clear all character tabulation stops in the active line.")
            }
            Self::ClearAllCharacterTabulationStops => {
                String::from("Clear all character tabulation stops.")
            }
            Self::ClearAllLineTabulationStops => String::from("Clear all line tabulation stops."),
        }
    }
}

impl FromStr for DeviceAttributes {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "0" => Self::Request,
            value @ _ => Self::Identify(
                value
                    .parse::<u32>()
                    .expect("Expected valid Device Attributes."),
            ),
        })
    }
}

impl ExplainSelection for DeviceAttributes {
    fn explain(&self) -> String {
        match self {
            Self::Request => {
                String::from("Request Device Attribute identification from the receiving device.")
            }
            Self::Identify(v) => {
                format!(
                    "The device sending this identifies as device with code {}.",
                    v
                )
            }
        }
    }
}

impl FromStr for AreaQualification {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "1" => Self::ProtectedGuarded,
            "2" => Self::GraphicCharacterInput,
            "3" => Self::NumericInput,
            "4" => Self::AlphabeticInput,
            "5" => Self::InputAlignedRight,
            "6" => Self::FillZeros,
            "7" => Self::SetCharacterTabulationStop,
            "8" => Self::ProtectedUnguarded,
            "9" => Self::FillSpaces,
            "10" => Self::InputAlignedLeft,
            "11" => Self::Reversed,
            _ => Self::UnprotectedUnguarded,
        })
    }
}

impl ExplainSelection for AreaQualification {
    fn explain(&self) -> String {
        match self {
            Self::UnprotectedUnguarded => String::from("is unprotected an unguarded"),
            Self::ProtectedGuarded => String::from("is protected and guarded"),
            Self::GraphicCharacterInput => String::from("is a graphic input area"),
            Self::NumericInput => String::from("is a numeric input area"),
            Self::AlphabeticInput => String::from("is an alphabetic input area"),
            Self::InputAlignedRight => {
                String::from("has input aligned to the last position of this area")
            }
            Self::FillZeros => String::from("will be filled with ZEROs"),
            Self::SetCharacterTabulationStop => String::from("indicates a beginning of a field"),
            Self::ProtectedUnguarded => String::from("is protected and unguarded"),
            Self::FillSpaces => String::from("will be filled with SPACEs"),
            Self::InputAlignedLeft => {
                String::from("has input aligned to the first position of the area")
            }
            Self::Reversed => {
                String::from("has the order of character positions in the input field reversed.")
            }
        }
    }
}

impl FromStr for DeviceStatusReport {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "1" => Self::BusyRepeat,
            "2" => Self::BusyLater,
            "3" => Self::MalfunctionRepeat,
            "4" => Self::MalfunctionLater,
            "5" => Self::RequestDeviceStatusReport,
            "6" => Self::RequestActivePositionReport,
            _ => Self::Ready,
        })
    }
}

impl ExplainSelection for DeviceStatusReport {
    fn explain(&self) -> String {
        match self {
            Self::Ready => String::from(
                "The sending device reports to be read and no malfunctions have been detected."
            ),
            Self::BusyRepeat => String::from(
                "The sending device is busy. Another Device Status Report must be requested later."
            ),
            Self::BusyLater => String::from(
                "The sending device is busy. Another Device Status Report will be sent later."
            ),
            Self::MalfunctionRepeat => String::from(
                concat!(
                    "Some malfunction has been detected by the sending device. Another Device Status Report must be ",
                    "requested later."
                )
            ),
            Self::MalfunctionLater => String::from(
                concat!(
                    "Some malfunction has been detected by the sending device. Another Device Status Report will ",
                    "be sent later."
                )
            ),
            Self::RequestDeviceStatusReport => String::from(
                "A device status report is requested."
            ),
            Self::RequestActivePositionReport => String::from(
                concat!(
                    "A report of the active presentation position or of the active data position in form of 'Active ",
                    "Position Report' (CPR) is requested from the receiving device."
                )
            )
        }
    }
}

impl FromStr for EraseArea {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "1" => Self::BeginToActivePosition,
            "2" => Self::BeginToEnd,
            _ => Self::ActivePositionToEnd,
        })
    }
}

impl ExplainSelection for EraseArea {
    fn explain(&self) -> String {
        match self {
            Self::ActivePositionToEnd => String::from(
                "erases the contents of the currently active qualified area from the current position to the end"
            ),
            Self::BeginToActivePosition => String::from(
                concat!(
                    "erases the contents of the currently active qualified area from the beginning of format area to ",
                    "the current position"
                )
            ),
            Self::BeginToEnd => String::from(
                "erases all contents of the currently active qualified area"
            ),
        }
    }
}

impl FromStr for ErasePage {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "1" => Self::BeginToActivePosition,
            "2" => Self::BeginToEnd,
            _ => Self::ActivePositionToEnd,
        })
    }
}

impl ExplainSelection for ErasePage {
    fn explain(&self) -> String {
        match self {
            Self::ActivePositionToEnd => String::from(
                "erases the contents of the currently active page from the current position to the end"
            ),
            Self::BeginToActivePosition => String::from(
                concat!(
                    "erases the contents of the currently active page from the beginning of format area to ",
                    "the current position"
                )
            ),
            Self::BeginToEnd => String::from(
                "erases all contents of the currently active page"
            ),
        }
    }
}

impl FromStr for EraseField {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "1" => Self::BeginToActivePosition,
            "2" => Self::BeginToEnd,
            _ => Self::ActivePositionToEnd,
        })
    }
}

impl ExplainSelection for EraseField {
    fn explain(&self) -> String {
        match self {
            Self::ActivePositionToEnd => String::from(
                "erases the contents of the currently active field from the current position to the end"
            ),
            Self::BeginToActivePosition => String::from(
                concat!(
                    "erases the contents of the currently active field from the beginning of format area to ",
                    "the current position"
                )
            ),
            Self::BeginToEnd => String::from(
                "erases all contents of the currently active field"
            ),
        }
    }
}

impl FromStr for EraseLine {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "1" => Self::BeginToActivePosition,
            "2" => Self::BeginToEnd,
            _ => Self::ActivePositionToEnd,
        })
    }
}

impl ExplainSelection for EraseLine {
    fn explain(&self) -> String {
        match self {
            Self::ActivePositionToEnd => String::from(
                "erases the contents of the currently active line from the current position to the end"
            ),
            Self::BeginToActivePosition => String::from(
                concat!(
                    "erases the contents of the currently active line from the beginning of format area to ",
                    "the current position"
                )
            ),
            Self::BeginToEnd => String::from(
                "erases all contents of the currently active line"
            ),
        }
    }
}

impl FromStr for Font {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "1" => Self::Alternative1,
            "2" => Self::Alternative2,
            "3" => Self::Alternative3,
            "4" => Self::Alternative4,
            "5" => Self::Alternative5,
            "6" => Self::Alternative6,
            "7" => Self::Alternative7,
            "8" => Self::Alternative8,
            "9" => Self::Alternative9,
            _ => Self::Primary,
        })
    }
}

impl ExplainSelection for Font {
    fn explain(&self) -> String {
        match self {
            Self::Primary => String::from("primary font"),
            Self::Alternative1 => String::from("alternative font 1"),
            Self::Alternative2 => String::from("alternative font 2"),
            Self::Alternative3 => String::from("alternative font 3"),
            Self::Alternative4 => String::from("alternative font 4"),
            Self::Alternative5 => String::from("alternative font 5"),
            Self::Alternative6 => String::from("alternative font 6"),
            Self::Alternative7 => String::from("alternative font 7"),
            Self::Alternative8 => String::from("alternative font 8"),
            Self::Alternative9 => String::from("alternative font 9"),
        }
    }
}

impl FromStr for GraphicCharacterCombination {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "1" => Self::StartOfCombination,
            "2" => Self::EndOfCombination,
            _ => Self::CombineTwo,
        })
    }
}

impl ExplainSelection for GraphicCharacterCombination {
    fn explain(&self) -> String {
        match self {
            Self::CombineTwo => String::from(
                "Combine the following two graphic characters into a single symbol."
            ),
            Self::StartOfCombination => String::from(
                concat!(
                    "Combine all following graphic characters into a single symbol, until the end of combination of ",
                    "characters is indicated."
                )
            ),
            Self::EndOfCombination => String::from(
                "Indicates the end of combining all previous graphic characters into a single symbol."
            ),
        }
    }
}

impl FromStr for IdentifyDeviceControlString {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "0" => Self::Diagnostic,
            "1" => Self::DynamicallyRedefinableCharacterSet,
            value @ _ => Self::Private(
                value
                    .parse::<u32>()
                    .expect("Expected valid Identify Device Control String."),
            ),
        })
    }
}

impl ExplainSelection for IdentifyDeviceControlString {
    fn explain(&self) -> String {
        match self {
            Self::Diagnostic => String::from(
                concat!(
                    "Subsequent 'Device Control Strings' (DCS) are intended for the diagnostic state of the ",
                    "'Status Report Transfer Mode'"
                )
            ),
            Self::DynamicallyRedefinableCharacterSet => String::from(
                concat!(
                    "Subsequent 'Device Control Strings' (DCS) are reserved for dynamically refinable character sets ",
                    "according to Standard ECMA-35."
                )
            ),
            Self::Private(_) => String::from(
                "Subsequent 'Device Control Strings' (DCS) are for private use."
            ),
        }
    }
}

impl FromStr for Justification {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "1" => Self::WordFill,
            "2" => Self::WordSpace,
            "3" => Self::LetterSpace,
            "4" => Self::Hyphenation,
            "5" => Self::Left,
            "6" => Self::Centre,
            "7" => Self::Right,
            "8" => Self::ItalianHyphenation,
            _ => Self::None,
        })
    }
}

impl ExplainSelection for Justification {
    fn explain(&self) -> String {
        match self {
            Self::None => {
                String::from("The following text is not formatted to a special justification.")
            }
            Self::WordFill => String::from("The following text uses word-fill justification."),
            Self::WordSpace => String::from("The following text uses word-space justification."),
            Self::LetterSpace => {
                String::from("The following text uses letter-space justification.")
            }
            Self::Hyphenation => String::from("The following text uses hyphenation justification."),
            Self::Left => String::from("The following text is left aligned."),
            Self::Centre => String::from("The following text is centred."),
            Self::Right => String::from("The following text is right aligned."),
            Self::ItalianHyphenation => {
                String::from("The following text uses italian hyphenation justification.")
            }
        }
    }
}

impl FromStr for MediaCopy {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "1" => Self::BeginTransferFromPrimary,
            "2" => Self::BeginTransferToSecondary,
            "3" => Self::BeginTransferFromSecondary,
            "4" => Self::StopRelayPrimary,
            "5" => Self::StartRelayPrimary,
            "6" => Self::StopRelaySecondary,
            "7" => Self::StartRelaySecondary,
            _ => Self::BeginTransferToPrimary,
        })
    }
}

impl ExplainSelection for MediaCopy {
    fn explain(&self) -> String {
        match self {
            Self::BeginTransferToPrimary => {
                String::from("Initiate transfer to a primary auxiliary device.")
            }
            Self::BeginTransferFromPrimary => {
                String::from("Initiate transfer from a primary auxiliary device.")
            }
            Self::BeginTransferToSecondary => {
                String::from("Initiate transfer to a secondary auxiliary device.")
            }
            Self::BeginTransferFromSecondary => {
                String::from("Initiate transfer from a secondary auxiliary device.")
            }
            Self::StopRelayPrimary => String::from("Stop relay to a primary auxiliary device."),
            Self::StartRelayPrimary => String::from("Start relay to a primary auxiliary device."),
            Self::StopRelaySecondary => String::from("Stop relay to a secondary auxiliary device."),
            Self::StartRelaySecondary => {
                String::from("Start relay to a secondary auxiliary device.")
            }
        }
    }
}

impl FromStr for PresentationExpandContract {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "1" => Self::Expanded,
            "2" => Self::Condensed,
            _ => Self::Normal,
        })
    }
}

impl ExplainSelection for PresentationExpandContract {
    fn explain(&self) -> String {
        match self {
            Self::Normal => String::from("normal mode, as specified by SCS, SHS or SPI"),
            Self::Expanded => {
                String::from("extended mode, multiplied by a factor not greater than 2")
            }
            Self::Condensed => {
                String::from("condensed mode, multiplied by a factor not less than 0.5")
            }
        }
    }
}

impl FromStr for PageFormat {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "1" => Self::WideBasicText,
            "2" => Self::TallBasicA4,
            "3" => Self::WideBasicA4,
            "4" => Self::TallLetter,
            "5" => Self::WideLetter,
            "6" => Self::TallExtendedA4,
            "7" => Self::WideExtendedA4,
            "8" => Self::TallLegal,
            "9" => Self::WideLegal,
            "10" => Self::A4ShortLines,
            "11" => Self::A4LongLines,
            "12" => Self::B5ShortLines,
            "13" => Self::B5LongLines,
            "14" => Self::B4ShortLines,
            "15" => Self::B4LongLines,
            _ => Self::TallBasicText,
        })
    }
}

impl ExplainSelection for PageFormat {
    fn explain(&self) -> String {
        match self {
            Self::TallBasicText => String::from("Set the page to tall basic communication format."),
            Self::WideBasicText => String::from("Set the page to wide basic communication format."),
            Self::TallBasicA4 => String::from("Set the page to tall basic A4 format."),
            Self::WideBasicA4 => String::from("Set the page to wide basic A4 format."),
            Self::TallLetter => String::from("Set the page to north american tall letter format."),
            Self::WideLetter => String::from("Set the page to north american wide letter format."),
            Self::TallExtendedA4 => String::from("Set the page to tall extended A4 format."),
            Self::WideExtendedA4 => String::from("Set the page to wide extended A4 format."),
            Self::TallLegal => String::from("Set the page to north american tall legal format."),
            Self::WideLegal => String::from("Set the page to north american wide legal format."),
            Self::A4ShortLines => String::from("Set the page to A4 short lines format."),
            Self::A4LongLines => String::from("Set the page to A4 long lines format."),
            Self::B5ShortLines => String::from("Set the page to B5 short lines format."),
            Self::B5LongLines => String::from("Set the page to B5 long lines format."),
            Self::B4ShortLines => String::from("Set the page to B4 short lines format."),
            Self::B4LongLines => String::from("Set the page to B4 long lines format."),
        }
    }
}

impl FromStr for ParallelText {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "1" => Self::BeginPrincipal,
            "2" => Self::BeginSupplementary,
            "3" => Self::BeginJapanesePhonetic,
            "4" => Self::BeginChinesePhonetic,
            "5" => Self::EndPhonetic,
            _ => Self::End,
        })
    }
}

impl ExplainSelection for ParallelText {
    fn explain(&self) -> String {
        match self {
            Self::End => String::from(
                "End of parallel texts."
            ),
            Self::BeginPrincipal => String::from(
                concat!(
                    "Beginning of principal text that should be displayed in parallel with one or more strings of ",
                    "supplementary text."
                )
            ),
            Self::BeginSupplementary => String::from(
                "Beginning of supplementary text that should be displayed in parallel to the principal text."
            ),
            Self::BeginJapanesePhonetic => String::from(
                concat!(
                    "Beginning of supplementary japanese phonetic annotation that should be displayed in parallel to ",
                    "the principal text."
                )
            ),
            Self::BeginChinesePhonetic => String::from(
                concat!(
                    "Beginning of supplementary chinese phonetic annotation that should be displayed in parallel to ",
                    "the principal text."
                )
            ),
            Self::EndPhonetic => String::from(
                "End of a string of supplementary phonetic annotations."
            ),
        }
    }
}

impl FromStr for Alignment {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "1" => Self::LineHomeLeader,
            "2" => Self::Centre,
            "3" => Self::CentreLeader,
            "4" => Self::LineLimit,
            "5" => Self::LineLimitLeader,
            "6" => Self::Justify,
            _ => Self::LineHome,
        })
    }
}

impl ExplainSelection for Alignment {
    fn explain(&self) -> String {
        match self {
            Self::LineHome => String::from(
                "flush to the line home position"
            ),
            Self::LineHomeLeader => String::from(
                "flush to the line home position, margin and fill with leader"
            ),
            Self::Centre => String::from(
                "centred between line home position and line limit position margins"
            ),
            Self::CentreLeader => String::from(
                "centred between line home position and line limit position margins and fill with leader"
            ),
            Self::LineLimit => String::from(
                "flush to the line limit position margin"
            ),
            Self::LineLimitLeader => String::from(
                "flush to the line limit position margin and fill with leader"
            ),
            Self::Justify => String::from(
                "flush to both margins"
            ),
        }
    }
}

impl FromStr for Mode {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "2" => Self::KeyboardActionMode,
            "3" => Self::ControlPresentationMode,
            "4" => Self::InsertionReplacementMode,
            "5" => Self::StatusReportTransferMode,
            "6" => Self::ErasureMode,
            "7" => Self::LineEditingMode,
            "8" => Self::BiDirectionalSupportMode,
            "9" => Self::DeviceComponentSelectMode,
            "10" => Self::CharacterEditingMode,
            "11" => Self::PositioningUnitMode,
            "12" => Self::SendReceiveMode,
            "13" => Self::FormatEffectorActionMode,
            "14" => Self::FormatEffectorTransferMode,
            "15" => Self::MultipleAreaTransferMode,
            "16" => Self::TransferTerminationMode,
            "17" => Self::StatusReportTransferMode,
            "18" => Self::TabulationStopMode,
            "21" => Self::GraphicRenditionCombinationMode,
            "22" => Self::ZeroDefaultMode,
            _ => Self::GuardedAreaTransferMode,
        })
    }
}

impl ExplainMode for Mode {
    fn name(&self) -> String {
        match self {
            Self::GuardedAreaTransferMode => String::from("Guarded Area Transfer Mode"),
            Self::KeyboardActionMode => String::from("Keyboard Action Mode"),
            Self::ControlPresentationMode => String::from("Control Presentation Mode"),
            Self::InsertionReplacementMode => String::from("Insertion Replacement Mode"),
            Self::StatusReportTransferMode => String::from("Status Report Transfer Mode"),
            Self::ErasureMode => String::from("Erasure Mode"),
            Self::LineEditingMode => String::from("Line Editing Mode"),
            Self::BiDirectionalSupportMode => String::from("Bi-Directional Support Mode"),
            Self::DeviceComponentSelectMode => String::from("Device Component Select Mode"),
            Self::CharacterEditingMode => String::from("Character Editing Mode"),
            Self::PositioningUnitMode => String::from("Positioning Unit Mode"),
            Self::SendReceiveMode => String::from("Send Receive Mode"),
            Self::FormatEffectorActionMode => String::from("Format Effector Action Mode"),
            Self::FormatEffectorTransferMode => String::from("Format Effector Transfer Mode"),
            Self::MultipleAreaTransferMode => String::from("Multiple Area Transfer mode"),
            Self::TransferTerminationMode => String::from("Transfer Termination Mode"),
            Self::SelectedAreaTransferMode => String::from("Selected Area Transfer Mode"),
            Self::TabulationStopMode => String::from("Tabulation Stop Mode"),
            Self::GraphicRenditionCombinationMode => {
                String::from("Graphic Rendition Combination Mode")
            }
            Self::ZeroDefaultMode => String::from("Zero Default Mode"),
        }
    }

    fn explain_reset(&self) -> String {
        match self {
            Self::GuardedAreaTransferMode => String::from(
                "Only the contents of unguarded areas in an eligible area are transmitted or transferred."
            ),
            Self::KeyboardActionMode => String::from(
                "All or part of the manual input facilities are enabled to be used."
            ),
            Self::ControlPresentationMode => String::from(
                "All control functions are performed as defined."
            ),
            Self::InsertionReplacementMode => String::from(
                concat!(
                    "The graphic symbol of a graphic character or a control function, for which a graphical ",
                    "representation is required, replaces (or, depending on the implementation, is combined with) the ",
                    "graphic symbol imaged at the active presentation position"
                )
            ),
            Self::StatusReportTransferMode => String::from(
                "Status reports in the form of 'Device Control String' (DCS) are not generated automatically."
            ),
            Self::ErasureMode => String::from(
                "Only the contents of unprotected areas are affected by an erasure control function."
            ),
            Self::LineEditingMode => String::from(
                concat!(
                    "The insertion of a line causes the contents of the active line and the following lines to be ",
                    "shifted in the direction of line progression. A line deletion causes the contents of the ",
                    "following lines to shifted in the opposite direction of line progression."
                )
            ),
            Self::BiDirectionalSupportMode => String::from(
                "Control functions are performed in the data component or the presentation component."
            ),
            Self::DeviceComponentSelectMode => String::from(
                "Certain control functions are performed in the presentation component at the current position."
            ),
            Self::CharacterEditingMode => String::from(
                concat!(
                    "The insertion of a character causes the following contents to be shifted in the direction of ",
                    "character progression. A character deletion causes the following contents to be shifted in the ",
                    "direction opposite of character progression."
                )
            ),
            Self::PositioningUnitMode => String::from(
                "The unit for numeric parameters of the position format effectors is one character position."
            ),
            Self::SendReceiveMode => String::from(
                "Data which are locally entered are immediately imaged."
            ),
            Self::FormatEffectorActionMode => String::from(
                "Formator functions are performed immediately and may be stored in addition to being performed."
            ),
            Self::FormatEffectorTransferMode => String::from(
                concat!(
                    "Formator functions may be inserted in a data stream to be transmitted or in data to be ",
                    "transferred to an auxiliary input/output device."
                )
            ),
            Self::MultipleAreaTransferMode => String::from(
                concat!(
                    "Only the contents of the selected area which contains the active presentation position are ",
                    "eligible to be transmitted or transferred."
                )
            ),
            Self::TransferTerminationMode => String::from(
                concat!(
                    "Only the contents of the character positions preceding the active presentation position in the ",
                    "presentation component are eligible to be transmitted or transferred."
                )
            ),
            Self::SelectedAreaTransferMode => String::from(
                "Only the contents of selected areas are eligible to be transmitted or transferred."
            ),
            Self::TabulationStopMode => String::from(
                concat!(
                    "Character tabulation stops in the presentation component are set or cleared in the active line ",
                    "and in the corresponding character positions of the preceding lines and the following lines."
                )
            ),
            Self::GraphicRenditionCombinationMode => String::from(
                concat!(
                    "Each occurrence of the control function 'Select Graphic Rendition' (SGR) cancels the effect of ",
                    "any preceding occurrence."
                )
            ),
            Self::ZeroDefaultMode => String::from(
                "A parameter value of 0 of a control functions means the number 0."
            ),
        }
    }

    fn explain_set(&self) -> String {
        match self {
            Self::GuardedAreaTransferMode => String::from(
                concat!(
                    "The contents of guarded as well as of unguarded areas in an eligible area are transmitted or ",
                    "transferred."
                )
            ),
            Self::KeyboardActionMode => String::from(
                "All or part of the manual input facilities are disabled."
            ),
            Self::ControlPresentationMode => String::from(
                "All control functions, except 'Reset Mode' are treated as graphic characters."
            ),
            Self::InsertionReplacementMode => String::from(
                concat!(
                    "The graphic symbol of a graphic character or a control function, for which a graphical ",
                    "representation is required,is inserted at the active presentation position."
                )
            ),
            Self::StatusReportTransferMode => String::from(
                concat!(
                    "Status reports in the form of 'Device Control String' (DCS) are included in every data stream ",
                    "transmitted or transferred."
                )
            ),
            Self::ErasureMode => String::from(
                "Only the contents of protected as well as protected areas are affected by an erasure control function."
            ),
            Self::LineEditingMode => String::from(
                concat!(
                    "The insertion of a line causes the contents of the active line and the following lines to be ",
                    "shifted in the direction of line progression. A line deletion causes the contents of the ",
                    "following lines to shifted in the opposite direction of line progression."
                )
            ),
            Self::BiDirectionalSupportMode => String::from(
                concat!(
                    "Control functions are performed in the data component. All bi-directional aspects of data are ",
                    "handled by the device itself."
                )
            ),
            Self::DeviceComponentSelectMode => String::from(
                "Certain control functions are performed in the data component at the current position."
            ),
            Self::CharacterEditingMode => String::from(
                concat!(
                    "The insertion of a character causes the following contents to be shifted in the direction ",
                    "opposite of character progression. A character deletion causes the following contents to be ",
                    "shifted in the direction of character progression."
                )
            ),
            Self::PositioningUnitMode => String::from(
                concat!(
                    "The unit for numeric parameters of the position format effectors is that established by 'Select ",
                    "Size Unit' (SSU)."
                )
            ),
            Self::SendReceiveMode => String::from(
                "Local input facilities are logically disconnected from the output mechanism."
            ),
            Self::FormatEffectorActionMode => String::from(
                "Formator functions are stored but not performed."
            ),
            Self::FormatEffectorTransferMode => String::from(
                concat!(
                    "No formator functions other than those received while the 'Format Effector Action Mode' (FEAM) ",
                    "is set to 'Store' are included in a transmitted data stream."
                )
            ),
            Self::MultipleAreaTransferMode => String::from(
                "The contents of all selected areas are eligible to be transmitted or transferred."
            ),
            Self::TransferTerminationMode => String::from(
                concat!(
                    "The contents of character positions preceding, following, and at the active position are ",
                    "eligible to be transmitted or transferred."
                )
            ),
            Self::SelectedAreaTransferMode => String::from(
                concat!(
                    "The contents of all character positions, irrespective of any explicitly defined selected areas, ",
                    "are eligible to be transmitted or transferred."
                )
            ),
            Self::TabulationStopMode => String::from(
                "Character tabulation stops in the presentation component are set or cleared in the active line only."
            ),
            Self::GraphicRenditionCombinationMode => String::from(
                concat!(
                    "Each occurrence of the control function 'Select Graphic Rendition' (SGR) cancels only those ",
                    "graphic rendition aspects to be changed that are specified by that SGR. All other graphic ",
                    "rendition aspects remain unchanged."
                )
            ),
            Self::ZeroDefaultMode => String::from(
                "A parameter value of 0 of a control functions means a default value that might be different from 0."
            ),
        }
    }
}

impl FromStr for PresentationVariant {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "1" => Self::LatinDecimals,
            "2" => Self::ArabicDecimals,
            "3" => Self::MirrorPairs,
            "4" => Self::MirrorFormulae,
            "5" => Self::Isolated,
            "6" => Self::Initial,
            "7" => Self::Medial,
            "8" => Self::Final,
            "9" => Self::DecimalFullStop,
            "10" => Self::DecimalComma,
            "11" => Self::VowelAboveOrBelow,
            "12" => Self::VowelAfterPreceding,
            "13" => Self::ContextualShapeArabicScriptWithLamAleph,
            "14" => Self::ContextualShapeArabicScript,
            "15" => Self::NoMirroring,
            "16" => Self::NoVowels,
            "17" => Self::SlantFollowsStringDirection,
            "18" => Self::NoContextualShapeArabicScript,
            "19" => Self::NoContextualShapeArabicScriptExceptDigits,
            "20" => Self::DeviceDependentDecimalDigits,
            "21" => Self::PersistCharacterForm,
            "22" => Self::DesistCharacterForm,
            _ => Self::Default,
        })
    }
}

impl ExplainSelection for PresentationVariant {
    fn explain(&self) -> String {
        match self {
            Self::Default => String::from(
                "Default presentation. Cancels the effect of any other preceding SAPV."
            ),
            Self::LatinDecimals => String::from(
                "The decimal digits are presented by means of the graphic symbols used in the Latin script."
            ),
            Self::ArabicDecimals => String::from(
                concat!(
                    "The decimal digits are presented by means of the graphic symbols used in the Arabic script, i.e. ",
                    "the Hindi symbols."
                )
            ),
            Self::MirrorPairs => String::from(
                concat!(
                    "When the direction of the character path is right-to-left, each of the graphic characters in the ",
                    "character set(s) in use which is one of a left/right handed pair (parenthesis, square brackets, ",
                    "curly brackets, greater-than/less-than signs, etc.) is presented as mirrored"
                )
            ),
            Self::MirrorFormulae => String::from(
                concat!(
                    "When the direction of the character path is right-to-left, all graphic characters which ",
                    "represent operators and delimiters in mathematical formulae and which are not symmetrical about ",
                    "a vertical axis are presented as mirrored about that vertical axis."
                )
            ),
            Self::Isolated => String::from(
                "The following graphic character is presented in its isolated form."
            ),
            Self::Initial => String::from(
                "The following graphic character is presented in its initial form."
            ),
            Self::Medial => String::from(
                "The following graphic character is presented in its medial form."
            ),
            Self::Final => String::from(
                "The following graphic character is presented in its final form."
            ),
            Self::DecimalFullStop => String::from(
                concat!(
                    "Where the bit combination 02/14 (FULL STOP) is intended to represent a decimal mark in a decimal ",
                    "number it shall be represented by means of the graphic symbol FULL STOP."
                )
            ),
            Self::DecimalComma => String::from(
                concat!(
                    "Where the bit combination 02/14 (FULL STOP) is intended to represent a decimal mark in a decimal ",
                    "number it shall be presented by means of the graphic symbol COMMA."
                )
            ),
            Self::VowelAboveOrBelow => String::from(
                "Vowels are presented above or below the preceding character."
            ),
            Self::VowelAfterPreceding => String::from(
                "Vowels are presented after the preceding character."
            ),
            Self::ContextualShapeArabicScriptWithLamAleph => String::from(
                concat!(
                    "Contextual shap determination of Arabic scripts, including the LAM-ALEPH ligature but excluding ",
                    "all other Arabic ligatures."
                )
            ),
            Self::ContextualShapeArabicScript => String::from(
                "Contextual shape determination of Arabic scripts, excluding all Arabic ligatures."
            ),
            Self::NoMirroring => String::from(
                "Cancels the effect of mirroring settings."
            ),
            Self::NoVowels => String::from(
                "Vowels are not presented."
            ),
            Self::SlantFollowsStringDirection => String::from(
                concat!(
                    "When the string direction is right-to-left, the italicized characters are slanted to the left, ",
                    "when the string direction is left-to-right, the italicized characters are slanted to the left."
                )
            ),
            Self::NoContextualShapeArabicScript => String::from(
                concat!(
                    "Contextual shape determination of Arabic scripts is not used, the graphic characters - including ",
                    "the digits - are presented in the form they are stored (pass-through)."
                )
            ),
            Self::NoContextualShapeArabicScriptExceptDigits => String::from(
                concat!(
                    "Contextual shape determination of Arabic scripts is not used, the graphic characters - excluding ",
                    "the digits - are presented in the form they are stored (pass-through)."
                )
            ),
            Self::DeviceDependentDecimalDigits => String::from(
                "The graphic symbols used to present the decimal digits are device dependent."
            ),
            Self::PersistCharacterForm => String::from(
                concat!(
                    "Establishes the effect of parameter values 'Isolated', 'Initial, 'Medial', and 'Final' for the ",
                    "following graphic characters until cancelled."
                )
            ),
            Self::DesistCharacterForm => String::from(
                concat!(
                    "Establishes the effect of parameter values 'Isolated', 'Initial', 'Medial', and 'Final' for the ",
                    "next single graphic character only."
                )
            ),
        }
    }
}

impl FromStr for CharacterOrientation {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "1" => Self::Rotate45,
            "2" => Self::Rotate90,
            "3" => Self::Rotate135,
            "4" => Self::Rotate180,
            "5" => Self::Rotate225,
            "6" => Self::Rotate270,
            "7" => Self::Rotate315,
            _ => Self::Normal,
        })
    }
}

impl ExplainSelection for CharacterOrientation {
    fn explain(&self) -> String {
        match self {
            Self::Normal => String::from("Rotate by 0°."),
            Self::Rotate45 => String::from("Rotate by 45°."),
            Self::Rotate90 => String::from("Rotate by 90°."),
            Self::Rotate135 => String::from("Rotate by 135°."),
            Self::Rotate180 => String::from("Rotate by 180°."),
            Self::Rotate225 => String::from("Rotate by 225°."),
            Self::Rotate270 => String::from("Rotate by 270°."),
            Self::Rotate315 => String::from("Rotate by 315°."),
        }
    }
}

impl FromStr for CharacterPath {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "2" => Self::RightToLeft,
            _ => Self::LefToRight,
        })
    }
}

impl ExplainSelection for CharacterPath {
    fn explain(&self) -> String {
        match self {
            Self::LefToRight => String::from("Left-to-right, or top-to-bottom."),
            Self::RightToLeft => String::from("Right-to-left, or bottom-to-top."),
        }
    }
}

impl FromStr for CharacterPathScope {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "1" => Self::InPresentationComponent,
            "2" => Self::InDataComponent,
            _ => Self::Undefined,
        })
    }
}

impl ExplainSelection for CharacterPathScope {
    fn explain(&self) -> String {
        match self {
            CharacterPathScope::Undefined => String::from(
                "The scope of the new character path is undefined."
            ),
            CharacterPathScope::InPresentationComponent => String::from(
                concat!(
                    "The content of the active line in the presentation component is updated to correspond to the ",
                    "content of the active line in the data component according to the newly established character ",
                    "path characteristics in the presentation component."
                )
            ),
            CharacterPathScope::InDataComponent => String::from(
                concat!(
                    "The content of the active line in the data component is updated to correspond to the content of ",
                    "the active line in the presentation component according to the newly established character path ",
                    "characteristics in the presentation component."
                )
            ),
        }
    }
}

impl FromStr for StringDirection {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "1" => Self::StartLeftToRight,
            "2" => Self::StartRightToLeft,
            _ => Self::End,
        })
    }
}

impl ExplainSelection for StringDirection {
    fn explain(&self) -> String {
        match self {
            Self::End => {
                String::from("End of a directed string - re-establish the previous direction.")
            }
            Self::StartLeftToRight => {
                String::from("Start of a directed string, establish the direction left-to-right.")
            }
            Self::StartRightToLeft => {
                String::from("Start of a directed string, establish the direction right-to-left.")
            }
        }
    }
}

impl FromStr for EditingExtend {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "1" => Self::ActiveLine,
            "2" => Self::ActiveField,
            "3" => Self::QualifiedArea,
            "4" => Self::All,
            _ => Self::ActivePage,
        })
    }
}

impl ExplainSelection for EditingExtend {
    fn explain(&self) -> String {
        match self {
            Self::ActivePage => String::from("the shifted part is limited to the active page"),
            Self::ActiveLine => String::from("the shifted part is limited to the active line"),
            Self::ActiveField => String::from("the shifted part is limited to the active field"),
            Self::QualifiedArea => {
                String::from("the shifted part is limited to the active qualified area")
            }
            Self::All => String::from("the shifted part is not limited"),
        }
    }
}

impl FromStr for Load {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "0" => Self::None,
            value @ _ => Self::Bin(
                value
                    .parse::<u32>()
                    .expect("Expected valid value for Load directive"),
            ),
        })
    }
}

impl ExplainSelection for Load {
    fn explain(&self) -> String {
        match self {
            Self::None => String::from("Eject sheet, no new sheet loaded."),
            Self::Bin(bin) => format!("Eject sheet, load a new sheet from bin {}.", bin),
        }
    }
}

impl FromStr for Stack {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "0" => Self::None,
            value @ _ => Self::Stacker(
                value
                    .parse::<u32>()
                    .expect("Expected valid value for Load directive"),
            ),
        })
    }
}

impl ExplainSelection for Stack {
    fn explain(&self) -> String {
        match self {
            Self::None => String::from("Eject sheet, no stacker specified."),
            Self::Stacker(stacker) => format!("Eject sheet into the stacker {}.", stacker),
        }
    }
}

impl FromStr for GraphicRendition {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "1" => Self::HighIntensity,
            "2" => Self::LowIntensity,
            "3" => Self::Italicized,
            "4" => Self::Underlined,
            "5" => Self::SlowlyBlinking,
            "6" => Self::RapidlyBlinking,
            "7" => Self::Negative,
            "8" => Self::Concealed,
            "9" => Self::CrossedOut,
            "10" => Self::PrimaryFont,
            "11" => Self::FirstAlternativeFont,
            "12" => Self::SecondAlternativeFont,
            "13" => Self::ThirdAlternativeFont,
            "14" => Self::ForthAlternativeFont,
            "15" => Self::FifthAlternativeFont,
            "16" => Self::SixthAlternativeFont,
            "17" => Self::SeventhAlternativeFont,
            "18" => Self::EighthAlternativeFont,
            "19" => Self::NinthAlternativeFont,
            "20" => Self::Fraktur,
            "21" => Self::DoublyUnderlined,
            "22" => Self::NormalIntensity,
            "23" => Self::NormalStyle,
            "24" => Self::NotUnderlined,
            "25" => Self::NotBlinking,
            "27" => Self::Positive,
            "28" => Self::Revealed,
            "29" => Self::NotCrossedOut,
            "30" => Self::BlackForeground,
            "31" => Self::RedForeground,
            "32" => Self::GreenForeground,
            "33" => Self::YellowForeground,
            "34" => Self::BlueForeground,
            "35" => Self::MagentaForeground,
            "36" => Self::CyanForeground,
            "37" => Self::WhiteForeground,
            "39" => Self::DefaultForeground,
            "40" => Self::BlackBackground,
            "41" => Self::RedBackground,
            "42" => Self::GreenBackground,
            "43" => Self::YellowBackground,
            "44" => Self::BlueBackground,
            "45" => Self::MagentaBackground,
            "46" => Self::CyanBackground,
            "47" => Self::WhiteBackground,
            "49" => Self::DefaultBackground,
            "51" => Self::Framed,
            "52" => Self::Encircled,
            "53" => Self::Overlined,
            "54" => Self::NotFramed,
            "55" => Self::NotOverlined,
            "60" => Self::IdeogramUnderline,
            "61" => Self::IdeogramUnderline,
            "62" => Self::IdeogramStressMarking,
            "63" => Self::CancelIdeogramRendition,
            _ => Self::Default,
        })
    }
}

impl ExplainSelection for GraphicRendition {
    fn explain(&self) -> String {
        match self {
            Self::Default => String::from("Default rendition, cancel all effects."),
            Self::HighIntensity => String::from("Bold or increased intensity."),
            Self::LowIntensity => String::from("Faint, decreased intensity or second color."),
            Self::Italicized => String::from("Italicized."),
            Self::Underlined => String::from("Singly underlined."),
            Self::SlowlyBlinking => String::from("Slowly blinking (less than 150 per minute)."),
            Self::RapidlyBlinking => String::from("Rapidly blinking (more than 150 per minute)."),
            Self::Negative => String::from("Negative image."),
            Self::Concealed => String::from("Concealed characters."),
            Self::CrossedOut => {
                String::from("Crossed-out (characters still legible but marked as to be deleted).")
            }
            Self::PrimaryFont => String::from("Primary (default) font."),
            Self::FirstAlternativeFont => String::from("First alternative font."),
            Self::SecondAlternativeFont => String::from("Second alternative font."),
            Self::ThirdAlternativeFont => String::from("Third alternative font."),
            Self::ForthAlternativeFont => String::from("Forth alternative font."),
            Self::FifthAlternativeFont => String::from("Fifth alternative font."),
            Self::SixthAlternativeFont => String::from("Sixth alternative font."),
            Self::SeventhAlternativeFont => String::from("Seventh alternative font."),
            Self::EighthAlternativeFont => String::from("Eighth alternative font."),
            Self::NinthAlternativeFont => String::from("Ninth alternative font."),
            Self::Fraktur => String::from("Fraktur (Gothic)."),
            Self::DoublyUnderlined => String::from("Doubly underlined."),
            Self::NormalIntensity => String::from("Normal intensity or normal color."),
            Self::NormalStyle => String::from("Normal style, not italicized, not fraktur."),
            Self::NotUnderlined => String::from("Not underlined."),
            Self::NotBlinking => String::from("Not blinking."),
            Self::Positive => String::from("Positive image."),
            Self::Revealed => String::from("Revealed characters."),
            Self::NotCrossedOut => String::from("Not crossed out."),
            Self::BlackForeground => String::from("Black foreground color."),
            Self::RedForeground => String::from("Red foreground color."),
            Self::GreenForeground => String::from("Green foreground color."),
            Self::YellowForeground => String::from("Yellow foreground color."),
            Self::BlueForeground => String::from("Blue foreground color."),
            Self::MagentaForeground => String::from("Magenta foreground color."),
            Self::CyanForeground => String::from("Cyan foreground color."),
            Self::WhiteForeground => String::from("White foreground color."),
            Self::DefaultForeground => String::from("Default foreground color."),
            Self::BlackBackground => String::from("Black background color."),
            Self::RedBackground => String::from("Red background color."),
            Self::GreenBackground => String::from("Green background color."),
            Self::YellowBackground => String::from("Yellow background color."),
            Self::BlueBackground => String::from("Blue background color."),
            Self::MagentaBackground => String::from("Magenta background color."),
            Self::CyanBackground => String::from("Cyan background color."),
            Self::WhiteBackground => String::from("White background color."),
            Self::DefaultBackground => String::from("Default background color."),
            Self::Framed => String::from("Framed."),
            Self::Encircled => String::from("Encircled."),
            Self::Overlined => String::from("Overlined."),
            Self::NotFramed => String::from("Not Framed."),
            Self::NotOverlined => String::from("Not Overlined."),
            Self::IdeogramUnderline => String::from("Ideogram underline or right side line."),
            Self::IdeogramDoubleUnderline => {
                String::from("Ideogram double underline or double line on the right side.")
            }
            Self::IdeogramStressMarking => String::from("Ideogram stress marking."),
            Self::CancelIdeogramRendition => String::from("Cancel Ideogram rendition settings."),
        }
    }
}

impl FromStr for CharacterSpacing {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "1" => Self::TwelveCharacters,
            "2" => Self::FifteenCharacters,
            "3" => Self::SixCharacters,
            "4" => Self::ThreeCharacters,
            "5" => Self::NineCharacters,
            "6" => Self::FourCharacters,
            _ => Self::TenCharacters,
        })
    }
}

impl ExplainSelection for CharacterSpacing {
    fn explain(&self) -> String {
        match self {
            Self::TenCharacters => {
                String::from("Set character spacing to 10 characters per 25.4mm.")
            }
            Self::TwelveCharacters => {
                String::from("Set character spacing to 12 characters per 25.4mm.")
            }
            Self::FifteenCharacters => {
                String::from("Set character spacing to 15 characters per 25.4mm.")
            }
            Self::SixCharacters => {
                String::from("Set character spacing to 6 characters per 25.4mm.")
            }
            Self::ThreeCharacters => {
                String::from("Set character spacing to 3 characters per 25.4mm.")
            }
            Self::NineCharacters => {
                String::from("Set character spacing to 9 characters per 25.4mm.")
            }
            Self::FourCharacters => {
                String::from("Set character spacing to 4 characters per 25.4mm.")
            }
        }
    }
}

impl FromStr for MovementDirection {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "1" => Self::Opposite,
            _ => Self::Normal,
        })
    }
}

impl ExplainSelection for MovementDirection {
    fn explain(&self) -> String {
        match self {
            Self::Normal => String::from(
                "Implicit movement is in the same direction as that of character progression.",
            ),
            Self::Opposite => String::from(
                "Implicit movement is in the opposite direction as that of character progression.",
            ),
        }
    }
}

impl FromStr for PresentationDirection {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "1" => Self::VerticalLinesRightToLeftTopToBottom,
            "2" => Self::VerticalLinesLeftToRightTopToBottom,
            "3" => Self::HorizontalLinesTopToBottomRightToLeft,
            "4" => Self::VerticalLinesLeftToRightBottomToTop,
            "5" => Self::HorizontalLinesBottomToTopRightToLeft,
            "6" => Self::HorizontalLinesBottomToTopLefToRight,
            "7" => Self::VerticalLinesRightToLeftBottomToTop,
            _ => Self::HorizontalLinesTopToBottomLeftToRight,
        })
    }
}

impl ExplainSelection for PresentationDirection {
    fn explain(&self) -> String {
        match self {
            Self::HorizontalLinesTopToBottomLeftToRight => String::from(
                "horizontal line orientation, top-to-bottom line progression, left-to-right character path"
            ),
            Self::VerticalLinesRightToLeftTopToBottom => String::from(
                "vertical line orientation, right-to-left line progression, top-to-bottom character path"
            ),
            Self::VerticalLinesLeftToRightTopToBottom => String::from(
                "vertical line orientation, left-to-right line progression, top-to-bottom character path"
            ),
            Self::HorizontalLinesTopToBottomRightToLeft => String::from(
                "horizontal line orientation, top-to-bottom line progression, right-to-left character path"
            ),
            Self::VerticalLinesLeftToRightBottomToTop => String::from(
                "vertical line orientation, left-to-right line progression, bottom-to-top character path"
            ),
            Self::HorizontalLinesBottomToTopRightToLeft => String::from(
                "horizontal line orientation, bottom-to-top line progression, right-to-left character path"
            ),
            Self::HorizontalLinesBottomToTopLefToRight => String::from(
                "horizontal line orientation, bottom-to-top line progression, left-to-right character path"
            ),
            Self::VerticalLinesRightToLeftBottomToTop => String::from(
                "vertical line orientation, right to left line progression, bottom-to-top character path"
            ),
        }
    }
}

impl FromStr for PresentationDirectionScope {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "1" => Self::InPresentationComponent,
            "2" => Self::InDataComponent,
            _ => Self::Undefined,
        })
    }
}

impl ExplainSelection for PresentationDirectionScope {
    fn explain(&self) -> String {
        match self {
            Self::Undefined => String::from("an undefined scope"),
            Self::InPresentationComponent => String::from("the presentation component"),
            Self::InDataComponent => String::from("the data component"),
        }
    }
}

impl FromStr for PrintQuality {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "1" => Self::MediumQualityMediumSpeed,
            "2" => Self::LowQualityHighSpeed,
            _ => Self::HighQualityLowSpeed,
        })
    }
}

impl ExplainSelection for PrintQuality {
    fn explain(&self) -> String {
        match self {
            Self::HighQualityLowSpeed => String::from("Print in high quality with low speed."),
            Self::MediumQualityMediumSpeed => {
                String::from("Print in medium quality with medium speed.")
            }
            Self::LowQualityHighSpeed => String::from("Print in low quality with high speed."),
        }
    }
}

impl FromStr for ReversedString {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "1" => Self::Start,
            _ => Self::End,
        })
    }
}

impl ExplainSelection for ReversedString {
    fn explain(&self) -> String {
        match self {
            Self::End => {
                String::from("End of a reversed string; re-establish the previous direction.")
            }
            Self::Start => String::from("Beginning of a reversed string; reverse the direction."),
        }
    }
}

impl FromStr for SizeUnit {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "1" => Self::Millimetre,
            "2" => Self::ComputerDecipoint,
            "3" => Self::Decidot,
            "4" => Self::Mil,
            "5" => Self::BasicMeasuringUnit,
            "6" => Self::Micrometer,
            "7" => Self::Pixel,
            "8" => Self::Decipoint,
            _ => Self::Character,
        })
    }
}

impl ExplainSelection for SizeUnit {
    fn explain(&self) -> String {
        match self {
            Self::Character => {
                String::from("Character. The dimension of this unit is device-dependent.")
            }
            Self::Millimetre => String::from("Millimetre."),
            Self::ComputerDecipoint => {
                String::from("Computer decipoint (0.03528 mm - 1/720 of 25.4 mm).")
            }
            Self::Decidot => String::from("Decidot (0.03759 mm - 10/266 mm)."),
            Self::Mil => String::from("Mil (0.0254 mm - 1/1000 of 25.4 mm)."),
            Self::BasicMeasuringUnit => {
                String::from("Basic Measuring Unit (BMU) (0.02117 mm - 1/1200 of 25.4 mm).")
            }
            Self::Micrometer => String::from("Micrometer (0.001 mm)"),
            Self::Pixel => {
                String::from("Pixel, the smallest increment that can be specified in the device.")
            }
            Self::Decipoint => String::from("Decipoint (0.03514mm - 35/996 mm)."),
        }
    }
}

impl FromStr for LineSpacing {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "1" => Self::FourLinesPer25,
            "2" => Self::ThreeLinesPer25,
            "3" => Self::TwelveLinesPer25,
            "4" => Self::EightLinesPer25,
            "5" => Self::SixLinesPer30,
            "6" => Self::FourLinesPer30,
            "7" => Self::ThreeLinesPer30,
            "8" => Self::TwelveLinesPer30,
            "9" => Self::TwoLinesPer25,
            _ => Self::SixLinesPer25,
        })
    }
}

impl ExplainSelection for LineSpacing {
    fn explain(&self) -> String {
        match self {
            Self::SixLinesPer25 => String::from("Set line spacing to 6 lines per 25 mm."),
            Self::FourLinesPer25 => String::from("Set line spacing to 4 lines per 25 mm."),
            Self::ThreeLinesPer25 => String::from("Set line spacing to 3 lines per 25 mm."),
            Self::TwelveLinesPer25 => String::from("Set line spacing to 12 lines per 25 mm."),
            Self::EightLinesPer25 => String::from("Set line spacing to 8 lines per 25 mm."),
            Self::SixLinesPer30 => String::from("Set line spacing to 6 lines per 30 mm."),
            Self::FourLinesPer30 => String::from("Set line spacing to 4 lines per 30 mm."),
            Self::ThreeLinesPer30 => String::from("Set line spacing to 3 lines per 30 mm."),
            Self::TwelveLinesPer30 => String::from("Set line spacing to 12 lines per 30 mm."),
            Self::TwoLinesPer25 => String::from("Set line spacing to 2 lines per 25 mm."),
        }
    }
}

impl FromStr for ClearTabulation {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "1" => Self::LineTabulationStopActiveLine,
            "2" => Self::AllCharacterTabulationStopsActiveLine,
            "3" => Self::AllCharacterTabulationStops,
            "4" => Self::AllTabulationStops,
            "5" => Self::AllTabulationStops,
            _ => Self::CharacterTabulationStopActivePosition,
        })
    }
}

impl ExplainSelection for ClearTabulation {
    fn explain(&self) -> String {
        match self {
            Self::CharacterTabulationStopActivePosition => String::from(
                "Clear the character tabulation stop at the active presentation position.",
            ),
            Self::LineTabulationStopActiveLine => {
                String::from("Clear the line tabulation stop at the active line.")
            }
            Self::AllCharacterTabulationStopsActiveLine => {
                String::from("Clear all character tabulation stops at the active line.")
            }
            Self::AllCharacterTabulationStops => {
                String::from("Clear all character tabulation stops.")
            }
            Self::AllLineTabulationStops => String::from("Clear all line tabulation stops."),
            Self::AllTabulationStops => String::from("Clear all tabulation stops."),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{c0::CR, explain::Explain};

    /// Test the output of short_name
    #[test]
    fn get_short_name() {
        assert_eq!(CR.short_name(), Some("CR"))
    }

    /// Test the output of long_name
    #[test]
    fn get_long_name() {
        assert_eq!(CR.long_name(), "Carriage Return")
    }

    /// Test the output of short_description
    #[test]
    fn get_short_description() {
        assert_eq!(CR.short_description(), "Move to the beginning of the line.")
    }

    /// Test the output of long_description
    #[test]
    fn get_long_description() {
        assert_eq!(CR.long_description(),
        concat!(
            "Move the cursor to the beginning of the line. The exact meaning depends on the setting of 'Device ",
            "Component Select Mode' (DCSM) and on the parameter value of 'Select Implicit Movement Direction' (SIMD).",
            "\n\nIf the DCSM is set to 'Presentation' and SIMD is set to 'Normal', it causes the active presentation ",
            "position to be moved to the line home position of the same line in the presentation component. The line ",
            "home position is established by the parameter value of 'Set Line Home' SLH.\nWith SIMD set to ",
            "'Opposite', it causes the active presentation position to be moved to the line limit position of the ",
            "same line in the presentation component. The line limit position is established by the parameter value ",
            "of 'Set Line Limit' (SLL).\n\nIf the DCSM is set to 'Data' and SIMD is set to 'Normal', it causes the ",
            "active data position to be moved to the line home position of the same line in the data component. The ",
            "line home position is established by the parameter value of 'Set Line Home' (SLH)\nWith SIMD set to ",
            "'Opposite', it causes the active data position to be moved to the line limit position of the same line ",
            "in the data component. The line limit position position is established by the parameter value of ",
            "'Set Line Limit' (SLL)."
        )
    )
    }
}
