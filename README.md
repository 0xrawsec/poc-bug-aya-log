# poc-bug-aya-log

repro project for issue: https://github.com/aya-rs/aya/issues/808

# Important

As the bug needs a custom `bpf-linker` to trigger, it is shipped with a compiled version of `bpf-linker`.
This latter is located under `poc-bug-aya-log-ebpf/bpf-linker` and is used by default to link the project.
