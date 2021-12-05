use sha2::{Digest, Sha256};

static SLASH: u8 = 0x2F;

pub fn encrypt(secret: String, file_path: String) -> String {
    let (fp, ext) = split_extension(file_path);

    let key = join_path(secret, fp.clone());
    let mut hasher = Sha256::new();
    hasher.update(key);
    let result = hasher.finalize();
    if let Some(ext) = ext {
        return format!("{}.{:x}.{}", fp, result, ext);
    }
    return format!("{}.{:x}", fp, result);
}

fn join_path(secret: String, file_path: String) -> String {
    let mut vec: Vec<u8> = secret.bytes().collect();
    let mut fp_bs: Vec<u8> = file_path.bytes().collect();

    if fp_bs[0] != SLASH {
        vec.push(SLASH);
    }
    vec.append(&mut fp_bs);
    return String::from_utf8(vec).unwrap();
}

fn split_extension(file_path: String) -> (String, Option<String>) {
    let l = file_path.len();
    let mut i = l;
    for c in file_path.chars().rev() {
        if c == '.' {
            break;
        }
        if c == '/' {
            return (file_path, None);
        }
        i -= 1;
    }
    if i == l || i < 2 {
        return (file_path, None);
    }
    if file_path.chars().nth(i - 2).unwrap() == '/' {
        return (file_path, None);
    }
    let (f, e) = file_path.split_at(i - 1);
    return (String::from(f), Some(String::from(&e[1..e.len()])));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_join() {
        assert_eq!(
            join_path(String::from("1234567"), String::from("test.png")),
            String::from("1234567/test.png")
        );
        assert_eq!(
            join_path(String::from("1234567"), String::from("/test.png")),
            String::from("1234567/test.png")
        );
        assert_eq!(
            join_path(String::from("1234567"), String::from("hoge/test.png")),
            String::from("1234567/hoge/test.png")
        );
        assert_eq!(
            join_path(String::from("1234567"), String::from("/hoge/test.png")),
            String::from("1234567/hoge/test.png")
        );
    }

    #[test]
    fn test_split_extension() {
        assert_eq!(
            split_extension(String::from("test.png")),
            (String::from("test"), Some(String::from("png"))),
        );
        assert_eq!(
            split_extension(String::from("test.test.png")),
            (String::from("test.test"), Some(String::from("png"))),
        );
        assert_eq!(
            split_extension(String::from("test.")),
            (String::from("test."), None),
        );
        assert_eq!(
            split_extension(String::from(".testpng")),
            (String::from(".testpng"), None),
        );
        assert_eq!(
            split_extension(String::from("hoge/.testpng")),
            (String::from("hoge/.testpng"), None),
        );

        // starts with slash
        assert_eq!(
            split_extension(String::from("/test.png")),
            (String::from("/test"), Some(String::from("png"))),
        );
        assert_eq!(
            split_extension(String::from("/test.test.png")),
            (String::from("/test.test"), Some(String::from("png"))),
        );
        assert_eq!(
            split_extension(String::from("/test.")),
            (String::from("/test."), None),
        );
        assert_eq!(
            split_extension(String::from("/.testpng")),
            (String::from("/.testpng"), None),
        );
        assert_eq!(
            split_extension(String::from("/hoge/.testpng")),
            (String::from("/hoge/.testpng"), None),
        );
    }
    #[test]
    fn test_encrypt() {
        assert_eq!(
            encrypt(String::from("abcdefg"), String::from("test.png")),
            String::from(
                "test.005b6e0de4e3fb99294e49326eaf21682f5977ce684448d82685f31f277f763c.png"
            ),
        );
        assert_eq!(
            encrypt(String::from("abcdefg"), String::from("test.webp")),
            String::from(
                "test.005b6e0de4e3fb99294e49326eaf21682f5977ce684448d82685f31f277f763c.webp"
            ),
        );
        assert_eq!(
            encrypt(String::from("abcdefg"), String::from("/test.webp")),
            String::from(
                "/test.005b6e0de4e3fb99294e49326eaf21682f5977ce684448d82685f31f277f763c.webp"
            ),
        );
        assert_eq!(
            encrypt(String::from("abcdefg"), String::from("test")),
            String::from("test.005b6e0de4e3fb99294e49326eaf21682f5977ce684448d82685f31f277f763c"),
        );
        assert_eq!(
            encrypt(String::from("abcdefg"), String::from(".test")),
            String::from(".test.919db07caab34978b98cc5c9b85ec315a6143d8550c0d7903fa01116e0843525"),
        );
        assert_eq!(
            encrypt(String::from("abcdefg"), String::from(".test.png")),
            String::from(
                ".test.919db07caab34978b98cc5c9b85ec315a6143d8550c0d7903fa01116e0843525.png"
            ),
        );

        // empty secret
        assert_eq!(
            encrypt("".to_string(), "dev/src/main.rs".to_string()),
            "dev/src/main.908ead2016a3c13dc55e410aeb6a80fb525256d56d7c2e27d017a5a866899f68.rs"
                .to_string(),
        )
    }
}
