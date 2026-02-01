use dioxus::prelude::*;

use super::info_box::InfoBox;

#[component]
pub fn About() -> Element {
	rsx! {
		section { class: "py-16 px-4",

			div { class: "container mx-auto max-w-4xl",

				// Section title
				h2 {
					class: "text-4xl font-comic font-bold text-center mb-8 text-sakura-700",
					style: "text-decoration: underline; text-decoration-color: #f076ad; text-decoration-thickness: 4px;",
					"About T4vrn"
				}

				// Grid layout (90s table style)
				div { class: "grid grid-cols-1 md:grid-cols-2 gap-4 mb-8",

					InfoBox { title: "🎯 Our Mission",
						p { class: "text-lg font-system leading-relaxed",
							"Lorem ipsum dolor sit amet, consectetur adipiscing "
							"elit. Morbi consequat, tellus vel maximus tincidunt, "
							"ipsum mi pulvinar diam, at condimentum diam risus sit "
							"amet eros. Ut egestas lacus quis tellus mattis lacinia."
						}
					}

					InfoBox { title: "✨ Our Vision",
						p { class: "text-lg font-system leading-relaxed",
							"Lorem ipsum dolor sit amet, consectetur adipiscing "
							"elit. Morbi consequat, tellus vel maximus tincidunt, "
							"ipsum mi pulvinar diam, at condimentum diam risus sit "
							"amet eros."
						}
					}
				}

				// Under construction
				div { class: "text-center mt-8",
					div { class: "text-6xl mb-4", "🚧" }
					p { class: "mt-4 text-lg font-mono blink-text text-sakura-800",
						"More info coming soon!"
					}
				}
			}
		}
	}
}
