use miette::Result;

pub fn run(code: String, name: String) -> Result<()> {
    crate::utils::new_error(&code, Some(&name))
}
