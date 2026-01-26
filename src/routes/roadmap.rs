use crate::components::roadmap::detail_view::TopicDetail;
use crate::components::roadmap::diagram::{DiagramData, RoadmapDiagram};
use crate::components::ui::footer::Footer;
use crate::components::ui::header::Header;
use crate::components::ui::hero::Hero;
use crate::data::get_topic_content;
use crate::data::{SECTIONS, get_all_dependencies, get_all_topics};
use crate::layout::tree::{LayoutConfig, compute_layout};
use leptos::*;

#[component]
pub fn RoadmapPage() -> impl IntoView {
    let config = LayoutConfig::default();

    let topics = get_all_topics();
    let dependencies = get_all_dependencies();

    let static_topics = Box::leak(topics.into_boxed_slice());
    let static_deps = Box::leak(dependencies.into_boxed_slice());

    let layout = compute_layout(SECTIONS, static_topics, static_deps, &config);

    // Search state
    let (search_term, set_search_term) = create_signal(String::new());

    // State for selected topic ID
    let (selected_topic_id, set_selected_topic_id) = create_signal(None::<&'static str>);

    // Derived signal for content
    let selected_content =
        create_memo(move |_| selected_topic_id.get().and_then(get_topic_content));

    // Check if drawer is open
    let is_drawer_open = create_memo(move |_| selected_topic_id.get().is_some());

    let handle_search = Callback::new(move |term: String| {
        set_search_term.set(term);
    });

    let handle_topic_click = Callback::new(move |id: &'static str| {
        set_selected_topic_id.set(Some(id));
    });

    let handle_close_detail = Callback::new(move |_| {
        set_selected_topic_id.set(None);
    });

    let diagram_props = DiagramData {
        sections: SECTIONS,
        topics: static_topics,
        dependencies: static_deps,
        layout,
        config,
        on_topic_click: handle_topic_click,
        search_term,
    };

    view! {
        <div class="roadmap-page">
            // Background Decorations
            <div class="bg-decorations">
                <div class="glow-orb"></div>
                <div class="noise-overlay"></div>
            </div>

            // Header
            <Header search_term=search_term on_search=handle_search />

            // Main Content
            <main class="main-content">
                // Hero Section
                <Hero />

                // Roadmap Container
                <div class="roadmap-container">
                    <RoadmapDiagram props=diagram_props />

                </div>
            </main>

            // Footer
            <Footer />

            // Drawer Backdrop (always mounted for fade-out animation)
            <div
                class=move || if is_drawer_open.get() { "drawer-backdrop drawer-backdrop--visible" } else { "drawer-backdrop" }
                on:click=move |_| handle_close_detail.call(())
            ></div>

            // Drawer Content
            {move || {
                selected_content.get().map(|content| {
                    view! {
                        <TopicDetail
                            content=content
                            on_close=handle_close_detail
                            is_open=is_drawer_open.get()
                        />
                    }
                })
            }}
        </div>
    }
}
