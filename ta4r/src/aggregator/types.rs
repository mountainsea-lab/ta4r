pub fn unwrap_or_err<T>(opt: Option<T>, field_name: &str) -> Result<T, String> {
    opt.ok_or_else(|| format!("missing {}", field_name))
}
