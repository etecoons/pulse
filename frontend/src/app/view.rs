use crate::app::App;
use crate::app::Msg;
use yew::prelude::*;

impl App {
    pub fn view_hud(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class={classes!("hud-visor-container", (!self.monitor_console).then_some("no-console"))}>
                { if self.monitor_console { self.view_console(ctx) } else { html! {} } }
                { self.view_metrics_grid() }
            </div>
        }
    }

    fn view_console(&self, ctx: &Context<Self>) -> Html {
        let console_title = if let Some(stats) = &self.stats {
            stats.hostname.to_uppercase()
        } else {
            "CONSOLE MONITOR".to_string()
        };

        html! {
            <div class="hud-console-wrapper">
                <div class="hud-console-header">
                    <span class="hostname-glow">
                        {console_title}
                    </span>
                    <div class="hud-console-controls">
                        <button onclick={ctx.link().callback(|_| Msg::DecreaseFontSize)} title="Decrease Font Size" class="font-btn">{"A-"}</button>
                        <button onclick={ctx.link().callback(|_| Msg::IncreaseFontSize)} title="Increase Font Size" class="font-btn">{"A+"}</button>
                        {
                            if self.console_paused {
                                html! {
                                    <button onclick={ctx.link().callback(|_| Msg::TogglePauseConsole)} title="Resume Auto-Scroll" class="font-btn pause-btn active-paused">{"PLAY"}</button>
                                }
                            } else {
                                html! {
                                    <button onclick={ctx.link().callback(|_| Msg::TogglePauseConsole)} title="Pause Auto-Scroll" class="font-btn pause-btn">{"PAUSE"}</button>
                                }
                            }
                        }
                        <button onclick={ctx.link().callback(|_| Msg::ClearTerminal)}>{"CLEAR"}</button>
                    </div>
                </div>
                <div class="hud-console-body" ref={self.console_ref.clone()} onmouseup={ctx.link().callback(|_| Msg::ConsoleMouseUp)} style={format!("font-size: {}rem;", self.console_font_size)}>
                    {for self.terminal_logs.iter().map(|log| {
                        let log_cls = if log.contains("ERROR") || log.contains("Error") || log.contains("Failed") || log.contains("failed") || log.contains("CRITICAL") {
                            "console-line error"
                        } else if log.contains("warning") || log.contains("WARNING") || log.contains("warn") || log.contains("WARN") {
                            "console-line warning"
                        } else if log.contains("[SYSTEM]") || log.contains("[WS]") {
                            "console-line system"
                        } else {
                            "console-line info"
                        };
                        html! { <div class={log_cls}>{log}</div> }
                    })}
                </div>
            </div>
        }
    }

    fn view_metrics_grid(&self) -> Html {
        html! {
            <div class="hud-visor-grid">
                { self.view_sys_info_card() }
                { if self.monitor_cpu { self.view_cpu_card() } else { html! {} } }
                { if self.monitor_memory { self.view_memory_card() } else { html! {} } }
                { if self.monitor_storage { self.view_storage_card() } else { html! {} } }
                { if self.monitor_network { self.view_network_card() } else { html! {} } }
                { if self.monitor_gpu { self.view_gpu_card() } else { html! {} } }
            </div>
        }
    }

    fn view_cpu_card(&self) -> Html {
        html! {
            <div class="hud-metric-card" title={self.stats.as_ref().map(|s| s.cpu_brand.clone()).unwrap_or_default()}>
                <h3>{"CPU"}</h3>
                {if let Some(stats) = &self.stats {
                    html! {
                        <div class="card-metric-block">
                            <div class="card-main-val">{format!("{:.1}%", stats.cpu_global)}</div>
                            <div class="card-subtext">{format!("{} Cores", stats.cpu_cores.len())}</div>
                            <div class="hud-bar-frame"><div class="hud-bar-fill" style={format!("width: {}%;", stats.cpu_global)}></div></div>
                            { self.render_sparkline(&self.cpu_history, 100.0) }
                        </div>
                    }
                } else {
                    html! {
                        <div class="card-metric-block">
                            <div class="card-loading">{"Connecting..."}</div>
                            { self.render_sparkline(&self.cpu_history, 100.0) }
                        </div>
                    }
                }}
            </div>
        }
    }

    fn view_memory_card(&self) -> Html {
        html! {
            <div class="hud-metric-card">
                <h3>{"MEMORY"}</h3>
                {if let Some(stats) = &self.stats {
                    let ram_used_gb = stats.ram_used as f32 / 1024.0 / 1024.0 / 1024.0;
                    let ram_total_gb = stats.ram_total as f32 / 1024.0 / 1024.0 / 1024.0;
                    let ram_percent = (stats.ram_used as f32 / stats.ram_total as f32 * 100.0).clamp(0.0, 100.0);
                    html! {
                        <div class="card-metric-block">
                            <div class="card-main-val">{format!("{:.1} / {:.1} GB", ram_used_gb, ram_total_gb)}</div>
                            <div class="card-subtext">{format!("{:.1}% Used", ram_percent)}</div>
                            <div class="hud-bar-frame"><div class="hud-bar-fill" style={format!("width: {}%;", ram_percent)}></div></div>
                            { self.render_sparkline(&self.ram_history, 100.0) }
                        </div>
                    }
                } else {
                    html! {
                        <div class="card-metric-block">
                            <div class="card-loading">{"Connecting..."}</div>
                            { self.render_sparkline(&self.ram_history, 100.0) }
                        </div>
                    }
                }}
            </div>
        }
    }

    fn view_storage_card(&self) -> Html {
        html! {
            <div class="hud-metric-card">
                <h3>{"STORAGE"}</h3>
                {if let Some(stats) = &self.stats {
                    let disk_used_gb = stats.disk_used as f32 / 1024.0 / 1024.0 / 1024.0;
                    let disk_total_gb = stats.disk_total as f32 / 1024.0 / 1024.0 / 1024.0;
                    let disk_percent = (stats.disk_used as f32 / stats.disk_total as f32 * 100.0).clamp(0.0, 100.0);
                    html! {
                        <div class="card-metric-block">
                            <div class="card-main-val">{format!("{:.1} / {:.1} GB", disk_used_gb, disk_total_gb)}</div>
                            <div class="card-subtext">{format!("{:.1}% Used", disk_percent)}</div>
                            <div class="hud-bar-frame"><div class="hud-bar-fill" style={format!("width: {}%;", disk_percent)}></div></div>
                            { self.render_sparkline(&self.disk_history, 100.0) }
                        </div>
                    }
                } else {
                    html! {
                        <div class="card-metric-block">
                            <div class="card-loading">{"Connecting..."}</div>
                            { self.render_sparkline(&self.disk_history, 100.0) }
                        </div>
                    }
                }}
            </div>
        }
    }

    fn view_network_card(&self) -> Html {
        html! {
            <div class="hud-metric-card">
                <h3>{"NETWORK"}</h3>
                {if let Some(stats) = &self.stats {
                    html! {
                        <div class="card-metric-block">
                            <div class="card-main-val">{format!("▼ {}  ▲ {}", self.format_bytes(stats.net_in), self.format_bytes(stats.net_out))}</div>
                            <div class="card-subtext">{"Throughput"}</div>
                            <div class="hud-bar-frame"><div class="hud-bar-fill animated-flow" style="width: 100%;"></div></div>
                            { self.render_sparkline(&self.net_history, 0.0) }
                        </div>
                    }
                } else {
                    html! {
                        <div class="card-metric-block">
                            <div class="card-loading">{"Connecting..."}</div>
                            { self.render_sparkline(&self.net_history, 0.0) }
                        </div>
                    }
                }}
            </div>
        }
    }

    fn view_gpu_card(&self) -> Html {
        html! {
            <>
            {if let Some(stats) = &self.stats {
                html! {
                    <>
                    {for stats.gpus.iter().enumerate().map(|(idx, gpu)| {
                        let temp_str = gpu.temp.map(|t| format!("{:.0}°C", t)).unwrap_or_else(|| "--".to_string());
                        let name_str = if gpu.name.is_empty() { format!("GPU {}", idx + 1) } else { gpu.name.clone() };
                        html! {
                            <div class="hud-metric-card" title={name_str}>
                                <h3>{format!("GPU {}", idx + 1)}</h3>
                                <div class="card-metric-block">
                                    <div class="card-main-val">{format!("{:.1}%", gpu.usage)}</div>
                                    <div class="card-subtext">{format!("Core Temp: {}", temp_str)}</div>
                                    <div class="hud-bar-frame"><div class="hud-bar-fill" style={format!("width: {}%;", gpu.usage)}></div></div>
                                    { self.render_sparkline(&self.gpu_histories[idx], 100.0) }
                                </div>
                            </div>
                        }
                    })}
                    </>
                }
            } else {
                html! {
                    <div class="hud-metric-card">
                        <h3>{"GPU"}</h3>
                        <div class="card-metric-block">
                            <div class="card-loading">{"Connecting..."}</div>
                            { self.render_sparkline(&[], 100.0) }
                        </div>
                    </div>
                }
            }}
            </>
        }
    }

    fn render_sparkline(&self, history: &[f32], max_val: f32) -> Html {
        if history.is_empty() {
            return html! {
                <div style="font-family: monospace; font-size: 0.8rem; color: var(--text-secondary); opacity: 0.5; padding: 0.5rem 0;">
                    {"Awaiting telemetry..."}
                </div>
            };
        }

        let width = 140.0;
        let height = 16.0;
        let points_count = history.len();

        let effective_max = if max_val > 0.0 {
            max_val
        } else {
            history.iter().copied().fold(0.0f32, f32::max).max(1.0)
        };

        let points = history
            .iter()
            .enumerate()
            .map(|(idx, &val)| {
                let x = if points_count > 1 {
                    (idx as f32 / (points_count - 1) as f32) * width
                } else {
                    0.0
                };
                let percent = (val / effective_max).clamp(0.0, 1.0);
                let y = height - (percent * (height - 3.0)) - 1.5;
                format!("{:.1},{:.1}", x, y)
            })
            .collect::<Vec<String>>()
            .join(" ");

        html! {
            <div style="width: 100%; height: 16px; margin-top: 0.3rem; opacity: 0.85;">
                <svg width="100%" height="16" viewBox={format!("0 0 {} {}", width, height)} preserveAspectRatio="none" style="display: block; overflow: visible;">
                    <polyline fill="none" stroke="var(--primary)" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" points={points} />
                </svg>
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

    fn view_sys_info_card(&self) -> Html {
        if let Some(stats) = &self.stats {
            let uptime_str = {
                let seconds = stats.uptime;
                let days = seconds / 86400;
                let hours = (seconds % 86400) / 3600;
                let minutes = (seconds % 3600) / 60;
                let secs = seconds % 60;
                if days > 0 {
                    format!("{}d {}h {}m", days, hours, minutes)
                } else if hours > 0 {
                    format!("{}h {}m {}s", hours, minutes, secs)
                } else {
                    format!("{}m {}s", minutes, secs)
                }
            };

            let os_ascii = self.get_os_ascii(&stats.os_name);

            html! {
                <div class="hud-metric-card sys-info-card" title="System Information">
                    <div class="sys-info-left">
                        <h3>{"SYSTEM INFO"}</h3>
                        <div class="card-metric-block sys-details">
                            <div class="sys-detail-row">
                                <span class="sys-detail-label">{"HOST:"}</span>
                                <span class="sys-detail-val hostname-glow">{ &stats.hostname }</span>
                            </div>
                            <div class="sys-detail-row">
                                <span class="sys-detail-label">{"OS:"}</span>
                                <span class="sys-detail-val">{ format!("{} {}", stats.os_name, stats.os_version).trim().to_string() }</span>
                            </div>
                            <div class="sys-detail-row">
                                <span class="sys-detail-label">{"KERNEL:"}</span>
                                <span class="sys-detail-val">{ &stats.kernel_version }</span>
                            </div>
                            <div class="sys-detail-row">
                                <span class="sys-detail-label">{"UPTIME:"}</span>
                                <span class="sys-detail-val">{ uptime_str }</span>
                            </div>
                        </div>
                    </div>
                    <div class="sys-info-right ascii-container">
                        <pre>{ os_ascii }</pre>
                    </div>
                </div>
            }
        } else {
            html! {
                <div class="hud-metric-card sys-info-card">
                    <div class="sys-info-left">
                        <h3>{"SYSTEM INFO"}</h3>
                        <div class="card-metric-block">
                            <div class="card-loading">{"Connecting..."}</div>
                        </div>
                    </div>
                </div>
            }
        }
    }

    fn get_os_ascii(&self, os_name: &str) -> &'static str {
        let os = os_name.to_lowercase();
        if os.contains("nixos") {
            "  ▗▄▄▄▖  ▗▖\n  ▐▌   ▗▞▘ \n ▗▞▘  ▐▌   \n ▐▘    ▝▚▄▖"
        } else if os.contains("ubuntu") {
            "   ▗▄▄▄▖ \n ▗▘  ▗  ▝▖\n ▐   ▜   ▌\n   ▝▀▀▀▘ "
        } else if os.contains("debian") {
            "  ▗▄▄▄▖\n ▐▘    \n ▐  ▗▄▖\n ▐▌  ▐▌\n  ▝▀▀▘ "
        } else if os.contains("arch") {
            "    ▞▜\n   ▞  ▜\n  ▞ ▞▀▜ ▜\n ▞ ▞   ▜ ▜"
        } else if os.contains("fedora") {
            "   ▗▄▄▖\n  ▐▌  ▝▖\n ▗▟▀▀▜▌\n  ▝▀▀▘ "
        } else {
            "   ▗▄▄▖ \n  ▐▌  ▐▌\n  ▐▛▀▀▜▌\n  ▐▌  ▐▌\n   ▝▀▀▘ "
        }
    }
}
