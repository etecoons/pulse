use crate::app::App;
use crate::app::Msg;
use yew::prelude::*;

impl App {
    pub fn view_login(&self, ctx: &Context<Self>) -> Html {
        let pin_len = self.pin_length;
        let error_html = self.error_message.as_ref().map(|err| {
            html! { <p id="pin-error" class="pin-error" style="display: block;">{err}</p> }
        });

        html! {
            <div class="login-container">
                <div class="login-box">
                    <div class="login-header" style="margin-bottom: 2rem;">
                        <div class="login-icon-frame" style="display: flex; justify-content: center; margin-bottom: 1rem;">
                            <img src="/favicon.svg" class="login-app-icon" alt="Pulse" style="width: 64px; height: 64px;" />
                        </div>
                        <h2 style="font-size: 1.5rem; font-weight: 500; color: var(--text); opacity: 0.8; line-height: 1.2;">
                            {"ENTER SECURITY PIN"}
                        </h2>
                    </div>
                    <form id="pin-form" onsubmit={ctx.link().callback(|e: SubmitEvent| { e.prevent_default(); Msg::SubmitPin })}>
                        <div class="pin-wrapper">
                            <input
                                type="password"
                                class="pin-input-field"
                                value={self.pin_input.clone()}
                                oninput={ctx.link().callback(|e: InputEvent| {
                                    let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                    Msg::PinInputChanged(input.value())
                                })}
                                placeholder={"• ".repeat(pin_len).trim().to_string()}
                                maxlength={pin_len.to_string()}
                                autofocus=true
                            />
                        </div>
                    </form>
                    <div class="pin-status">
                        {error_html}
                    </div>
                </div>
            </div>
        }
    }

    pub fn view_hud(&self, ctx: &Context<Self>) -> Html {
        let uptime_str = if let Some(stats) = &self.stats {
            let uptime_hours = stats.uptime as f32 / 3600.0;
            if uptime_hours >= 24.0 {
                format!("{:.1} Days", uptime_hours / 24.0)
            } else {
                format!("{:.1} Hours", uptime_hours)
            }
        } else {
            "--".to_string()
        };

        html! {
            <div class="hud-visor-container">
                <div class="hud-visor-grid">
                    // CPU Card
                    <div class="hud-metric-card">
                        <h3>{"PROCESSOR LOAD"}</h3>
                        {if let Some(stats) = &self.stats {
                            html! {
                                <div class="card-metric-block">
                                    <div class="card-main-val">{format!("{:.1}%", stats.cpu_global)}</div>
                                    <div class="card-subtext">{format!("{} Threads", stats.cpu_cores.len())}</div>
                                    <div class="hud-bar-frame">
                                        <div class="hud-bar-fill" style={format!("width: {}%;", stats.cpu_global)}></div>
                                    </div>
                                </div>
                            }
                        } else {
                            html! { <div class="card-loading">{"Loading..."}</div> }
                        }}
                    </div>

                    // RAM Card
                    <div class="hud-metric-card">
                        <h3>{"SYSTEM MEMORY"}</h3>
                        {if let Some(stats) = &self.stats {
                            let ram_used_gb = stats.ram_used as f32 / 1024.0 / 1024.0 / 1024.0;
                            let ram_total_gb = stats.ram_total as f32 / 1024.0 / 1024.0 / 1024.0;
                            let ram_percent = (stats.ram_used as f32 / stats.ram_total as f32 * 100.0).min(100.0).max(0.0);
                            html! {
                                <div class="card-metric-block">
                                    <div class="card-main-val">{format!("{:.1} / {:.1} GB", ram_used_gb, ram_total_gb)}</div>
                                    <div class="card-subtext">{format!("{:.1}% Used", ram_percent)}</div>
                                    <div class="hud-bar-frame">
                                        <div class="hud-bar-fill" style={format!("width: {}%;", ram_percent)}></div>
                                    </div>
                                </div>
                            }
                        } else {
                            html! { <div class="card-loading">{"Loading..."}</div> }
                        }}
                    </div>

                    // Storage Card
                    <div class="hud-metric-card">
                        <h3>{"DISK STORAGE"}</h3>
                        {if let Some(stats) = &self.stats {
                            let disk_used_gb = stats.disk_used as f32 / 1024.0 / 1024.0 / 1024.0;
                            let disk_total_gb = stats.disk_total as f32 / 1024.0 / 1024.0 / 1024.0;
                            let disk_percent = (stats.disk_used as f32 / stats.disk_total as f32 * 100.0).min(100.0).max(0.0);
                            html! {
                                <div class="card-metric-block">
                                    <div class="card-main-val">{format!("{:.1} / {:.1} GB", disk_used_gb, disk_total_gb)}</div>
                                    <div class="card-subtext">{format!("{:.1}% Used", disk_percent)}</div>
                                    <div class="hud-bar-frame">
                                        <div class="hud-bar-fill" style={format!("width: {}%;", disk_percent)}></div>
                                    </div>
                                </div>
                            }
                        } else {
                            html! { <div class="card-loading">{"Loading..."}</div> }
                        }}
                    </div>

                    // Network Card
                    <div class="hud-metric-card">
                        <h3>{"NETWORK TRAFFIC"}</h3>
                        {if let Some(stats) = &self.stats {
                            html! {
                                <div class="card-metric-block">
                                    <div class="card-main-val download-glow">{format!("↓ {}", self.format_bytes(stats.net_in))}</div>
                                    <div class="card-subtext upload-glow">{format!("↑ {}", self.format_bytes(stats.net_out))}</div>
                                </div>
                            }
                        } else {
                            html! { <div class="card-loading">{"Loading..."}</div> }
                        }}
                    </div>

                    // GPU Card
                    <div class="hud-metric-card">
                        <h3>{"GPU ACCELERATION"}</h3>
                        {if let Some(stats) = &self.stats {
                            if let Some(gpu) = &stats.gpu {
                                html! {
                                    <div class="card-metric-block">
                                        <div class="card-main-val">{format!("{:.0}%", gpu.usage)}</div>
                                        <div class="card-subtext" title={gpu.name.clone()}>{&gpu.name}</div>
                                        <div class="hud-bar-frame">
                                            <div class="hud-bar-fill" style={format!("width: {}%;", gpu.usage)}></div>
                                        </div>
                                    </div>
                                }
                            } else {
                                html! {
                                    <div class="card-metric-block">
                                        <div class="card-main-val" style="color: var(--text-muted); font-size: 1.5rem;">{"OFFLINE"}</div>
                                        <div class="card-subtext">{"No Active GPU"}</div>
                                    </div>
                                }
                            }
                        } else {
                            html! { <div class="card-loading">{"Loading..."}</div> }
                        }}
                    </div>
                </div>

                <div class="hud-console-wrapper">
                    <div class="hud-console-header">
                        <span>{"CONSOLE MONITOR TERMINAL"}</span>
                        <div class="hud-console-controls">
                            <span>{format!("Uptime: {}", uptime_str)}</span>
                            <button onclick={ctx.link().callback(|_| Msg::ClearTerminal)}>{"CLEAR"}</button>
                            <button onclick={ctx.link().callback(|_| Msg::ToggleTerminal)}>
                                {if self.terminal_open { "COLLAPSE" } else { "EXPAND" }}
                            </button>
                        </div>
                    </div>
                    {if self.terminal_open {
                        html! {
                            <div class="hud-console-body">
                                {for self.terminal_logs.iter().rev().map(|log| {
                                    html! { <div class="console-line">{log}</div> }
                                })}
                            </div>
                        }
                    } else {
                        html! {}
                    }}
                </div>
            </div>
        }
    }

    fn format_bytes(&self, bytes: u64) -> String {
        if bytes >= 1024 * 1024 {
            format!("{:.2} MB/s", bytes as f64 / 1024.0 / 1024.0)
        } else if bytes >= 1024 {
            format!("{:.1} KB/s", bytes as f64 / 1024.0)
        } else {
            format!("{} B/s", bytes)
        }
    }
}
