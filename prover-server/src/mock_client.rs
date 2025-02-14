use clap::Parser;
use jsonrpsee::http_client::{HttpClient, HttpClientBuilder};
use jsonrpsee_core::client::ClientT;
use jsonrpsee_core::rpc_params;
use prover_server::prove::ProofResult;
use prover_server::spec::ZkSpec;
use prover_server::utils::kroma_info;
use std::fs;
use std::time::Duration;
use types::eth::BlockTrace;

const CLIENT_TIMEOUT_SEC: u64 = 10800;
const DEFAULT_RPC_SERVER_ENDPOINT: &str = "http://127.0.0.1:3030";

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long = "prove")]
    prove: Option<bool>,

    #[clap(short, long = "spec")]
    spec: Option<bool>,
}

async fn test_request_proof(cli: HttpClient) -> bool {
    let trace_str =
        fs::read_to_string("zkevm/tests/traces/kroma/multiple_transfers_0.json").unwrap();
    let trace: BlockTrace = serde_json::from_str(&trace_str).unwrap();

    kroma_info(format!(
        "Send 'prove' request with height({})",
        trace.header.number.unwrap()
    ));

    let params = rpc_params![trace_str];
    let proof_result: ProofResult = cli.request("prove", params).await.unwrap();

    kroma_info(format!(
        "Got:\n - final_pair: {:?}\n - proof: {:?}",
        proof_result.final_pair, proof_result.proof
    ));

    true
}

async fn test_request_spec(cli: HttpClient) -> bool {
    kroma_info("Send 'spec' request to prover-server");
    let params = rpc_params![];
    let zk_spec: ZkSpec = cli.request("spec", params).await.unwrap();

    kroma_info(format!(
        "Got: \
        \n - agg_degree: {}\
        \n - degree: {}\
        \n - chain_id: {}\
        \n - max_txs: {}\
        \n - max_call_data: {}",
        zk_spec.agg_degree,
        zk_spec.degree,
        zk_spec.chain_id,
        zk_spec.max_txs,
        zk_spec.max_call_data
    ));

    true
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    env_logger::init();
    let args = Args::parse();

    let http_client = HttpClientBuilder::default()
        .request_timeout(Duration::from_secs(CLIENT_TIMEOUT_SEC))
        .build(DEFAULT_RPC_SERVER_ENDPOINT)
        .unwrap();

    if args.spec.is_some() {
        let _ = test_request_spec(http_client.clone()).await;
    }
    if args.prove.is_some() {
        let _ = test_request_proof(http_client).await;
    }
}
