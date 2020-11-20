# Constrained Type

[![Coverage Status](https://coveralls.io/repos/github/ronlobo/constrained_type/badge.svg?branch=trunk)](https://coveralls.io/github/ronlobo/constrained_type?branch=trunk)

This is a simple project (personal learning) to help creating on the fly value objects aka constrained types.

It provides some helper functions to construct these from Rust primitives.

## Motivation

Constrained types guarantee valid state and behaviour from dynamic runtime inputs after construction.

This can be useful when creating simple wrapper types or so called newtypes.

If an input does not meet the validation criteria, an error result is returned instead.

The goal is to remove defensive code statements, ease implementing business invariants and guarantee correct state at runtime.

Heavily inspired by <a href="https://github.com/swlaschin/DomainModelingMadeFunctional">"Domain Modelling Made Functional"</a>.

For more complex types take a look at the various builder crates.
