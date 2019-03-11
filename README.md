# Extending Ruby with Rust benchmarks

Benchmarks Helix vs FFI when extending Ruby with Rust.

Helix uses C extensions to extend Ruby and compiles a native binary which is then required by Ruby.

FFI, on the other hand, uses libffi to call C from Ruby and requires dynamic
shared objects (.dylib / .so).

Also, the Ruby implementation is just to have an idea of how the Rails `blank?`
method compares to the Rust implementations. I'm totally aware this is like
comparing apples and oranges as the ruby benchmark uses a regular expression.

These benchmarks compare the two. They are relatively basic for now but
contributions are welcome.

## TODO

* Add [rutie](https://github.com/danielpclark/rutie).


## How to run the benchmarks

```
$ bundle install
$ bundle exec rake bench
```

## Results

```
Warming up --------------------------------------
 helix - blank check   120.429k i/100ms
   ffi - blank check   205.089k i/100ms
  ruby - blank check   153.415k i/100ms
Calculating -------------------------------------
 helix - blank check      1.640M (± 1.5%) i/s -      8.310M in   5.067506s
   ffi - blank check      3.968M (± 2.0%) i/s -     19.894M in   5.015064s
  ruby - blank check      2.421M (± 1.3%) i/s -     12.120M in   5.007353s

Comparison:
   ffi - blank check:  3968344.9 i/s
  ruby - blank check:  2420806.3 i/s - 1.64x  slower
 helix - blank check:  1640136.7 i/s - 2.42x  slower
```
