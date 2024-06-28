# Propositional Logic

This crate offers tools for defining and manipulating logical propositions using symbols and connectives like and, or, and implies. It simplifies the creation of logical expressions and the evaluation of their truth values within defined logical contexts.

Useful for educational purposes, AI projects, and any application requiring formal logical reasoning.

## Examples
```rust
use propositional::prelude::*;

let rain = symbol!("it's raining");
let cloud = symbol!("it's cloudy");

let world = and!(
    implies!(rain, cloud),
    rain
);

println!("It is cloudy? {:?}", check(&world, &cloud));
//-> It is cloudy? Some(true)
```
Source: [wikipedia.org](https://en.wikipedia.org/wiki/Propositional_calculus#Arguments)

```rust
use propositional::prelude::*;

let rain = Symbol("It is raining.");
let hagrid = Symbol("Harry visited Hagrid.");
let dumbledore = Symbol("Harry visited Dumbledore.");

let knowledge = and!(
    implies!(not!(rain), hagrid),
    or!(hagrid, dumbledore),
    not!(and!(hagrid, dumbledore)),
    dumbledore
);

println!("It is raining? {:?}", check(&knowledge, &rain));
//-> It is raining? Some(true)
```
Source: [CS50](https://youtu.be/HWQLez87vqM?si=ULqkreSQPM2Y1n42&t=1692)

## Acknowledgments
Based on: CS50’s Introduction to Artificial Intelligence with Python.