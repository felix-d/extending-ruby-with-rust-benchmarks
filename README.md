# Extending Ruby with Rust benchmarks

Benchmarks Helix vs FFI vs Rutie when extending Ruby with Rust.

Helix and Rutie use C extensions to extend Ruby and compiles a native binary which is then loaded by Ruby.

FFI, on the other hand, uses libffi to call C from Ruby and requires dynamic
shared objects (.dylib / .so).

Also, the Ruby implementation is just to have an idea of how the Rails `blank?`
method compares to the Rust implementations. I'm totally aware this is like
comparing apples and oranges as the ruby benchmark uses a regular expression.

The benchmarks are relatively basic for now but
contributions are welcome.

## How to run the benchmarks

Make sure ruby is installed with --enable-shared.

```
$ ruby-install ruby 2.6.1 -- --enable-shared
```

```
$ bundle install
$ bundle exec rake bench
```

## Results

```
Warming up --------------------------------------
 helix - blank check   114.597k i/100ms
   ffi - blank check   219.232k i/100ms
 rutie - blank check   216.659k i/100ms
  ruby - blank check   152.847k i/100ms
Calculating -------------------------------------
 helix - blank check      1.553M (± 6.2%) i/s -      7.793M in   5.042093s
   ffi - blank check      4.230M (± 4.6%) i/s -     21.266M in   5.038519s
 rutie - blank check      4.274M (± 2.7%) i/s -     21.449M in   5.023018s
  ruby - blank check      2.316M (± 2.0%) i/s -     11.616M in   5.017375s

Comparison:
 rutie - blank check:  4273585.4 i/s
   ffi - blank check:  4230487.4 i/s - same-ish: difference falls within error
  ruby - blank check:  2316146.4 i/s - 1.85x  slower
 helix - blank check:  1552843.9 i/s - 2.75x  slower
```
