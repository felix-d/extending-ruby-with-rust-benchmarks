require 'helix_runtime'
require 'ffi'
require 'ffi_benchmarks/native'
require 'benchmark/ips'

module FFIBindings
  extend FFI::Library
  ffi_lib 'target/release/libffi_benchmarks.dylib'
  attach_function :ffi_is_blank, [:string], :bool
end


class String
  def helix_is_blank?
    HelixBenchmark.is_blank(self)
  end

  def ffi_is_blank?
    FFIBindings.ffi_is_blank(self)
  end

  BLANK_RE = /\A[[:space:]]*\z/

  def ruby_blank?
    BLANK_RE === self
  end
end

module Bench
  def self.run
    Benchmark.ips do |bench|
      bench.report("helix - blank check") do
        "foo".helix_is_blank?
      end

      bench.report("ffi - blank check") do
        "foo".ffi_is_blank?
      end

      bench.report("ruby - blank check") do
        "foo".ruby_blank?
      end

      bench.compare!
    end
  end
end
