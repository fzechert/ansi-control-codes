# Changelog

## Version 1.0.0

- Specified the correct minimum rust compiler version `1.62.1` in `Cargo.toml`
- Added a parser that can parse ansi-control-codes from a string. The parser is behind the feature flag `parser`.
- A new function `ControlFunction::private_use()` has been added, that allows specification of control sequences
  in the private use / experimental section of the specification.
- Added a new feature `explain` that can explain what an ansi-control-code does.
