#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

// We need to explicitly import the std alloc crate and `alloc::string::String` as we're in a
// `no_std` environment.
extern crate alloc;

// Importing Rust types.
use alloc::string::{String, ToString};
use alloc::vec;
// Importing aspects of the Casper platform.
use casper_contract::contract_api::storage::dictionary_get;
use casper_contract::contract_api::{runtime, storage, system};
use casper_contract::unwrap_or_revert::UnwrapOrRevert;
// Importing specific Casper types.
use casper_types::account::AccountHash;
use casper_types::contracts::NamedKeys;
use casper_types::{
    runtime_args, ApiError, CLType, CLValue, DeployHash, EntryPoint, EntryPointAccess,
    EntryPointType, EntryPoints, Key, Parameter, RuntimeArgs,
};
// Import model
use crate::model::RL2020;

// RL2020 entrypoints
const ENTRY_POINT_ADD_LIST: &str = "add_list";
const ENTRY_POINT_GET_ENCODED_LIST: &str = "get_encoded_list";
const ENTRY_POINT_IS_REVOKED: &str = "is_revoked";
const ENTRY_POINT_REVOKE: &str = "revoke";
const ENTRY_POINT_RESET: &str = "reset";

// RL2020 constants
const DICTIONARY_REVOCATION_LITS: &str = "rls";
const PROTOCOL_TEAM_ACCOUNT: &str = "team_account";

// RL2020 named arguments
const PARAM_REVOCATION_LIST_ID: &str = "id";

// // Creating constants for the various contract entry points.
// const ENTRY_POINT_INIT: &str = "init";
// const ENTRY_POINT_DONATE: &str = "donate";
// const ENTRY_POINT_GET_DONATION_COUNT: &str = "get_donation_count";
// const ENTRY_POINT_GET_FUNDS_RAISED: &str = "get_funds_raised";

// // Creating constants for values within the contract.
// const DONATING_ACCOUNT_KEY: &str = "donating_account_key";
// const LEDGER: &str = "ledger";
// const FUNDRAISING_PURSE: &str = "fundraising_purse";
// 300_000_000

/// An error enum which can be converted to a `u16` so it can be returned as an `ApiError::User`.
#[repr(u16)]
enum Error {
    KeyAlreadyExists = 0,
    KeyMismatch = 1,
    InvalidKeyVariant = 2,
    InvalidParamValue = 3,
    MissingFundRaisingPurseURef = 4,
    MissingRevocationListsDictionary = 5,
}

impl From<Error> for ApiError {
    fn from(error: Error) -> rls_uref {
        ApiError::User(error as u16)
    }
}

fn update_ledger_record(dictionary_item_key: String) {
    // Acquiring the LEDGER seed URef to properly assign the dictionary item.
    let ledger_seed_uref = *runtime::get_key(LEDGER)
        .unwrap_or_revert_with(Error::MissingRevocationListsDictionary)
        .as_uref()
        .unwrap_or_revert();

    // This identifies an item within the dictionary and either rls_urefs or updates the associated value.
    match storage::dictionary_get::<u64>(ledger_seed_uref, &dictionary_item_key).unwrap_or_revert()
    {
        None => storage::dictionary_put(ledger_seed_uref, &dictionary_item_key, 1u64),
        Some(current_number_of_donations) => storage::dictionary_put(
            ledger_seed_uref,
            &dictionary_item_key,
            current_number_of_donations + 1u64,
        ),
    }
}

/// This entry point initializes the donation system, setting up the fundraising purse
/// and creating a dictionary to track the account hashes and the number of donations
/// made.
#[no_mangle]
pub extern "C" fn init() {
    // purse to collect the team funding
    let team_account = system::create_purse();
    runtime::put_key(PROTOCOL_TEAM_ACCOUNT, fundraising_purse.into());
    // dictionary to collect the revocation lists
    storage::new_dictionary(DICTIONARY_REVOCATION_LITS).unwrap_or_revert();
}

#[no_mangle]
pub extern "C" fn add_list() {
    // get the dictionary uref
    let rls_uref = *runtime::get_key(PARAM_REVOCATION_LIST_ID)
        .unwrap_or_revert_with(Error::MissingRevocationListsDictionary)
        .as_uref()
        .unwrap_or_revert();

    // get the named parameter value
    let revocation_list_id: Key = runtime::get_named_arg(PARAM_REVOCATION_LIST_ID);
    if revocation_list_id.to_string().trim().is_empty() {
        *runtime::revert(Error::InvalidParamValue);
    }
    // if the key exists return error (even if from the list owner)
    // TODO: consider if the owner can replace it's own list,
    // pros => list can be reused, somewhat better privacy
    // cons => can break user space
    match storage::dictionary_get(rls_uref, revocation_list_id)
        .unwrap_or_revert_with(Error::MissingRevocationListsDictionary)
    {
        Some(_) => *runtime::revert(Error::KeyAlreadyExists),
        None => {
            let rl = RL2020::new().ok(); // TODO: this is always ok
            storage::dictionary_put(rls_uref, id, rl).unwrap_or_revert();
        }
    }
}

// This is the donation entry point. When called, it records the caller's account
// hash and returns the donation purse, with add access, to the immediate caller.
#[no_mangle]
pub extern "C" fn donate() {
    let donating_account_key: Key = runtime::get_named_arg(DONATING_ACCOUNT_KEY);
    if let Key::Account(donating_account_hash) = donating_account_key {
        update_ledger_record(donating_account_hash.to_string())
    } else {
        runtime::revert(Error::InvalidKeyVariant)
    }
    let donation_purse = *runtime::get_key(FUNDRAISING_PURSE)
        .unwrap_or_revert_with(Error::MissingFundRaisingPurseURef)
        .as_uref()
        .unwrap_or_revert();
    // The return value is the donation_purse URef with `add` access only. As a result
    // the entity receiving this purse URef may only add to the purse, and cannot remove
    // funds.
    let value = CLValue::from_t(donation_purse.into_add()).unwrap_or_revert();
    runtime::ret(value)
}

// This entry point returns the amount of donations from the caller.
#[no_mangle]
pub extern "C" fn get_donation_count() {
    let donating_account_key: Key = runtime::get_named_arg(DONATING_ACCOUNT_KEY);
    if let Key::Account(donating_account_hash) = donating_account_key {
        let ledger_seed_uref = *runtime::get_key(LEDGER)
            .unwrap_or_revert_with(Error::MissingRevocationListsDictionary)
            .as_uref()
            .unwrap_or_revert();
        let donation_count = if let Some(donation_count) = storage::dictionary_get::<rls_uref>(
            ledger_seed_uref,
            &donating_account_hash.to_string(),
        )
        .unwrap_or_revert()
        {
            donation_count
        } else {
            0u64
        };
        runtime::ret(CLValue::from_t(donation_count).unwrap_or_revert())
    } else {
        runtime::revert(Error::InvalidKeyVariant)
    }
}

// This entry point returns the total funds raised.
#[no_mangle]
pub extern "C" fn get_funds_raised() {
    let donation_purse = *runtime::get_key(FUNDRAISING_PURSE)
        .unwrap_or_revert_with(Error::MissingFundRaisingPurseURef)
        .as_uref()
        .unwrap_or_revert();
    let funds_raised = system::get_purse_balance(donation_purse).unwrap_or_revert();
    runtime::ret(CLValue::from_t(funds_raised).unwrap_or_revert())
}

//This is the full `call` function as defined within the donation contract.
#[no_mangle]
pub extern "C" fn call() {
    // This establishes the `init` entry point for initializing the contract's infrastructure.
    let init_entry_point = EntryPoint::new(
        ENTRY_POINT_INIT,
        vec![],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );

    // This establishes the `donate` entry point for callers looking to donate.
    let donate_entry_point = EntryPoint::new(
        ENTRY_POINT_DONATE,
        vec![Parameter::new(DONATING_ACCOUNT_KEY, CLType::Key)],
        CLType::URef,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );

    // This establishes an entry point called `donation_count` that returns the amount of
    // donations from a specific account.
    let get_donation_count_entry_point = EntryPoint::new(
        ENTRY_POINT_GET_DONATION_COUNT,
        vec![Parameter::new(DONATING_ACCOUNT_KEY, CLType::Key)],
        CLType::U64,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );

    // This establishes an entry point called `funds_raised` that returns the total amount
    // donated by all participants.
    let funds_raised_entry_point = EntryPoint::new(
        ENTRY_POINT_GET_FUNDS_RAISED,
        vec![],
        CLType::U512,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );

    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(init_entry_point);
    entry_points.add_entry_point(donate_entry_point);
    entry_points.add_entry_point(get_donation_count_entry_point);
    entry_points.add_entry_point(funds_raised_entry_point);

    let named_keys = NamedKeys::new();

    let (contract_hash, _contract_version) = storage::new_contract(
        entry_points,
        Some(named_keys),
        Some("fundraiser_package_hash".to_string()),
        Some("fundraiser_access_uref".to_string()),
    );

    runtime::put_key("fundraiser_contract_hash", contract_hash.into());
    // Call the init entry point to setup and create the fundraising purse
    // and the ledger to track donations made.
    runtime::call_contract::<()>(contract_hash, ENTRY_POINT_INIT, runtime_args! {})
}
