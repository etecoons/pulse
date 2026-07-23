use gloo_net::http::Request;
use gloo_timers::callback::Timeout;
use serde_json::Value;
use shared_frontend::i18n::strings::{StringKey, lookup};
use shared_frontend::storage::StorageService;
use yew::prelude::*;

use crate::app::App;
use crate::app::Msg;
use crate::i18n::save_language;
use crate::types::SystemStats;

#[derive(serde::Serialize)]
struct VerifyPinPayload {
    pin: Option<String>,
}

impl App {
    pub fn update_app(&mut self, ctx: &Context<Self>, msg: Msg) -> bool {
        match msg {
            Msg::LoadConfig(json) => {
                self.site_title = json["siteTitle"].as_str().unwrap_or("Pulse").to_string();
                self.pin_required = json["pinRequired"].as_bool().unwrap_or(false);
                self.pin_length = json["pinLength"].as_u64().unwrap_or(0) as usize;
                self.enable_translation = json["enableTranslation"].as_bool().unwrap_or(false);
                self.enable_themes = json["enableThemes"].as_bool().unwrap_or(true);
                self.enable_print = json["enablePrint"].as_bool().unwrap_or(false);
                self.monitor_cpu = json["monitorCpu"].as_bool().unwrap_or(true);
                self.monitor_memory = json["monitorMemory"].as_bool().unwrap_or(true);
                self.monitor_storage = json["monitorStorage"].as_bool().unwrap_or(true);
                self.monitor_network = json["monitorNetwork"].as_bool().unwrap_or(true);
                self.monitor_gpu = json["monitorGpu"].as_bool().unwrap_or(true);
                self.enable_coffee = json["enableCoffee"].as_bool().unwrap_or(true);

                if !self.pin_required {
                    self.is_authenticated = true;
                    self.connect_ws(ctx);
                } else {
                    let link = ctx.link().clone();
                    wasm_bindgen_futures::spawn_local(async move {
                        if let Ok(resp) = Request::get("/api/auth-check").send().await {
                            if resp.status() == 200 {
                                link.send_message(Msg::PinResponse(true, None, None, None));
                            }
                        }
                    });
                }
                true
            }
            Msg::PinInputChanged(val) => {
                let filtered: String = val.chars().filter(|c| c.is_ascii_digit()).collect();
                self.pin_input = filtered;
                if self.pin_input.len() == self.pin_length {
                    ctx.link().send_message(Msg::SubmitPin);
                }
                true
            }
            Msg::SubmitPin => {
                let pin = self.pin_input.clone();
                let link = ctx.link().clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let payload = VerifyPinPayload { pin: Some(pin) };
                    if let Ok(req) = Request::post("/api/verify-pin").json(&payload)
                        && let Ok(resp) = req.send().await
                    {
                        if resp.status() == 200 {
                            link.send_message(Msg::PinResponse(true, None, None, None));
                        } else if let Ok(json) = resp.json::<Value>().await {
                            let err = json["error"]
                                .as_str()
                                .unwrap_or("Verification failed")
                                .to_string();
                            let attempts = json["attemptsLeft"].as_u64().map(|v| v as usize);
                            let lockout = json["lockoutMinutes"].as_u64().or_else(|| {
                                if err.contains("Please try again in") {
                                    err.split("Please try again in ")
                                        .nth(1)
                                        .and_then(|s| s.split(' ').next())
                                        .and_then(|s| s.parse::<u64>().ok())
                                } else {
                                    None
                                }
                            });
                            link.send_message(Msg::PinResponse(
                                false,
                                Some(err),
                                attempts,
                                lockout,
                            ));
                        }
                    }
                });
                true
            }
            Msg::PinResponse(success, error, attempts_left, lockout_minutes) => {
                self.handle_pin_response(ctx, success, error, attempts_left, lockout_minutes)
            }
            Msg::Logout => {
                let link = ctx.link().clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let _ = Request::post("/api/logout").send().await;
                    link.send_message(Msg::PinResponse(false, None, None, None));
                });
                if let Some(ws) = self.ws.take() {
                    let _ = ws.close();
                }
                self.stats = None;
                true
            }
            Msg::UpdateStats(stats) => self.handle_update_stats(stats),
            Msg::WsError(_err) => {
                self.active_notification = Some((
                    lookup(StringKey::StatusOffline, self.language).to_string(),
                    "error".to_string(),
                ));
                self.ws = None;
                let link = ctx.link().clone();
                Timeout::new(5000, move || {
                    link.send_message(Msg::Reconnect);
                })
                .forget();
                true
            }
            Msg::WsLog(msg) => {
                let (display_msg, cls) = if msg.contains("Connection established") {
                    (
                        lookup(StringKey::StatusOnline, self.language).to_string(),
                        "success",
                    )
                } else {
                    (msg.clone(), "info")
                };
                self.active_notification = Some((display_msg.clone(), cls.to_string()));
                let link = ctx.link().clone();
                Timeout::new(3000, move || {
                    link.send_message(Msg::ClearNotification(msg));
                })
                .forget();
                true
            }
            Msg::Reconnect => {
                if self.is_authenticated {
                    self.connect_ws(ctx);
                }
                true
            }
            Msg::ToggleTheme => {
                let current =
                    shared_frontend::theme::Theme::from_name(&self.theme).unwrap_or_default();
                let idx = shared_frontend::theme::Theme::ALL
                    .iter()
                    .position(|&t| t == current)
                    .unwrap_or(0);
                let next = shared_frontend::theme::Theme::ALL
                    [(idx + 1) % shared_frontend::theme::Theme::ALL.len()];
                self.theme = next.name().to_string();
                StorageService::new().set_item("theme", &self.theme);

                if let Some(window) = web_sys::window()
                    && let Some(doc) = window.document()
                    && let Some(elem) = doc.document_element()
                {
                    elem.set_class_name(&self.theme);
                    let _ = elem.set_attribute("data-theme", &self.theme);
                }
                self.show_notification(
                    ctx,
                    lookup(StringKey::StatusThemeChanged, self.language).to_string(),
                    "success".to_string(),
                );
                true
            }
            Msg::ChangeLanguage(lang) => {
                self.language = lang;
                save_language(lang);
                true
            }
            Msg::ClearNotification(msg_to_clear) => {
                if let Some((current_msg, _)) = &self.active_notification {
                    if current_msg == &msg_to_clear {
                        self.active_notification = None;
                    }
                }
                true
            }
            Msg::CycleOsOverride => self.handle_cycle_os_override(ctx),
            Msg::CheckFallback => {
                let ws_connected = self
                    .ws
                    .as_ref()
                    .map(|w| w.ready_state() == 1)
                    .unwrap_or(false);
                if !ws_connected && self.is_authenticated {
                    let link = ctx.link().clone();
                    wasm_bindgen_futures::spawn_local(async move {
                        if let Ok(resp) = Request::get("/api/stats").send().await {
                            if let Ok(stats) = resp.json::<SystemStats>().await {
                                link.send_message(Msg::UpdateStats(stats));
                            }
                        }
                    });
                }
                false
            }
            Msg::Print => {
                if let Some(window) = web_sys::window() {
                    let print_res = window.print();
                    if print_res.is_ok() {
                        self.show_notification(
                            ctx,
                            lookup(StringKey::StatusPrintSuccess, self.language).to_string(),
                            "success".to_string(),
                        );
                    } else {
                        self.show_notification(
                            ctx,
                            lookup(StringKey::StatusPrintFailure, self.language).to_string(),
                            "error".to_string(),
                        );
                    }
                }
                false
            }
        }
    }
}
