# ANSI Escape Code Library

ANSI escape sequences are a standard for in-band signalling to control cursor location, color, font styling, and
other options on video text terminals and terminal emulators.

This library contains all ANSI Escape Codes that are defined in the [ISO 6429 Standard][iso-6429]. ISO 6429 is
the international standard that followed from the efforts of aligning the european [ECMA-48 Standard][ecma-48] and
the american [ANSI X3.64 Standard][ansi-x364].

## Documentation

Find the latest documentation of this crate at <https://docs.rs/crate/ansi-control-codes/latest>.

## Source Code Repository

The source code for this library is hosted at <https://git.zechert.net/fzechert/ansi-control-codes>.

## Contribution

Contributions are welcome. Please keep in mind that the source code is not at github, but on my own git server.
If you want to contribute, feel free to contact me.

## Development / Maintenance Status

This crate is under active development and maintenance.

## Source Material

The second, and newer, editions of the [ECMA-48 Standard][ecma-48] are based on the text of the
[ISO 6429 Standard][iso-6429] and are technically identical with it. Since the [ISO 6429 Standard][iso-6429] is not
freely available on the internet, this implementation is based on the publicly available documents of the
[ECMA-48 Standard][ecma-48]. In particular on edition 5 of the [ECMA-48 Standard][ecma-48], which is identical to
the third edition of [ISO-6429][iso-6429].

The [ANSI X3.64 Standard][ansi-x364] has been withdrawn by ANSI in 1994 in favour of the international standard.

You can read more about the history of the standards on [Wikipedia: ANSI escape code][wikipedia-ansi].

[ansi-x364]: https://nvlpubs.nist.gov/nistpubs/Legacy/FIPS/fipspub86.pdf
[ascii-table]: https://en.wikipedia.org/wiki/ASCII#/media/File:USASCII_code_chart.png
[ecma-48]: https://www.ecma-international.org/publications-and-standards/standards/ecma-48/
[iso-6429]: https://www.iso.org/standard/12782.html
[wikipedia-ansi]: https://en.wikipedia.org/wiki/ANSI_escape_code