fn main() {
    let policy_name = "example";
    let entrypoint = "is_something";

    opa::build::policy(policy_name)
        // Assuming .rego filename is same as `policy_name`
        .add_source(format!("src/policies/{}.rego", policy_name))
        .add_entrypoint(format!("{}.{}", policy_name, entrypoint))
        .compile()
        .unwrap();
}