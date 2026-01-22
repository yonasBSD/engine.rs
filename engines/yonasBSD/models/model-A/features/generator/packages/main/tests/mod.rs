#[cfg(test)]
pub mod helpers;
#[cfg(test)]
pub mod integration;

#[cfg(test)]
#[ctor::ctor] // use the ctor crate to run this once before all tests
fn setup() {
    color_eyre::config::HookBuilder::default()
        .display_env_section(false) // Hide the "RUST_BACKTRACE" noise
        .install()
        .unwrap();
}
