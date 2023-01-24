
use::hc_zome_coordinator_type_desc_api::{HolonDescriptorBuilderImpl, DescriptorBuilderStubsFactory};


impl DescriptorBuilder for HolonDescriptorBuilderImpl {
    fn with_type_name(&self,type_name:String)->Box<dyn DescriptorBuilder>{
       Box::new(HolonDescriptorBuilderImpl {
             type_name: Some(type_name),
             description: self.description.clone(),
         })
     }
     fn with_description(&self,description:String)->Box<dyn DescriptorBuilder>{
        Box::new(HolonDescriptorBuilderImpl {
             type_name: self.type_name.clone(),
             description: Some(description),
         })
     }
 }

#[derive(Debug, Clone, Default, new, Eq, PartialEq)]
pub struct HolonDescriptorBuilderImpl {
    type_name: Option<String>,
    description: Option<String>,
}

#[derive(Debug, Clone, new, Serialize, Deserialize, SerializedBytes)]
pub struct DescriptorBuilderFactoryImpl {
    // data that influences how factories are created
}

impl DescriptorBuilderFactory for DescriptorBuilderFactoryImpl {
    fn make_holon_descriptor(&self)->Box<HolonDescriptor> {
        Box::new(HolonDescriptorBuilderStubsImpl::new())
    }
}

// impl Debug for dyn DescriptorBuilder {
//     fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
//         write!(f, "DescriptorBuilder: {:?}", self)
//     }
// }