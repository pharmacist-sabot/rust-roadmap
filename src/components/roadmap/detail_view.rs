use crate::models::roadmap::{BadgeKind, TopicContent};
use leptos::{ev::KeyboardEvent, *};
use std::time::Duration;
use web_sys::window;

/// State ของ Terminal
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
    let content_for_keydown = content.clone();

    let handle_keydown = move |ev: KeyboardEvent| {
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
                    set_state.set(TerminalState::Opening);
                    // Open Link Logic
                    let url = content_for_keydown.resources[idx].url;
                    set_timeout(
                        move || {
                            if let Some(w) = window() {
                                let _ = w.open_with_url_and_target(url, "_blank");
                            }
                            // Reset state after opening
                            set_state.set(TerminalState::Browsing);
                        },
                        Duration::from_millis(800),
                    ); // Fake delay for realism
                }
                "n" | "N" | "Escape" => {
                    set_state.set(TerminalState::Browsing); // Cancel
                }
                _ => {}
            },
            TerminalState::Opening => {} // Ignore input while opening
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
            <div class="flex-grow w-full terminal-backdrop" on:click=move |_| on_close.call(())></div>

            // Terminal Panel
            <div class="terminal-panel" on:click=move |e| e.stop_propagation()>
                // Header
                <div class="terminal-header">
                    <div class="terminal-controls">
                        <span class="term-btn term-close" on:click=move |_| on_close.call(())></span>
                        <span class="term-btn term-min"></span>
                        <span class="term-btn term-max"></span>
                    </div>
                    <div class="terminal-title" id="terminal-title">"rust-roadmap — interactive-tui"</div>
                    <div class="w-10"></div>
                </div>

                // Body
                <div class="terminal-body font-mono">
                    // Static Description (Always visible at top)
                    <div class="mb-6 border-b border-gray-800 pb-4">
                        <div class="cmd-prompt text-sm text-gray-500 mb-2">
                            "$ cat README.md"
                        </div>
                        <h2 class="text-lg md:text-xl font-bold text-white mb-2">
                            {"# "}{content.title}
                        </h2>
                        <p class="text-sm md:text-base text-gray-400 leading-relaxed">
                            {content.description}
                        </p>
                    </div>

                    // Interactive Area
                    <div class="interactive-zone min-h-[200px]">
                        {move || match state.get() {
                            TerminalState::Browsing => view! {
                                <div>
                                    <div class="cmd-prompt text-green-400 mb-2">
                                        "ferris@rust:~/resources $ ls -l"
                                    </div>
                                    <div class="flex flex-col gap-1">
                                        {content.resources.iter().enumerate().map(|(i, res)| {
                                            let is_selected = i == selected_idx.get();
                                            let cursor = if is_selected { ">" } else { " " };
                                            let row_class = if is_selected { "bg-[#2d2d2d] text-white" } else { "text-gray-500" };
                                            let badge_color = match res.badge {
                                                BadgeKind::Official => "text-blue-400",
                                                _ => "text-yellow-600",
                                            };

                                            view! {
                                                <div
                                                    class=format!("flex items-center gap-3 px-2 py-1 cursor-pointer {}", row_class)
                                                    on:click=move |_| {
                                                        set_selected_idx.set(i);
                                                        set_state.set(TerminalState::Confirming(i));
                                                    }
                                                >
                                                    <span class="text-orange-500 font-bold w-4">{cursor}</span>
                                                    <span class=format!("text-xs w-24 {}", badge_color)>
                                                        {match res.badge {
                                                            BadgeKind::Official => "r--r--r--",
                                                            _ => "rwx------",
                                                        }}
                                                    </span>
                                                    <span class="font-mono">{res.label}</span>
                                                </div>
                                            }
                                        }).collect_view()}
                                    </div>
                                    <div class="mt-4 text-xs text-gray-600">
                                        "[Use Arrow Keys/Vim to move, Enter to select]"
                                    </div>
                                </div>
                            }.into_view(),

                            TerminalState::Confirming(idx) => view! {
                                <div>
                                    <div class="cmd-prompt text-green-400 mb-2">
                                        "ferris@rust:~/resources $ open \"" {content.resources[idx].label} "\""
                                    </div>
                                    <div class="text-yellow-300 mb-2">
                                        "warning: you are about to open an external link."
                                    </div>
                                    <div class="text-white">
                                        "Url: " <span class="text-blue-400 underline">{content.resources[idx].url}</span>
                                    </div>
                                    <div class="mt-4 font-bold text-white blink-cursor">
                                        "Proceed? [Y/n] "
                                    </div>
                                </div>
                            }.into_view(),

                            TerminalState::Opening => view! {
                                <div>
                                    <div class="text-green-500">
                                        "Opening default browser..."
                                    </div>
                                    <div class="text-gray-500 text-sm mt-1">
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
