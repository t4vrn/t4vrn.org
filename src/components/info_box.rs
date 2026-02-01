use dioxus::prelude::*;

#[component]
pub fn InfoBox(title: String, children: Element) -> Element {
	rsx! {
		div { class: "table-cell-90s",
			h3 { class: "text-2xl font-bold mb-4 text-sakura-800 font-system",
				"{title}"
			}
			{children}
		}
	}
}
