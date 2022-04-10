// Login Form
use web_sys::HtmlInputElement;
use yew::{Context, Html, html, NodeRef, Component};
use wasm_bindgen_futures::spawn_local;
use std::collections::HashMap;
use gloo_console::log;

// use crate::yew_http;

pub enum Msg {
    FillValue,
}

pub struct Input {
    user: String,       // value of the text field
    pass: String,
    user_ref: NodeRef,  // ref to user element
    pass_ref: NodeRef   // ref to pass element

}

impl Component for Input {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self{
        Self {
            user: String::from(""),
            pass: String::from(""),
            user_ref: NodeRef::default(), // this is the init of a node reference
            pass_ref: NodeRef::default(), // this is the init of a node reference
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool{
        match msg {
            Msg::FillValue => {
                if let Some(input) = self.user_ref.cast::<HtmlInputElement>() {
                    if let Some(input) = self.pass_ref.cast::<HtmlInputElement>() {
                        // cast each node ref to html element
                        self.pass = input.value()
                    }
                    self.user = input.value();
                    let user_name = self.user.clone();
                    let user_pass = self.pass.clone();
                    log!("user email", user_name.to_owned());
                    spawn_local( async {
                        let mut user = HashMap::new();
                        user.insert("user_email", user_name);
                        user.insert("password", user_pass);

                        let client = reqwest::Client::new();
                        client.post("http://localhost:8080/user")
                            .json(&user)
                            .send()
                            .await
                            .expect("send");
                    });
                    true
                    // if there are elements rerender page
                }
                else {
                    false
                }
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html{
        let onclick = ctx.link().callback(|_| Msg::FillValue);
        // function for storing values in controlers
        html! {
            <div>
                <div>
                    <h1>{"Welcome Back To Culinary Ocean"}</h1>
                </div>
                <div id="Login Form">
                    <label>{"UserName"}</label>
                    <br/>
                    <input
                        class="bg-gray-700 rounded-md"
                        type="username"
                        ref={self.user_ref.clone()}
                        />
                    <br/>
                    <br/>
                    <label>{"Password"}</label>
                    <br/>
                    <input
                        class="bg-gray-700 rounded-md"
                        type="password"
                        ref={self.pass_ref.clone()}
                        />
                    <br/>
                    <input
                        class="mt-2 bg-gray-500 rounded hover:rounded-md"
                        type="submit"
                        onclick = {onclick}
                        />
                    <br/>
                </div>
                <div id="Welcome messege">
                    if &self.user != "" && &self.pass != "" {
                        <p class=" text-align center" >{ format!{
                            "Welcome Back {} Nice Password: {}",
                            &self.user, &self.pass} }
                        </p>
                    }
                </div>
            </div>
        }
    }
}
