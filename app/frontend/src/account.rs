use yew::prelude::*;

pub struct AccountPage {
    user: String,
}

impl Component for AccountPage {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        // query database grab relevent info about user
        Self {
            user: "User".to_string(),
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <div>
                    <h1>{format!("Welcome Back {}",
                                self.user.clone())}
                    </h1>
                </div>
                <div>
                    // This will be our new recipe button
                    <h1>{"new recipe"}</h1>
                </div>
            </div>
        }
    }
}
