use compiletest_rs::common::{Config, Mode};
use std::path::PathBuf;

#[test]
fn compile_fail() {
    let mut config = Config::default();
    config.mode = Mode::CompileFail;
    config.src_base = PathBuf::from("tests/compile_fail");
    config.target_rustcflags = Some("-L ../target/debug -L ../target/debug/deps".into());
    // config.link_deps();
    config.clean_rmeta();

    compiletest_rs::run_tests(&config);
}
