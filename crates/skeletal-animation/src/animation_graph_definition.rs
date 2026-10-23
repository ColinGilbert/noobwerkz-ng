use petgraph::graph::Graph;
use petgraph::graph::NodeIndex;

safe_index::new! {
    TransitionDefinitionIndex,
    map: TransitionDefinitionContainer,
}

pub struct TransitionDefinition {
    pub duration: web_time::Duration,
}

pub enum GenericNodeDefinition {
    Sampler(SamplerNodeDefinition),
    BlendTreeOneDim(BlendTreeOneDimDefinition)
}

pub struct SamplerNodeDefinition {
    pub speed: f32,
    pub animation_name: String,
    pub looping: bool,
    pub name: String,
}

pub struct BlendTreeOneDimDefinition {
    pub animation_names: Vec<String>,
    pub name: String
}

pub struct AnimationGraphDefinition {
    pub graph: Graph<GenericNodeDefinition, TransitionDefinition>,
    pub root: Option<NodeIndex>,
}


impl AnimationGraphDefinition {
    pub fn new() -> Self {
        let graph: Graph<GenericNodeDefinition, TransitionDefinition> = Graph::<GenericNodeDefinition, TransitionDefinition>::default();
        Self {
            graph,
            root: None
        }
    }
}