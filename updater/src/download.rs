use crate::{Error, Result};
use core::fmt::{self, Display, Formatter};
use core::str::{self, FromStr};
use pgp::types::PublicKeyTrait;
use pgp::{SignedPublicKey, StandaloneSignature};
use ring::digest;
use std::io::{self, Read};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Sha1([u8; 32]);
impl Sha1 {
  pub fn try_from_slice(slice: &[u8]) -> Option<Self> {
    Some(Sha1(slice.try_into().ok()?))
  }

  pub fn calculate_read(mut r: impl Read) -> io::Result<Self> {
    let mut hasher = digest::Context::new(&digest::SHA256);
    let mut buf = [0u8; 1024 * 64];
    loop {
      let amount = r.read(&mut buf)?;
      if amount == 0 {
        return Ok(Self::try_from_slice(hasher.finish().as_ref()).unwrap());
      }
      hasher.update(&buf[..amount]);
    }
  }
}
impl FromStr for Sha1 {
  type Err = ();
  fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
    fn read_digit(x: u8) -> core::result::Result<u8, ()> {
      match x {
        b'a'..=b'f' => Ok(x - (b'a' - 10)),
        b'A'..=b'F' => Ok(x - (b'A' - 10)),
        b'0'..=b'9' => Ok(x - b'0'),
        _ => Err(()),
      }
    }

    if s.len() != 64 {
      return Err(());
    }
    let mut buf = [0u8; 32];
    for (dst, bytes) in buf.iter_mut().zip(s.as_bytes().chunks(2)) {
      *dst = (read_digit(bytes[0])? << 4) | read_digit(bytes[1])?;
    }
    Ok(Self(buf))
  }
}
impl Display for Sha1 {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    static LUT: [u8; 16] = [
      b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'a', b'b', b'c', b'd', b'e',
      b'f',
    ];
    let mut buf = [0u8; 64];
    for (dst, &x) in buf.chunks_mut(2).zip(self.0.iter()) {
      dst[0] = LUT[(x >> 4) as usize];
      dst[1] = LUT[(x & 0xf) as usize];
    }
    f.write_str(str::from_utf8(&buf).unwrap())
  }
}

pub fn download_file(agent: &ureq::Agent, name: &'static str, url: &str) -> Result<Vec<u8>> {
  let mut buf = Vec::new();
  agent
    .get(url)
    .call()
    .map_err(|e| Error::AssetHttp(name, Box::new(e)))?
    .into_reader()
    .read_to_end(&mut buf)
    .map_err(|e| Error::AssetIo(name, e))?;
  Ok(buf)
}

pub fn checksum_and_verify_file(
  agent: &ureq::Agent,
  name: &'static str,
  url: &str,
  headers: &[(&str, &str)],
  expected: Option<&Sha1>,
  pgp_sig: Option<(&SignedPublicKey, &StandaloneSignature)>,
) -> Result<Sha1> {
  let mut req = agent.get(url);
  for &(header, value) in headers {
    req = req.set(header, value);
  }
  let mut response = req
    .call()
    .map_err(|e| Error::AssetHttp(name, Box::new(e)))?
    .into_reader();

  let sha1 = if let Some((key, sig)) = pgp_sig {
    let mut content = Vec::new();
    response
      .read_to_end(&mut content)
      .map_err(|e| Error::AssetIo(name, e))?;
    let verified = (key.is_signing_key() && sig.verify(key, &content).is_ok())
      || key
        .public_subkeys
        .iter()
        .any(|key| key.is_signing_key() && sig.verify(key, &content).is_ok());
    if !verified {
      return Err(Error::AssetSigFailed(name));
    }
    Sha1::try_from_slice(digest::digest(&digest::SHA256, &content).as_ref()).unwrap()
  } else {
    Sha1::calculate_read(response).map_err(|e| Error::AssetIo(name, e))?
  };

  if let Some(expected) = expected {
    if *expected != sha1 {
      return Err(Error::AssetChecksumMismatch(name));
    }
  }
  Ok(sha1)
}
