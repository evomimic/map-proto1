

use crate::authority::AuthorityLevel;



pub enum Ops {
    Create,
    Update, 
    Delete,
}


// pub enum Entity {
//     Holon, 
//     Descriptor,
//     Map,
// }

pub fn reader(input: ProcessInput) -> ExternResult<OpResult> {
    read(input)
}

pub fn steward(input: ProcessInput, action: Ops) -> ExternResult<OpResult> {
    match action {
        Ops::Create => create(input),
        Ops::Update => update(input),
        Ops::Delete => delete(input),
    }
}


pub fn read(input: ProcessInput) -> ExternResult<OpResult> {
    match input {
        ProcessInput::TypeDescriptor(td) => get(td),
    }
}

pub fn create(input: ProcessInput) -> ExternResult<OpResult> {
    match input {
        // ProcessInput::Holon(h) => ,
        ProcessInput::TypeDescriptor(td) => create_typedescriptor(td),
    }
}

fn get_typedescriptor(action_hash: ActionHash) -> ExternResult<Option<Record>> {
    get(action_hash, GetOptions::default())
}


pub fn create_typedescriptor(builder_input: TypeDescriptorBuilderInput) -> ExternResult<impl DescriptorBuilder> {

    
}

///create
// let typedescriptor_hash = create_entry(
//     &EntryTypes::TypeDescriptor(typedescriptor.clone()),
// )?;
// let record = get(typedescriptor_hash.clone(), GetOptions::default())?
//     .ok_or(
//         wasm_error!(
//             WasmErrorInner::Guest(String::from("Could not find the newly created Typedescriptor"))
//         ),
//     )?;
// Ok(record)


// #[derive(Serialize, Deserialize, Debug)]
// pub struct UpdateTypedescriptorInput {
//     original_action_hash: ActionHash,
//     updated_typedescriptor: TypeDescriptor,
// }

// pub fn update_typedescriptor(input: UpdateTypedescriptorInput) -> ExternResult<Record> {
//     let updated_typedescriptor_hash = update_entry(
//         input.original_action_hash,
//         &input.updated_typedescriptor,
//     )?;
//     let record = get(updated_typedescriptor_hash.clone(), GetOptions::default())?
//         .ok_or(
//             wasm_error!(
//                 WasmErrorInner::Guest(String::from("Could not find the newly updated Typedescriptor"))
//             ),
//         )?;
//     Ok(record)
// }

// pub fn delete_typedescriptor(action_hash: ActionHash) -> ExternResult<ActionHash> {
//     delete_entry(action_hash)
// }