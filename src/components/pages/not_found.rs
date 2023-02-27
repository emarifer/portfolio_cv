use yew::{function_component, html, Html};

#[function_component(NotFound)]
pub fn not_found() -> Html {
    html! {
         <main
           class="absolute top-0 bottom-0 right-0 left-0 flex justify-center items-center"
         >
           <h1 class="text-6xl text-indigo-700 font-extrabold">
             {"Error 404: Not Found"}
           </h1>
         </main>
    }
}
