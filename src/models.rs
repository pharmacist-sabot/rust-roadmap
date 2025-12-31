#[derive(Clone, Debug, PartialEq)]
pub enum ResourceType {
    Official,
    Article,
    Video,
}

impl ResourceType {
    pub fn color_class(&self) -> &'static str {
        match self {
            ResourceType::Official => "bg-blue-600 text-white",
            ResourceType::Article => "bg-yellow-400 text-black",
            ResourceType::Video => "bg-purple-600 text-white",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            ResourceType::Official => "Official",
            ResourceType::Article => "Article",
            ResourceType::Video => "Video",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ResourceLink {
    pub kind: ResourceType,
    pub title: String,
    pub url: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TopicContent {
    pub id: String,
    pub title: String,
    pub description: String,
    pub links: Vec<ResourceLink>,
}
