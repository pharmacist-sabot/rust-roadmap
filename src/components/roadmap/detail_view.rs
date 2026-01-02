use crate::models::roadmap::{BadgeKind, TopicContent};
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
                                let (badge_class, badge_text) = match res.badge {
                                    BadgeKind::Official => ("badge official", "Official"),
                                    BadgeKind::OpenSource => ("badge opensource", "Open Source"),
                                    BadgeKind::Other(s) => ("badge default", s),
                                };
                                view! {
                                    <div class="resource-item">
                                        <span class=badge_class>{badge_text}</span>
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
