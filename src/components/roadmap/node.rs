//! SVG node component for rendering roadmap topics.

use crate::models::roadmap::{Level, TopicType};
use leptos::*;

#[derive(Clone, Debug, PartialEq)]
pub struct NodeData {
    pub id: &'static str,
    pub title: &'static str,
    pub level: Level,
    pub topic_type: TopicType, // เพิ่ม Field นี้
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

#[component]
pub fn RoadmapNode(props: NodeData) -> impl IntoView {
    // เลือก class ตามประเภทหัวข้อ
    let type_class = match props.topic_type {
        TopicType::Main => "type-main",
        TopicType::Sub => "type-sub",
    };

    let class_attr = format!("roadmap-node {}", type_class);

    view! {
        <g class=class_attr data-topic-id=props.id>
            <rect
                x=props.x
                y=props.y
                width=props.width
                height=props.height
                rx="4"
                ry="4"
                class="node-rect"
            />
            <text
                x=props.x + props.width / 2.0
                y=props.y + props.height / 2.0
                class="node-text"
                text-anchor="middle"
                dominant-baseline="central"
            >
                {props.title}
            </text>
        </g>
    }
}
