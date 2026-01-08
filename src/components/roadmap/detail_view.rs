use crate::models::roadmap::{BadgeKind, TopicContent};
use leptos::*;

#[component]
pub fn TopicDetail(content: TopicContent, on_close: Callback<()>, is_open: bool) -> impl IntoView {
    let drawer_class = if is_open {
        "drawer drawer--open"
    } else {
        "drawer"
    };

    // Get section ID by parsing the content title (simple approach)
    let section_label = "Introduction"; // This could be passed as a prop for accuracy

    view! {
        <div
            class=drawer_class
            role="dialog"
            aria-modal="true"
            aria-labelledby="drawer-title"
        >
            // Drawer Header
            <div class="drawer__header">
                <div class="drawer__header-content">
                    <div class="drawer__section-label">
                        {section_label}
                    </div>
                    <h1 class="drawer__title" id="drawer-title">
                        {content.title}
                    </h1>
                </div>
                <button
                    class="drawer__close"
                    on:click=move |_| on_close.call(())
                    aria-label="Close drawer"
                >
                    <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                        <line x1="18" y1="6" x2="6" y2="18"></line>
                        <line x1="6" y1="6" x2="18" y2="18"></line>
                    </svg>
                </button>
            </div>

            // Drawer Body
            <div class="drawer__body">
                // Description Section
                <div class="drawer__section">
                    <p class="drawer__description">
                        {content.description}
                    </p>
                </div>

                // Learning Status Section
                <div class="drawer__section">
                    <div class="drawer__status-card">
                        <div class="drawer__status-header">
                            <span class="drawer__status-label">"Learning Status"</span>
                            <span class="drawer__status-percent">"0% Complete"</span>
                        </div>
                        <div class="drawer__progress-bar">
                            <div class="drawer__progress-fill" style="width: 0%"></div>
                        </div>
                        <button class="drawer__status-button">
                            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                <path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"></path>
                                <polyline points="22 4 12 14.01 9 11.01"></polyline>
                            </svg>
                            "Mark as Done"
                        </button>
                    </div>
                </div>

                // Resources Section
                <div class="drawer__section">
                    <h3 class="drawer__section-title">
                        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                            <polyline points="16 18 22 12 16 6"></polyline>
                            <polyline points="8 6 2 12 8 18"></polyline>
                        </svg>
                        "Resources"
                    </h3>
                    <div class="drawer__resources">
                        {if content.resources.is_empty() {
                            view! {
                                <div class="drawer__empty">
                                    "No resources listed yet."
                                </div>
                            }.into_view()
                        } else {
                            content.resources.iter().map(|res| {
                                let badge_class = match res.badge {
                                    BadgeKind::Official => "drawer__resource-badge badge--official",
                                    BadgeKind::OpenSource => "drawer__resource-badge badge--opensource",
                                    BadgeKind::Article => "drawer__resource-badge badge--article",
                                    BadgeKind::Video => "drawer__resource-badge badge--video",
                                    BadgeKind::Feed => "drawer__resource-badge badge--feed",
                                    _ => "drawer__resource-badge badge--default",
                                };

                                let badge_text = match res.badge {
                                    BadgeKind::Official => "Official",
                                    BadgeKind::OpenSource => "OpenSource",
                                    BadgeKind::Article => "Article",
                                    BadgeKind::Video => "Video",
                                    BadgeKind::Feed => "Feed",
                                    BadgeKind::Other(s) => s,
                                };

                                view! {
                                    <a
                                        href={res.url}
                                        target="_blank"
                                        rel="noopener noreferrer"
                                        class="drawer__resource-card"
                                    >
                                        <div class="drawer__resource-info">
                                            <svg class="drawer__resource-icon" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                                <path d="M4 19.5A2.5 2.5 0 0 1 6.5 17H20"></path>
                                                <path d="M6.5 2H20v20H6.5A2.5 2.5 0 0 1 4 19.5v-15A2.5 2.5 0 0 1 6.5 2z"></path>
                                            </svg>
                                            <span class="drawer__resource-label">{res.label}</span>
                                        </div>
                                        <div class="drawer__resource-meta">
                                            <span class=badge_class>{badge_text}</span>
                                            <svg class="drawer__resource-external" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                                <path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"></path>
                                                <polyline points="15 3 21 3 21 9"></polyline>
                                                <line x1="10" y1="14" x2="21" y2="3"></line>
                                            </svg>
                                        </div>
                                    </a>
                                }
                            }).collect_view()
                        }}
                    </div>
                </div>

                // Code Snippet Section
                <div class="drawer__section">
                    <h3 class="drawer__section-title">
                        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                            <polyline points="16 18 22 12 16 6"></polyline>
                            <polyline points="8 6 2 12 8 18"></polyline>
                        </svg>
                        "Syntax Preview"
                    </h3>
                    <div class="drawer__code-preview">
                        <div class="drawer__code-line">
                            <span class="drawer__code-keyword">"fn"</span>
                            " "
                            <span class="drawer__code-function">"main"</span>
                            "() {"
                        </div>
                        <div class="drawer__code-line" style="padding-left: 1rem;">
                            <span class="drawer__code-comment">"// This is a standard Rust entry point"</span>
                        </div>
                        <div class="drawer__code-line" style="padding-left: 1rem;">
                            <span class="drawer__code-macro">"println!"</span>
                            "("
                            <span class="drawer__code-string">"\"Hello, "{content.title}"!\""</span>
                            ");"
                        </div>
                        <div class="drawer__code-line">
                            "}"
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
