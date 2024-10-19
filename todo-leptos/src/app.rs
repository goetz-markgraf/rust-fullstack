use crate::server::*;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;



#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/todo-leptos.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}



/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let todo_list = create_resource(
        || (),
        move |_| async { get_todos().await.unwrap_or(vec![]) },
    );
    let (new_todo_title, set_new_todo_title) = create_signal(String::new());
    let create_todo_action = create_action(move |msg: &String| {
        let msg = msg.clone();
        async move {
            if let Ok(todo) = add_todo(msg).await {
                todo_list.update(|list| {
                    if let Some(list) = list.as_mut() {
                        list.push(todo)
                    }
                    set_new_todo_title(String::new())
                });
            }
        }
    });

    view! {
        <h1>"Todos!"</h1>
        <Suspense fallback=move || view!{"Loading..."}>
            <TodoList todos=move || todo_list.get().unwrap_or(vec![])/>
        </Suspense>
        <input
            type="text"
            placeholder="What needs to be done?"
            prop:value=new_todo_title
            on:input=move |ev| set_new_todo_title(event_target_value(&ev))
        />
        <br/>
        <button on:click=move|_| create_todo_action.dispatch(new_todo_title()) >"Add Todo"</button>
    }
}



/// Renders a list of todos.
#[component]
fn TodoList(todos: impl Fn() -> Vec<Todo> + 'static) -> impl IntoView {
    view! {
        <ul>
        <For
            each=todos
            key=|todo| todo.id
            children=|todo| view! { <SingleTodoItem single_todo=move || todo.clone()/> }/>
        </ul>
    }
}



/// Renders a single todo item.
#[component]
fn SingleTodoItem(single_todo: impl Fn() -> Todo + 'static) -> impl IntoView {
    let single_todo = single_todo();
    let style = if single_todo.completed {
        Some("completed".to_string())
    } else {
        None
    };
    view! {
        <li class={style}>
            <input type="checkbox" checked=single_todo.completed/>
            {" "}
            <span>{single_todo.description}</span>
        </li>
    }
}



/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}
