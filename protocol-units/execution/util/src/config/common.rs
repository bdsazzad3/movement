use std::str::FromStr;

use aptos_crypto::{ed25519::Ed25519PrivateKey, Uniform, ValidCryptoMaterialStringExt};
use aptos_types::chain_id::ChainId;
use godfig::env_default;

// The default Maptos API listen hostname
env_default!(
	default_maptos_rest_listen_hostname,
	"MAPTOS_API_LISTEN_HOSTNAME",
	String,
	"0.0.0.0".to_string()
);

// The default Maptos API listen port
env_default!(default_maptos_rest_listen_port, "MAPTOS_API_LISTEN_PORT", u16, 30731);

// The default Maptos API connection hostname
env_default!(
	default_maptos_rest_connection_hostname,
	"MAPTOS_API_CONNECTION_HOSTNAME",
	String,
	"0.0.0.0".to_string()
);

// The default Maptos API connection port
env_default!(default_maptos_rest_connection_port, "MAPTOS_API_CONNECTION_PORT", u16, 30731);

// The default faucet API listen hostname
env_default!(
	default_maptos_faucet_rest_listen_hostname,
	"FAUCET_API_LISTEN_HOSTNAME",
	String,
	"0.0.0.0".to_string()
);

// The default faucet API listen port
env_default!(default_maptos_faucet_rest_listen_port, "FAUCET_API_LISTEN_PORT", u16, 30732);

// The default faucet API connection hostname
env_default!(
	default_maptos_faucet_rest_connection_hostname,
	"FAUCET_API_CONNECTION_HOSTNAME",
	String,
	"0.0.0.0".to_string()
);

// The default faucet API connection port
env_default!(default_maptos_faucet_rest_connection_port, "FAUCET_API_CONNECTION_PORT", u16, 30732);

// The default fin API listen hostname
env_default!(
	default_fin_rest_listen_hostname,
	"MAPTOS_FIN_VIEW_API_LISTEN_HOSTNAME",
	String,
	"0.0.0.0".to_string()
);

// The default fin API listen port
env_default!(default_fin_rest_listen_port, "MAPTOS_FIN_VIEW_API_LISTEN_PORT", u16, 30733);

// The default fin API connection hostname
env_default!(
	default_fin_rest_connection_hostname,
	"MAPTOS_FIN_VIEW_API_CONNECTION_HOSTNAME",
	String,
	"0.0.0.0".to_string()
);

// The default chain id
env_default!(default_maptos_chain_id, "MAPTOS_CHAIN_ID", ChainId, ChainId::from_str("27").unwrap());

// The default private key
pub fn default_maptos_private_key() -> Ed25519PrivateKey {
	match std::env::var("MAPTOS_PRIVATE_KEY") {
		Ok(val) => Ed25519PrivateKey::from_encoded_string(&val).unwrap(),
		Err(_) => Ed25519PrivateKey::generate(&mut rand::thread_rng()),
	}
}

env_default!(
	default_maptos_indexer_grpc_listen_hostname,
	"MAPTOS_INDEXER_GRPC_LISTEN_HOSTNAME",
	String,
	"0.0.0.0".to_string()
);

env_default!(
	default_maptos_indexer_grpc_listen_port,
	"MAPTOS_INDEXER_GRPC_LISTEN_PORT",
	u16,
	30734
);

env_default!(
	default_maptos_indexer_grpc_connection_hostname,
	"MAPTOS_INDEXER_GRPC_CONNECTION_HOSTNAME",
	String,
	"0.0.0.0".to_string()
);

env_default!(
	default_maptos_indexer_grpc_connection_port,
	"MAPTOS_INDEXER_GRPC_CONNECTION_PORT",
	u16,
	30734
);

env_default!(
	default_maptos_indexer_grpc_inactivity_timeout,
	"MAPTOS_INDEXER_GRPC_INACTIVITY_TIMEOUT_SEC",
	u64,
	60
);

env_default!(
	default_maptos_indexer_grpc_ping_interval,
	"MAPTOS_INDEXER_GRPC_PING_INTERVAL_SEC",
	u64,
	10
);

env_default!(default_maptos_ledger_prune_window, "MAPTOS_LEDGER_PRUNING_WINDOW", u64, 50_000_000);

env_default!(
	default_maptos_state_merkle_prune_window,
	"MAPTOS_STATE_MERKLE_PRUNING_WINDOW",
	u64,
	100_000 // see comment in aptos-core, this essentially needs to exceed TPS
);

env_default!(
	default_maptos_epoch_snapshot_prune_window,
	"MAPTOS_EPOCH_SNAPSHOT_PRUNING_WINDOW",
	u64,
	50_000_000
);

env_default!(
	default_postgres_connection_string,
	"INDEXER_PROCESSOR_POSTGRES_CONNECTION_STRING",
	String,
	"postgresql://postgres:password@localhost:5432".to_string()
);

env_default!(
	default_indexer_processor_auth_token,
	"INDEXER_PROCESSOR_AUTH_TOKEN",
	String,
	"auth_token".to_string()
);

env_default!(default_max_transactions_in_flight, "MAPTOS_MAX_TRANSACTIONS_IN_FLIGHT", u64, 12000);
