use crypto::digest::Digest;
use crypto::md5::Md5;

static SALT: &str = "CloudThinkSpacke";

pub fn generate_md5(password: String) -> String {
    let mut md5 = Md5::new();
    md5.input_str((password + SALT).as_str());
    md5.result_str()
}

pub fn check_password(old_password: String, new_password: String) -> bool {
    let new_password = generate_md5(new_password);
    new_password == old_password
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let old_password = generate_md5("123456".to_string());
        let ok = check_password(old_password, "123456".to_string());
        assert!(ok);
    }
}