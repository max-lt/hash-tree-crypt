use crypto::digest::Digest;
use crypto::sha1::Sha1;

pub fn hash(input: &[u8]) -> [u8; 20] {
  let mut hasher = Sha1::new();
  
  hasher.input(input);

  let mut bin: [u8; 20] = [0; 20];

  hasher.result(&mut bin);

  return bin;
}

#[test]
fn hash_password() {
  let password = "hello world";
  let mut seed = hash(password.as_bytes());

  let expect = [
    0x6d, 0xa3, 0xb6, 0x49, 0xea,
    0x46, 0xee, 0x06, 0xa7, 0x9d,
    0x61, 0xbe, 0x7b, 0x10, 0x0e,
    0x8b, 0xf3, 0x9a, 0x6e, 0x49
  ];

  seed.rotate_left(8);

  let result = hash(&seed);

  assert_eq!(result, expect);
}
