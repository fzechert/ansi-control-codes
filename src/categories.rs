//! This module re-exports categories of control functions.

/// All Control Functions that are Categorized as Delimiters.
pub mod delimiters {
    pub use crate::c1::{APC, DCS, OSC, PM, SOS, ST};
    pub use crate::independent_control_functions::CMD;
}

/// All Control Functions that are Categorized as Introducers.
pub mod introducers {
    pub use crate::c0::ESC;
    pub use crate::c1::{CSI, SCI};
}

/// All Control Functions that are Categorized as Shift Functions.
pub mod shift_functions {
    pub use crate::c0::{LS0, LS1, SI, SO};
    pub use crate::c1::{SS2, SS3};
    pub use crate::independent_control_functions::{LS1R, LS2, LS2R, LS3, LS3R};
}

/// All Control Functions that are Categorized as Format Effectors.
pub mod format_effectors {
    pub use crate::c0::{BS, CR, FF, HT, LF, VT};
    pub use crate::c1::{HTJ, HTS, NEL, PLD, PLU, RI, VTS};
    pub use crate::control_sequences::{
        HPA, HPB, HPR, HVP, PPA, PPB, PPR, TBC, TSR, VPA, VPB, VPR,
    };
}

/// All Control Functions that are Categorized as Presentation Control Functions.
pub mod presentation_control_functions {
    pub use crate::c1::{BPH, NBH};
    pub use crate::control_sequences::{
        DTA, FNT, GCC, GSM, GSS, JFY, PEC, PFS, PTX, QUAD, SACS, SAPV, SCO, SCP, SCS, SDS, SGR,
        SHS, SIMD, SLH, SLL, SLS, SPD, SPH, SPI, SPL, SPQR, SRCS, SRS, SSU, SSW, STAB, SVS, TAC,
        TALE, TATE, TCC, TSS,
    };
}

/// All Control Functions that are Categorized as Editor Functions.
pub mod editor_functions {
    pub use crate::control_sequences::{DCH, DL, EA, ECH, ED, EF, EL, ICH, IL};
}

/// All Control Functions that are Categorized as Cursor Control Functions.
pub mod cursor_control_functions {
    pub use crate::control_sequences::{
        CBT, CHA, CHT, CNL, CPL, CTC, CUB, CUD, CUF, CUP, CUU, CVT,
    };
}

/// All Control Functions that are Categorized as Display Control Functions.
pub mod display_control_functions {
    pub use crate::control_sequences::{NP, PP, SD, SL, SR, SU};
}

/// All Control Functions that are Categorized as Device Control Functions.
pub mod device_control_functions {
    pub use crate::c0::{DC1, DC2, DC3, DC4};
}

/// All Control Functions that are Categorized as Information Separators.
pub mod information_separators {
    pub use crate::c0::{IS1, IS2, IS3, IS4};
}

/// All Control Functions that are Categorized as Area Definition Functions.
pub mod area_definition_functions {
    pub use crate::c1::{EPA, ESA, SPA, SSA};
    pub use crate::control_sequences::DAQ;
}

/// All Control Functions that are Categorized as Mode Setting Functions.
pub mod mode_setting_functions {
    pub use crate::control_sequences::{RM, SM};
}

/// All Control Functions that are Categorized as Transmission Control Functions.
pub mod transmission_control_functions {
    pub use crate::c0::{ACK, DLE, ENQ, EOT, ETB, ETX, NAK, SOH, STX, SYN};
}

/// All Control Functions that are Categorized as Miscellaneous Control Functions.
pub mod miscellaneous_control_functions {
    pub use crate::c0::{BEL, CAN, EM, NUL, SUB};
    pub use crate::c1::{CCH, MW, PU1, PU2, STS};
    pub use crate::control_sequences::{CPR, DA, DSR, FNK, IDCS, IGS, MC, REP, SEE, SEF};
    pub use crate::independent_control_functions::{DMI, EMI, INT, RIS};
}
