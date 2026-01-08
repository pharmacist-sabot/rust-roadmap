//! Application data aggregator.

pub mod sections;

use self::sections::*;
use crate::models::roadmap::{Dependency, Section, SectionLayout, Topic, TopicContent};

/// Defines the order of sections (The Spine).
pub const SECTIONS: &[Section] = &[
    Section {
        id: s01_introduction::SECTION_ID,
        title: "",
        order: 1,
        layout: SectionLayout::Grid { cols: 2 },
    },
    Section {
        id: s02_setup::SECTION_ID,
        title: "",
        order: 2,
        layout: SectionLayout::List,
    },
    Section {
        id: s03_language_basics::SECTION_ID,
        title: "",
        order: 3,
        layout: SectionLayout::List,
    },
    Section {
        id: s04_project_structure::SECTION_ID,
        title: "",
        order: 4,
        layout: SectionLayout::List,
    },
    Section {
        id: s05_advanced_types_traits::SECTION_ID,
        title: "",
        order: 5,
        layout: SectionLayout::List,
    },
    Section {
        id: s06_memory_lifetimes::SECTION_ID,
        title: "",
        order: 6,
        layout: SectionLayout::List,
    },
    Section {
        id: s07_error_handling_safety::SECTION_ID,
        title: "",
        order: 7,
        layout: SectionLayout::List,
    },
    Section {
        id: s08_testing_tdd::SECTION_ID,
        title: "",
        order: 8,
        layout: SectionLayout::List,
    },
    Section {
        id: s09_concurrency_parallelism::SECTION_ID,
        title: "",
        order: 9,
        layout: SectionLayout::List,
    },
    Section {
        id: s10_asynchronous_rust::SECTION_ID,
        title: "",
        order: 10,
        layout: SectionLayout::List,
    },
    Section {
        id: s11_macros_metaprogramming::SECTION_ID,
        title: "",
        order: 11,
        layout: SectionLayout::List,
    },
    Section {
        id: s12_serialization_data::SECTION_ID,
        title: "",
        order: 12,
        layout: SectionLayout::List,
    },
    Section {
        id: s13_networking_io::SECTION_ID,
        title: "",
        order: 13,
        layout: SectionLayout::List,
    },
    Section {
        id: s14_databases_orm::SECTION_ID,
        title: "",
        order: 14,
        layout: SectionLayout::List,
    },
    Section {
        id: s15_documentation_docs::SECTION_ID,
        title: "",
        order: 15,
        layout: SectionLayout::List,
    },
    Section {
        id: s16_debugging_tools::SECTION_ID,
        title: "",
        order: 16,
        layout: SectionLayout::List,
    },
    Section {
        id: s17_performance_optimization::SECTION_ID,
        title: "",
        order: 17,
        layout: SectionLayout::List,
    },
    Section {
        id: s18_cli_utilities::SECTION_ID,
        title: "",
        order: 18,
        layout: SectionLayout::List,
    },
    Section {
        id: s19_web_applications::SECTION_ID,
        title: "",
        order: 19,
        layout: SectionLayout::List,
    },
    Section {
        id: s20_webassembly_wasm::SECTION_ID,
        title: "",
        order: 20,
        layout: SectionLayout::List,
    },
    Section {
        id: s21_gui_desktop::SECTION_ID,
        title: "",
        order: 21,
        layout: SectionLayout::List,
    },
    Section {
        id: s22_embedded_systems::SECTION_ID,
        title: "",
        order: 22,
        layout: SectionLayout::List,
    },
    Section {
        id: s23_game_dev_graphics::SECTION_ID,
        title: "",
        order: 23,
        layout: SectionLayout::List,
    },
    Section {
        id: s24_cryptography_security::SECTION_ID,
        title: "",
        order: 24,
        layout: SectionLayout::List,
    },
    Section {
        id: s25_unsafe_rust::SECTION_ID,
        title: "",
        order: 25,
        layout: SectionLayout::List,
    },
    Section {
        id: s26_ffi_interop::SECTION_ID,
        title: "",
        order: 26,
        layout: SectionLayout::List,
    },
    Section {
        id: s27_package_management_deepdive::SECTION_ID,
        title: "",
        order: 27,
        layout: SectionLayout::List,
    },
];

/// Aggregates topics from all modular files.
pub fn get_all_topics() -> Vec<Topic> {
    let mut topics = Vec::new();
    topics.extend(s01_introduction::get_topics());
    topics.extend(s02_setup::get_topics());
    topics.extend(s03_language_basics::get_topics());
    topics.extend(s04_project_structure::get_topics());
    topics.extend(s05_advanced_types_traits::get_topics());
    topics.extend(s06_memory_lifetimes::get_topics());
    topics.extend(s07_error_handling_safety::get_topics());
    topics.extend(s08_testing_tdd::get_topics());
    topics.extend(s09_concurrency_parallelism::get_topics());
    topics.extend(s10_asynchronous_rust::get_topics());
    topics.extend(s11_macros_metaprogramming::get_topics());
    topics.extend(s12_serialization_data::get_topics());
    topics.extend(s13_networking_io::get_topics());
    topics.extend(s14_databases_orm::get_topics());
    topics.extend(s15_documentation_docs::get_topics());
    topics.extend(s16_debugging_tools::get_topics());
    topics.extend(s17_performance_optimization::get_topics());
    topics.extend(s18_cli_utilities::get_topics());
    topics.extend(s19_web_applications::get_topics());
    topics.extend(s20_webassembly_wasm::get_topics());
    topics.extend(s21_gui_desktop::get_topics());
    topics.extend(s22_embedded_systems::get_topics());
    topics.extend(s23_game_dev_graphics::get_topics());
    topics.extend(s24_cryptography_security::get_topics());
    topics.extend(s25_unsafe_rust::get_topics());
    topics.extend(s26_ffi_interop::get_topics());
    topics.extend(s27_package_management_deepdive::get_topics());
    topics
}

/// Aggregates dependencies from all modular files + Spine connections.
pub fn get_all_dependencies() -> Vec<Dependency> {
    let mut deps = Vec::new();

    // 1. Internal Module Dependencies
    deps.extend(s01_introduction::get_dependencies());
    deps.extend(s02_setup::get_dependencies());
    deps.extend(s03_language_basics::get_dependencies());
    deps.extend(s04_project_structure::get_dependencies());
    deps.extend(s05_advanced_types_traits::get_dependencies());
    deps.extend(s06_memory_lifetimes::get_dependencies());
    deps.extend(s07_error_handling_safety::get_dependencies());
    deps.extend(s08_testing_tdd::get_dependencies());
    deps.extend(s09_concurrency_parallelism::get_dependencies());
    deps.extend(s10_asynchronous_rust::get_dependencies());
    deps.extend(s11_macros_metaprogramming::get_dependencies());
    deps.extend(s12_serialization_data::get_dependencies());
    deps.extend(s13_networking_io::get_dependencies());
    deps.extend(s14_databases_orm::get_dependencies());
    deps.extend(s15_documentation_docs::get_dependencies());
    deps.extend(s16_debugging_tools::get_dependencies());
    deps.extend(s17_performance_optimization::get_dependencies());
    deps.extend(s18_cli_utilities::get_dependencies());
    deps.extend(s19_web_applications::get_dependencies());
    deps.extend(s20_webassembly_wasm::get_dependencies());
    deps.extend(s21_gui_desktop::get_dependencies());
    deps.extend(s22_embedded_systems::get_dependencies());
    deps.extend(s23_game_dev_graphics::get_dependencies());
    deps.extend(s24_cryptography_security::get_dependencies());
    deps.extend(s25_unsafe_rust::get_dependencies());
    deps.extend(s26_ffi_interop::get_dependencies());
    deps.extend(s27_package_management_deepdive::get_dependencies());

    // 2. Spine Connections (The Backbone)
    let spine_connections = vec![
        ("intro", "setup_env"),
        ("setup_env", "basics"),
        ("basics", "project_structure_spine"),
        ("project_structure_spine", "advanced_types_traits_spine"),
        ("advanced_types_traits_spine", "memory_lifetimes_spine"),
        ("memory_lifetimes_spine", "error_handling_safety_spine"),
        ("error_handling_safety_spine", "testing_tdd_spine"),
        ("testing_tdd_spine", "concurrency_parallelism_spine"),
        ("concurrency_parallelism_spine", "asynchronous_rust_spine"),
        ("asynchronous_rust_spine", "macros_metaprogramming_spine"),
        ("macros_metaprogramming_spine", "serialization_data_spine"),
        ("serialization_data_spine", "networking_io_spine"),
        ("networking_io_spine", "databases_orm_spine"),
        ("databases_orm_spine", "documentation_docs_spine"),
        ("documentation_docs_spine", "debugging_tools_spine"),
        ("debugging_tools_spine", "performance_optimization_spine"),
        ("performance_optimization_spine", "cli_utilities_spine"),
        ("cli_utilities_spine", "web_applications_spine"),
        ("web_applications_spine", "webassembly_wasm_spine"),
        ("webassembly_wasm_spine", "gui_desktop_spine"),
        ("gui_desktop_spine", "embedded_systems_spine"),
        ("embedded_systems_spine", "game_dev_graphics_spine"),
        ("game_dev_graphics_spine", "cryptography_security_spine"),
        ("cryptography_security_spine", "unsafe_rust_spine"),
        ("unsafe_rust_spine", "ffi_interop_spine"),
        ("ffi_interop_spine", "package_management_deepdive_spine"),
    ];

    for (from, to) in spine_connections {
        deps.push(Dependency { from, to });
    }

    deps
}

/// Aggregates content lookup from all decentralized content files.
pub fn get_topic_content(id: &str) -> Option<TopicContent> {
    if let Some(c) = s01_introduction::content::get_content(id) {
        return Some(c);
    }
    if let Some(c) = s02_setup::content::get_content(id) {
        return Some(c);
    }
    if let Some(c) = s03_language_basics::content::get_content(id) {
        return Some(c);
    }
    if let Some(c) = s04_project_structure::content::get_content(id) {
        return Some(c);
    }
    if let Some(c) = s05_advanced_types_traits::content::get_content(id) {
        return Some(c);
    }
    if let Some(c) = s06_memory_lifetimes::content::get_content(id) {
        return Some(c);
    }
    if let Some(c) = s07_error_handling_safety::content::get_content(id) {
        return Some(c);
    }
    if let Some(c) = s08_testing_tdd::content::get_content(id) {
        return Some(c);
    }
    if let Some(c) = s09_concurrency_parallelism::content::get_content(id) {
        return Some(c);
    }
    if let Some(c) = s10_asynchronous_rust::content::get_content(id) {
        return Some(c);
    }
    if let Some(c) = s11_macros_metaprogramming::content::get_content(id) {
        return Some(c);
    }
    if let Some(c) = s12_serialization_data::content::get_content(id) {
        return Some(c);
    }
    if let Some(c) = s13_networking_io::content::get_content(id) {
        return Some(c);
    }
    if let Some(c) = s14_databases_orm::content::get_content(id) {
        return Some(c);
    }
    if let Some(c) = s15_documentation_docs::content::get_content(id) {
        return Some(c);
    }
    if let Some(c) = s16_debugging_tools::content::get_content(id) {
        return Some(c);
    }
    if let Some(c) = s17_performance_optimization::content::get_content(id) {
        return Some(c);
    }
    if let Some(c) = s18_cli_utilities::content::get_content(id) {
        return Some(c);
    }
    if let Some(c) = s19_web_applications::content::get_content(id) {
        return Some(c);
    }
    if let Some(c) = s20_webassembly_wasm::content::get_content(id) {
        return Some(c);
    }
    if let Some(c) = s21_gui_desktop::content::get_content(id) {
        return Some(c);
    }
    if let Some(c) = s22_embedded_systems::content::get_content(id) {
        return Some(c);
    }
    if let Some(c) = s23_game_dev_graphics::content::get_content(id) {
        return Some(c);
    }
    if let Some(c) = s24_cryptography_security::content::get_content(id) {
        return Some(c);
    }
    if let Some(c) = s25_unsafe_rust::content::get_content(id) {
        return Some(c);
    }
    if let Some(c) = s26_ffi_interop::content::get_content(id) {
        return Some(c);
    }
    if let Some(c) = s27_package_management_deepdive::content::get_content(id) {
        return Some(c);
    }
    None
}
