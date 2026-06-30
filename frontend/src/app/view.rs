use crate::app::App;
use yew::prelude::*;

impl App {
    pub fn view_hud(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="hud-visor-container no-console">
                { self.view_metrics_grid() }
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
}
