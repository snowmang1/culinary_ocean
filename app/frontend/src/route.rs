
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/Login")]
    Login,
    #[at("/Account")]
    Account,
    #[not_found]
    #[at("/404")]
    NotFound,
}
