This utility is to run some basic code copies/transforms.
The reason this crate exists is because our ECS requires all components to be defined in the same crate.
All this does at present is copy a file called "components_and_entities_common.rs" from the shared crate to the others.