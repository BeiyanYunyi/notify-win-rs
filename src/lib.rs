#![deny(clippy::all)]

use napi_derive::napi;
use winrt_toast::{Scenario, Toast, ToastDuration, ToastManager};

#[napi]
pub const POWERSHELL_APP_ID: &'static str =
  "{1AC14E77-02E7-4E5D-B744-2EB1AE5198B7}\\WindowsPowerShell\\v1.0\\powershell.exe";

pub struct Toaster {
  manager: ToastManager,
}

#[napi(js_name = "Toaster")]
pub struct JsToaster {
  toaster: Toaster,
}

#[napi(object)]
pub struct ToastParams {
  pub text1: Option<String>,
  pub text2: String,
  pub text3: Option<String>,
}

#[napi]
impl JsToaster {
  #[napi(constructor)]
  pub fn new(app_id: Option<String>) -> Self {
    let manager = match app_id {
      Some(app_id) => ToastManager::new(app_id),
      None => ToastManager::new(POWERSHELL_APP_ID),
    };
    JsToaster {
      toaster: Toaster { manager },
    }
  }
  #[napi]
  pub fn noti(&self, not: ToastParams) -> bool {
    let mut toast = Toast::new();
    if let Some(text1) = not.text1 {
      toast.text1(&text1);
    }
    if let Some(text3) = not.text3 {
      toast.text3(&text3);
    }
    toast
      .text2(&not.text2)
      .scenario(Scenario::Reminder)
      .duration(ToastDuration::Short);
    let res = self.toaster.manager.show(&toast);
    if res.is_err() {
      return false;
    }
    return true;
  }
}
