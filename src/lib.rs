#![deny(clippy::all)]

use napi_derive::napi;
use winrt_notification::{Duration, Sound, Toast};

#[napi(object)]
pub struct ToastParams {
  pub title: Option<String>,
  pub text1: String,
  pub text2: Option<String>,
}

#[napi]
pub fn noti(not: ToastParams) -> bool {
  let toast = Toast::new(Toast::POWERSHELL_APP_ID);
  let toast = match not.title {
    Some(title) => toast.title(&title),
    None => toast,
  };
  let toast = match not.text2 {
    Some(text2) => toast.text2(&text2),
    None => toast,
  };
  let res = toast
    .text1(&not.text1)
    .sound(Some(Sound::SMS))
    .duration(Duration::Short)
    .show();
  if res.is_err() {
    return false;
  }
  return true;
}
