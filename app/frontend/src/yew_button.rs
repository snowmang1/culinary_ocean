use gloo::console::log;
use yew::{Context, Component, html, Html, Properties};

// helper functions
pub fn array_init() -> [String; 32]{
    let a: [String; 32] = Default::default();
    a
}

pub enum Msg {
    Cycle,
}

#[derive(PartialEq, Properties)]
pub struct Props {
    #[prop_or( String::from("Hello there") )]
    prop1: String,
    #[prop_or( String::from("0") )]
    prop2: String,
    #[prop_or( array_init() )]
    tape: [String; 32],
}

pub struct Form {
    prop_select: usize,
}
impl Component for Form {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self{
        Self {
            prop_select: Default::default(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool{
        match msg {
            Msg::Cycle => {
                self.prop_select = (self.prop_select + 1)%32; // increment counting value
                log!("cycle prop");
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html{
        let onclick = ctx.link().callback(|_| Msg::Cycle);
        let prop = match self.prop_select%2 {
            0 => ctx.props().prop1.clone(),
            1 => ctx.props().prop2.to_string(),
            _ => String::from("0")
        };
        html! {
            <div class="mt-8 mb-4">
                <h1>
                    {
                        format!(
                            "Tape[index]: {}\n
                            Prop: {}",
                            ctx.props().tape[self.prop_select], prop,
                        )
                    }
                </h1>
                <button class="mt-2 bg-indigo-500 rounded-md" {onclick}> {"switch"} </button>
            </div>
        }
    }
}
