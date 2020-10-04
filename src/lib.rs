use hdk3::prelude::*;
mod transactor;
mod utils;

pub fn error<T>(reason: &str) -> ExternResult<T> {
    Err(HdkError::Wasm(WasmError::Zome(String::from(reason))))
}

entry_defs![
    Path::entry_def(),
    transactor::Request::entry_def()
];

/** credit requests **/

#[hdk_extern]
pub fn create_request(
    request_input: transactor::CreateRequestInput,
) -> ExternResult<EntryHash> {
    transactor::create_request(request_input)
}

#[derive(Clone, Serialize, Deserialize, SerializedBytes)]
pub struct GetMyRequestsOutput(Vec<(EntryHash, transactor::Request)>);
#[hdk_extern]
pub fn get_my_requests(_: ()) -> ExternResult<GetMyRequestsOutput> {
    let requests = transactor::get_my_requests()?;

    Ok(GetMyRequestsOutput(requests))
}
