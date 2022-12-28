// Requests provide context around an Action
// We could have Requests take a vector of Actions (assuming sequential execution)
// But is is simpler and more expressive to allow Action Objects to contain other actions and choreograph their execution
pub trait Request {
    fn action(&self)-> Box<dyn Action>;
    fn handle(&self);
}

pub trait Response {
    fn status(&self)->ResponseStatus;
}

pub enum ResponseStatus{
    New,
    AwaitingResponse,
    Completed,
    TimedOut,
}


pub enum ResultStatus {
    Unavailable,
    Success,
    Error,
}
pub trait Result {
    fn status(&self)->ResultStatus;
    fn errors(&self)->Vec<CommandError>;
    fn next_actions(&self)->Vec<Box<dyn Request>>;

}

// An action may either be a query or a command
// the Action trait can be used to support the Decorator pattern
// e.g., a Validator and an Authorizer (both of which implement Action trait) could be inserted in front
// the struct that actually implements the Action
pub trait Action {
    fn execute(&self);
}

// Commands are, possibly undoable, actions that modify state (i.e., mutators)
pub trait Command : Action {
    fn undo(&self)->Box<dyn Result>; // add a default implementation here that simply returns Error result saying this command is not undoable
    fn redo(&self)->Box<dyn Result>; // add a default implementation here that simply returns Error result saying this command is not redoable
}

pub struct CommandError {
    code: String,
    message: String,
}
