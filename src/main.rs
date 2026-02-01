use dioxus::prelude::*;

mod components;

use crate::components::xp_window::XPWindow;
use crate::components::{about::About, footer::Footer, hero::Hero};

fn main() {
	launch(App);
}

#[derive(Clone, Copy)]
struct AppContext {
	dark_mode: Signal<bool>,
}

// Hook to access app context from any component
fn use_ctx() -> AppContext {
	use_context()
}

// Hook to create and initialize the app context
fn use_create_ctx() -> AppContext {
	let mut dark_mode = use_signal(|| false);

	// Initialize dark mode detection
	use_hook(|| {
		spawn(async move {
			let mut eval_result = document::eval(
				r#"
				const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');

				// Listen for changes
				mediaQuery.addEventListener('change', (e) => {
					dioxus.send(e.matches);
				});

				return mediaQuery.matches;
				"#,
			);

			// Get initial value and log it
			if let Ok(prefers_dark) = eval_result.recv::<bool>().await {
				info!(
					"OS dark mode preference: {}",
					if prefers_dark { "dark" } else { "light" }
				);
				dark_mode.set(prefers_dark);
			}

			// Listen for changes
			while let Ok(prefers_dark) = eval_result.recv::<bool>().await {
				info!(
					"OS dark mode preference changed to: {}",
					if prefers_dark { "dark" } else { "light" }
				);
				dark_mode.set(prefers_dark);
			}
		});
	});

	// Register the provider so that subsquent use_context() calls can get the context
	use_context_provider(|| AppContext { dark_mode })
}

#[component]
fn App() -> Element {
	// let c = use_create_ctx();

	rsx! {
		document::Link { rel: "stylesheet", href: "output.css" }
		document::Title { "T4vrn - For VR Netizens" }
		document::Meta {
			name: "description",
			content: "T4vrn - For VR Netizens",
		}
		document::Meta {
			name: "viewport",
			content: "width=device-width, initial-scale=1.0",
		}

		div { class: "min-h-screen flex flex-col bg-sakura-200",
			Hero {}
			About {}
			Footer {}
		}

	}
}
