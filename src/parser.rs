//! # Parser for ansi-control-codes
//!
//! This module contains a parser implementation that can be used to parse string-like types into a sequence
//! of ansi-control-codes (represented by [`ControlFunction`]s) and strings that do not contain any ansi-control-codes.
//!
//! To use the parser module, enable the feature `parser` in your `Cargo.toml`.  
//!
//! ```text
//! cargo add ansi-control-codes --features parser
//! ```
//!
//! ## Example Usage
//!
//! ```
//! use ansi_control_codes::c1::NEL;
//! use ansi_control_codes::parser::{TokenStream, Token};
//!
//! let to_be_parsed = format!("This text{}is spread across{}multiple lines.", NEL, NEL);
//! let parsed = TokenStream::from(&to_be_parsed);
//!
//! let parts: Vec<Token> = parsed.collect();
//! println!("{:?}", parts);
//!
//! assert_eq!(parts[0], Token::String("This text"));
//! assert_eq!(parts[1], Token::ControlFunction(NEL));
//! assert_eq!(parts[2], Token::String("is spread across"));
//! assert_eq!(parts[3], Token::ControlFunction(NEL));
//! assert_eq!(parts[4], Token::String("multiple lines."));
//! ```

use crate::{c0::*, c1::*, independent_control_functions::*, ControlFunction};

/// All C0 Codes that can be parsed without any lookahaed (all C0 codes except for ESC)
const C0_CODES: [ControlFunction; 31] = [
    ACK, BEL, BS, CAN, CR, DC1, DC2, DC3, DC4, DLE, EM, ENQ, EOT, ETB, ETX, FF, HT, IS1, IS2, IS3,
    IS4, LF, NAK, NUL, SI, SO, SOH, STX, SUB, SYN, VT,
];

/// All C1 Codes that can be parsed without any lookahaed (all C1 codes except for CSI)
const C1_CODES: [ControlFunction; 27] = [
    BPH, NBH, NEL, SSA, ESA, HTS, HTJ, VTS, PLD, PLU, RI, SS2, SS3, DCS, PU1, PU2, STS, CCH, MW,
    SPA, EPA, SOS, SCI, ST, OSC, PM, APC,
];

/// All independent control codes.
const INDEPDENDENT_CODES: [ControlFunction; 10] =
    [DMI, INT, EMI, RIS, CMD, LS2, LS3, LS3R, LS2R, LS1R];

/// Lower bound of valid characters for control function values.
/// Control sequences end with characters between 04/00 and 06/15
/// (07 / 00 - 07 / 15 is also allowed as private-use area).
const CONTROL_FUNCTION_LOWER_BOUND: u8 = ascii!(04 / 00).as_bytes()[0];

/// Upper bound of valid characters for control function values.
/// Control sequences end with characters between 04/00 and 06/15
/// (07 / 00 - 07 / 15 is also allowed as private-use area).
const CONTROL_FUNCTION_UPPER_BOUND: u8 = ascii!(07 / 15).as_bytes()[0];

/// Lower bound of valid parameter bytes.
/// Parameter bytes can be between 03 / 00 and 03 / 15.
const PARAMETER_LOWER_BOUND: u8 = ascii!(03 / 00).as_bytes()[0];

/// Upper bound of valid parameter bytes.
/// Parameter bytes can be between 03 / 00 and 03 / 15.
const PARAMETER_UPPER_BOUND: u8 = ascii!(03 / 15).as_bytes()[0];

/// Parameter separator byte.
const PARAMETER_SEPARATOR: &str = ascii!(03 / 11);

/// A Token contains a part of the parsed string. Each part is either a String that does not contain any
/// ansi-control-codes (represented by [`Token::String`]), or a ansi-control-code (represented by
/// [`Token::ControlFunction`]).
///
/// A `Token` can be obtained by creating a [`TokenStream`] and iterating over it.
#[derive(Debug, PartialEq, Eq)]
pub enum Token<'a> {
    /// A string slice that does not contain any valid ansi-control-code.
    String(&'a str),
    /// A valid ansi-control-code that was found in the parsed string.
    ControlFunction(ControlFunction<'a>),
}

/// A TokenStream is a stream of [`Token`]s that were parsed from an input string.
/// The TokenStream implements the [`Iterator`] interface, which can be used to extract the result of a parse operation.
///
/// The parse operation can never fail. If invalid ansi-control-codes are detected in the input string, they will be
/// emitted as normal Strings ([`Token::String`]). Only valid ansi-control-codes will be emitted as ControlFunctions
/// ([`Token::ControlFunction`]).
#[derive(Debug)]
pub struct TokenStream<'a> {
    value: &'a str,
    position: usize,
    max_position: usize,
}

impl<'a> TokenStream<'a> {
    /// Parse the given string `value` into a [`TokenStream`].
    ///
    /// The [`TokenStream`] can be iterated over to inspect the result of the parse operation.
    pub fn from(value: &'a str) -> Self {
        TokenStream {
            value,
            // invariant: position always points to a valid character boundary inside the string stored in value.
            position: 0,
            max_position: value.len(),
        }
    }

    fn get_next_char_boundary(&self, position: usize) -> usize {
        // invariant: position is a valid character boundary. Next character boundary is at least at position + 1
        // no more boundaries can be discovered, if position >= self.value.len()
        if position >= self.max_position {
            return position;
        }

        let mut next_boundary = position + 1;
        while !self.value.is_char_boundary(next_boundary) {
            next_boundary += 1
        }
        next_boundary
    }

    fn emit_current_string(&mut self, position: usize) -> Option<Token<'a>> {
        let mut emit_token = None;
        if position != self.position {
            emit_token = Some(Token::String(&self.value[self.position..position]));

            self.position = position;
        }

        emit_token
    }
}
impl<'a> Iterator for TokenStream<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut current_position = self.position;
        while current_position < self.max_position {
            let next_char_boundary = self.get_next_char_boundary(current_position);

            // invariant: self.value[self.position..next_char_boundary] is a valid sub-string of value, as
            // current_position and next_char_boundary both point to valid character boundaries.
            // invariant: self.value[current_position..next_char_boundary] is a valid character.

            let current_char = &self.value[current_position..next_char_boundary];
            if !current_char.is_ascii() {
                // all ansi-control-codes are valid ascii. Non-ascii characters can never be part of an
                // ansi-control-code
                current_position = next_char_boundary;
                continue;
            }

            // when encountering an ASCII character, this might be beginning, or part of an already started
            // ansci-control-code. It also might just be a normal ascii character that has nothing to do with any
            // ansi-control-code.

            // All c0 control codes are 1 character long and can be identified directly, except for ESC which might
            // introduce a longer sequence. All of those, except ESC, are stored in the array C0_CODES
            if let Some(ansi_control_code) = C0_CODES
                .into_iter()
                .find(|c0_code| c0_code == &current_char)
            {
                // detected a C0 ansi-control-code. But there might be other data that we need to emit from
                // previous iterations of this loop that detected string data.
                return self.emit_current_string(current_position).or_else(|| {
                    // there was no string to emit before the control function, so we can emit the control function
                    // instead. We need to change the position of the iterator first.
                    self.position = next_char_boundary;
                    Some(Token::ControlFunction(ansi_control_code))
                });
            }

            // This is either ESC (maybe introducing a longer sequence of control codes), or a normal string
            // ESC is a special scenario, as it might introduce longer escape sequences
            if ESC == current_char {
                // if we have reached the end of the string, the ESC cannot be part of a longer sequence
                if self.max_position == next_char_boundary {
                    // detected an ESC. But there might be other data that we need to emit from
                    // previous iterations of this loop that detected string data.
                    return self.emit_current_string(current_position).or_else(|| {
                        // there was no string to emit before the control function, so we can emit the control function
                        // instead. We need to change the position of the iterator first.
                        self.position = next_char_boundary;
                        Some(Token::ControlFunction(ESC))
                    });
                }

                // we need to look-ahead to find if this is part of a longer sequence
                // possible next character is one of C1, independent control function, or CSI
                let next_next_char_boundary = self.get_next_char_boundary(next_char_boundary);

                // invariant: self.value[self.position..next_next_char_boundary] is a valid sub-string of value, as
                // current_position and next_next_char_boundary both point to valid character boundaries.
                // invariant: self.value[next_char_boundary..next_next_char_boundary] is a valid character.
                // invariant: self.value[current_position..next_next_char_boundary] is a valid string.

                let current_char = &self.value[next_char_boundary..next_next_char_boundary];
                let control_sequence = &self.value[current_position..next_next_char_boundary];
                if !current_char.is_ascii() {
                    // C1, independent control function, and CSI are valid ascii characters. If we find a non-ascii
                    // character. this cannot be a control character or sequence. This is a standalone ESC character.
                    // Emit the ESC. But there might be other data that we need to emit from
                    // previous iterations of this loop that detected string data.
                    return self.emit_current_string(current_position).or_else(|| {
                        // there was no string to emit before the control function, so we can emit the control function
                        // instead. We need to change the position of the iterator first.
                        self.position = next_char_boundary;
                        Some(Token::ControlFunction(ESC))
                    });
                }

                // A ASCII character might be a continuation of a longer control sequence, or it might just be normal
                // text. If it is a continuation of a control function, it needs to be one of the C1 codes, one of
                // the independent control codes, or a CSI starting a control sequence.

                // Handle C1 Codes
                // All C1 control codes are 1 character long and can be identified directly, except for CSI which might
                // introduce a longer sequence. All of those, except CSI, are stored in the array C1_CODES
                if let Some(ansi_control_code) = C1_CODES
                    .into_iter()
                    .find(|c1_code| c1_code == &control_sequence)
                {
                    // detected a C1 ansi-control-code. But there might be other data that we need to emit from
                    // previous iterations of this loop that detected string data.
                    return self.emit_current_string(current_position).or_else(|| {
                        // there was no string to emit before the control function, so we can emit the control function
                        // instead. We need to change the position of the iterator first.
                        self.position = next_next_char_boundary;
                        Some(Token::ControlFunction(ansi_control_code))
                    });
                }

                // Handle Independent Control Functions
                // All Independent Control Functions are 1 character long, and can be identified directly.
                // All Independent control functions are stored in the array INDEPENDENT_CODES
                if let Some(ansi_control_code) = INDEPDENDENT_CODES
                    .into_iter()
                    .find(|independent_code| independent_code == &control_sequence)
                {
                    // detected a C1 ansi-control-code. But there might be other data that we need to emit from
                    // previous iterations of this loop that detected string data.
                    return self.emit_current_string(current_position).or_else(|| {
                        // there was no string to emit before the control function, so we can emit the control function
                        // instead.
                        self.position = next_next_char_boundary;
                        Some(Token::ControlFunction(ansi_control_code))
                    });
                }

                // If the character is CSI, it introduces a control sequence
                if control_sequence == CSI {
                    // between the CSI character and the function value. To find the parameter list, we need to find
                    // the end of the control sequence. Possible end values of the sequence depend on the sequence type.
                    // Sequences can start with or without an intermediate byte.

                    let control_sequence_position = next_next_char_boundary;

                    let lower_bound = CONTROL_FUNCTION_LOWER_BOUND;
                    let upper_bound = CONTROL_FUNCTION_UPPER_BOUND;
                    let parameter_lower_bound = PARAMETER_LOWER_BOUND;
                    let parameter_upper_bound = PARAMETER_UPPER_BOUND;

                    let mut intermediate_byte = false;

                    // try to find a function value between lower_bound and upper_bound
                    let mut current_position_cs = control_sequence_position;
                    let mut next_position_cs =
                        self.get_next_char_boundary(control_sequence_position);
                    'control_sequence_loop: loop {
                        let current_char = &self.value[current_position_cs..next_position_cs];

                        // non-ascii (multi-byte) values are never valid parameters to a control sequence, this is
                        // invalid!
                        if current_char.as_bytes().len() != 1 {
                            break 'control_sequence_loop;
                        }

                        // does this end the control function?
                        if current_char.as_bytes()[0] >= lower_bound
                            && current_char.as_bytes()[0] <= upper_bound
                        {
                            // detected the end of a control function
                            let control_function_value = if intermediate_byte {
                                &self.value[current_position_cs - 1..next_position_cs]
                            } else {
                                current_char
                            };
                            let parameters_unparsed = if intermediate_byte {
                                &self.value[control_sequence_position..(current_position_cs - 1)]
                            } else {
                                &self.value[control_sequence_position..current_position_cs]
                            };
                            let parameters = parameters_unparsed
                                .split(PARAMETER_SEPARATOR)
                                .map(String::from)
                                .collect();

                            // emit string token (if any) or the control function
                            return self.emit_current_string(current_position).or_else(|| {
                                // there was no string to emit before the control function, so we can emit the control
                                // function instead.
                                self.position = next_position_cs;
                                Some(Token::ControlFunction(ControlFunction::new_sequence(
                                    control_function_value,
                                    parameters,
                                )))
                            });
                        } else if intermediate_byte {
                            // we have already seen an intermediate byte, but now the control function is still
                            // not terminated. This is invalid!
                            break 'control_sequence_loop;
                        } else if current_char.as_bytes()[0] < parameter_lower_bound
                            || current_char.as_bytes()[0] > parameter_upper_bound
                        {
                            // this is not a valid function value, and not a valid parameter byte
                            // if it is not the intermediate byte, this is invalid!
                            intermediate_byte = current_char == ascii!(02 / 00);
                            if !intermediate_byte {
                                break 'control_sequence_loop;
                            }
                        }

                        // end of string reached?
                        // this does not end the control function
                        // check the next character (or exit, if there are no more characters).
                        if next_position_cs == self.max_position {
                            // nothing else to do anymore, reached end of string, this can't be valid
                            // since there was no valid end to this control sequence.
                            break 'control_sequence_loop;
                        }
                        current_position_cs = next_position_cs;
                        next_position_cs = self.get_next_char_boundary(current_position_cs);
                    }
                } else {
                    // found ESC that did not introduce a longer sequence, emit as-is.
                    return self.emit_current_string(current_position).or_else(|| {
                        // there was no string to emit before the control function, so we can emit the control function
                        // instead.
                        self.position = next_char_boundary;
                        Some(Token::ControlFunction(ESC))
                    });
                }
            }

            current_position = next_char_boundary;
        }

        // reached end of the input string.
        // emit the last token, if there is still some parts of the input that have not been emitted yet.
        self.emit_current_string(current_position)
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        c0::{BEL, CR, ESC, LF},
        c1::{BPH, CSI, NBH, SOS},
        control_sequences::{
            DeviceAttributes, PrintQuality, ReversedString, TabulationControl, CHA, CHT, CTC, CUP,
            DA, SPQR, SRS, SSW, SU, TCC,
        },
        independent_control_functions::{DMI, EMI, RIS},
        ControlFunction,
    };

    use super::{Token, TokenStream};

    #[test]
    fn test_simple_ascii_string() {
        let simple_ascii_input = "Hello World";
        let mut token_stream = TokenStream::from(simple_ascii_input);

        let first_element = token_stream.next();
        let second_element = token_stream.next();

        assert!(first_element.is_some_and(|value| value == Token::String(simple_ascii_input)));
        assert!(second_element.is_none());
    }

    #[test]
    fn test_simple_non_ascii_string() {
        let simple_non_ascii_input = "Löwe 老虎 Léopard";
        let mut token_stream = TokenStream::from(simple_non_ascii_input);

        let first_element = token_stream.next();
        let second_element = token_stream.next();

        assert!(first_element.is_some_and(|value| value == Token::String(simple_non_ascii_input)));
        assert!(second_element.is_none());
    }

    #[test]
    fn test_simple_ascii_string_with_c0() {
        let simple_ascii_input = "Ring the bell";
        let input = format!("{}{}", simple_ascii_input, BEL);

        let mut token_stream = TokenStream::from(&input);

        let first_element = token_stream.next();
        let second_element = token_stream.next();
        let third_element = token_stream.next();

        assert!(first_element.is_some_and(|value| value == Token::String(simple_ascii_input)));
        assert!(second_element.is_some_and(|value| value == Token::ControlFunction(BEL)));
        assert!(third_element.is_none());
    }

    #[test]
    fn test_simple_non_ascii_string_with_c0() {
        let simple_non_ascii_input = "Löwe 老虎 Léopard";
        let input = format!("{}{}{}", simple_non_ascii_input, CR, LF);

        let mut token_stream = TokenStream::from(&input);

        let first_element = token_stream.next();
        let second_element = token_stream.next();
        let third_element = token_stream.next();
        let forth_element = token_stream.next();

        assert!(first_element.is_some_and(|value| value == Token::String(simple_non_ascii_input)));
        assert!(second_element.is_some_and(|value| value == Token::ControlFunction(CR)));
        assert!(third_element.is_some_and(|value| value == Token::ControlFunction(LF)));
        assert!(forth_element.is_none());
    }

    #[test]
    fn test_simple_ascii_string_with_interleaved_c0() {
        let line1 = "Line1";
        let line2 = "Line2";
        let input = format!("{}{}{}", line1, LF, line2);

        let mut token_stream = TokenStream::from(&input);

        let first_element = token_stream.next();
        let second_element = token_stream.next();
        let third_element = token_stream.next();
        let forth_element = token_stream.next();

        assert!(first_element.is_some_and(|value| value == Token::String(line1)));
        assert!(second_element.is_some_and(|value| value == Token::ControlFunction(LF)));
        assert!(third_element.is_some_and(|value| value == Token::String(line2)));
        assert!(forth_element.is_none());
    }

    #[test]
    fn test_simple_non_ascii_string_with_interleaved_c0() {
        let line1 = "Löwe";
        let line2 = "老虎";
        let input = format!("{}{}{}", line1, LF, line2);

        let mut token_stream = TokenStream::from(&input);

        let first_element = token_stream.next();
        let second_element = token_stream.next();
        let third_element = token_stream.next();
        let forth_element = token_stream.next();

        assert!(first_element.is_some_and(|value| value == Token::String(line1)));
        assert!(second_element.is_some_and(|value| value == Token::ControlFunction(LF)));
        assert!(third_element.is_some_and(|value| value == Token::String(line2)));
        assert!(forth_element.is_none());
    }

    #[test]
    fn test_single_esc() {
        let esc = ESC.to_string();
        let mut token_stream = TokenStream::from(&esc);

        let first_element = token_stream.next();
        let second_element = token_stream.next();

        assert!(first_element.is_some_and(|value| value == Token::ControlFunction(ESC)));
        assert!(second_element.is_none());
    }

    #[test]
    fn test_esc_at_end_of_string() {
        let text = "I have to escape";
        let escape = format!("{}{}", text, ESC);
        let mut token_stream = TokenStream::from(&escape);

        let first_element = token_stream.next();
        let second_element = token_stream.next();
        let third_element = token_stream.next();

        assert!(first_element.is_some_and(|value| value == Token::String(text)));
        assert!(second_element.is_some_and(|value| value == Token::ControlFunction(ESC)));
        assert!(third_element.is_none());
    }

    #[test]
    fn test_esc_at_start_of_non_ascii_string() {
        let text = "í have to escape";
        let escape = format!("{}{}", ESC, text);
        let mut token_stream = TokenStream::from(&escape);

        let first_element = token_stream.next();
        let second_element = token_stream.next();
        let third_element = token_stream.next();

        assert!(first_element.is_some_and(|value| value == Token::ControlFunction(ESC)));
        assert!(second_element.is_some_and(|value| value == Token::String(text)));
        assert!(third_element.is_none());
    }

    #[test]
    fn test_esc_at_start_of_ascii_string() {
        let text = "i have to escape";
        let escape = format!("{}{}", ESC, text);
        let mut token_stream = TokenStream::from(&escape);

        let first_element = token_stream.next();
        let second_element = token_stream.next();
        let third_element = token_stream.next();

        println!("{:?}", first_element);
        println!("{:?}", second_element);
        println!("{:?}", third_element);

        assert!(first_element.is_some_and(|value| value == Token::ControlFunction(ESC)));
        assert!(second_element.is_some_and(|value| value == Token::String(text)));
        assert!(third_element.is_none());
    }

    #[test]
    fn test_c1_at_start_of_string() {
        let text = format!("{}This might be in the next line", BPH);
        let result = TokenStream::from(&text).collect::<Vec<Token>>();

        assert_eq!(
            result,
            vec![
                Token::ControlFunction(BPH),
                Token::String("This might be in the next line")
            ]
        )
    }

    #[test]
    fn test_c1_at_end_of_string() {
        let text = format!("No break is permitted at the end of this string{}", NBH);
        let result = TokenStream::from(&text).collect::<Vec<Token>>();

        assert_eq!(
            result,
            vec![
                Token::String("No break is permitted at the end of this string"),
                Token::ControlFunction(NBH)
            ]
        )
    }

    #[test]
    fn test_c1_in_between_ascii_strings() {
        let text = format!("Line1{}Maybe Line2", BPH);
        let result = TokenStream::from(&text).collect::<Vec<Token>>();

        assert_eq!(
            result,
            vec![
                Token::String("Line1"),
                Token::ControlFunction(BPH),
                Token::String("Maybe Line2")
            ]
        )
    }

    #[test]
    fn test_c1_in_between_non_ascii_strings() {
        let text = format!("老{}虎", SOS);
        let result = TokenStream::from(&text).collect::<Vec<Token>>();

        assert_eq!(
            result,
            vec![
                Token::String("老"),
                Token::ControlFunction(SOS),
                Token::String("虎")
            ]
        )
    }

    #[test]
    fn test_independent_code_at_start_of_string() {
        let text = format!("{}Back to normal", RIS);
        let result = TokenStream::from(&text).collect::<Vec<Token>>();

        assert_eq!(
            result,
            vec![Token::ControlFunction(RIS), Token::String("Back to normal")]
        )
    }

    #[test]
    fn test_independent_code_at_end_of_string() {
        let text = format!("Now enabling manual input{}", EMI);
        let result = TokenStream::from(&text).collect::<Vec<Token>>();

        assert_eq!(
            result,
            vec![
                Token::String("Now enabling manual input"),
                Token::ControlFunction(EMI)
            ]
        )
    }

    #[test]
    fn test_independent_code_in_between_of_ascii_strings() {
        let text = format!(
            "Now enabling manual input{} And now {}disabling it again",
            EMI, DMI
        );
        let result = TokenStream::from(&text).collect::<Vec<Token>>();

        assert_eq!(
            result,
            vec![
                Token::String("Now enabling manual input"),
                Token::ControlFunction(EMI),
                Token::String(" And now "),
                Token::ControlFunction(DMI),
                Token::String("disabling it again"),
            ]
        )
    }

    #[test]
    fn test_independent_code_in_between_of_non_ascii_strings() {
        let text = format!(
            "Now enabling manual input{} And now 老{}老disabling it again",
            EMI, DMI
        );
        let result = TokenStream::from(&text).collect::<Vec<Token>>();

        assert_eq!(
            result,
            vec![
                Token::String("Now enabling manual input"),
                Token::ControlFunction(EMI),
                Token::String(" And now 老"),
                Token::ControlFunction(DMI),
                Token::String("老disabling it again"),
            ]
        )
    }

    #[test]
    fn test_invalid_control_sequence() {
        let invalid_sequence = format!("{}{}{}", ESC, CSI, "ä");
        let result = TokenStream::from(&invalid_sequence).collect::<Vec<Token>>();

        assert_eq!(
            result,
            vec![
                Token::ControlFunction(ESC),
                Token::String(&format!("{}{}", CSI, "ä")),
            ]
        )
    }

    #[test]
    fn test_invalid_control_sequence_with_lookalike_arguments() {
        let invalid_sequence = format!("{}{}{}{}", ESC, CSI, "1;2", "ä");
        let result = TokenStream::from(&invalid_sequence).collect::<Vec<Token>>();

        assert_eq!(
            result,
            vec![
                Token::ControlFunction(ESC),
                Token::String(&format!("{}{}{}", CSI, "1;2", "ä")),
            ]
        )
    }

    #[test]
    fn test_invalid_control_sequence_with_no_end() {
        let invalid_sequence = format!("{}{}", ESC, CSI);
        let result = TokenStream::from(&invalid_sequence).collect::<Vec<Token>>();

        assert_eq!(
            result,
            vec![
                Token::ControlFunction(ESC),
                Token::String(&format!("{}", CSI)),
            ]
        )
    }

    #[test]
    fn test_invalid_control_sequence_with_intermediate() {
        let invalid_sequence = format!("{}{}{}{}", ESC, CSI, ascii!(02 / 00), "ä");
        let result = TokenStream::from(&invalid_sequence).collect::<Vec<Token>>();

        assert_eq!(
            result,
            vec![
                Token::ControlFunction(ESC),
                Token::String(&format!("{}{}{}", CSI, ascii!(02 / 00), "ä")),
            ]
        )
    }

    #[test]
    fn test_invalid_control_sequence_with_intermediate_with_lookalike_arguments() {
        let invalid_sequence = format!("{}{}{}{}{}", ESC, CSI, ascii!(02 / 00), "1;2", "ä");
        let result = TokenStream::from(&invalid_sequence).collect::<Vec<Token>>();

        assert_eq!(
            result,
            vec![
                Token::ControlFunction(ESC),
                Token::String(&format!("{}{}{}{}", CSI, ascii!(02 / 00), "1;2", "ä")),
            ]
        )
    }

    #[test]
    fn test_invalid_control_sequence_with_intermediate_with_no_end() {
        let invalid_sequence = format!("{}{}{}", ESC, CSI, ascii!(02 / 00));
        let result = TokenStream::from(&invalid_sequence).collect::<Vec<Token>>();

        assert_eq!(
            result,
            vec![
                Token::ControlFunction(ESC),
                Token::String(&format!("{}{}", CSI, ascii!(02 / 00))),
            ]
        )
    }

    #[test]
    fn test_invalid_control_sequence_with_no_end_and_parameters() {
        let invalid_sequence = format!("{}{}{}", ESC, CSI, "1;2");
        let result = TokenStream::from(&invalid_sequence).collect::<Vec<Token>>();

        assert_eq!(
            result,
            vec![
                Token::ControlFunction(ESC),
                Token::String(&format!("{}{}", CSI, "1;2")),
            ]
        )
    }

    #[test]
    fn test_valid_control_sequence_no_intermediate_standalone() {
        let valid_sequence = format!("{}", CHA(None));
        let result = TokenStream::from(&valid_sequence).collect::<Vec<Token>>();

        assert_eq!(result, vec![Token::ControlFunction(CHA(None))])
    }

    #[test]
    fn test_valid_control_sequence_no_intermediate_beginning_of_string() {
        let valid_sequence = format!("{}Hello", CHA(None));
        let result = TokenStream::from(&valid_sequence).collect::<Vec<Token>>();

        assert_eq!(
            result,
            vec![Token::ControlFunction(CHA(None)), Token::String("Hello")]
        )
    }

    #[test]
    fn test_valid_control_sequence_no_intermediate_end_of_string() {
        let valid_sequence = format!("Hello{}", CHT(8.into()));
        let result = TokenStream::from(&valid_sequence).collect::<Vec<Token>>();

        assert_eq!(
            result,
            vec![
                Token::String("Hello"),
                Token::ControlFunction(CHT(8.into()))
            ]
        )
    }

    #[test]
    fn test_valid_control_sequence_no_intermediate_middle_of_string() {
        let valid_sequence = format!(
            "Take control{} over tabulations",
            CTC(TabulationControl::ClearAllLineTabulationStops.into())
        );
        let result = TokenStream::from(&valid_sequence).collect::<Vec<Token>>();

        assert_eq!(
            result,
            vec![
                Token::String("Take control"),
                Token::ControlFunction(CTC(TabulationControl::ClearAllLineTabulationStops.into())),
                Token::String(" over tabulations")
            ]
        )
    }

    #[test]
    fn test_valid_control_sequence_with_intermediate_standalone() {
        let valid_sequence = format!("{}", SPQR(PrintQuality::HighQualityLowSpeed.into()));
        let result = TokenStream::from(&valid_sequence).collect::<Vec<Token>>();

        assert_eq!(result, vec![Token::ControlFunction(SPQR(None))])
    }

    #[test]
    fn test_valid_control_sequence_with_intermediate_beginning_of_string() {
        let valid_sequence = format!(
            "{}desreveR{}",
            SRS(ReversedString::Start.into()),
            SRS(ReversedString::End.into())
        );
        let result = TokenStream::from(&valid_sequence).collect::<Vec<Token>>();

        assert_eq!(
            result,
            vec![
                Token::ControlFunction(SRS(ReversedString::Start.into())),
                Token::String("desreveR"),
                Token::ControlFunction(SRS(ReversedString::End.into()))
            ]
        )
    }

    #[test]
    fn test_valid_control_sequence_with_intermediate_end_of_string() {
        let valid_sequence = format!("No more spaces after me!{}", SSW(0));
        let result = TokenStream::from(&valid_sequence).collect::<Vec<Token>>();

        assert_eq!(
            result,
            vec![
                Token::String("No more spaces after me!"),
                Token::ControlFunction(SSW(0))
            ]
        )
    }

    #[test]
    fn test_valid_control_sequence_with_intermediate_middle_of_string() {
        let valid_sequence = format!("Hold tight!{}We are going up!", SU(50.into()));
        let result = TokenStream::from(&valid_sequence).collect::<Vec<Token>>();

        assert_eq!(
            result,
            vec![
                Token::String("Hold tight!"),
                Token::ControlFunction(SU(50.into())),
                Token::String("We are going up!")
            ]
        )
    }

    #[test]
    fn test_valid_control_sequence_with_multiple_parameters() {
        let valid_sequence = format!("All or nothing@>Ä{}", TCC(6, 12.into()));
        let result = TokenStream::from(&valid_sequence).collect::<Vec<Token>>();

        assert_eq!(
            result,
            vec![
                Token::String("All or nothing@>Ä"),
                Token::ControlFunction(TCC(6, 12.into())),
            ]
        )
    }

    #[test]
    fn test_example_a() {
        let example = "\x1b[0u\x1b[62c\x1b[23;6H";
        let result = TokenStream::from(&example).collect::<Vec<Token>>();

        assert_eq!(
            result,
            vec![
                Token::ControlFunction(
                    ControlFunction::private_use("u", vec![String::from("0")]).unwrap()
                ),
                Token::ControlFunction(DA(DeviceAttributes::Identify(62).into())),
                Token::ControlFunction(CUP(23.into(), 6.into()))
            ]
        )
    }
}
