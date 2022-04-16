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
                <h1>{self.user.clone()}</h1>
            </div>
        }
    }
}
