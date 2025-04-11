// This file is part of Noir.

// Copyright (c) Haderech Pte. Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use anyhow::{Context, Result};
use aptos_api_types::PendingTransaction;
use aptos_rest_client::Client as ApiClient;
use aptos_sdk::transaction_builder::TransactionBuilder;
use aptos_types::{
    chain_id::ChainId,
    function_info::FunctionInfo,
    move_utils::MemberId,
    transaction::{
        authenticator::AccountAuthenticator, script::EntryFunction, SignedTransaction,
        TransactionPayload,
    },
};
use move_core_types::account_address::AccountAddress;
use std::{
    borrow::Cow,
    str::FromStr,
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Clone, Debug)]
pub struct AAClient {
    pub api_client: ApiClient,
    auth_func: Cow<'static, str>,
    entry_func: Cow<'static, str>,
    chain_id: u8,
    timeout: u64,
}

impl AAClient {
    pub fn new(
        api_client: ApiClient,
        auth_func: String,
        entry_func: String,
        chain_id: u8,
        timeout: u64,
    ) -> Self {
        Self {
            api_client,
            auth_func: Cow::from(auth_func),
            entry_func: Cow::from(entry_func),
            chain_id,
            timeout,
        }
    }

    pub async fn submit_transaction(
        &self,
        sender: AccountAddress,
        tx: Vec<u8>,
        sequence_number: u64,
        max_gas_amount: u64,
        gas_unit_price: u64,
    ) -> Result<PendingTransaction> {
        let transaction = self.get_aa_transaction(
            tx,
            sender,
            sequence_number,
            max_gas_amount,
            gas_unit_price,
            self.chain_id,
            self.timeout,
        );

        Ok(self
            .api_client
            .submit(&transaction)
            .await
            .context("Failed to submit transaction")?
            .into_inner())
    }

    pub fn get_aa_transaction(
        &self,
        tx: Vec<u8>,
        sender: AccountAddress,
        sequence_number: u64,
        max_gas_amount: u64,
        gas_unit_price: u64,
        chain_id: u8,
        timeout: u64,
    ) -> SignedTransaction {
        let entry_func = MemberId::from_str(&self.entry_func).unwrap();
        let raw_transaction = TransactionBuilder::new(
            TransactionPayload::EntryFunction(EntryFunction::new(
                entry_func.module_id,
                entry_func.member_id,
                vec![],
                vec![bcs::to_bytes(&sender).unwrap(), bcs::to_bytes(&tx).unwrap()],
            )),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
                + timeout,
            ChainId::new(chain_id),
        )
        .sender(sender)
        .sequence_number(sequence_number)
        .max_gas_amount(max_gas_amount)
        .gas_unit_price(gas_unit_price)
        .build();

        let authenticator = AccountAuthenticator::abstraction(
            FunctionInfo::from_str(&self.auth_func).unwrap(),
            vec![],
            vec![],
        );

        SignedTransaction::new_single_sender(raw_transaction, authenticator)
    }
}
