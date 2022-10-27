use web_sys::window;
use yew::prelude::*;
use yew_router::prelude::*;

//use crate::api::request;
use crate::app::set_jwt;
use crate::routes::htl_auth::UserInfo;
use crate::REDIRECT;

/// State handle for the [`use_user_context`] hook.
pub struct UseUserContextHandle {
    pub inner: UseStateHandle<UserInfo>,
    history: AnyHistory,
}

impl UseUserContextHandle {
    pub fn login(&self, value: UserInfo) {
        // Set global token after logged in
        set_jwt(Some(value.token.clone()));
        self.inner.set(value);
        //Redirect to entries
        //self.history.push(Route::Entries);
    }

    pub fn logout(&self) {
        // Clear global token after logged out
        set_jwt(None);
        self.inner.set(UserInfo::default());
        // Redirect to login
        let href = format!("https://auth.htl-hl.ac.at/authorize.php?response_type=code&client_id=holi.htl-hl.ac.at&redirect_uri={REDIRECT}&state=new");
        window().unwrap().location().set_href(&href).unwrap();
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
