// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::config::BridgeNodeConfig;
use crate::config::EthConfig;
use crate::config::SuiConfig;
use crate::crypto::BridgeAuthorityKeyPair;
use crate::crypto::BridgeAuthorityPublicKeyBytes;
use crate::server::APPLICATION_JSON;
use crate::types::{AddTokensOnSuiAction, BridgeAction};
use anyhow::anyhow;
use ethers::core::k256::ecdsa::SigningKey;
use ethers::middleware::SignerMiddleware;
use ethers::prelude::*;
use ethers::providers::{Http, Provider};
use ethers::signers::Wallet;
use fastcrypto::ed25519::Ed25519KeyPair;
use fastcrypto::secp256k1::Secp256k1KeyPair;
use fastcrypto::traits::EncodeDecodeBase64;
use futures::future::join_all;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;
use sui_config::Config;
use sui_json_rpc_types::SuiExecutionStatus;
use sui_json_rpc_types::SuiTransactionBlockEffectsAPI;
use sui_json_rpc_types::SuiTransactionBlockResponseOptions;
use sui_sdk::wallet_context::WalletContext;
use sui_test_transaction_builder::TestTransactionBuilder;
use sui_types::base_types::SuiAddress;
use sui_types::bridge::BridgeChainId;
use sui_types::bridge::{BRIDGE_MODULE_NAME, BRIDGE_REGISTER_FOREIGN_TOKEN_FUNCTION_NAME};
use sui_types::crypto::get_key_pair;
use sui_types::crypto::SuiKeyPair;
use sui_types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use sui_types::transaction::{ObjectArg, TransactionData};
use sui_types::BRIDGE_PACKAGE_ID;

pub type EthSigner = SignerMiddleware<Provider<Http>, Wallet<SigningKey>>;

/// Generate Bridge Authority key (Secp256k1KeyPair) and write to a file as base64 encoded `privkey`.
pub fn generate_bridge_authority_key_and_write_to_file(
    path: &PathBuf,
) -> Result<(), anyhow::Error> {
    let (_, kp): (_, BridgeAuthorityKeyPair) = get_key_pair();
    let eth_address = BridgeAuthorityPublicKeyBytes::from(&kp.public).to_eth_address();
    println!(
        "Corresponding Ethereum address by this ecdsa key: {:?}",
        eth_address
    );
    let sui_address = SuiAddress::from(&kp.public);
    println!(
        "Corresponding Sui address by this ecdsa key: {:?}",
        sui_address
    );
    let base64_encoded = kp.encode_base64();
    std::fs::write(path, base64_encoded)
        .map_err(|err| anyhow!("Failed to write encoded key to path: {:?}", err))
}

/// Generate Bridge Client key (Secp256k1KeyPair or Ed25519KeyPair) and write to a file as base64 encoded `flag || privkey`.
pub fn generate_bridge_client_key_and_write_to_file(
    path: &PathBuf,
    use_ecdsa: bool,
) -> Result<(), anyhow::Error> {
    let kp = if use_ecdsa {
        let (_, kp): (_, Secp256k1KeyPair) = get_key_pair();
        let eth_address = BridgeAuthorityPublicKeyBytes::from(&kp.public).to_eth_address();
        println!(
            "Corresponding Ethereum address by this ecdsa key: {:?}",
            eth_address
        );
        SuiKeyPair::from(kp)
    } else {
        let (_, kp): (_, Ed25519KeyPair) = get_key_pair();
        SuiKeyPair::from(kp)
    };
    let sui_address = SuiAddress::from(&kp.public());
    println!("Corresponding Sui address by this key: {:?}", sui_address);

    let contents = kp.encode_base64();
    std::fs::write(path, contents)
        .map_err(|err| anyhow!("Failed to write encoded key to path: {:?}", err))
}

/// Generate Bridge Node Config template and write to a file.
pub fn generate_bridge_node_config_and_write_to_file(
    path: &PathBuf,
    run_client: bool,
) -> Result<(), anyhow::Error> {
    let mut config = BridgeNodeConfig {
        server_listen_port: 9191,
        metrics_port: 9184,
        bridge_authority_key_path: PathBuf::from("/path/to/your/bridge_authority_key"),
        sui: SuiConfig {
            sui_rpc_url: "your_sui_rpc_url".to_string(),
            sui_bridge_chain_id: BridgeChainId::SuiTestnet as u8,
            bridge_client_key_path: None,
            bridge_client_gas_object: None,
            sui_bridge_module_last_processed_event_id_override: None,
        },
        eth: EthConfig {
            eth_rpc_url: "your_eth_rpc_url".to_string(),
            eth_bridge_proxy_address: "0x0000000000000000000000000000000000000000".to_string(),
            eth_bridge_chain_id: BridgeChainId::EthSepolia as u8,
            eth_contracts_start_block_fallback: Some(0),
            eth_contracts_start_block_override: None,
        },
        approved_governance_actions: vec![],
        run_client,
        db_path: None,
    };
    if run_client {
        config.sui.bridge_client_key_path = Some(PathBuf::from("/path/to/your/bridge_client_key"));
        config.db_path = Some(PathBuf::from("/path/to/your/client_db"));
    }
    config.save(path)
}

pub async fn get_eth_signer_client(url: &str, private_key_hex: &str) -> anyhow::Result<EthSigner> {
    let provider = Provider::<Http>::try_from(url)
        .unwrap()
        .interval(std::time::Duration::from_millis(2000));
    let chain_id = provider.get_chainid().await?;
    let wallet = Wallet::from_str(private_key_hex)
        .unwrap()
        .with_chain_id(chain_id.as_u64());
    Ok(SignerMiddleware::new(provider, wallet))
}

pub async fn publish_and_register_coins_return_add_coins_on_sui_action(
    wallet_context: &WalletContext,
    bridge_arg: ObjectArg,
    token_packages_dir: Vec<PathBuf>,
    token_ids: Vec<u8>,
    token_prices: Vec<u64>,
    nonce: u64,
) -> BridgeAction {
    assert!(token_ids.len() == token_packages_dir.len());
    assert!(token_prices.len() == token_packages_dir.len());
    let sui_client = wallet_context.get_client().await.unwrap();
    let quorum_driver_api = Arc::new(sui_client.quorum_driver_api().clone());
    let rgp = sui_client
        .governance_api()
        .get_reference_gas_price()
        .await
        .unwrap();

    let senders = wallet_context.get_addresses();
    // We want each sender to deal with one coin
    assert!(senders.len() >= token_packages_dir.len());

    // publish coin packages
    let mut publish_tokens_tasks = vec![];

    for (token_package_dir, sender) in token_packages_dir.iter().zip(senders.clone()) {
        let gas = wallet_context
            .get_one_gas_object_owned_by_address(sender)
            .await
            .unwrap()
            .unwrap();
        let tx = TestTransactionBuilder::new(sender, gas, rgp)
            .publish(token_package_dir.to_path_buf())
            .build();
        let tx = wallet_context.sign_transaction(&tx);
        let api_clone = quorum_driver_api.clone();
        publish_tokens_tasks.push(tokio::spawn(async move {
            api_clone.execute_transaction_block(
                tx,
                SuiTransactionBlockResponseOptions::new()
                    .with_effects()
                    .with_input()
                    .with_events()
                    .with_object_changes()
                    .with_balance_changes(),
                Some(sui_types::quorum_driver_types::ExecuteTransactionRequestType::WaitForLocalExecution),
            ).await
        }));
    }
    let publish_coin_responses = join_all(publish_tokens_tasks).await;

    let mut token_type_names = vec![];
    let mut register_tasks = vec![];
    for (response, sender) in publish_coin_responses.into_iter().zip(senders.clone()) {
        let response = response.unwrap().unwrap();
        assert_eq!(
            response.effects.unwrap().status(),
            &SuiExecutionStatus::Success
        );
        let object_changes = response.object_changes.unwrap();
        let mut tc = None;
        let mut type_ = None;
        let mut uc = None;
        let mut metadata = None;
        for object_change in &object_changes {
            if let o @ sui_json_rpc_types::ObjectChange::Created { object_type, .. } = object_change
            {
                if object_type.name.as_str().starts_with("TreasuryCap") {
                    assert!(tc.is_none() && type_.is_none());
                    tc = Some(o.clone());
                    type_ = Some(object_type.type_params.first().unwrap().clone());
                } else if object_type.name.as_str().starts_with("UpgradeCap") {
                    assert!(uc.is_none());
                    uc = Some(o.clone());
                } else if object_type.name.as_str().starts_with("CoinMetadata") {
                    assert!(metadata.is_none());
                    metadata = Some(o.clone());
                }
            }
        }
        let (tc, type_, uc, metadata) =
            (tc.unwrap(), type_.unwrap(), uc.unwrap(), metadata.unwrap());

        // register with the bridge
        let mut builder = ProgrammableTransactionBuilder::new();
        let bridge_arg = builder.obj(bridge_arg).unwrap();
        let uc_arg = builder
            .obj(ObjectArg::ImmOrOwnedObject(uc.object_ref()))
            .unwrap();
        let tc_arg = builder
            .obj(ObjectArg::ImmOrOwnedObject(tc.object_ref()))
            .unwrap();
        let metadata_arg = builder
            .obj(ObjectArg::ImmOrOwnedObject(metadata.object_ref()))
            .unwrap();
        builder.programmable_move_call(
            BRIDGE_PACKAGE_ID,
            BRIDGE_MODULE_NAME.into(),
            BRIDGE_REGISTER_FOREIGN_TOKEN_FUNCTION_NAME.into(),
            vec![type_.clone()],
            vec![bridge_arg, tc_arg, uc_arg, metadata_arg],
        );
        let pt = builder.finish();
        let gas = wallet_context
            .get_one_gas_object_owned_by_address(sender)
            .await
            .unwrap()
            .unwrap();
        let tx = TransactionData::new_programmable(sender, vec![gas], pt, 1_000_000_000, rgp);
        let signed_tx = wallet_context.sign_transaction(&tx);
        let api_clone = quorum_driver_api.clone();
        register_tasks.push(async move {
            api_clone
                .execute_transaction_block(
                    signed_tx,
                    SuiTransactionBlockResponseOptions::new().with_effects(),
                    None,
                )
                .await
        });
        token_type_names.push(type_);
    }
    for response in join_all(register_tasks).await {
        assert_eq!(
            response.unwrap().effects.unwrap().status(),
            &SuiExecutionStatus::Success
        );
    }

    BridgeAction::AddTokensOnSuiAction(AddTokensOnSuiAction {
        nonce,
        chain_id: BridgeChainId::SuiCustom,
        native: false,
        token_ids,
        token_type_names,
        token_prices,
    })
}

pub async fn wait_for_server_to_be_up(server_url: String, timeout_sec: u64) -> anyhow::Result<()> {
    let now = std::time::Instant::now();
    loop {
        if let Ok(true) = reqwest::Client::new()
            .get(server_url.clone())
            .header(reqwest::header::ACCEPT, APPLICATION_JSON)
            .send()
            .await
            .map(|res| res.status().is_success())
        {
            break;
        }
        if now.elapsed().as_secs() > timeout_sec {
            anyhow::bail!("Server is not up and running after {} seconds", timeout_sec);
        }
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }
    Ok(())
}
