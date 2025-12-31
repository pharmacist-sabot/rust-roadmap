use leptos::*;

#[component]
pub fn RoadmapNode(
    title: String,
    // Callback function เมื่อกดปุ่ม
    #[prop(into)] on_click: Callback<()>,
    #[prop(default = false)] is_main: bool,
) -> impl IntoView {
    // Style เลียนแบบ roadmap.sh (Box shadows + Borders)
    let base_style = "cursor-pointer select-none transition-all active:translate-x-[2px] active:translate-y-[2px] active:shadow-none border-2 border-black rounded px-6 py-3 font-bold text-center";

    let color_style = if is_main {
        "bg-yellow-400 hover:bg-yellow-300 shadow-[4px_4px_0px_0px_rgba(0,0,0,1)] text-slate-900"
    } else {
        "bg-yellow-100 hover:bg-yellow-50 shadow-[4px_4px_0px_0px_rgba(0,0,0,1)] text-slate-800 text-sm"
    };

    view! {
        <div
            class={format!("{} {}", base_style, color_style)}
            on:click=move |_| on_click.call(())
        >
            {title}
        </div>
    }
}
