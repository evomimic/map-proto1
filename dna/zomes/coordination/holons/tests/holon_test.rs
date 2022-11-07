#![warn(warnings)]

use std::collections::BTreeMap;
use futures::future;
use hc_zome_coordination_holons::HolonDescriptorBuilder;
use hdk::prelude::*;
use holo_hash::AgentPubKey;
use holo_hash::AnyLinkableHash;
use holo_hash::AnyLinkableHashB64;
use holochain::sweettest::{
  SweetAgents, SweetAppBatch, SweetCell, SweetConductor, SweetConductorBatch, SweetDnaFile,
};

const DNA_FILEPATH: &str = "../../../workdir/dna/map_proto1.dna";


#[tokio::test(flavor = "multi_thread")]
pub async fn test_commmit_typedescriptor() {

  let (conductor, agent, cell) = setup_conductor().await;

  let test_string = "This is a test string";

  let test_builder = HolonDescriptorBuilder::new(test_string); 

  let builder_input = TypeDescriptorBuilderInput {
    metaspace_id: ObjectId::String("12345"),
    branch_id: ObjectId::String("a"),
    descriptor_type: TypeDescriptor::Holon(test_builder)
  };

  let action_hash = conductor
    .call(
      &cell.zome("hc_zome_coordination_holons"),
      "commit_typedescriptor",
      builder_input,
    )
    .await;

  println!(action_hash)
}


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

/// Mock Conductor

async fn setup_conductor() -> (SweetConductor, AgentPubKey, SweetCell) {
  let dna = SweetDnaFile::from_bundle(std::path::Path::new(DNA_FILEPATH))
    .await
    .unwrap();

  let mut conductor = SweetConductor::from_standard_config().await;

  let holo_core_agent = SweetAgents::one(conductor.keystore()).await;
  let app = conductor
    .setup_app_for_agent("app", holo_core_agent.clone(), &[dna.clone()])
    .await
    .unwrap();

  let cell = app.into_cells()[0].clone();

  let agent_hash = holo_core_agent.into_inner();
  let agent = AgentPubKey::from_raw_39(agent_hash).unwrap();

  (conductor, agent, cell)
}