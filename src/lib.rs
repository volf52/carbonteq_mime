#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

#[napi]
#[derive(Debug, Clone)]
pub struct Mime(mime::Mime);

#[napi]
impl Mime {
  #[napi]
  pub fn parse_str(mime_str: String) -> Option<Mime> {
    match mime_str.parse::<mime::Mime>() {
      Err(_) => None,
      Ok(mm) => Some(Self(mm)),
    }
  }

  #[napi]
  pub fn to_str(&self) -> String {
    self.0.essence_str().to_string()
  }

  #[napi]
  pub fn TEXT_PLAIN() -> Mime {
    Self(mime::TEXT_PLAIN)
  }

  #[napi]
  pub fn TEXT_HTML() -> Mime {
    Self(mime::TEXT_HTML)
  }

  #[napi]
  pub fn JSON() -> Mime {
    Self(mime::APPLICATION_JSON)
  }

  #[napi]
  pub fn JPG() -> Mime {
    Self(mime::IMAGE_JPEG)
  }

  #[napi]
  pub fn PNG() -> Mime {
    Self(mime::IMAGE_PNG)
  }

  #[napi]
  pub fn is_valid_mime(mime_str: String) -> bool {
    mime_str.parse::<mime::Mime>().is_ok()
  }

  // #[napi(getter)]
  // pub fn type_(&self) -> String {
  //   self.0.type_().to_string()
  // }

  #[napi]
  pub fn is_text(&self) -> bool {
    self.0.type_() == mime::TEXT
  }

  #[napi]
  pub fn is_image(&self) -> bool {
    self.0.type_() == mime::IMAGE
  }
}
