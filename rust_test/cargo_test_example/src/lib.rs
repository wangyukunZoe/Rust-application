#[cfg(test)]
mod tests {
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 3 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four."))
        }
    }
}

/*
控制测试如何运行
*   改変 cargo test 的行：添加命令行参数
*  默认行为：
-   并行运行
-   所有测试
-  捕获（不显示）所有输出，使读取与测试结果相关的输出更容易。
*  命令行参数：
-  针对 cargo test 的参数：紧跟 cargo test 后
-  针对测试可执行程序：放在--之后
*   cargo test -help
*   cargo test - --help

*/
/*
cargo test --help 显示 test 后面可以显示哪些参数
wangyukun@wangyukundeMacBook-Pro-3 cargo_test_example % cargo test --help                                                                          [0]
Execute all unit and integration tests and build examples of a local package

Usage: cargo test [OPTIONS] [TESTNAME] [-- [ARGS]...]

Arguments:
  [TESTNAME]  If specified, only run tests containing this string in their names
  [ARGS]...   Arguments for the test binary

Options:
      --no-run                   Compile, but don't run tests
      --no-fail-fast             Run all tests regardless of failure
      --future-incompat-report   Outputs a future incompatibility report at the end of the build
      --message-format <FMT>     Error format [possible values: human, short, json, json-diagnostic-short, json-diagnostic-rendered-ansi,
                                 json-render-diagnostics]
  -q, --quiet                    Display one character per test instead of one line
  -v, --verbose...               Use verbose output (-vv very verbose/build.rs output)
      --color <WHEN>             Coloring [possible values: auto, always, never]
      --config <KEY=VALUE|PATH>  Override a configuration value
  -Z <FLAG>                      Unstable (nightly-only) flags to Cargo, see 'cargo -Z help' for details
  -h, --help                     Print help

Package Selection:
  -p, --package [<SPEC>]  Package to run tests for
      --workspace         Test all packages in the workspace
      --exclude <SPEC>    Exclude packages from the test
      --all               Alias for --workspace (deprecated)

Target Selection:
      --lib               Test only this package's library
      --bins              Test all binaries
      --bin [<NAME>]      Test only the specified binary
      --examples          Test all examples
      --example [<NAME>]  Test only the specified example
      --tests             Test all targets that have `test = true` set
      --test [<NAME>]     Test only the specified test target
      --benches           Test all targets that have `bench = true` set
      --bench [<NAME>]    Test only the specified bench target
      --all-targets       Test all targets (does not include doctests)
      --doc               Test only this library's documentation

Feature Selection:
  -F, --features <FEATURES>  Space or comma separated list of features to activate
      --all-features         Activate all available features
      --no-default-features  Do not activate the `default` feature

Compilation Options:
  -j, --jobs <N>                Number of parallel jobs, defaults to # of CPUs.
  -r, --release                 Build artifacts in release mode, with optimizations
      --profile <PROFILE-NAME>  Build artifacts with the specified profile
      --target [<TRIPLE>]       Build for the target triple
      --target-dir <DIRECTORY>  Directory for all generated artifacts
      --unit-graph              Output build graph in JSON (unstable)
      --timings[=<FMTS>]        Timing output formats (unstable) (comma separated): html, json

Manifest Options:
      --manifest-path <PATH>  Path to Cargo.toml
      --lockfile-path <PATH>  Path to Cargo.lock (unstable)
      --ignore-rust-version   Ignore `rust-version` specification in packages
      --locked                Assert that `Cargo.lock` will remain unchanged
      --offline               Run without accessing the network
      --frozen                Equivalent to specifying both --locked and --offline

Run `cargo help test` for more detailed information.
Run `cargo test -- --help` for test binary options.
wangyukun@wangyukundeMacBook-Pro-3 cargo_test_example %                                                                                            [0]
*/

/*
 cargo test -- --help 显示test -- 后面可以填写哪些参数
*/

/*
并行运行测试

- ﻿运行多个测试：默认使用多个线程并行运行。

- ﻿﻿运行快

- ﻿确保测试之间：

- ﻿﻿不会互相依赖
- ﻿不依赖于某个共享状态（环境、工作目录、环境变量等等）

如果不想并行的运行测试，或者想精确的运行测试
可以用

-  -fest-threads 参数
*  传递给 二进制文件
*   不想以并行方式运行测试，或想对线程数进行细粒度控制
*  可以使用--test-threads 参数，后边跟着线程的数量
*   MI: cargo test ----test-threads=1
*/
