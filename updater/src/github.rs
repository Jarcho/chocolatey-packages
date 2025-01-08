use core::fmt;
use serde::{de, Deserialize, Deserializer};

pub struct Asset {
  pub name: String,
  pub url: String,
}

pub struct Release {
  pub tag: String,
  pub assets: Vec<Asset>,
}

impl<'de> Deserialize<'de> for Asset {
  fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
    struct V;
    impl<'de> de::Visitor<'de> for V {
      type Value = Asset;

      fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("an asset structure")
      }

      fn visit_map<A: de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut name = None;
        let mut url = None;

        while let Some(key) = map.next_key::<String>()? {
          match key.as_str() {
            "name" if name.is_some() => {
              return Err(<A::Error as de::Error>::duplicate_field("name"))
            }
            "browser_download_url" if url.is_some() => {
              return Err(<A::Error as de::Error>::duplicate_field(
                "browser_download_url",
              ))
            }
            "name" => name = Some(map.next_value()?),
            "browser_download_url" => url = Some(map.next_value()?),
            _ => {
              map.next_value::<de::IgnoredAny>()?;
            }
          }
        }

        let Some(name) = name else {
          return Err(<A::Error as de::Error>::missing_field("name"));
        };
        let Some(url) = url else {
          return Err(<A::Error as de::Error>::missing_field(
            "browser_download_url",
          ));
        };
        Ok(Asset { name, url })
      }
    }
    deserializer.deserialize_map(V)
  }
}

struct OptRelease(Option<Release>);
impl<'de> Deserialize<'de> for OptRelease {
  fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
    struct V;
    impl<'de> de::Visitor<'de> for V {
      type Value = OptRelease;

      fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("a release structure")
      }

      fn visit_map<A: de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut tag = None;
        let mut assets = None;
        let mut is_prerelease: Option<bool> = None;

        while let Some(key) = map.next_key::<String>()? {
          match key.as_str() {
            "tag_name" if tag.is_some() => {
              return Err(<A::Error as de::Error>::duplicate_field("tag_name"))
            }
            "assets" if assets.is_some() => {
              return Err(<A::Error as de::Error>::duplicate_field("assets"))
            }
            "prerelease" if is_prerelease.is_some() => {
              return Err(<A::Error as de::Error>::duplicate_field("prerelease"))
            }
            "tag_name" => tag = Some(map.next_value()?),
            "assets" => assets = Some(map.next_value()?),
            "prerelease" => is_prerelease = Some(map.next_value()?),
            _ => {
              map.next_value::<de::IgnoredAny>()?;
            }
          }
        }

        let Some(is_prerelease) = is_prerelease else {
          return Err(<A::Error as de::Error>::missing_field("prerelease"));
        };
        let Some(tag) = tag else {
          return Err(<A::Error as de::Error>::missing_field("tag"));
        };
        let Some(assets) = assets else {
          return Err(<A::Error as de::Error>::missing_field("assets"));
        };
        Ok(OptRelease(
          (!is_prerelease).then_some(Release { tag, assets }),
        ))
      }
    }
    deserializer.deserialize_map(V)
  }
}

impl<'de> Deserialize<'de> for Release {
  fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
    struct V;
    impl<'de> de::Visitor<'de> for V {
      type Value = Release;

      fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("a sequence of release structures")
      }

      fn visit_seq<A: de::SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        while let Some(x) = seq.next_element()? {
          if let OptRelease(Some(x)) = x {
            while seq.next_element::<de::IgnoredAny>()?.is_some() {}
            return Ok(x);
          }
        }
        Err(de::Error::custom("no suitable releases"))
      }
    }
    deserializer.deserialize_seq(V)
  }
}

pub trait AssetExtractor<const N: usize>: Sized {
  fn extract(
    self,
    assets: Vec<Asset>,
    names: &'static [&'static str; N],
  ) -> crate::Result<[String; N]>;
}
macro_rules! impl_asset_extractor {
    ($count:literal: $($ty:ident$(:$field:tt)? ),*) => {
        #[allow(unused_parens)]
        impl<$($ty),*> AssetExtractor<$count> for ($($ty),*)
        where $($ty: FnMut(&str) -> bool),* {
            fn extract(mut self, assets: Vec<Asset>, names: &'static [&'static str; $count]) -> crate::Result<[String; $count]> {
                let mut res = [const { String::new() }; $count];
                let mut found = [false; $count];
                for asset in assets {
                    let i = $(if (self$(.$field)?)(&asset.name) { 0 $(+ $field)? } else)* {
                        continue;
                    };
                    if found[i] {
                        return Err(crate::Error::AssetDuplicate(names[i]));
                    }
                    found[i] = true;
                    res[i] = asset.url;
                }
                if let Some(i) = found.iter().position(|x| !x) {
                    return Err(crate::Error::AssetMissing(names[i]));
                }
                Ok(res)
            }
        }
    };
}
impl_asset_extractor!(1: F0);
impl_asset_extractor!(2: F0:0, F1:1);
impl_asset_extractor!(3: F0:0, F1:1, F2:2);
impl_asset_extractor!(4: F0:0, F1:1, F2:2, F3:3);

/// Extracts the download url for the matching assets.
///
/// This will error if any assets are missing, or if there are multiple matches for any of the
/// requested assets.
pub fn extract_assets<const N: usize>(
  assets: Vec<Asset>,
  names: &'static [&'static str; N],
  extractor: impl AssetExtractor<N>,
) -> crate::Result<[String; N]> {
  extractor.extract(assets, names)
}
