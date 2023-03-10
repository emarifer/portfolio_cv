use chrono::{Datelike, Utc};
use wasm_bindgen::{
    prelude::{wasm_bindgen, Closure},
    JsCast,
};
use yew::{function_component, html, use_effect_with_deps, use_state, Callback, Html, MouseEvent};
use yew_router::prelude::BrowserRouter;

use crate::components::{content::Content, navbar::Navbar, sidebar::Sidebar};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[function_component(App)]
pub fn app() -> Html {
    let active_route = use_state(|| String::new());

    let show_sidebar = use_state(|| false);

    let show_button_to_top = use_state(|| false);

    let scroll_to_top = Callback::from(|_e: MouseEvent| {
        web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .document_element()
            .unwrap()
            .scroll_to_with_scroll_to_options(
                web_sys::ScrollToOptions::new()
                    .behavior(web_sys::ScrollBehavior::Smooth)
                    .top(0.0),
            );
    });

    {
        let show_button_to_top = show_button_to_top.clone();

        use_effect_with_deps(
            move |_| {
                let mut scroll_listener = None;

                if let Some(window) = web_sys::window() {
                    let listener = {
                        // Es necesario crear un scope porque el objeto window es movido al closure
                        let window = window.clone();
                        // Crea un Closure a partir de un Box<dyn Fn> - este tiene que ser 'static
                        Closure::<dyn Fn()>::wrap(Box::new(move || {
                            let scroll_top = window.scroll_y().unwrap();
                            // log(&scroll_top.to_string()); // imprime en consola el valor del scroll_y
                            show_button_to_top.set(scroll_top > 300.0);
                        }))
                    };

                    window
                        .add_event_listener_with_callback(
                            "scroll",
                            listener.as_ref().unchecked_ref(),
                        )
                        .unwrap();

                    scroll_listener = Some(listener)
                }
                // Esta es la funci??n de limpieza(cleanup) del use_effect_with_deps
                move || drop(scroll_listener)
            },
            (), // Tupla de dependencias: No hay, por lo que el use_effect_with_deps solo se hace
                // la primera vez
        );
    }

    let button_to_up = {
        let show_button_to_top = show_button_to_top.clone();

        if *show_button_to_top {
            html! {
                <button onclick={scroll_to_top}
                  class="bg-sky-600 shadow-md shadow-sky-800 hover:bg-sky-400 rounded-full fixed bottom-4 right-4 p-2 z-50">
                  <svg fill="currentColor" class="w-6" viewBox="0 0 16 16">
                    <path d="m7.247 4.86-4.796 5.481c-.566.647-.106 1.659.753 1.659h9.592a1 \
                    1 0 0 0 .753-1.659l-4.796-5.48a1 1 0 0 0-1.506 0z"/>
                  </svg>
                </button>
            }
        } else {
            html!()
        }
    };

    html! {
        <BrowserRouter>
          <header>
            <Navbar activeroute={active_route.clone()} activesidebar={show_sidebar.clone()} />
          </header>

          <main>
            <Content activeroute={active_route.clone()} />
          </main>

          <footer class="text-center pt-6 pb-10">
            <a
              class="italic hover:text-sky-500 ease-in duration-300"
              href="https://github.com/emarifer/portfolio_cv/"
              target="_blank"
            >
              {format!("Designed & Developed by Enrique Mar??n | MIT Licensed | Copyright ?? {}", Utc::now().year())}
            </a>
          </footer>

          <Sidebar activesidebar={show_sidebar} activeroute={active_route} />

          {button_to_up}
        </BrowserRouter>
    }
}

/*
 * AUNQUE M??S ABAJO SE DESCRIBE (EN Stackoverflow) UNA MANERA DE PASAR UN CLOSURE DE RUST
 * COMO Callback PARA CREAR UN LISTENER, EN LA DOCUMENTACI??N DE YEW SE DESCRIBE UNA FORMA
 * M??S CORRECTA DE HACER ESTO, QUE ES COMO SE HA HECHO AQU??. VER:
 * https://yew.rs/docs/concepts/html/events#using-closure-verbose
 * https://yew.rs/docs/concepts/basic-web-technologies/wasm-bindgen#closure
 * HAY UNA MANERA MENOS ??VERBOSA?? DE HACERLO USANDO LA BIBLIOTECA gloo-events. VER:
 * https://yew.rs/docs/concepts/html/events#using-gloo-concise
 *
 * CONVERTIR UN CLOSURE DE RUST EN js_sys::Function. VER:
 * https://stackoverflow.com/questions/60054963/how-to-convert-closure-to-js-sysfunction#60058227
 * https://rustwasm.github.io/wasm-bindgen/reference/passing-rust-closures-to-js.html
 * https://rustwasm.github.io/wasm-bindgen/examples/closures.html
 * https://github.com/rustwasm/wasm-bindgen/issues/843#issuecomment-422095718
 * https://rustwasm.github.io/wasm-bindgen/examples/closures.html
 * https://users.rust-lang.org/t/why-closure-can-be-converted-into-js-function-in-wasm/76981
 * https://docs.rs/yew/0.20.0/yew/functional/fn.use_effect_with_deps.html
 *
 * DOCUMENTACI??N SOBRE LOS M??TODOS DE WINDOW. VER:
 * https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Element.html#method.scroll_with_scroll_to_options
 * https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.ScrollToOptions.html
 * https://developer.mozilla.org/en-US/docs/Web/API/Element/scroll
 *
 * EJEMPLO HECHO EN REACT. VER:
 * https://github.com/emarifer/lpcv-app/blob/main/src/App.tsx
 * https://github.com/emarifer/lpcv-app/blob/main/src/components/ButtonToTop/ButtonToTop.tsx
 */
