fn main() -> Result<(), anyhow::Error> {
    // For now, entering this in the script - cli to come, etc
    let policy_name = "example";
    let entrypoint = "is_something";

    opa::build::policy(policy_name)
        // Assuming .rego filename is same as `policy_name`
        .add_source(format!("src/policies/{}.rego", policy_name))
        .add_entrypoint(format!("{}.{}", policy_name, entrypoint))
        .compile()?;
    Ok(())
}
