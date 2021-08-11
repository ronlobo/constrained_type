<div align="center">
  <h1>Constrained Type</h1>
  <p>
    <strong>On the fly value objects in Rust</strong>
  </p>
  <p>

[![crates.io](https://img.shields.io/crates/v/constrained_type?label=latest)](https://crates.io/crates/constrained_type)
[![Documentation](https://docs.rs/constrained_type/badge.svg?version=0.2.2)](https://docs.rs/constrained_type/0.2.2)
[![Version](https://img.shields.io/badge/rustc-1.46+-ab6000.svg)](https://blog.rust-lang.org/2020/03/12/Rust-1.46.html)
![MIT or Apache 2.0 licensed](https://img.shields.io/crates/l/constrained_type.svg)
[![Dependency Status](https://deps.rs/crate/constrained_type/0.2.2/status.svg)](https://deps.rs/crate/constrained_type/0.2.2)
<br />
[![build status](https://github.com/ronlobo/constrained_type/workflows/CI%20%28Linux%29/badge.svg?branch=trunk&event=push)](https://github.com/ronlobo/constrained_type/actions)
[![Coverage Status](https://coveralls.io/repos/github/ronlobo/constrained_type/badge.svg?branch=trunk)](https://coveralls.io/github/ronlobo/constrained_type?branch=trunk)
![downloads](https://img.shields.io/crates/d/constrained_type.svg)

  </p>
</div>

# Constrained Type

This is a simple project (personal learning) to help create on the fly value objects aka constrained types.

It provides some helper functions to construct these from Rust primitives and turn them into domain primitives, new types, value objects, you name it.

## Motivation

Constrained types guarantee valid state and behaviour from dynamic runtime inputs after construction.

This can be useful when creating simple wrapper types, so called newtypes, value objects or domain primitives.

If an input does not meet the validation criteria, it returns an error result instead.

The goal is to remove defensive code statements, ease implementing business invariants and guarantee correct state at runtime.

Heavily inspired by <a href="https://github.com/swlaschin/DomainModelingMadeFunctional">"Domain Modelling Made Functional"</a>.

For more complex types, please take a look at the various builder crates.
