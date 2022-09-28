use yew::prelude::*;
<<<<<<< HEAD
use crate::hooks::use_user_context;

=======
use yew_router::prelude::{use_history, History};

use crate::hooks::use_user_context;
use crate::components::Auth;
use super::Route;
>>>>>>> a74e52c (Add logout button)

#[function_component(Logout)]
pub fn logout() -> Html {
    let user_ctx = use_user_context();
    user_ctx.logout();

<<<<<<< HEAD
    html!(        
=======
    html!(
        
>>>>>>> a74e52c (Add logout button)
    )
}