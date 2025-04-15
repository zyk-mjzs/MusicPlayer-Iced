use std::{borrow::Cow, sync::Once, vec};

use iced::{font::load, futures::{self, executor::block_on, StreamExt}};
use iced_runtime::{Action, Task};

mod fa;
pub use fa::*;

pub const REGULAR_FONT_DATA: &[u8] =
    include_bytes!("../assets/fontawesome-free-6.7.2-web/webfonts/fa-regular-400.ttf");

pub const BRANDS_FONT_DATA: &[u8] =
    include_bytes!("../assets/fontawesome-free-6.7.2-web/webfonts/fa-brands-400.ttf");

pub const SOLID_FONT_DATA: &[u8] =
    include_bytes!("../assets/fontawesome-free-6.7.2-web/webfonts/fa-solid-900.ttf");

static INIT: Once = Once::new();
pub fn load_font_awesome() {
    // INIT.call_once(|| {
        let regular_task = load(REGULAR_FONT_DATA);
        let solid_task = load(SOLID_FONT_DATA);
        let brands_task = load(BRANDS_FONT_DATA);

        let combined_task = Task::batch(vec![regular_task, solid_task, brands_task]);

        block_on(async {
            if let Some(mut stream) = iced_runtime::task::into_stream(combined_task) {
                while let Some(_) = stream.next().await {
                    // Font loading is handled by the runtime
                }
            }
        });
    // });
}
