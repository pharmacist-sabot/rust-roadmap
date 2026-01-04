use crate::models::roadmap::TopicContent;
use leptos::{ev::KeyboardEvent, html::Div, *};
use web_sys::window;

/// Log Types for the Terminal History
#[derive(Clone, PartialEq, Eq, Debug)]
enum LogType {
    Command(String),
    Output(String),
    Error(String),
    Link {
        label: String,
        url: String,
        id: usize,
    },
    System(String),
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct LogLine {
    content: LogType,
}

#[component]
pub fn TopicDetail(content: TopicContent, on_close: Callback<()>) -> impl IntoView {
    // --- State ---
    let (history, set_history) = create_signal::<Vec<LogLine>>(vec![]);

    // Command history for Up/Down navigation
    let (cmd_history, set_cmd_history) = create_signal::<Vec<String>>(vec![]);
    let (history_index, set_history_index) = create_signal::<Option<usize>>(None);

    let (input, set_input) = create_signal(String::new());

    // We can use this to manually control cursor blink if needed, or just CSS
    let (_cursor_visible, _set_cursor_visible) = create_signal(true);

    let content_ref = store_value(content.clone());
    let container_ref = create_node_ref::<Div>();

    // --- Helpers ---
    let push_log = move |log: LogType| {
        set_history.update(|h| h.push(LogLine { content: log }));
        // Request layout update to scroll
        request_animation_frame(move || {
            if let Some(d) = container_ref.get() {
                d.set_scroll_top(d.scroll_height());
            }
        });
    };

    let execute_command = move |cmd: String| {
        let trimmed = cmd.trim();
        push_log(LogType::Command(cmd.clone()));

        // Add to history if not empty
        if !trimmed.is_empty() {
            set_cmd_history.update(|h| h.push(cmd.clone()));
            set_history_index.set(None); // Reset history cursor
        }

        if trimmed.is_empty() {
            return;
        }

        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        match parts[0] {
            "help" => {
                push_log(LogType::Output("Available commands:".to_string()));
                push_log(LogType::Output("  ls       - List resources".to_string()));
                push_log(LogType::Output(
                    "  open <ID> - Open resource by ID".to_string(),
                ));
                push_log(LogType::Output("  cat      - Show description".to_string()));
                push_log(LogType::Output("  clear    - Clear terminal".to_string()));
                push_log(LogType::Output("  whoami   - Current user".to_string()));
                push_log(LogType::Output("  exit     - Close terminal".to_string()));
            }
            "clear" => {
                set_history.set(vec![]);
            }
            "exit" => {
                on_close.call(());
            }
            "ls" => {
                let c = content_ref.get_value();
                push_log(LogType::System(format!(
                    "Listing resources for '{}':",
                    c.title
                )));
                for (i, res) in c.resources.iter().enumerate() {
                    push_log(LogType::Link {
                        label: format!("[{}] {}", i + 1, res.label), // 1-based index for UX
                        url: res.url.to_string(),
                        id: i,
                    });
                }
            }
            "cat" => {
                let c = content_ref.get_value();
                push_log(LogType::Output(c.description.to_string()));
            }
            "whoami" => {
                push_log(LogType::Output("ferris".to_string()));
            }
            "open" => {
                if parts.len() < 2 {
                    push_log(LogType::Error("Usage: open <ID>".to_string()));
                } else {
                    if let Ok(id) = parts[1].parse::<usize>() {
                        let c = content_ref.get_value();
                        if id > 0 && id <= c.resources.len() {
                            let idx = id - 1; // Convert back to 0-based
                            let url = c.resources[idx].url.to_string();
                            push_log(LogType::System(format!("Opening: {}", url)));
                            if let Some(w) = window() {
                                let _ = w.open_with_url_and_target(&url, "_blank");
                            }
                        } else {
                            push_log(LogType::Error(format!("ID {} not found.", id)));
                        }
                    } else {
                        push_log(LogType::Error("Invalid ID number.".to_string()));
                    }
                }
            }
            _ => {
                push_log(LogType::Error(format!("command not found: {}", parts[0])));
            }
        }
    };

    // --- On Mount: Run 'ls' automatically ---
    create_effect(move |_| {
        // Run only once when history is empty
        if history.get_untracked().is_empty() {
            push_log(LogType::System(
                "Welcome to Rust Roadmap Terminal v2.0".to_string(),
            ));
            push_log(LogType::System(
                "Type 'help' to see available commands.".to_string(),
            ));
            execute_command("ls".to_string());
        }
    });

    // --- Handlers ---
    let handle_keydown = move |ev: KeyboardEvent| {
        let key = ev.key();

        // Prevent default behavior for Arrow keys to stop scrolling if needed
        if key == "ArrowUp" || key == "ArrowDown" {
            ev.prevent_default();
        }

        if ev.meta_key() || ev.ctrl_key() || ev.alt_key() {
            return;
        }

        match key.as_str() {
            "Enter" => {
                let cmd = input.get();
                set_input.set(String::new());
                execute_command(cmd);
            }
            "Backspace" => {
                set_input.update(|s| {
                    let _ = s.pop();
                });
            }
            "ArrowUp" => {
                let cmds = cmd_history.get();
                if cmds.is_empty() {
                    return;
                }

                let current_idx = history_index.get();
                let next_idx = match current_idx {
                    None => Some(cmds.len() - 1),
                    Some(i) => {
                        if i > 0 {
                            Some(i - 1)
                        } else {
                            Some(0)
                        }
                    }
                };

                if let Some(i) = next_idx {
                    set_history_index.set(Some(i));
                    set_input.set(cmds[i].clone());
                }
            }
            "ArrowDown" => {
                let cmds = cmd_history.get();
                if cmds.is_empty() {
                    return;
                }

                let current_idx = history_index.get();
                let next_idx = match current_idx {
                    None => None,
                    Some(i) => {
                        if i < cmds.len() - 1 {
                            Some(i + 1)
                        } else {
                            None
                        }
                    }
                };

                set_history_index.set(next_idx);
                if let Some(i) = next_idx {
                    set_input.set(cmds[i].clone());
                } else {
                    set_input.set(String::new());
                }
            }
            "Escape" => {
                on_close.call(());
            }
            k if k.len() == 1 => {
                // Printable characters
                set_input.update(|s| s.push_str(k));
            }
            _ => {}
        }

        // Always scroll to bottom on typing
        if let Some(d) = container_ref.get() {
            d.set_scroll_top(d.scroll_height());
        }
    };

    // helper for clicking a link (bypass typing)
    let click_open_link = move |url: String| {
        // Simulate typing the command for effect
        push_log(LogType::Command(format!("open \"{}\"", url)));
        if let Some(w) = window() {
            let _ = w.open_with_url_and_target(&url, "_blank");
        }
    };

    // Auto focus
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
            on:keydown=handle_keydown
            tabindex="-1"
            ref=container_ref
            style="outline: none;" // Remove focus ring
        >
            <div class="flex-grow w-full terminal-backdrop" on:click=move |_| on_close.call(())></div>

            <div class="terminal-panel" on:click=move |e| e.stop_propagation()>
                // --- Window Header ---
                <div class="terminal-header">
                    <div class="terminal-controls">
                        <span class="term-btn term-close" on:click=move |_| on_close.call(())></span>
                        <span class="term-btn term-min"></span>
                        <span class="term-btn term-max"></span>
                    </div>
                    // Dynamic title based on CWD
                    <div class="terminal-title">
                        {move || format!("ferris@rust: ~/{}", content_ref.get_value().title.to_lowercase().replace(" ", "-"))}
                    </div>
                    <div class="w-10"></div>
                </div>

                // --- Terminal Body (History + Prompt) ---
                <div class="terminal-body font-mono text-sm leading-relaxed" style="scroll-behavior: smooth;">
                    // 1. History
                    {move || history.get().into_iter().map(|line| {
                        match line.content {
                            LogType::Command(cmd) => view! {
                                <div class="mb-1">
                                    <span class="text-green-400 mr-2 md:mr-4">"ferris@rust:~$ "</span>
                                    <span class="text-white">{cmd}</span>
                                </div>
                            }.into_view(),
                            LogType::Output(out) => view! {
                                // Break all for long outputs
                                <div class="mb-1 text-gray-300 w-full whitespace-pre-wrap pl-0 break-all">{out}</div>
                            }.into_view(),
                            LogType::Error(err) => view! {
                                <div class="mb-1 text-red-400">{err}</div>
                            }.into_view(),
                            LogType::System(msg) => view! {
                                <div class="mb-1 text-blue-400 italic opacity-80">{msg}</div>
                            }.into_view(),
                            LogType::Link { label, url, .. } => view! {
                                <div class="mb-1">
                                    <span
                                        class="text-yellow-400 underline cursor-pointer hover:text-yellow-200 transition-colors"
                                        on:click={
                                            let u = url.clone();
                                            move |_| click_open_link(u.clone())
                                        }
                                    >
                                        {label}
                                    </span>
                                </div>
                            }.into_view(),
                        }
                    }).collect_view()}

                    // 2. Active Prompt
                    <div class="flex items-center mt-2 group">
                        <span class="text-green-400 mr-2 shrink-0 md:mr-4">"ferris@rust:~$ "</span>
                        <span class="text-white whitespace-pre-wrap break-all">
                            {move || input.get()}
                            <span class="inline-block w-2.5 h-4 bg-gray-500 ml-0.5 animate-pulse align-middle"></span>
                        </span>
                    </div>

                    // 3. Mobile Shortcuts (Quick Actions)
                    <div class="mt-6 pt-4 border-t border-gray-800 lg:hidden flex gap-2 overflow-x-auto pb-2" on:click=move |e| e.stop_propagation()>
                        <button
                            class="px-3 py-1 bg-[#2d2d2d] rounded text-xs text-green-400 border border-gray-700 active:bg-gray-700"
                            on:click=move |_| execute_command("ls".to_string())
                        >
                            "ls"
                        </button>
                        <button
                            class="px-3 py-1 bg-[#2d2d2d] rounded text-xs text-blue-400 border border-gray-700 active:bg-gray-700"
                            on:click=move |_| execute_command("help".to_string())
                        >
                            "help"
                        </button>
                        <button
                            class="px-3 py-1 bg-[#2d2d2d] rounded text-xs text-yellow-400 border border-gray-700 active:bg-gray-700"
                            on:click=move |_| execute_command("clear".to_string())
                        >
                            "clear"
                        </button>
                         <button
                            class="px-3 py-1 bg-[#2d2d2d] rounded text-xs text-red-400 border border-gray-700 active:bg-gray-700"
                            on:click=move |_| on_close.call(())
                        >
                            "exit"
                        </button>
                    </div>
                </div>
            </div>
        </div>
    }
}
