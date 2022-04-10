mod login;

use yew::{html, function_component};

#[function_component(Dom)]
fn dom_handler() -> Html{
    // structure component only no input should be taken FOR NOW
    use login::Input;
    html!{
        <div class="flex justify-center">
            <div class="grid grid-cols-1 gap-10">
                // text changing NOT A FORM
                <Input />
            </div>
        </div>
    }
}

fn main() {
    // using fxn component to format all struct components
    yew::start_app::<Dom>();
}
