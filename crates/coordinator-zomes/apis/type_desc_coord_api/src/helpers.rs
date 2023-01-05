// use hdk::prelude::*;
// // use derive_new::new;
// use hc_zome_integrity_type_desc::descriptor::descriptor::HolonDescriptor;



// // pub fn create_hc_entry(input: Holon) -> ExternResult<EntryHash> {
// //   create_entry(string_target.clone())?;
// //   let entry_hash = hash_entry(input)?;
// //   Ok(entry_hash)
// // }


// pub fn get_entry_by_actionhash(action_hash: ActionHash) -> ExternResult<HolonDescriptor> { // TODO: Trait bound return type
//   let record = get_record_by_action(action_hash, GetOptions::default())?;
//   match record.entry() {
//     record::RecordEntry::Present(entry) => {
//       HolonDescriptor::try_from(entry.clone()).or(Err(wasm_error!(WasmErrorInner::Guest(format!(
//         "Couldn't convert Record entry {:?} into data type {}",
//         entry,
//         std::any::type_name::<Holon>()
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



