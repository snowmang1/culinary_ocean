
use crate::util::FetchState;
use crate::util::fetch_rec;

use gloo_console::log;
use reqwest;
use std::collections::HashMap;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;

pub enum Msg {
    FormCon,
    SetFetchState(FetchState<HashMap<String, String>>),
    GetRec,
}

pub struct AccountPage {
    // post vars
    user: String,
    recipe_name: String,
    recipe_instr: String,
    name_ref: NodeRef,
    instr_ref: NodeRef,
    // get vars
    fetch_state: FetchState<HashMap<String, String>>,
}

impl Component for AccountPage {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        // query database grab relevent info about user
        Self {
            user: "Snow".to_string(),
            recipe_name: String::from(""),
            recipe_instr: String::from(""),
            name_ref: NodeRef::default(), instr_ref: NodeRef::default(),
            fetch_state: FetchState::NotFetching,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::FormCon => {
                if let Some(input) = self.name_ref.cast::<HtmlInputElement>() {
                    if let Some(input) = self.instr_ref.cast::<HtmlInputElement>() {
                        self.recipe_instr = input.value();
                    }
                    self.recipe_name = input.value();
                    // start posting process
                    let user_name = self.user.clone();
                    let rec_name = self.recipe_name.clone();
                    let rec = self.recipe_instr.clone();
                    let pass = "123".to_string(); // password does not matter so it is magic
                    log!("user email", user_name.to_owned()); // log email

                    spawn_local(async {
                        let mut user = HashMap::new();
                        user.insert("user_email", user_name);
                        user.insert("password", pass);
                        user.insert("instructions", rec);
                        user.insert("ingredients", rec_name);
                        let client = reqwest::Client::new();
                        client
                            .post("http://localhost:8080/user")
                            .json(&user)
                            .send()
                            .await
                            .expect("send");
                    });

                    true
                } else {
                    false
                }
            } // formcon
            Msg::SetFetchState(fetch_state)  => {
                self.fetch_state = fetch_state;
                true
            }
            Msg::GetRec => {
                let url = "http://localhost:8080/user/Snow".to_string();
                ctx.link().send_future(async {
                    match fetch_rec(url).await {
                        Ok(map) => Msg::SetFetchState(FetchState::Success(map)),
                        Err(err) => Msg::SetFetchState(FetchState::Failed(err)),
                    }
                });
                ctx.link()
                    .send_message(Msg::SetFetchState(FetchState::Fetching));
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = ctx.link().callback(|_| Msg::FormCon);
        // use crate::util::json_parse;

        match &self.fetch_state {
            FetchState::NotFetching => html! {
                    <div>
                        <div>
                            <h1>{format!("Welcome Back {}",
                                        self.user.clone())}
                            </h1>
                        </div>
                        <div id="Recipe Form">
                            // form with text input
                            <label>{"Recipe Name"}</label>
                            <br/>
                            <input
                                class="bg-gray-700 rounded-md"
                                ref={self.name_ref.clone()}
                            />
                            <br/>
                            <label>{"Recipe Instructions"}</label>
                            <br/>
                            <input
                                class="bg-gray-700 rounded-md"
                                ref={self.instr_ref.clone()}
                            />
                            <input
                                class="mt-2 bg-gray-500 rounded hover:rounded-md"
                                type="submit"
                                onclick = {onclick}
                                />
                        </div>
                        <div id="test">
                            if &self.recipe_name != "" && &self.recipe_instr != ""{
                                <div>
                                    <h1>
                                        { format!{
                                        "recipe name == {}",
                                        &self.recipe_name}
                                        }
                                    </h1>
                                    <h1>
                                        { format! {
                                        "recipe instructions == {}",
                                        &self.recipe_instr}
                                        }
                                    </h1>
                                </div>
                            }
                        </div>
                        <div id="display rec">
                            <button onclick={ctx.link().callback(|_| Msg::GetRec)}>
                                {"get rec"}
                            </button>
                        </div>
                    </div>
            },

            // TODO these need the page html
            FetchState::Fetching => html! {"Fetching"},

            FetchState::Success(data) => html! {
                    <div>
                        <div>
                            <h1>{format!("Welcome Back {}",
                                        self.user.clone())}
                            </h1>
                        </div>
                        <div id="Recipe Form">
                            // form with text input
                            <label>{"Recipe Name"}</label>
                            <br/>
                            <input
                                class="bg-gray-700 rounded-md"
                                ref={self.name_ref.clone()}
                            />
                            <br/>
                            <label>{"Recipe Instructions"}</label>
                            <br/>
                            <input
                                class="bg-gray-700 rounded-md"
                                ref={self.instr_ref.clone()}
                            />
                            <input
                                class="mt-2 bg-gray-500 rounded hover:rounded-md"
                                type="submit"
                                onclick = {onclick}
                                />
                        </div>
                        <div id="test">
                            if &self.recipe_name != "" && &self.recipe_instr != ""{
                                <div>
                                    <h1>
                                        { format!{
                                        "recipe name == {}",
                                        &self.recipe_name}
                                        }
                                    </h1>
                                    <h1>
                                        { format! {
                                        "recipe instructions == {}",
                                        &self.recipe_instr}
                                        }
                                    </h1>
                                </div>
                            }
                        </div>
                        <div id="display rec">
                            <button onclick={ctx.link().callback(|_| Msg::GetRec)}>
                                {"get rec"}
                            </button>
                        </div>
                        // rec blocks
                        <div>
                            <h1>{data["ingredients"].clone()}</h1>
                            <h1>{data["instructions"].clone()}</h1>
                        </div>
                    </div>
            },

            FetchState::Failed(err) => html!{ err },

        }
    }
}
