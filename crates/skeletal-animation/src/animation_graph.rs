// TODO: Stub out
use crate::animation::*;
use crate::animation_graph_definition::*;
use crate::blend_multiplicative::blend_multiplicative;
use crate::local_to_model::local_to_model;
use crate::operation_result::LocalToLocalResults;
use crate::operation_result::LocalToModelResults;
use anyhow::anyhow;
use noobwerkz_shared::skeleton::*;
use petgraph::graph::{EdgeIndex, Graph, NodeIndex};
use std::collections::HashMap;
use std::collections::VecDeque;
use std::sync::Arc;

safe_index::new!(
    AnimationIndex,
    map: Animations,
);

safe_index::new! (
SamplerNodeIndex,
map: SamplerNodesContainer
);

safe_index::new! {
BlendTreeOneDimNodeIndex,
map: BlendTreeOneDimNodesContainer
}

safe_index::new! {
TransitionIndex,
map: TransitionsContainer
}

pub struct Transition {
    pub duration: web_time::Duration,
    pub seek: web_time::Duration,
    pub started: bool,
}

impl Transition {
    pub fn new(duration: web_time::Duration) -> Transition {
        Transition {
            duration,
            seek: web_time::Duration::from_nanos(0),

            started: false,
        }
    }

    pub fn reset(&mut self) {
        self.seek = web_time::Duration::from_nanos(0);
        self.started = false;
    }
}

pub enum GenericNode {
    Sampler(SamplerNodeIndex),
    BlendTreeOneDim(BlendTreeOneDimNodeIndex),
}
pub struct SamplerNode {
    pub speed: f32,
    pub looping: bool,
    pub index: AnimationIndex,
}

impl SamplerNode {
    pub fn new(speed: f32, looping: bool, index: AnimationIndex) -> Self {
        SamplerNode {
            speed,
            looping,
            index,
        }
    }
}

pub struct AnimationGraph {
    skeleton: Arc<Skeleton>,
    graph: Graph<GenericNode, TransitionIndex>,
    animations: Animations<Animation>,
    samplers: SamplerNodesContainer<SamplerNode>,
    transitions: TransitionsContainer<Transition>,
    root_node_idx: NodeIndex,
    current_node_idx: Option<NodeIndex>,
    current_edge_idx: Option<EdgeIndex>,
    target: NodeIndex,
    on_a_transition: bool,
    path: VecDeque<EdgeIndex>,
    node_names: HashMap<String, NodeIndex>,
    output_locals: LocalToLocalResults,
    pub output_model: LocalToModelResults,
    duration: web_time::Duration,
}

impl AnimationGraph {
    pub fn new(
        skeleton: Arc<Skeleton>,
        animations: Animations<Animation>,
        animgraph_definition: &AnimationGraphDefinition,
        animations_by_name: &HashMap<String, AnimationIndex>,
    ) -> Result<Self, anyhow::Error> {
        let mut graph = Graph::<GenericNode, TransitionIndex>::with_capacity(
            animgraph_definition.graph.node_count(),
            animgraph_definition.graph.edge_count(),
        );
        let mut samplers = SamplerNodesContainer::<SamplerNode>::new();
        // Go over each node in the animgraph's definition and add it to the final graph, saving its definition node/final node pair in a map
        let mut node_definitions_to_node_mappings = HashMap::<NodeIndex, NodeIndex>::new();
        let mut node_names = HashMap::<String, NodeIndex>::new();
        for node_definition_idx in animgraph_definition.graph.node_indices() {
            let node_definition = animgraph_definition.graph.node_weight(node_definition_idx);
            match node_definition {
                Some(def) => {
                    match def {
                        GenericNodeDefinition::Sampler(val) => {
                            if !animations_by_name.contains_key(&val.animation_name) {
                                return Err(anyhow!(
                                    "Could not find animation name {}",
                                    &val.animation_name
                                ));
                            }
                            if node_names.contains_key(&val.name) {
                                return Err(anyhow!("Duplicate node name: {}", &val.name));
                            }
                            let animation_index = &animations_by_name[&val.animation_name];
                            let sampler_node =
                                SamplerNode::new(val.speed, val.looping, *animation_index);
                            let sampler_idx = samplers.push(sampler_node);
                            let node_idx = graph.add_node(GenericNode::Sampler(
                                SamplerNodeIndex::from(sampler_idx),
                            ));
                            node_names.insert(val.name.clone(), node_idx);
                            node_definitions_to_node_mappings.insert(node_definition_idx, node_idx);
                        }
                        GenericNodeDefinition::BlendTreeOneDim(_val) => {} // TODO: DO LATER
                    }
                }
                None => {
                    return Err(anyhow!("Invalid node definition"));
                }
            }
        }

        let mut transitions = TransitionsContainer::<Transition>::new();

        // Go over each edge in the animgraph's definition and add it to the final graph, using the node mapping to find the appropriate final node.
        for edge_index in animgraph_definition.graph.edge_indices() {
            let endpoints = animgraph_definition.graph.edge_endpoints(edge_index);
            match endpoints {
                Some(endpoint_indices) => {
                    if !node_definitions_to_node_mappings.contains_key(&endpoint_indices.0) {
                        return Err(anyhow!("Couldn't find \"from\" node from edge"));
                    }
                    if !node_definitions_to_node_mappings.contains_key(&endpoint_indices.1) {
                        return Err(anyhow!("Couldn't find \"to\" node from edge"));
                    }

                    let edge_def = animgraph_definition.graph.edge_weight(edge_index);
                    let duration: web_time::Duration;
                    match edge_def {
                        Some(val) => {
                            duration = val.duration;
                        }
                        None => {
                            return Err(anyhow!(
                                "Invalid edge weight found while adding transition"
                            ));
                        }
                    }
                    let transition = Transition::new(duration);
                    let transition_idx = transitions.push(transition);
                    let _ = graph.add_edge(endpoint_indices.0, endpoint_indices.1, transition_idx);
                }
                None => return Err(anyhow!("Invalid edge endpoints")),
            }
        }

        let root_node_idx = node_definitions_to_node_mappings[&animgraph_definition.root.unwrap()];
        let current_node_idx = Some(root_node_idx);
        let target = root_node_idx;
        let path = VecDeque::<EdgeIndex>::new();

        Ok(AnimationGraph {
            skeleton: skeleton.clone(),
            animations,
            graph,
            samplers,
            transitions,
            root_node_idx,
            current_node_idx,
            current_edge_idx: None,
            target,
            path,
            on_a_transition: false,
            node_names,
            output_locals: LocalToLocalResults::new(),
            output_model: LocalToModelResults::new(),
            duration: web_time::Duration::new(0,0),
        })
        // Err(anyhow!("Method unimplemented"))
    }

    pub fn evaluate(&mut self, dt: web_time::Duration) -> Result<(), anyhow::Error> {
        self.duration += dt;
        self.output_locals = LocalToLocalResults::new();
        // Handle the transition case
        if self.on_a_transition {
            println!("On a transition!");
            match self.current_edge_idx {
                Some(edge_idx) => {
                    let transition_idx = self.graph.edge_weight(edge_idx).unwrap();
                    // Calculate the time taken and check whether or not the transition is finished.
                    self.transitions[*transition_idx].seek += self.duration;
                    let finished: bool;
                    if self.transitions[*transition_idx].seek
                        >= self.transitions[*transition_idx].duration
                    {
                        finished = true;
                    } else {
                        finished = false;
                    }
                    // If finished, move onto the next node.
                    if finished {
                        self.transitions[*transition_idx].reset();
                        let front = self.path.front();
                        match front {
                            Some(val) => {
                                self.current_edge_idx = Some(*val);
                                self.path.pop_front();
                                self.on_a_transition = false;
                            }
                            None => {}
                        }
                    } else {
                        // We need to evaluate the
                        let edge = self
                            .graph
                            .edge_endpoints(self.current_edge_idx.unwrap())
                            .unwrap();
                        let from = self.graph.node_weight(edge.0).unwrap();
                        // Otherwise, evaluate the transition
                        let mut from_results = LocalToLocalResults::new(); // TODO: Init inside match clause
                        match from {
                            GenericNode::Sampler(val) => {
                                self.animations[self.samplers[*val].index].seek(self.duration.as_secs_f32());
                                from_results =
                                    self.animations[self.samplers[*val].index].playback();
                            }
                            GenericNode::BlendTreeOneDim(_) => {
                                // self.blend_trees_one_dim[*val].update(dt);
                                // TODO
                            }
                        }
                        let to = self.graph.node_weight(edge.1).unwrap();
                        let mut to_results = LocalToLocalResults::new(); // TODO: Init inside match clause
                        match to {
                            GenericNode::Sampler(val) => {
                                self.animations[self.samplers[*val].index].seek(self.duration.as_secs_f32());
                                to_results = self.animations[self.samplers[*val].index].playback();
                            }
                            GenericNode::BlendTreeOneDim(_) => {
                                // self.blend_trees_one_dim[*val].update(dt);
                                // TODO
                            }
                        }
                        // Set the blend layer weights based on time elapsed.
                        self.transitions[*transition_idx].seek =
                            self.transitions[*transition_idx].seek + dt;
                        let ratio = (self.transitions[*transition_idx].duration.as_millis() as f32
                            / self.transitions[*transition_idx].seek.as_millis() as f32)
                            .clamp(0.0, 1.0);
                        self.output_locals = blend_multiplicative(
                            &from_results,
                            &to_results,
                            ratio,
                            &self.skeleton.valid,
                        );
                    }
                }
                None => return Err(anyhow! {"Invalid current transition during evaluation."}),
            }

            let last_transition_idx = self
                .graph
                .edge_weight(self.current_edge_idx.unwrap())
                .unwrap();
            let last_transition = &self.transitions[*last_transition_idx];

            if last_transition.seek >= last_transition.duration {
                self.transitions[*last_transition_idx].reset();
                match self.path.front() {
                    Some(val) => {
                        self.current_edge_idx = Some(*val);
                        self.current_node_idx = None;
                        self.path.pop_front();
                        // first_time_on_transition = true;
                    }
                    None => {
                        // The path is empty. We now set the current node/edge as the "to" node of the current edge
                        let t = self
                            .graph
                            .edge_endpoints(self.current_edge_idx.unwrap())
                            .unwrap()
                            .1;
                        if t != self.target {
                            return Err(anyhow! {"Path ended on a non-target node"});
                        }
                        self.current_node_idx = Some(t);
                        self.current_edge_idx = None;
                        // first_time_on_node = true;
                    }
                }
            }
        } else {
            // If we are on a node. Far simpler to evaluate
            match self.current_node_idx {
                Some(val) => {
                    let node = self.graph.node_weight(val).unwrap();
                    match node {
                        GenericNode::Sampler(val) => {
                            // println!("DT: {:?}", dt);
                            self.animations[self.samplers[*val].index].seek(self.duration.as_secs_f32());
                          // self.output_locals = LocalToLocalResults::new();
                            self.output_locals =
                                self.animations[self.samplers[*val].index].playback();
                        }
                        GenericNode::BlendTreeOneDim(_) => {
                            // TODO
                        }
                    }
                }
                None => return Err(anyhow! {"Invalid current node during evaluation."}),
            }
        }

       // println!("{:?}, {:?}. {:?}", &self.output_locals.translations, &self.output_locals.rotations, &self.output_locals.scales );
        self.output_model = local_to_model(self.skeleton.clone(), &self.output_locals);

        // Now we check whether or not we are in need of transitioning through the path list
        // let mut first_time_on_transition = false;
        // let mut first_time_on_node = false;


        Ok(())
    }

    pub fn get_node_by_name(&mut self, node_name: String) -> Option<NodeIndex> {
        if self.node_names.contains_key(&node_name) {
            let idx = self.node_names[&node_name];
            return Some(idx);
        }
        None
    }

    // TODO
    pub fn set_target_node_by_idx(&mut self, _node_idx: NodeIndex) {
        //self.dfs(node_idx);
    }

    pub fn set_target_node_by_name(&mut self, node_name: String) {
        if self.node_names.contains_key(&node_name) {
            let _node_idx = self.node_names[&node_name];
            //self.dfs(node_idx);
        }
    }

    pub fn reset(&mut self) {
        self.current_node_idx = Some(self.root_node_idx);
        self.current_edge_idx = None;
        self.path.clear();
    }
}
