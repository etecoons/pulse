use crate::app::App;
use yew::prelude::*;

impl App {
    pub fn handle_cycle_os_override(&mut self, ctx: &Context<Self>) -> bool {
        let next_idx = match self.os_override {
            None => Some(0),
            Some(idx) => {
                if idx >= 14 {
                    None
                } else {
                    Some(idx + 1)
                }
            }
        };
        self.os_override = next_idx;
        let notify_text = match next_idx {
            None => "OS Logo: Auto-Detect".to_string(),
            Some(0) => "OS Logo: UBI".to_string(),
            Some(1) => "OS Logo: Ubuntu".to_string(),
            Some(2) => "OS Logo: Debian".to_string(),
            Some(3) => "OS Logo: Arch Linux".to_string(),
            Some(4) => "OS Logo: Fedora".to_string(),
            Some(5) => "OS Logo: Pop!_OS".to_string(),
            Some(6) => "OS Logo: Unraid".to_string(),
            Some(7) => "OS Logo: Gentoo".to_string(),
            Some(8) => "OS Logo: GNU Guix".to_string(),
            Some(9) => "OS Logo: Windows 11".to_string(),
            Some(10) => "OS Logo: Talos Linux".to_string(),
            Some(11) => "OS Logo: AWS Bottlerocket".to_string(),
            Some(12) => "OS Logo: Flatcar Linux".to_string(),
            Some(13) => "OS Logo: Alpine Linux".to_string(),
            Some(14) => "OS Logo: Fallback/Tux".to_string(),
            _ => "OS Logo: Custom".to_string(),
        };
        self.notify(ctx, notify_text);
        true
    }
}
