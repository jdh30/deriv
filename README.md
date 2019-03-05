# Symbolic derivative benchmark

This benchmark is designed to stress memory management and can be used to measure both the performance of different strategies and also
their efficiency in terms of peak memory consumption. I originally wrote the program in F# and ported it myself to OCaml, C++,
Mathematica and Swift. This repository also includes two Rust ports by other people for comparison.

## Overview

The challenge is to use a few simple rules for differentiation to compute the symbolic derivatives of the mathematical expression xˣ
performing some simple symbolic simplifications along the way. This results in a huge symbolic expression.

The resulting programs are interesting not only as benchmarks but also to show people how symbolic and logic programs can be written in
different languages and, in particular, why languages from the ML and Haskell families are so good at this.

## Results

My measurements of throughput performance and memory consumption using this benchmark show that the reference counted Swift, Mathematica
and Rust solutions all require more time and more memory to complete any given problem size than the OCaml and F# solutions that use
tracing garbage collectors.

## Criticism

This benchmark has some imperfections:

* Simply by unboxing union types this benchmark can be made to produce no garbage from temporaries at all. Whereas the OCaml version
allocates and collects GBs of garbage in the form of temporary symbolic expressions the Swift solution (which unboxes aggressively in
order to amortise the overheads of reference counting) produces no short-lived garbage at all.
* This benchmark alone is too specific to allow any strong conclusions to be drawn about the characteristics of reference counting vs
tracing garbage collection. However, it provides compelling evidence undermining the folklore belief that tracing GCs require several times
more memory than reference counting to do the same thing (which was my intention when I designed it).
* The original versions of the benchmark make extensive use of pattern matching. This is difficult to port to other languages including
even Rust because it is not yet possible to pattern match "through" an `Rc` in Rust. However, pattern matching is the future and this
program makes idiomatic use of it.
* My results have been criticised for using mature production-quality languages rather than toy implementations that might eliminate some
confounding factors. Although I believe there is great value in assessing the capabilities of real tools I am also keen to see the results
from, for example, C solutions that employ different memory management strategies.
