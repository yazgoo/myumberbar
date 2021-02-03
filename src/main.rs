extern crate umberbar;

use std::time::Duration;
use umberbar::{Conf, umberbar, WidgetPosition, Sources, Logos, ThemedWidgets, Palette, Widget};
use std::collections::HashMap;
#[tokio::main]
async fn main() {
    /* width (in characters) of the bar */
    let terminal_width = 222;
    /* color palette we will use for each bar items */
    let palette = Palette::red_pink_turquoise_spring();
    /* hash which will contain all widgets */
    let mut widgets : HashMap<WidgetPosition, Vec<Widget>> = HashMap::new();
    /* widgets which will be aligned on the left (from left to right), built with a given theme */
    let lefts = ThemedWidgets::powerline(
                WidgetPosition::Left, vec![
                (Sources::battery(), Logos::battery()),
                (Sources::cpu(), Logos::cpu()),
                (Sources::cpu_temp(), Logos::cpu_temp()),
                (Sources::window(terminal_width / 3), Logos::window()),
                ], &palette);
    /* widgets which will be aligned on the right (from right to left), built with a given theme */
    let rights = ThemedWidgets::powerline(
                    WidgetPosition::Right, vec![
                    (Sources::date(), Logos::date()),
                    (Sources::memory(), Logos::memory()),
                    ], &palette);
    widgets.insert(lefts.0, lefts.1);
    widgets.insert(rights.0, rights.1);
    umberbar(Conf {
        font: "DroidSansMono Nerd Font".to_string(),
        font_size: 7,
        terminal_width: terminal_width as u16,
        /* frequency for refreshing the bar */
        refresh_time: Duration::from_secs(5),
        widgets: widgets,
    }).run().await;
}
