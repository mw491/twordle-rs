mod app;
mod pick_word;

use app::{GameType, Twordle};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Daily,
    #[at("/unlimited")]
    Unlimited,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Daily => html! {
            <Twordle game_type={GameType::Daily} />
        },
        Route::Unlimited => html! {
            <Twordle game_type={GameType::Unlimited} />
        },
    }
}

#[function_component]
fn App() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
