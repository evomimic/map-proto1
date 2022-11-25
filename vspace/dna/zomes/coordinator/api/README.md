# VSpace Coordinator API Definitions

This tier offers API declarations (traits) for Vspace controllers exposed via the WASM API. 

* **_Command Trait_** -- implemented by various Command objects
  * fn execute
  * fn undo
  * fn redo

* **_ActionController Trait_** -- implemented by the ActionController
    * fn execute
    * fn undo
    * fn redo
