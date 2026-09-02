//! Query hooks for TCC Launcher

use std::sync::Mutex;
use freya::query::{
    Mutation, MutationCapability, MutationStateData, QueriesStorage, Query, QueryCapability, UseMutation, UseQuery, use_mutation, use_query,
};
use tcc_auth::{AccountKind, MinecraftAccount};
use tcc_core::LauncherError;
use uuid::Uuid;

static HANDLED_LOGIN_CODE: Mutex<Option<String>> = Mutex::new(None);

pub fn login_code_already_handled(_user_code: &str) -> bool {
    false // No Microsoft login in TCC
}

pub fn reset_login_code_dedup() {
    *HANDLED_LOGIN_CODE.lock().unwrap() = None;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ListAccountsKeys;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct DefaultAccountKeys {
    pub fallback: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AccountKeys {
    pub id: Uuid,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ListAccountsQuery;

impl QueryCapability for ListAccountsQuery {
    type Ok = Vec<MinecraftAccount>;
    type Err = LauncherError;
    type Keys = ListAccountsKeys;

    async fn run(&self, _keys: &Self::Keys) -> Result<Self::Ok, Self::Err> {
        Ok(crate::launcher::state()?.auth.list_accounts().await)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct DefaultAccountQuery;

impl QueryCapability for DefaultAccountQuery {
    type Ok = Option<MinecraftAccount>;
    type Err = LauncherError;
    type Keys = DefaultAccountKeys;

    async fn run(&self, keys: &Self::Keys) -> Result<Self::Ok, Self::Err> {
        let state = crate::launcher::state()?;
        let account = state.auth.default_account().await?;
        if account.is_some() || !keys.fallback {
            return Ok(account);
        }

        let accounts = state.auth.list_accounts().await;
        Ok(accounts.into_iter().next())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AccountQuery;

impl QueryCapability for AccountQuery {
    type Ok = Option<MinecraftAccount>;
    type Err = LauncherError;
    type Keys = AccountKeys;

    async fn run(&self, keys: &Self::Keys) -> Result<Self::Ok, Self::Err> {
        Ok(crate::launcher::state()?.auth.get_account(keys.id).await)
    }
}

pub fn use_accounts() -> UseQuery<ListAccountsQuery> {
    use_query(Query::new(ListAccountsKeys, ListAccountsQuery))
}

pub fn use_default_account(fallback: bool) -> UseQuery<DefaultAccountQuery> {
    use_query(Query::new(
        DefaultAccountKeys { fallback },
        DefaultAccountQuery,
    ))
}

pub fn use_current_account() -> UseQuery<DefaultAccountQuery> {
    use_default_account(true)
}

pub fn use_account(id: Uuid) -> UseQuery<AccountQuery> {
    use_query(Query::new(AccountKeys { id }, AccountQuery))
}

pub fn try_accounts(query: &UseQuery<ListAccountsQuery>) -> Option<Vec<MinecraftAccount>> {
    super::view_state::settled_or_loading(query)
}

pub fn try_default_account(query: &UseQuery<DefaultAccountQuery>) -> Option<MinecraftAccount> {
    super::view_state::settled_or_loading(query).flatten()
}

pub fn try_account(query: &UseQuery<AccountQuery>) -> Option<MinecraftAccount> {
    super::view_state::settled_or_loading(query).flatten()
}

async fn invalidate_auth_queries(account_id: Option<Uuid>) {
    QueriesStorage::<ListAccountsQuery>::invalidate_matching(ListAccountsKeys).await;
    for fallback in [false, true] {
        QueriesStorage::<DefaultAccountQuery>::invalidate_matching(DefaultAccountKeys {
            fallback,
        })
        .await;
    }
    if let Some(id) = account_id {
        QueriesStorage::<AccountQuery>::invalidate_matching(AccountKeys { id }).await;
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AddOfflineAccountMutation;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AddOfflineAccountKeys {
    pub username: String,
}

impl MutationCapability for AddOfflineAccountMutation {
    type Ok = MinecraftAccount;
    type Err = LauncherError;
    type Keys = AddOfflineAccountKeys;

    async fn run(&self, keys: &Self::Keys) -> Result<Self::Ok, Self::Err> {
        Ok(crate::launcher::state()?.auth.add_offline_account(keys.username.clone()).await?)
    }

    async fn on_settled(&self, _keys: &Self::Keys, result: &Result<Self::Ok, Self::Err>) {
        if let Ok(account) = result {
            invalidate_auth_queries(Some(account.id)).await;
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct RemoveAccountMutation;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct RemoveAccountKeys {
    pub id: Uuid,
}

impl MutationCapability for RemoveAccountMutation {
    type Ok = ();
    type Err = LauncherError;
    type Keys = RemoveAccountKeys;

    async fn run(&self, keys: &Self::Keys) -> Result<Self::Ok, Self::Err> {
        Ok(crate::launcher::state()?.auth.remove_account(keys.id).await?)
    }

    async fn on_settled(&self, keys: &Self::Keys, result: &Result<Self::Ok, Self::Err>) {
        if result.is_ok() {
            invalidate_auth_queries(Some(keys.id)).await;
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SetDefaultAccountMutation;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SetDefaultAccountKeys {
    pub id: Option<Uuid>,
}

impl MutationCapability for SetDefaultAccountMutation {
    type Ok = ();
    type Err = LauncherError;
    type Keys = SetDefaultAccountKeys;

    async fn run(&self, keys: &Self::Keys) -> Result<Self::Ok, Self::Err> {
        Ok(crate::launcher::state()?.auth.set_default_account(keys.id).await?)
    }

    async fn on_settled(&self, keys: &Self::Keys, result: &Result<Self::Ok, Self::Err>) {
        if result.is_ok() {
            invalidate_auth_queries(keys.id).await;
        }
    }
}

pub type UseSetDefaultAccount = UseMutation<SetDefaultAccountMutation>;
pub type UseRemoveAccount = UseMutation<RemoveAccountMutation>;

pub fn use_add_offline_account() -> UseMutation<AddOfflineAccountMutation> {
    use_mutation(Mutation::new(AddOfflineAccountMutation))
}

pub fn use_remove_account() -> UseMutation<RemoveAccountMutation> {
    use_mutation(Mutation::new(RemoveAccountMutation))
}

pub fn use_set_default_account() -> UseMutation<SetDefaultAccountMutation> {
    use_mutation(Mutation::new(SetDefaultAccountMutation))
}

/// Whether the mutation has yet to produce a settled result.
pub fn mutation_is_pending<M: MutationCapability>(mutation: &UseMutation<M>) -> bool {
    matches!(
        &*mutation.read().state(),
        MutationStateData::Pending | MutationStateData::Loading { .. }
    )
}

/// Whether the mutation is actually in flight right now.
pub fn mutation_is_running<M: MutationCapability>(mutation: &UseMutation<M>) -> bool {
    mutation.read().state().is_loading()
}

pub fn mutation_error<M>(mutation: &UseMutation<M>) -> Option<M::Err>
where
    M: MutationCapability,
    M::Err: Clone,
{
    let reader = mutation.read();
    match &*reader.state() {
        MutationStateData::Settled { res: Err(err), .. } => Some(err.clone()),
        MutationStateData::Loading {
            res: Some(Err(err)),
        } => Some(err.clone()),
        _ => None,
    }
}

pub fn accounts_have_microsoft(_accounts: &[MinecraftAccount]) -> bool {
    false // No Microsoft accounts in TCC
}