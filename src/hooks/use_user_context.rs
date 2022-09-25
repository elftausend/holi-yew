use std::ops::Deref;

use reqwest::Method;
use yew::prelude::*;
use yew_hooks::use_async;
use yew_router::prelude::*;

use crate::api::request;
use crate::app::set_jwt;
use crate::routes::login::UserInfo;
use crate::routes::Route;

/// State handle for the [`use_user_context`] hook.
pub struct UseUserContextHandle {
    inner: UseStateHandle<UserInfo>,
    history: AnyHistory,
}

impl UseUserContextHandle {
    pub fn login(&self, value: UserInfo) {
        // Set global token after logged in
        set_jwt(Some(value.token.clone()));
        self.inner.set(value);
        //Redirect to entries
        self.history.push(Route::Entries);
    }

    pub fn logout(&self) {
        // Clear global token after logged out
        set_jwt(None);
        self.inner.set(UserInfo::default());
        // Redirect to login
        self.history.push(Route::Login);
    }
}

impl Deref for UseUserContextHandle {
    type Target = UserInfo;

    fn deref(&self) -> &Self::Target {
        &(*self.inner)
    }
}

impl Clone for UseUserContextHandle {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            history: self.history.clone(),
        }
    }
}

impl PartialEq for UseUserContextHandle {
    fn eq(&self, other: &Self) -> bool {
        *self.inner == *other.inner
    }
}

/// This hook is used to manage user context.
pub fn use_user_context() -> UseUserContextHandle {
    let inner = use_context().unwrap();
    let history = use_history().unwrap();

    UseUserContextHandle { inner, history }
}
