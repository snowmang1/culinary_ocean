mod account;
mod login;
mod route;

use route::Route;
use yew::prelude::*;
use yew_router::prelude::*;

// function formater for login page
#[function_component(Login)]
pub fn dom_handler() -> Html {
    // TODO add this html code to login rs and just use struct component
    use login::Input;
    html! {
        <div class="flex justify-center">
            <div class="grid grid-cols-1 gap-10">
                <Input />
            </div>
        </div>
    }
}

#[function_component(NotFound)]
fn nf() -> Html {
    // 404 page not found page
    // TODO hook this up with actix someday
    html! {
        <div>
            <h1>{ "404 Page Not Found" }</h1>
            <Link<Route> to={Route::Login}>{ "click to go home" }</Link<Route>>
        </div>
    }
}

fn switch(routes: &Route) -> Html {
    use account::AccountPage;
    match routes {
        Route::Login => html! { <Login /> },
        Route::Account => html! { <AccountPage /> },
        // all valid page routes get inserted above here
        Route::NotFound => html! { <NotFound /> },
    }
}

#[function_component(Main)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

fn main() {
    // using fxn component to format all struct components
    yew::start_app::<Main>();
}
