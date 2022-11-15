use hdk::prelude::*;
use derive_new::new;

// #[derive(new, Default, Debug)]
// pub struct TestBuild {
//     #[new(value = "test_build")]
//     test_build: String,
// }

pub fn create_test_entry(input: TestEntry) -> ExternResult<EntryHash> {
  create_entry(string_target.clone())?;
  let entry_hash = hash_entry(input)?;
  Ok(entry_hash)
}

pub fn create_test_descriptor(input: TestDescriptor) -> ExternResult<ActionHash> {
    create_entry(input)
}


// pub fn get_entry_by_actionhash(action_hash: ActionHash) -> ExternResult<TestEntry> {
//   let record = get_record_by_action(action_hash, GetOptions::default())?;
//   match record.entry() {
//     record::RecordEntry::Present(entry) => {
//       TestEntry::try_from(entry.clone()).or(Err(wasm_error!(WasmErrorInner::Guest(format!(
//         "Couldn't convert Record entry {:?} into data type {}",
//         entry,
//         std::any::type_name::<TestEntry>()
//       )))))
//     },
//     _ => Err(wasm_error!(WasmErrorInner::Guest(format!(
//       "Record {:?} does not have an entry",
//       record
//     )))),
//   }
// }

// fn get_record_by_actionhash(
//   action_hash: ActionHash,
//   get_options: GetOptions,
// ) -> ExternResult<Record> {
//   match get(action_hash.clone(), get_options)? {
//     Some(record) => Ok(record),
//     None => Err(wasm_error!(WasmErrorInner::Guest(format!(
//       "There is no record at the hash {}",
//       action_hash
//     )))),
//   }
// }