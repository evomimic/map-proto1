use hc_zome_integrity_metaspace_exports::TestHolonDescriptor;
use hdk::prelude::*;

pub(crate) mod authority;
pub(crate) use authority::Authority;
pub mod test_helpers;
pub use test_helpers::*;
pub use test_helpers::{TestBuild};
mod crud;
use crud::*;
use crud::{Ops};
pub mod builder;
pub use builder::*;
pub use builder::{TypeDescriptorBuilder};


pub trait MetaSpace {
    fn create_branch(branch: Branch) -> ExternResult<()>;
    fn get_branch(id: String) -> ExternResult<Branch>;
    fn get_all_branches() -> ExternResult<Vec<Branch>>;
}

pub trait DescriptorBuilder {

    fn build(&self) -> ExternResult<Self>;

    fn create(input:TypeDescriptorBuilderInput) -> ExternResult<TypeDescriptorBuilder>;

    // fn deep_copy(metaspace_id: ObjectId, branch_id: ObjectId, descriptor_type: TypeDescriptor) -> impl DescriptorBuilder;

    // fn derive(metaspace_id: ObjectId, branch_id: ObjectId, descriptor_type: TypeDescriptor) -> impl DescriptorBuilder;

    // fn cancel(&self) -> impl Temporary;
}


impl DescriptorBuilder for Branch {
    fn build(&self) -> Self;
    fn create(input:TypeDescriptorBuilderInput) -> TypeDescriptorBuilder;
}

pub enum OpResult {
    Nothing,
    OpReturnType,
}

pub enum ObjectId {
    Bytes(u8),
    String(String)
}

struct TypeDescriptorBuilderInput {
    metaspace_id: ObjectId,
    branch_id: ObjectId,
    descriptor_type: TypeDescriptor,
}


// struct CrudInput {
//     authority: Authority,
//     input: ProcessInput,
//     op: Ops,
// }



// #[hdk_extern]
// pub fn start_build_typedescriptor(input: TypeDescriptorBuilderInput) -> ExternResult<TypeDescriptorBuilder> {
//     match input.descriptor_type {
//         TypeDescriptor::Holon(_) => HolonDescriptorBuilder::new(),
//         _ => { wasm_error!(
//                 WasmErrorInner::Guest(String::from("Only testing Holon"))
//               ) }
//     }
// }

// #[hdk_extern]
// pub fn continue_build_typedescriptor(input: TypeDescriptorBuilderInput) -> ExternResult<TypeDescriptorBuilder> {
//     match input.descriptor_type {
//         TypeDescriptor::Holon(_) => HolonDescriptorBuilder::new(),
//         _ => { wasm_error!(
//                 WasmErrorInner::Guest(String::from("Only testing Holon"))
//               ) }
//     }
// }

#[hdk_extern]
pub fn commit_typedescriptor(input: TypeDescriptorBuilderInput) -> ExternResult<ActionHash> {
    let descriptor = match input.descriptor_type {
        TypeDescriptor::Holon(_) => { let builder = HolonDescriptorBuilder::default();
                                        TestHolonDescriptor::new(builder)
                                    }
        _ => { wasm_error!(
                WasmErrorInner::Guest(String::from("Only testing Holon"))
              ) }
    }?;
    create_entry(descriptor)
}


/// TEST HELPERS

#[hdk_extern]
pub fn create_test_entry(input: TestEntry) -> ExternResult<ActionHash> {
    test_helpers::create_test_entry(input)
}






////


// #[hdk_extern]
// pub fn crud(input: CrudInput) -> ExternResult<OpResult> {

//     let result = match input.authority {
//         Authority::Reader => crud::reader(input.input),
//         Authority::Steward => crud::steward(input.input, input.op)
//         }?;
    
//     Ok(result)
// }