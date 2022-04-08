use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/counter")]
    Counter,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(Counter)]
fn counter() -> Html {
    let data = use_state(|| Option::<u32>::None);
    {
        let data = data.clone();
        use_effect(move || {
            if data.is_none() {
                spawn_local(async move {
                    let resp = Request::post("/api/counter").send().await.unwrap();

                    if !resp.ok() {
                        log::error!(
                            "Error fetching data {} ({})",
                            resp.status(),
                            resp.status_text()
                        );
                        return;
                    }

                    let content = match resp.text().await {
                        Err(err) => {
                            log::error!("Error fetching data {err}");
                            return;
                        }
                        Ok(content) => content,
                    };

                    let count = match content.parse() {
                        Err(err) => {
                            log::error!("Data is not a number: {err}");
                            return;
                        }
                        Ok(count) => count,
                    };

                    data.set(Some(count));
                });
            }

            || {}
        });
    }

    html! {
        <div>
            <div>{data.map(|d|d.to_string()).unwrap_or_default()}</div>
        </div>
    }
}

// -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-

fn switch(routes: &Route) -> Html {
    dbg!(routes.to_path());
    match routes {
        Route::Home => html! { <h1>{ "Home" }</h1> },
        Route::Counter => html! { <Counter /> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    console_error_panic_hook::set_once();

    yew::start_app::<App>();
}
