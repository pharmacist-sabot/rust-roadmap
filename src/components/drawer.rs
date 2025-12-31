use crate::models::TopicContent;
use leptos::*;

#[component]
pub fn Drawer(
    // Signal: ข้อมูล Topic ที่เลือก
    selected_topic: ReadSignal<Option<TopicContent>>,
    // Setter: เพื่อสั่งปิด Drawer
    set_selected_topic: WriteSignal<Option<TopicContent>>,
) -> impl IntoView {
    // Helper เพื่อเช็คว่าควรแสดงหรือไม่
    let is_open = move || selected_topic.get().is_some();

    view! {
        // 1. Overlay (Background สีดำ)
        <div
            class="fixed inset-0 bg-black/50 z-40 transition-opacity duration-300"
            // ถ้าไม่เปิด ให้ซ่อน (pointer-events-none) และจางหาย (opacity-0)
            class:opacity-0=move || !is_open()
            class:pointer-events-none=move || !is_open()
            class:opacity-100=is_open
            on:click=move |_| set_selected_topic.set(None)
        ></div>

        // 2. The Drawer Panel
        <aside
            class="fixed top-0 right-0 h-full w-full md:w-[600px] bg-white z-50 shadow-2xl transform transition-transform duration-300 ease-in-out drawer-scroll overflow-y-auto"
            // ถ้าเปิด translate-x-0 (อยู่กับที่), ถ้าปิด translate-x-full (หลบไปขวาสุด)
            class:translate-x-full=move || !is_open()
            class:translate-x-0=is_open
        >
            <Show when=is_open fallback=|| ()>
                {move || {
                    let topic = selected_topic.get().unwrap();
                    view! {
                         <div class="p-8 relative">
                            // ปุ่ม Close
                            <button
                                class="absolute top-6 right-6 text-gray-400 hover:text-gray-800 text-2xl font-bold"
                                on:click=move |_| set_selected_topic.set(None)
                            >
                                "✕"
                            </button>

                            // Title
                            <h2 class="text-4xl font-extrabold mb-6 text-slate-900">{&topic.title}</h2>

                            // Description
                            <p class="text-gray-600 text-lg leading-relaxed mb-8">
                                {&topic.description}
                            </p>

                            // Divider
                            <div class="flex items-center mb-6">
                                <div class="h-px bg-green-300 flex-1"></div>
                                <span class="px-3 py-1 border border-green-300 rounded-full text-green-600 text-sm font-bold bg-green-50">
                                    "♥ Free Resources"
                                </span>
                                <div class="h-px bg-green-300 flex-1"></div>
                            </div>

                            // Resource List
                            <div class="space-y-3">
                                {topic.links.iter().map(|link| {
                                    view! {
                                        <a href=&link.url target="_blank" class="group block p-3 rounded-lg hover:bg-gray-50 border border-transparent hover:border-gray-200 transition-all">
                                            <div class="flex items-center gap-3">
                                                <span class={format!("px-2 py-[2px] rounded text-[10px] font-bold uppercase tracking-wider {}", link.kind.color_class())}>
                                                    {link.kind.label()}
                                                </span>
                                                <span class="font-semibold text-slate-700 group-hover:text-blue-600 underline decoration-transparent group-hover:decoration-blue-600 underline-offset-2 transition-all">
                                                    {&link.title}
                                                </span>
                                            </div>
                                        </a>
                                    }
                                }).collect_view()}
                            </div>
                        </div>
                    }
                }}
            </Show>
        </aside>
    }
}
