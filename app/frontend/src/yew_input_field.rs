use web_sys::HtmlInputElement;
use yew::{Context, Html, html, NodeRef, Component};

pub enum Msg {
    FillValue,
}

pub struct Input {
    value: String,      // value of the text field
    node_ref: NodeRef   // reference to html element
}

impl Component for Input {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self{
        Self {
            value: String::from(""),
            node_ref: NodeRef::default() // this is the init of a node reference
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool{
        match msg {
            Msg::FillValue => {
                if let Some(input) = self.node_ref.cast::<HtmlInputElement>() {
                    self.value = input.value();
                    true
                }
                else {
                    false
                }
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html{
        let onclick = ctx.link().callback(|_| Msg::FillValue);
        html! {
            <div>
                <label>{"input label"}</label>
                <br/>
                <input
                    class="bg-gray-700 rounded-md"
                    type="text"
                    ref={self.node_ref.clone()}
                    />
                <br/>
                <input
                    class="mt-2 bg-gray-500 rounded hover:rounded-md"
                    type="submit"
                    onclick = {onclick}
                    />
                <br/>
                <p class="m-4" >{&self.value}</p>
            </div>
        }
    }
}
