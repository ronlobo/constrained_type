:construction: ⍺ :construction:

A simple library to create on the fly value objects aka constrained types.

Motivation:

Statically typed languages allow for better guarantees of correctness upon successful compilation.    

However, not everything can be type-checked at compile-time, e.g. not all actor inputs are known at runtime.

When it comes to data integrity in our systems, there is nothing more time wasting than having to write
retro scripts to fix the inconsistencies caused by invalid state changes in the first place. 

Poisoned inputs also open the door for random system crashes and security vulnerabilities.

There are some opinions around where to apply these validations, how to classify and handle them.

Ideally, invalid state changes are prevented during a type's instantiation process, not only for example at the RestAPI or DB repository level.  

Constrained types guarantee a valid state and behaviour from dynamic runtime inputs and result in an error otherwise.
This error is very specific to the domain and problem and also called domain error.

Constrained types push their validation to the construction process of the type.
This helps removing defensive code statements, implementing business invariants and correct state at runtime.

Heavily inspired by <a href="https://github.com/swlaschin/DomainModelingMadeFunctional">"Domain Modelling Made Functional"</a>. 