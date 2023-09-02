use leptos::*;
use leptos::html::Input;
use leptos_meta::*;
use leptos_router::*;

use crate::error_template::{AppError, ErrorTemplate};

#[server(AddTodo, "/api")]
pub async fn add_todo(title: String) -> Result<(), ServerFnError> {
    println!("add todo: {title}");
    Ok(())
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! { cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/simple-fullstack.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|cx| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { cx,
                <ErrorTemplate outside_errors/>
            }
            .into_view(cx)
        }>
            <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <TodoList/> }/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn TodoList(cx: Scope) -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(cx, 0);
    let entries: RwSignal<Vec<String>> = create_rw_signal(cx, Vec::new());
    let input_ref: NodeRef<Input> = create_node_ref(cx);

    view! { cx,
        <h1>"My Todos"</h1>
        <input type="text" value="placeholder" node_ref=input_ref/>

        <div class="px-"></div>
        <button on:click=move |_| {
            let text  = input_ref.get().expect("input to exist").value();

            spawn_local(async {
                add_todo("test".to_owned()).await.unwrap();
            })
        } class="bg-amber-600 px-5 py-3 text-white rounded-lg">"Add todo" {count}</button>
    }
}
