//! Control Strings.
//!
//! A control string is a string of bit combinations which may occur in the data stream as a logical entity for
//! control purposes. A control string consists of an opening delimiter, a command string or a character string,
//! and a terminating delimiter, the String Terminator (`ST`).
//!
//! A command string is a sequence of bit combinations in the range `00/08` to `00/13` and `02/00` to `07/14`.
//!
//! A character string is a sequence of any bit combination, except those representing Start Of String (`SOS`) or String
//! Terminator (`ST`).
