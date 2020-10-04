use crate::utils;
use hdk3::sys_time;
use hdk3::prelude::timestamp::Timestamp;
use hdk3::prelude::*;

#[derive(Clone, SerializedBytes, Serialize, Deserialize)]
pub enum RequestStatus {
    Pending,
    Canceled,
    Completed // If it's already accepted
}

#[hdk_entry(id = "request", visibility = "public")]
#[serde(rename_all = "camelCase")]
#[derive(Clone)]
pub struct Request {
    status: RequestStatus,
    debtor_pub_key: AgentPubKey,
    creditor_pub_key: AgentPubKey,
    amount: f64,
    timestamp: Timestamp
}

#[derive(Clone, Serialize, Deserialize, SerializedBytes)]
#[serde(rename_all = "camelCase")]
pub struct CreateRequestInput {
    pub amount: f64,
    pub receiver: AgentPubKey
}

/**
 * Creates a new request, linking from the debtor to the creditor public key
 */
pub fn create_request(
    request_data:CreateRequestInput
) -> ExternResult<EntryHash> {
    let agent_info = agent_info!()?;
    let now = sys_time!()?;
    let ts:Timestamp = Timestamp(now.as_secs(),now.as_nanos())
    let request = Request {
        status: RequestStatus::Pending,
        debtor_pub_key: agent_info.agent_latest_pubkey.clone(),
        creditor_pub_key: request_data.receiver,
        amount: request_data.amount,
        timestamp: ts
    };

    create_entry!(request.clone())?;
    let request_hash = hash_entry!(request.clone())?;
    let path = my_requests_path()?;
    path.ensure()?;
    create_link!(path.hash()?, request_hash.clone())?;

    let creditor_path = requests_path_for_agent(receiver.clone());
    creditor_path.ensure()?;
    create_link!(creditor_path.hash()?, request_hash.clone())?;

    Ok(request_hash)
}

/**
 * Returns the requests in which the agent is the debtor or the creditor
 */
pub fn get_my_requests() -> ExternResult<Vec<(EntryHash, Request)>> {
    let path = my_requests_path()?;
    let links = get_links!(path.hash()?)?;

    links
        .into_inner()
        .iter()
        .map(|link| utils::try_get_and_convert::<Request>(link.target.clone()))
        .collect()
}

/** Private helpers **/
fn my_requests_path() -> ExternResult<Path> {
    let agent_info = agent_info!()?;
    Ok(requests_path_for_agent(
        agent_info.agent_latest_pubkey,
    ))
}

fn requests_path_for_agent(public_key: AgentPubKey) -> Path {
    Path::from(format!("requests.{:?}", public_key))
}
