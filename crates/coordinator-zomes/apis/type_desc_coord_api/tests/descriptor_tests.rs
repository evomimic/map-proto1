#![warn(warnings)]

use futures::future;
use std::collections::BTreeMap;

use hdk::prelude::*;
use holochain::sweettest::{
    SweetAgents, SweetAppBatch, SweetCell, SweetConductor, SweetConductorBatch, SweetDnaFile,
};

// const DNA_FILEPATH: &str = "../../../workdir/dna/map_proto1.dna";

// #[tokio::test(flavor = "multi_thread")]
// pub async fn test_create_holondescriptorbuilder() {
//     let (conductor, agent, cell) = setup_conductor().await;

// }

// #[tokio::test(flavor = "multi_thread")]
// pub async fn test_create_holondescriptorbuilder() {
//     let (conductor, agent, cell) = setup_conductor().await;

//     let type_header = TypeHeader {
//       name: "This is a test HolonType",
//       description: "Test description",
//     };

//     let descriptor = HolonDescriptor {
//       header: type_header
//     };

//     let test_builder = HolonDescriptor::new(descriptor);

//     let builder_input = TypeDescriptorBuilderInput {
//       metaspace_id: ObjectId::String("12345"),
//       branch_id: ObjectId::String("a"),
//       descriptor_type: TypeDescriptor::Holon(test_builder)
//     };

//     let holon_descriptor = DescriptorBuilder::create(builder_input).unwrap();
//     println!(holon_descriptor)
// }

// #[tokio::test(flavor = "multi_thread")]
// pub async fn test_commmit_typedescriptor() {

//   let (conductor, agent, cell) = setup_conductor().await;

//   let type_header = TypeHeader {
//     name: "This is a test HolonType",
//     description: "Test description",
//   };

//   let descriptor = HolonDescriptor {
//     header: type_header
//   };

//   let test_builder = HolonDescriptor::new(descriptor);

//   let builder_input = TypeDescriptorBuilderInput {
//     metaspace_id: ObjectId::String("12345"),
//     branch_id: ObjectId::String("a"),
//     descriptor_type: TypeDescriptor::Holon(test_builder)
//   };

//   let action_hash = conductor
//     .call(
//       &cell.zome("hc_zome_coordination_holons"),
//       "commit_typedescriptor",
//       builder_input,
//     )
//     .await;

//   println!(action_hash)
// }

// #[tokio::test(flavor = "multi_thread")]
// pub async fn test_get_entry_by_actionhash() {
//   let (conductor, _agent, cell) = setup_conductor().await;

//   let test_entry = TestEntry {
//     example_field: "test".to_string(),
//   };

//   let action_hash: ActionHash = conductor
//     .call(&cell.zome("hc_zome_coordination_holons"), "create_test_entry", test_entry)
//     .await;

//   let retrieval: TestEntry = conductor
//     .call(
//       &cell.zome("map_proto1"),
//       "get_entry_by_actionhash",
//       action_hash,
//     )
//     .await;
//   assert_eq!("test".to_string(), retrieval.example_field);
// }

// #[tokio::test(flavor = "multi_thread")]
// pub async fn test_get_all_holons() {

//   let (conductor, _agent, cell1): (SweetConductor, AgentPubKey, SweetCell) =
//     setup_conductor().await;

//   let descriptors: Vec<HolonDescriptor> = conductor
//     .call(&cell.zome("hc_zome_coordination_holons"), "get_all_holon_types")
//     .await;

//   println!(descriptors);

//   let action_hash1: ActionHash = conductor
//     .call(&cell.zome("hc_zome_coordination_holons"), "create__hc_entry", descriptors[0])
//     .await;

//   let holon1: HolonDescriptor = conductor
//     .call(
//       &cell.zome("map_proto1"),
//       "get_entry_by_actionhash",
//       action_hash1,
//     )
//     .await;

//   let action_hash2: ActionHash = conductor
//     .call(&cell.zome("hc_zome_coordination_holons"), "create_hc_entry", descriptors[1])
//     .await;

//   let holon2: HolonDescriptor = conductor
//   .call(
//     &cell.zome("map_proto1"),
//     "get_entry_by_actionhash",
//     action_hash2,
//   )
//   .await;

//   let action_hash3: ActionHash = conductor
//   .call(&cell.zome("hc_zome_coordination_holons"), "create_hc_entry", descriptors[2])
//   .await;

//   let holon3: HolonDescriptor = conductor
//     .call(
//       &cell.zome("map_proto1"),
//       "get_entry_by_actionhash",
//       action_hash3,
//     )
//     .await;

//   let holons: Vec<HolonDescriptor> = vec![holon1, holon2, holon3];

//   println!(holons);

//   assert_eq!(holons, descriptors);

// }

// Mock Conductor

// async fn setup_conductor() -> (SweetConductor, AgentPubKey, SweetCell) {
//   let dna = SweetDnaFile::from_bundle(std::path::Path::new(DNA_FILEPATH))
//     .await
//     .unwrap();

//   let mut conductor = SweetConductor::from_standard_config().await;

//   let holo_core_agent = SweetAgents::one(conductor.keystore()).await;
//   let app = conductor
//     .setup_app_for_agent("app", holo_core_agent.clone(), &[dna.clone()])
//     .await
//     .unwrap();

//   let cell = app.into_cells()[0].clone();

//   let agent_hash = holo_core_agent.into_inner();
//   let agent = AgentPubKey::from_raw_39(agent_hash).unwrap();

//   (conductor, agent, cell)
// }
