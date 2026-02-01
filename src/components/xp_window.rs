use dioxus::prelude::*;

#[component]
pub fn XPWindow(
	title: String,
	#[props(default = true)] closable: bool,
	children: Element,
) -> Element {
	let mut show = use_signal(|| true);

	rsx! {
		div {
			class: "window",
			style: "width: 400px; margin: 20px;",
			hidden: !show(),

			div { class: "title-bar",
				div { class: "title-bar-text", "{title}" }
				div { class: "title-bar-controls",
					if closable {
						button {
							class: "close",
							aria_label: "Close",
							onclick: move |_| show.set(false),
						}
					}
				}
			}

			div { class: "window-body", {children} }
		}
	}
}
