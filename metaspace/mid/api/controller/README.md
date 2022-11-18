# Metaspace Mid-Tier API Controllers

This tier offers API declarations (traits) for Metaspace controllers exposed via the WASM API. 

* **_Command Trait_** -- implemented by various Command objects
  * fn execute
  * fn undo
  * fn redo

* **_ActionController Trait_** -- implemented by the ActionController
    * fn execute
    * fn undo
    * fn redo