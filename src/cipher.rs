pub fn cipher(key: &[u8], text: &[u8]) -> Result<Vec<u8>, String> {
  //if key.len() < 3 { // commented because it's faster to just get the 3rd and check if it's an error
  if key.get(2).is_none() {
    return Err("too small key".to_owned());
  }
  let mut ciphered = Vec::new();
  let mut iterations = 0;

  for character in text {
    ciphered.push(character ^ key.get(iterations).unwrap().to_owned());
    iterations += 1;
    if iterations > key.len() {
      iterations = 0;
    }
  }

  Ok(ciphered)
}
