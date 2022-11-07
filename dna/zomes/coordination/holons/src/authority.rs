use hdk::hdi::prelude::AgentPubKey;


pub(crate) struct Agent {
    pub pub_key: AgentPubKey,
    //pub meta_data:

}

pub(crate) enum Authority {
    Steward,
    Reader
}