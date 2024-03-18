(component $root(core module $MockMemory
        (func $realloc (export "realloc") (param i32 i32 i32 i32) (result i32)
            (i32.const 0)
        )
        (memory $memory (export "memory") 15)
    )
    (core instance $memory (instantiate $MockMemory))
    (import "wasi:cli/terminal-input" (instance $wasi:cli/terminal-input
        (export $std::cli::TerminalInput "terminal-input" (type (sub resource)))
    ))
    (alias export $wasi:cli/terminal-input "terminal-input" (type $std::cli::TerminalInput))
    (import "wasi:cli/terminal-output" (instance $wasi:cli/terminal-output
        (export $std::cli::TerminalOutput "terminal-output" (type (sub resource)))
    ))
    (alias export $wasi:cli/terminal-output "terminal-output" (type $std::cli::TerminalOutput))
    (import "wasi:filesystem/types" (instance $wasi:filesystem/types
        (export $std::fs::Descriptor "descriptor" (type (sub resource)))
    ))
    (alias export $wasi:filesystem/types "descriptor" (type $std::fs::Descriptor))
    (import "wasi:io/streams" (instance $wasi:io/streams
        (export $std::io::InputStream "input-stream" (type (sub resource)))
        (export $std::io::OutputStream "output-stream" (type (sub resource)))
        (export "[method]input-stream.read" (func
        ))
        (export "[method]output-stream.blocking-write-and-flush" (func
        ))
        (export "[method]flush" (func
        ))
        (export "[method]output-stream.write" (func
        ))
    ))
    (alias export $wasi:io/streams "input-stream" (type $std::io::InputStream))
    (alias export $wasi:io/streams "output-stream" (type $std::io::OutputStream))
    (alias export $wasi:io/streams "[method]input-stream.read" (func $std::io::InputStream::read))
    (alias export $wasi:io/streams "[method]output-stream.blocking-write-and-flush" (func $std::io::OutputStream::blocking_write_and_flush))
    (alias export $wasi:io/streams "[method]flush" (func $std::io::OutputStream::flush))
    (alias export $wasi:io/streams "[method]output-stream.write" (func $std::io::OutputStream::write))
    (import "wasi:io/error" (instance $wasi:io/error
        (export $std::io::IoError "error" (type (sub resource)))
    ))
    (alias export $wasi:io/error "error" (type $std::io::IoError))
    (import "wasi:cli/stderr" (instance $wasi:cli/stderr
        (export "get-stderr" (func
        ))
    ))
    (alias export $wasi:cli/stderr "get-stderr" (func $std::io::standard_error))
    (import "wasi:cli/stdin" (instance $wasi:cli/stdin
        (export "get-stdin" (func
        ))
    ))
    (alias export $wasi:cli/stdin "get-stdin" (func $std::io::standard_input))
    (import "wasi:cli/stdout" (instance $wasi:cli/stdout
        (export "get-stdout" (func
        ))
    ))
    (alias export $wasi:cli/stdout "get-stdout" (func $std::io::standard_output))
    (import "wasi:random/insecure" (instance $wasi:random/insecure
        (export "get-insecure-random-bytes" (func
        ))
    ))
    (alias export $wasi:random/insecure "get-insecure-random-bytes" (func $std::rand::fast_random_seed))
    (import "wasi:random/random" (instance $wasi:random/random
        (export "get-random-u64" (func
        ))
    ))
    (alias export $wasi:random/random "get-random-u64" (func $std::rand::safe_random_seed))
    (import "wasi:clocks/monotonic-clock" (instance $wasi:clocks/monotonic-clock
        (export "now" (func
        ))
        (export "resolution" (func
        ))
    ))
    (alias export $wasi:clocks/monotonic-clock "now" (func $std::time::now))
    (alias export $wasi:clocks/monotonic-clock "resolution" (func $std::time::resolution))
    (import "unstable:debugger/print" (instance $unstable:debugger/print
        (export "print-bool" (func
            (param "value" bool)
        ))
        (export "print-char" (func
            (param "value" char)
        ))
        (export "print-i64" (func
            (param "value" s16)
        ))
        (export "print-i64" (func
            (param "value" s32)
        ))
        (export "print-i64" (func
            (param "value" s64)
        ))
        (export "print-i64" (func
            (param "value" s8)
        ))
        (export "print-u16" (func
            (param "value" u16)
        ))
        (export "print-u32" (func
            (param "value" u32)
        ))
        (export "print-i64" (func
            (param "value" u64)
        ))
        (export "print-u8" (func
            (param "value" u8)
        ))
    ))
    (alias export $unstable:debugger/print "print-bool" (func $std::time::print_bool))
    (alias export $unstable:debugger/print "print-char" (func $std::time::print_char))
    (alias export $unstable:debugger/print "print-i64" (func $std::time::print_i16))
    (alias export $unstable:debugger/print "print-i64" (func $std::time::print_i32))
    (alias export $unstable:debugger/print "print-i64" (func $std::time::print_i64))
    (alias export $unstable:debugger/print "print-i64" (func $std::time::print_i8))
    (alias export $unstable:debugger/print "print-u16" (func $std::time::print_u16))
    (alias export $unstable:debugger/print "print-u32" (func $std::time::print_u32))
    (alias export $unstable:debugger/print "print-i64" (func $std::time::print_u64))
    (alias export $unstable:debugger/print "print-u8" (func $std::time::print_u8))
    (import "wasi:clocks/wall-clock" (instance $wasi:clocks/wall-clock
        (export "resolution" (func
        ))
        (export "now" (func
        ))
    ))
    (alias export $wasi:clocks/wall-clock "resolution" (func $std::time::unix_resolution))
    (alias export $wasi:clocks/wall-clock "now" (func $std::time::unix_time))
    (core func $std::io::InputStream::read (canon lower
        (func $wasi:io/streams "[method]input-stream.read")
    ))
    (core func $std::io::OutputStream::blocking_write_and_flush (canon lower
        (func $wasi:io/streams "[method]output-stream.blocking-write-and-flush")
    ))
    (core func $std::io::OutputStream::flush (canon lower
        (func $wasi:io/streams "[method]flush")
    ))
    (core func $std::io::OutputStream::write (canon lower
        (func $wasi:io/streams "[method]output-stream.write")
    ))
    (core func $std::io::standard_error (canon lower
        (func $wasi:cli/stderr "get-stderr")
    ))
    (core func $std::io::standard_input (canon lower
        (func $wasi:cli/stdin "get-stdin")
    ))
    (core func $std::io::standard_output (canon lower
        (func $wasi:cli/stdout "get-stdout")
    ))
    (core func $std::rand::fast_random_seed (canon lower
        (func $wasi:random/insecure "get-insecure-random-bytes")
    ))
    (core func $std::rand::safe_random_seed (canon lower
        (func $wasi:random/random "get-random-u64")
    ))
    (core func $std::time::now (canon lower
        (func $wasi:clocks/monotonic-clock "now")
    ))
    (core func $std::time::resolution (canon lower
        (func $wasi:clocks/monotonic-clock "resolution")
    ))
    (core func $std::time::print_bool (canon lower
        (func $unstable:debugger/print "print-bool")
    ))
    (core func $std::time::print_char (canon lower
        (func $unstable:debugger/print "print-char")
    ))
    (core func $std::time::print_i16 (canon lower
        (func $unstable:debugger/print "print-i64")
    ))
    (core func $std::time::print_i32 (canon lower
        (func $unstable:debugger/print "print-i64")
    ))
    (core func $std::time::print_i64 (canon lower
        (func $unstable:debugger/print "print-i64")
    ))
    (core func $std::time::print_i8 (canon lower
        (func $unstable:debugger/print "print-i64")
    ))
    (core func $std::time::print_u16 (canon lower
        (func $unstable:debugger/print "print-u16")
    ))
    (core func $std::time::print_u32 (canon lower
        (func $unstable:debugger/print "print-u32")
    ))
    (core func $std::time::print_u64 (canon lower
        (func $unstable:debugger/print "print-i64")
    ))
    (core func $std::time::print_u8 (canon lower
        (func $unstable:debugger/print "print-u8")
    ))
    (core func $std::time::unix_resolution (canon lower
        (func $wasi:clocks/wall-clock "resolution")
    ))
    (core func $std::time::unix_time (canon lower
        (func $wasi:clocks/wall-clock "now")
    ))
    (core module $Main
        (import "wasi:io/streams" "[method]input-stream.read" (func $std::io::InputStream::read
        ))
        (import "wasi:io/streams" "[method]output-stream.blocking-write-and-flush" (func $std::io::OutputStream::blocking_write_and_flush
        ))
        (import "wasi:io/streams" "[method]flush" (func $std::io::OutputStream::flush
        ))
        (import "wasi:io/streams" "[method]output-stream.write" (func $std::io::OutputStream::write
        ))
        (import "wasi:cli/stderr" "get-stderr" (func $std::io::standard_error
        ))
        (import "wasi:cli/stdin" "get-stdin" (func $std::io::standard_input
        ))
        (import "wasi:cli/stdout" "get-stdout" (func $std::io::standard_output
        ))
        (import "wasi:random/insecure" "get-insecure-random-bytes" (func $std::rand::fast_random_seed
        ))
        (import "wasi:random/random" "get-random-u64" (func $std::rand::safe_random_seed
        ))
        (import "wasi:clocks/monotonic-clock" "now" (func $std::time::now
        ))
        (import "wasi:clocks/monotonic-clock" "resolution" (func $std::time::resolution
        ))
        (import "unstable:debugger/print" "print-bool" (func $std::time::print_bool
            (param $value i32)
        ))
        (import "unstable:debugger/print" "print-char" (func $std::time::print_char
            (param $value i32)
        ))
        (import "unstable:debugger/print" "print-i64" (func $std::time::print_i16
            (param $value i32)
        ))
        (import "unstable:debugger/print" "print-i64" (func $std::time::print_i32
            (param $value i32)
        ))
        (import "unstable:debugger/print" "print-i64" (func $std::time::print_i64
            (param $value i64)
        ))
        (import "unstable:debugger/print" "print-i64" (func $std::time::print_i8
            (param $value i32)
        ))
        (import "unstable:debugger/print" "print-u16" (func $std::time::print_u16
            (param $value i32)
        ))
        (import "unstable:debugger/print" "print-u32" (func $std::time::print_u32
            (param $value i32)
        ))
        (import "unstable:debugger/print" "print-i64" (func $std::time::print_u64
            (param $value i64)
        ))
        (import "unstable:debugger/print" "print-u8" (func $std::time::print_u8
            (param $value i32)
        ))
        (import "wasi:clocks/wall-clock" "resolution" (func $std::time::unix_resolution
        ))
        (import "wasi:clocks/wall-clock" "now" (func $std::time::unix_time
        ))
        (func $std::time::let_us_random
        )
        (func $std::time::test
        )
    )
    (core instance $main (instantiate $Main
        (with "wasi:cli/terminal-input" (instance
        ))
        (with "wasi:cli/terminal-output" (instance
        ))
        (with "wasi:filesystem/types" (instance
        ))
        (with "wasi:io/streams" (instance
            (export "[method]input-stream.read" (func $std::io::InputStream::read))
            (export "[method]output-stream.blocking-write-and-flush" (func $std::io::OutputStream::blocking_write_and_flush))
            (export "[method]flush" (func $std::io::OutputStream::flush))
            (export "[method]output-stream.write" (func $std::io::OutputStream::write))
        ))
        (with "wasi:io/error" (instance
        ))
        (with "wasi:cli/stderr" (instance
            (export "get-stderr" (func $std::io::standard_error))
        ))
        (with "wasi:cli/stdin" (instance
            (export "get-stdin" (func $std::io::standard_input))
        ))
        (with "wasi:cli/stdout" (instance
            (export "get-stdout" (func $std::io::standard_output))
        ))
        (with "wasi:random/insecure" (instance
            (export "get-insecure-random-bytes" (func $std::rand::fast_random_seed))
        ))
        (with "wasi:random/random" (instance
            (export "get-random-u64" (func $std::rand::safe_random_seed))
        ))
        (with "wasi:clocks/monotonic-clock" (instance
            (export "now" (func $std::time::now))
            (export "resolution" (func $std::time::resolution))
        ))
        (with "unstable:debugger/print" (instance
            (export "print-bool" (func $std::time::print_bool))
            (export "print-char" (func $std::time::print_char))
            (export "print-i64" (func $std::time::print_i16))
            (export "print-i64" (func $std::time::print_i32))
            (export "print-i64" (func $std::time::print_i64))
            (export "print-i64" (func $std::time::print_i8))
            (export "print-u16" (func $std::time::print_u16))
            (export "print-u32" (func $std::time::print_u32))
            (export "print-i64" (func $std::time::print_u64))
            (export "print-u8" (func $std::time::print_u8))
        ))
        (with "wasi:clocks/wall-clock" (instance
            (export "resolution" (func $std::time::unix_resolution))
            (export "now" (func $std::time::unix_time))
        ))
    ))
)