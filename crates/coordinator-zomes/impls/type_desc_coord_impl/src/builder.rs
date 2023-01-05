
use::hc_zome_coordinator_type_desc_api::HolonDescriptorBuilderImpl;


impl HolonDescriptorBuilder for HolonDescriptorBuilderImpl {
    fn with_type_name(&self, type_name:String)-> Self {   
        self.header.type_name = type_name;
        self
    }
    fn with_description(&self, description:String)-> Self {
        self.header.description = description;
        self
    }
    // fn with_is_dependent(&self, is_dependent:bool)-> Self {

    // }
    fn create() -> HolonDescriptorBuilderImpl {
        // TODO: Factory
    }
}