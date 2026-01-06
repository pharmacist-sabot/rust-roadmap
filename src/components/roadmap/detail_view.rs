use crate::models::roadmap::{BadgeKind, TopicContent};
use leptos::leptos_dom::helpers::{TimeoutHandle, set_timeout_with_handle};
use leptos::{ev::KeyboardEvent, *};
use std::time::Duration;
use web_sys::window;

#[derive(Clone, Copy, PartialEq, Eq)]
enum TerminalState {
    Browsing,
    Confirming(usize),
    Opening,
}

#[component]
pub fn TopicDetail(content: TopicContent, on_close: Callback<()>) -> impl IntoView {
    // 1. State
    let (state, set_state) = create_signal(TerminalState::Browsing);
    let (selected_idx, set_selected_idx) = create_signal(0); // Cursor position
    let _cmd_slug = content.title.to_lowercase().replace(" ", "-");
    let resources_len = content.resources.len();

    // Store handle for cleanup
    let timeout_handle = store_value(None::<TimeoutHandle>);

    on_cleanup(move || {
        if let Some(handle) = timeout_handle.get_value() {
            handle.clear();
        }
    });

    let content_for_open = content.clone();
    let confirm_open = move |idx: usize| {
        set_state.set(TerminalState::Opening);
        let url = content_for_open.resources[idx].url.to_string();

        let handle = set_timeout_with_handle(
            move || {
                if let Some(w) = window() {
                    let _ = w.open_with_url_and_target(&url, "_blank");
                }
                set_state.set(TerminalState::Browsing);
            },
            Duration::from_millis(800),
        )
        .ok();

        timeout_handle.set_value(handle);
    };

    let handle_keydown = {
        let confirm_open = confirm_open.clone();
        move |ev: KeyboardEvent| {
            let current_state = state.get();
            match current_state {
                TerminalState::Browsing => match ev.key().as_str() {
                    "ArrowUp" | "k" => {
                        // Support Vim bindings too!
                        set_selected_idx.update(|i| {
                            *i = if *i == 0 {
                                resources_len.saturating_sub(1)
                            } else {
                                *i - 1
                            }
                        });
                        ev.prevent_default();
                    }
                    "ArrowDown" | "j" => {
                        set_selected_idx.update(|i| {
                            *i = if *i >= resources_len.saturating_sub(1) {
                                0
                            } else {
                                *i + 1
                            }
                        });
                        ev.prevent_default();
                    }
                    "Enter" => {
                        if resources_len > 0 {
                            set_state.set(TerminalState::Confirming(selected_idx.get()));
                        }
                    }
                    "Escape" => on_close.call(()),
                    _ => {}
                },
                TerminalState::Confirming(idx) => match ev.key().as_str() {
                    "y" | "Y" | "Enter" => {
                        confirm_open(idx);
                    }
                    "n" | "N" | "Escape" => {
                        set_state.set(TerminalState::Browsing); // Cancel
                    }
                    _ => {}
                },
                TerminalState::Opening => {} // Ignore input while opening
            }
        }
    };

    // Auto-focus container to capture keys (or attach to window via useEffect)
    let container_ref = create_node_ref::<html::Div>();
    create_effect(move |_| {
        if let Some(div) = container_ref.get() {
            let _ = div.focus();
        }
    });

    view! {
        <div
            class="terminal-overlay"
            role="dialog"
            aria-modal="true"
            aria-labelledby="terminal-title"
            on:keydown=handle_keydown
            tabindex="-1"
            ref=container_ref
        >
            // Backdrop
            <div class="terminal-backdrop" on:click=move |_| on_close.call(())></div>

            // Terminal Panel
            <div class="terminal-panel" on:click=move |e| e.stop_propagation()>
                // Header
                <div class="terminal-header">
                    <div class="terminal-controls">
                        <span class="term-btn term-close" on:click=move |_| on_close.call(())></span>
                        <span class="term-btn term-min"></span>
                        <span class="term-btn term-max"></span>
                    </div>
                    <div class="terminal-title" id="terminal-title">"rust-roadmap"</div>
                    <div class="terminal-spacer"></div>
                </div>

                // Body
                <div class="terminal-body">
                    // Static Description (Always visible at top)
                    <div class="topic-section">
                        <div class="topic-cmd">
                            "$ cat README.md"
                        </div>
                        <h2 class="topic-heading">
                            {"# "}{content.title}
                        </h2>
                        <p class="topic-text">
                            {content.description}
                        </p>
                    </div>

                    // Interactive Area
                    <div class="interactive-zone">
                        {move || match state.get() {
                            TerminalState::Browsing => view! {
                                <div>
                                    <div class="resource-prompt">
                                        "ferris@rust:~/resources $ ls -l"
                                    </div>
                                    <div class="resource-list">
                                        {content.resources.iter().enumerate().map(|(i, res)| {
                                            let is_selected = i == selected_idx.get();
                                            let cursor = if is_selected { ">" } else { " " };
                                            let row_class = if is_selected { "resource-row selected" } else { "resource-row" };
                                            let badge_class = match res.badge {
                                                BadgeKind::Official => "resource-permission badge-official",
                                                BadgeKind::OpenSource => "resource-permission badge-opensource",
                                                BadgeKind::Article => "resource-permission badge-article",
                                                BadgeKind::Video => "resource-permission badge-video",
                                                BadgeKind::Feed => "resource-permission badge-feed",
                                                _ => "resource-permission badge-default",
                                            };

                                            view! {
                                                <div
                                                    class=row_class
                                                    on:click=move |_| {
                                                        set_selected_idx.set(i);
                                                        set_state.set(TerminalState::Confirming(i));
                                                    }
                                                >
                                                    <span class="resource-cursor">{cursor}</span>
                                                    <span class=badge_class>
                                                        {match res.badge {
                                                            BadgeKind::Official => "r--r--r--",     // Read-only
                                                            BadgeKind::OpenSource => "rwxr-xr-x",   // Executable
                                                            BadgeKind::Article => "-rw-r--r--",     // File
                                                            BadgeKind::Video => "-rw-r--r--",       // File
                                                            BadgeKind::Feed => "lrwxrwxrwx",        // Symlink
                                                            _ => "---------",
                                                        }}
                                                    </span>
                                                    <span class="resource-label">{res.label}</span>
                                                </div>
                                            }
                                        }).collect_view()}
                                    </div>
                                    <div class="keyboard-hint">
                                        "[Use Arrow Keys/Vim to move, Enter to select]"
                                    </div>
                                </div>
                            }.into_view(),

                            TerminalState::Confirming(idx) => {
                                let on_confirm = confirm_open.clone();
                                view! {
                                <div>
                                    <div class="resource-prompt">
                                        "ferris@rust:~/resources $ open \"" {content.resources[idx].label} "\""
                                    </div>
                                    <div class="confirm-warning">
                                        "warning: you are about to open an external link."
                                    </div>
                                    <div class="confirm-url">
                                        "Url: " <span class="confirm-url-link">{content.resources[idx].url}</span>
                                    </div>
                                    <div class="confirm-prompt blink-cursor">
                                        "Proceed? [Y/n] "
                                    </div>

                                    // Touch fallback (Mobile/Tablet only)
                                    <div class="mobile-actions">
                                        <button
                                            class="btn-confirm"
                                            on:click=move |_| on_confirm(idx)
                                        >
                                            "[Y] Yes"
                                        </button>

                                        <button
                                            class="btn-cancel"
                                            on:click=move |_| set_state.set(TerminalState::Browsing)
                                        >
                                            "[N] No"
                                        </button>
                                    </div>

                                    <div class="mobile-hint">
                                        "(Keyboard: Y / N / Enter)"
                                    </div>
                                </div>
                            }.into_view()
                            },

                            TerminalState::Opening => view! {
                                <div>
                                    <div class="opening-message">
                                      "Opening default browser..."
                                    </div>
                                    <div class="opening-pid">
                                        "Child process spawned (PID 1337)"
                                    </div>
                                </div>
                            }.into_view()
                        }}
                    </div>
                </div>
            </div>
        </div>
    }
}
