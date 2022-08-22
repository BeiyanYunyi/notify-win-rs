#![deny(clippy::all)]

use napi_derive::napi;
use std::time::Duration;
use winrt_toast::{Scenario, Toast, ToastDuration, ToastManager};

#[napi]
pub const POWERSHELL_APP_ID: &'static str =
  "{1AC14E77-02E7-4E5D-B744-2EB1AE5198B7}\\WindowsPowerShell\\v1.0\\powershell.exe";

#[napi]
pub const WINDOWS_STORE_APP_ID: &'static str = "Microsoft.WindowsStore_8wekyb3d8bbwe!App";

#[napi(js_name = "Toast")]
pub struct JsToast {
  toast: Toast,
}

#[napi]
impl JsToast {
  #[napi(constructor)]
  pub fn new() -> Self {
    JsToast {
      toast: Toast::new(),
    }
  }
  /// The first text element, usually the title.
  #[napi]
  pub fn text1(&mut self, text1: String) -> &Self {
    self.toast.text1(text1);
    self
  }
  /// The second text element, usually the body.
  #[napi]
  pub fn text2(&mut self, text2: String) -> &Self {
    self.toast.text2(text2);
    self
  }
  /// The third text element, usually the body or attribution.
  #[napi]
  pub fn text3(&mut self, text3: String) -> &Self {
    self.toast.text3(text3);
    self
  }

  /// Set the scenario of this toast.\
  /// The scenario adjusts a few behaviors to create a consistent and unified user experience.\
  /// 1 for `Scenario::Alarm`\
  /// 2 for `Scenario::IncomingCall`\
  /// 3 for `Scenario::Urgent`\
  /// other for `Scenario::Reminder`\
  /// The value must between 0 and 255.
  #[napi]
  pub fn scenario(&mut self, js_scenario: u8) -> &Self {
    let scenario: Scenario = match js_scenario {
      0 => Scenario::Reminder,
      1 => Scenario::Alarm,
      2 => Scenario::IncomingCall,
      3 => Scenario::Urgent,
      _ => Scenario::Reminder,
    };
    self.toast.scenario(scenario);
    self
  }
  /// The amount of time the toast should display.\
  /// true for `ToastDuration::Short`\
  /// false for `ToastDuration::Long`
  #[napi]
  pub fn duration(&mut self, js_duration: bool) -> &Self {
    let duration: ToastDuration = match js_duration {
      false => ToastDuration::Short,
      true => ToastDuration::Long,
    };
    self.toast.duration(duration);
    self
  }
  /// Set the tag of this toast.
  #[napi]
  pub fn tag(&mut self, tag: String) -> &Self {
    self.toast.tag(tag);
    self
  }
  /// Set the group of this toast.
  #[napi]
  pub fn group(&mut self, group: String) -> &Self {
    self.toast.group(group);
    self
  }
  /// Set a remote id for the notification that enables the system to correlate this notification with another one generated on another device.
  #[napi]
  pub fn remote_id(&mut self, remote_id: String) -> &Self {
    self.toast.remote_id(remote_id);
    self
  }
  /// Set the expiration time of this toats, starting from the moment it is shown.
  /// After expiration, the toast will be removed from the Notification Center.
  /// The value must be a u32 in milliseconds.
  #[napi]
  pub fn expires_in(&mut self, expires_in: u32) -> &Self {
    self
      .toast
      .expires_in(Duration::from_micros(expires_in as u64));
    self
  }
}

#[napi(js_name = "Toaster")]
pub struct JsToaster {
  manager: ToastManager,
}

#[napi]
impl JsToaster {
  #[napi(constructor)]
  pub fn new(app_id: Option<String>) -> Self {
    let manager = match app_id {
      Some(app_id) => ToastManager::new(app_id),
      None => ToastManager::new(WINDOWS_STORE_APP_ID),
    };
    JsToaster { manager }
  }
  /// Show a toast.
  #[napi]
  pub fn show(&self, toast: &JsToast) -> bool {
    let res = self.manager.show(&toast.toast);
    if res.is_err() {
      return false;
    }
    return true;
  }
}
