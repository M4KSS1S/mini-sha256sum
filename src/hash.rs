use sha2::{Digest, Sha256};

fn to_hex(bytes : &[u8]) -> String
{
    bytes.iter().map(|f| format!("{f:02x}")).collect()
}

pub fn sha256_hex(data: &[u8]) -> String
{
    let mut digest = Sha256::new();
    digest.update(data);
    let string: String = to_hex(&digest.clone().finalize());
    string
}

#[cfg(test)]
mod tests
{
    use super::*;
    #[test]
    fn test_hello()
    {
        assert_eq!(sha256_hex(b"hello"),"2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824");
    }

    #[test]
    fn test_empty()
    {
        assert_eq!(sha256_hex(b""), "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855")
    }
}