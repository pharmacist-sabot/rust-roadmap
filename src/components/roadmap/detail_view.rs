use crate::models::roadmap::TopicContent;
use leptos::*;

#[component]
pub fn TopicDetail(content: TopicContent, on_close: Callback<()>) -> impl IntoView {
    view! {
        <div class="modal-overlay" on:click=move |_| on_close.call(())>
            <div class="modal-content" on:click=move |e| { e.stop_propagation(); }>
                <div class="modal-header">
                    <h2>{content.title}</h2>
                    <button class="close-btn" on:click=move |_| on_close.call(())>
                        "×"
                    </button>
                </div>
                <div class="modal-body">
                    <p class="topic-description">{content.description}</p>

                    <div class="resources-section">
                        <h3>"Free Resources"</h3>
                        <div class="resources-list">
                            {content.resources.into_iter().map(|res| {
                                let badge_class = match res.badge {
                                    "Official" => "badge official",
                                    "OpenSource" => "badge opensource",
                                    _ => "badge default"
                                };
                                view! {
                                    <div class="resource-item">
                                        <span class=badge_class>{res.badge}</span>
                                        <a href=res.url target="_blank" class="resource-link">{res.label}</a>
                                    </div>
                                }
                            }).collect_view()}
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
