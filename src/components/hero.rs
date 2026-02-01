use dioxus::prelude::*;

#[component]
pub fn Hero() -> Element {
	rsx! {
		section { class: "relative overflow-hidden py-10 px-4 bg-sakura-800",

			div { class: "container mx-auto max-w-4xl relative z-10",

				// Main heading
				h1 {
					class: "text-6xl font-comic font-bold text-center mb-4 text-shadow-3d text-sakura-300",
					"T4vrn"
				}

				// Tagline
				h2 { class: "text-3xl font-comic font-bold text-center mb-4 text-sakura-300",
					"For VR Netizens"
				}

				// CTA Button
				div { class: "flex justify-center gap-4",
					a {
						href: "https://discord.gg/AX4Vjvzt6m",
						target: "_blank",
						rel: "noopener noreferrer",
						class: "btn-90s transform hover:scale-105 transition-transform",
						img {
							src: "https://media2.giphy.com/media/v1.Y2lkPTc5MGI3NjExamZzZHhiNm9kbDZzcTczZ2phdHpxZW9tNWF6MXBjaWdkZDcyNjgzaSZlcD12MV9pbnRlcm5hbF9naWZfYnlfaWQmY3Q9cw/UQ1EI1ML2ABQdbebup/giphy.gif",
							alt: "Cat image",
							class: "inline-block h-[1.5em] m-1 w-auto -scale-x-100",
						}
						"Join The Discord!"
					}
				}
			}
		}
	}
}
