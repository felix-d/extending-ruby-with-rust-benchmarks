require 'helix_runtime/build_task'

HelixRuntime::BuildTask.new

task default: :build

task bench: :build do
  exec 'ruby -Ilib benchmark.rb'
end
