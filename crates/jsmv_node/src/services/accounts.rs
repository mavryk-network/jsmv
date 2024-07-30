use std::collections::HashMap;

use actix_web::{
    get,
    web::{Data, Path, Query, ServiceConfig},
    HttpResponse, Responder, Scope,
};
use anyhow::anyhow;
use jsmv_api::KvValue;
use jsmv_proto::context::account::Account;
use mavkit::MavkitRollupClient;

use crate::Result;

use super::Service;

fn construct_storage_key(address: &str, key: &Option<String>) -> String {
    match key {
        Some(value) if !value.is_empty() => format!("/jsmv_kv/{}/{}", address, value),
        _ => format!("/jsmv_kv/{}", address),
    }
}

#[get("/{address}/nonce")]
async fn nonce(
    rollup_client: Data<MavkitRollupClient>,
    path: Path<String>,
) -> Result<impl Responder> {
    let key = format!("/jsmv_account/{}", path.into_inner());

    let value = rollup_client.get_value(&key).await?;

    let nonce = match value {
        Some(value) => {
            bincode::deserialize::<Account>(&value)
                .map_err(|_| anyhow!("Failed to deserialize account"))?
                .nonce
        }
        None => return Ok(HttpResponse::NotFound().finish()),
    };

    Ok(HttpResponse::Ok().json(nonce))
}

#[get("/{address}/code")]
async fn code(
    rollup_client: Data<MavkitRollupClient>,
    path: Path<String>,
) -> Result<impl Responder> {
    let key = format!("/jsmv_account/{}", path.into_inner());

    let value = rollup_client.get_value(&key).await?;

    let code = match value {
        Some(value) => {
            bincode::deserialize::<Account>(&value)
                .map_err(|_| anyhow!("Failed to deserialize account"))?
                .contract_code
        }
        None => return Ok(HttpResponse::NotFound().finish()),
    };

    Ok(HttpResponse::Ok().json(code))
}

#[get("/{address}/balance")]
async fn balance(
    rollup_client: Data<MavkitRollupClient>,
    path: Path<String>,
) -> Result<impl Responder> {
    let key = format!("/jsmv_account/{}", path.into_inner());

    let value = rollup_client.get_value(&key).await?;

    let balance = match value {
        Some(value) => {
            bincode::deserialize::<Account>(&value)
                .map_err(|_| anyhow!("Failed to deserialize account"))?
                .amount
        }
        None => return Ok(HttpResponse::NotFound().finish()),
    };

    Ok(HttpResponse::Ok().json(balance))
}

#[get("/{address}/kv")]
async fn kv(
    rollup_client: Data<MavkitRollupClient>,
    path: Path<String>,
    query: Query<HashMap<String, String>>,
) -> Result<impl Responder> {
    let address = path.into_inner();
    let key_option = query.get("key").cloned();

    let storage_key = construct_storage_key(&address, &key_option);

    let value = rollup_client.get_value(&storage_key).await?;

    let value = match value {
        Some(value) => bincode::deserialize::<KvValue>(&value)
            .map_err(|_| anyhow!("Failed to deserialize account"))?,
        None => return Ok(HttpResponse::NotFound().finish()),
    };

    Ok(HttpResponse::Ok().json(value))
}

#[get("/{address}/kv/subkeys")]
async fn kv_subkeys(
    rollup_client: Data<MavkitRollupClient>,
    path: Path<String>,
    query: Query<HashMap<String, String>>,
) -> Result<impl Responder> {
    let address = path.into_inner();

    let key_option = query.get("key").cloned();

    let storage_key = construct_storage_key(&address, &key_option);

    let value = rollup_client.get_subkeys(&storage_key).await?;

    let value = match value {
        Some(value) => value,
        None => return Ok(HttpResponse::NotFound().finish()),
    };

    Ok(HttpResponse::Ok().json(value))
}

pub struct AccountsService;

impl Service for AccountsService {
    fn configure(cfg: &mut ServiceConfig) {
        let scope = Scope::new("/accounts")
            .service(nonce)
            .service(code)
            .service(balance)
            .service(kv)
            .service(kv_subkeys);

        cfg.service(scope);
    }
}