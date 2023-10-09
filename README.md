# rust-cli-example
sample command line application that demonstrates non-functional requirements
* command line arguments
* configuration from yaml
* documentation
* error handling
* global shared configuration 
* logging
* tests

## references
* [anyhow](https://docs.rs/anyhow/latest/anyhow/type.Result.html#example) for error handling - at least in main
* [clap](https://docs.rs/clap/latest/clap/) command line parameter parsing module
* [clap verbosity flag](https://docs.rs/clap-verbosity-flag/latest/clap_verbosity_flag/) is this the best way to do log verbosity?
* [command line apps in rust](https://rust-cli.github.io/book/index.html)
* [log4rs](https://github.com/estk/log4rs) logging module
* [OnceLock](https://doc.rust-lang.org/stable/std/sync/struct.OnceLock.html) for global config.  Requires rust 1.70
* [rust doc](https://doc.rust-lang.org/rustdoc/what-is-rustdoc.html) documentation
* [serde yaml](https://docs.rs/serde_yaml/latest/serde_yaml/) for working with yaml files

### testing
[assert cmd](https://docs.rs/assert_cmd/latest/assert_cmd/) for integration testing command line apps
[assert fs](https://docs.rs/assert_fs/latest/assert_fs/) for testing that requires files and directories
[predicates](https://docs.rs/predicates/latest/predicates/) simplifies assertions. May be generally useful for functional booleans

## additional resources 
* [clap complete](https://docs.rs/clap_complete/latest/clap_complete/) bash completion for clap 
* [compile time logging config](https://github.com/estk/log4rs/blob/master/examples/compile_time_config.rs) so that users don't need a config
* [figment](https://docs.rs/figment/latest/figment/) for configuration if I was sharing this with others.  Also see testing capabilities
