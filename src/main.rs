use dioxus::prelude::*;

fn main() {
    launch(app);
}

fn app() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        div {
            style: "text-align: center; padding: 50px; font-family: Arial, sans-serif;",
            h1 { "🚀 Dioxus 计数器应用" }
            p {
                style: "font-size: 48px; margin: 30px 0;",
                "计数: {count}"
            }
            div {
                style: "display: flex; gap: 10px; justify-content: center;",
                button {
                    style: "padding: 10px 20px; font-size: 18px; cursor: pointer;",
                    onclick: move |_| count += 1,
                    "➕ 增加"
                }
                button {
                    style: "padding: 10px 20px; font-size: 18px; cursor: pointer;",
                    onclick: move |_| count -= 1,
                    "➖ 减少"
                }
                button {
                    style: "padding: 10px 20px; font-size: 18px; cursor: pointer;",
                    onclick: move |_| count.set(0),
                    "🔄 重置"
                }
            }
        }
    }
}
