use dioxus::prelude::*;

#[component]
pub fn Webring() -> Element {
	rsx! {
		div { class: "border-4 border-dashed border-sakura-300 bg-sakura-700 p-6 mb-6",
			div { class: "flex justify-center gap-8 flex-wrap",
				a {
					href: "https://discord.gg/AX4Vjvzt6m",
					target: "_blank",
					rel: "noopener noreferrer",
					class: "text-webring-cyan hover:text-webring-magenta underline font-bold text-lg",
					"Discord"
				}
				a {
					href: "https://github.com/t4vrn",
					target: "_blank",
					rel: "noopener noreferrer",
					class: "text-webring-cyan hover:text-webring-magenta underline font-bold text-lg",
					"GitHub"
				}
			}
		}
	}
}
