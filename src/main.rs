use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <h1>{"Hello world!"}</h1>
            <p>{"Yo, i'm Akane. I'm just a random jr. programmer, that in future wants to do something."}</p>
            <p>{"I hate OOP languages, and love low-medium level and statically-typed programming languages(my favorite programming language is Rust btw, \
                also this site was made in Rust)."}</p>
            <p>{"I know C, C++, Python and Rust(looking for learning Javascript), I use Garuda Linux, and use Neovim for literally every programming language."}</p>
            <p>{"Also privacy is really important for me, so i take care of it. I have an un-googled android, i use LibreWolf as default browser(the default browser on my phone is TOR), \
                and use proton services(ProtonMail and ProtonDrive, looking for buying a RaspberryPi for creating my own drive with NextCloud)."}</p>
            <p>{"That's it."}</p>
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
