mod components;
mod data;
mod models;

use components::drawer::Drawer;
use components::roadmap_node::RoadmapNode;
use leptos::*;
use models::TopicContent;

#[component]
fn App() -> impl IntoView {
    // State จัดการว่าเลือกหัวข้อไหนอยู่
    let (selected_topic, set_selected_topic) = create_signal::<Option<TopicContent>>(None);

    view! {
        <div class="min-h-screen bg-white text-slate-900 font-sans">
            // --- HEADER ---
            <header class="p-6 border-b border-gray-200 text-center">
                <h1 class="text-3xl font-black mb-2">"Rust Developer Roadmap"</h1>
                <p class="text-gray-500">"Step by step guide to becoming a Rust developer"</p>
            </header>

            // --- MAIN AREA ---
            <main class="relative p-10 flex flex-col items-center">

                // เส้นประแกนกลาง (Dotted Line)
                <div class="absolute top-0 bottom-0 left-1/2 w-0.5 border-l-2 border-blue-400 border-dotted -z-10 transform -translate-x-1/2"></div>

                // --- ROADMAP TREE ---
                <div class="flex flex-col gap-12 items-center w-full max-w-4xl">

                    // Root Node
                    <div class="relative">
                        <RoadmapNode
                            title="Rust".to_string()
                            is_main=true
                            // แก้ไขจุดที่ 1: เปลี่ยน || เป็น |_|
                            on_click=move |_| {}
                        />
                    </div>

                    // Introduction Node (Interactive)
                    <div class="relative">
                         // เส้นเชื่อมสีฟ้า (Line Connector)
                         <div class="absolute -top-12 left-1/2 h-12 w-1 bg-blue-500 transform -translate-x-1/2"></div>

                         <RoadmapNode
                            title="Introduction".to_string()
                            is_main=true
                            // แก้ไขจุดที่ 2: เปลี่ยน || เป็น |_|
                            on_click=move |_| set_selected_topic.set(Some(data::get_introduction_data()))
                        />
                    </div>

                    // กิ่งก้านสาขา (Branches Example)
                    <div class="grid grid-cols-2 gap-20 relative pt-8">
                         // เส้นแนวนอนเชื่อม
                         <div class="absolute top-0 left-1/4 right-1/4 h-1 bg-blue-500 border-t-2 border-blue-500 border-dotted"></div>
                         // เส้นดิ่งซ้ายขวา
                         <div class="absolute top-0 left-1/4 h-8 w-1 bg-blue-500 transform -translate-x-1/2"></div>
                         <div class="absolute top-0 right-1/4 h-8 w-1 bg-blue-500 transform translate-x-1/2"></div>

                         <RoadmapNode
                            title="What is Rust?".to_string()
                            is_main=false
                            // แก้ไขจุดที่ 3: เปลี่ยน || เป็น |_|
                            on_click=move |_| set_selected_topic.set(Some(data::get_introduction_data()))
                        />
                         <RoadmapNode
                            title="Why Rust?".to_string()
                            is_main=false
                            // แก้ไขจุดที่ 4: เปลี่ยน || เป็น |_|
                            on_click=move |_| {}
                        />
                    </div>
                </div>

            </main>

            // --- DRAWER COMPONENT ---
            <Drawer selected_topic=selected_topic set_selected_topic=set_selected_topic />
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> })
}
