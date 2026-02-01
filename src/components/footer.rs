use dioxus::prelude::*;

use super::webring::Webring;

#[component]
pub fn Footer() -> Element {
	rsx! {
		footer { class: "bg-sakura-800 text-white py-8 px-4 mt-auto",

			div { class: "container mx-auto max-w-4xl",

				Webring {}

				// Copyright
				div { class: "text-center text-sm font-mono",
					p { class: "mb-2", "© 2026 T4vrn. T4vrn is a trademark." }
					p { class: "text-sakura-200", "Code licensed under MIT-0 or Apache-2.0" }
				}
			}
		}
	}
}
