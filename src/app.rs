use cfg_if::cfg_if;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::views::rate_view_service::ViewItemRate;

cfg_if! {
    if #[cfg(feature = "ssr")] {

use crate::views::rate_view_service::get_view_item_rate;



         pub async fn db_get_rates(page: i64, page_size: i64)  -> Result<Vec<ViewItemRate>, ServerFnError> {
         let vec = get_view_item_rate(page, page_size).await;
         Ok(vec)
        }


    }

}

#[server(GetRates, "/api")]
pub async fn get_rates(cx: Scope) -> Result<Vec<ViewItemRate>, ServerFnError> {
    // this is just an example of how to access server context injected in the handlers
    let req = use_context::<actix_web::HttpRequest>(cx);

    if let Some(req) = req {
        println!("req.path = {:#?}", req.path());
    }

    db_get_rates(1, 30).await
}

/*


*/

/*
#[server(GetRates, "/api")]
pub async fn get_rates(cx: Scope) -> Result<Vec<Rate>, ServerFnError> {
    // this is just an example of how to access server context injected in the handlers

    let req = use_context::<actix_web::HttpRequest>(cx);

    if let Some(req) = req {
        println!("req.path = {:#?}", req.path());
    }

    let repo = DB::init().await.unwrap().rates;
    let vec = repo.list(1, 10).await;
    Ok(vec)
}

#[server(AddRate, "/api")]
pub async fn add_rate(title: String) -> Result<(), ServerFnError> {
    // fake API delay
    std::thread::sleep(std::time::Duration::from_millis(1250));

    Ok(())
}

#[server(DeleteRate, "/api")]
pub async fn delete_rate(id: u64) -> Result<(), ServerFnError> {
    std::thread::sleep(std::time::Duration::from_millis(1250));

    Ok(())
}
*/

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/condominum_manager_rs.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <HomePage/> }/>
         <Route path="" view=|cx| view! {
                        cx,
                        <HomePage/>
                    }/>
                </Routes>
            </main>
        </Router>
    }
}


/// Renders the home page of your application.
#[component]
pub fn HomePage(cx: Scope) -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(cx, 0);
    let on_click = move |_| set_count.update(|count| *count += 1);


    let rates = create_resource(cx, move || (), move |_| get_rates(cx));

    view! {
            cx,
           <div class="rates-view">
        <div class="header">
         <h1>"Welcome to Leptos!"</h1>
            <button on:click=on_click>"Click Me: " {count}</button>
        </div>
   <div class="table-div">
         <Transition fallback=move || view! {cx, <p>"Loading..."</p> }>
                    {move || {
                        let existing_todos = {
                            move || {
                                rates.read(cx)
                                    .map(move |rates| match rates {
                                        Err(e) => {
                                            view! { cx, <pre class="error">"Server Error: " {e.to_string()}</pre>}.into_view(cx)
                                        }
                                        Ok(rates) => {
                                            if rates.is_empty() {
                                                view! { cx, <p>"No rates were found."</p> }.into_view(cx)
                                            } else {

                                            rates
                                                .into_iter()
                                                    .map(move |rate| {
                                                        view! {
                                                            cx,
                                                            <div class="card">
                                                      <span>{rate._id}</span>
                                                      <span>{rate.rate}</span>
                                                      <span>{rate.from_currency.to_string()}</span>
                                                      <span>{rate.to_currency.to_string()}</span>
                                                      <span>{rate.date_of_rate}</span>
                                                      <span>{rate.source.to_string()}</span>
                                                      <span>{rate.created_at}</span>

                                                            </div>
                                                        }
                                                    })
                                                    .collect_view(cx)

                                           /* view! {
                                                cx,
                                               <div class="table-data">
                                                /* <table class="table">
                                                <tbody>
                                                 <tr>*/
                                                {rows}
                                               /* </tr>
                                                </tbody>
                                                </table>*/
                                                </div>
                                            }.into_view(cx)*/
                                            }
                                        }
                                    })
                                    .unwrap_or_default()
                            }
                        };

                        view! {
                            cx,
                            <div>
                                {existing_todos}
                            </div>
                        }
                    }
                }
                </Transition>
        </div>

        <div class="footer">
         <h1>"Welcome to Leptos!"</h1>
            <button on:click=on_click>"Click Me: " {count}</button>
        </div>
        </div>
        }
}

// #[component]
// fn PrintRate(cx: Scope) -> impl IntoView {
//     let add_rate = create_server_multi_action::<AddRate>(cx);
//     let delete_rate = create_server_action::<DeleteRate>(cx);
//     let submissions = add_rate.submissions();
//
//     let rates = create_resource(
//         cx,
//         move || (add_rate.version().get()),
//         move |_| get_rates(cx),
//     );
//
//     view! {
//         cx,
//         <div>
//             <MultiActionForm
//                 // we can handle client-side validation in the on:submit event
//                 // leptos_router implements a `FromFormData` trait that lets you
//                 // parse deserializable types from form data and check them
//                 on:submit=move |ev| {
//                     let data = AddRate::from_event(&ev).expect("to parse form data");
//                     // silly example of validation: if the todo is "nope!", nope it
//                     if data.title == "nope!" {
//                         // ev.prevent_default() will prevent form submission
//                         ev.prevent_default();
//                     }
//                 }
//                 action=add_rate>
//                 <label>
//                     "Add a Todo"
//                     <input type="text" name="title"/>
//                 </label>
//                 <input type="submit" value="Add"/>
//             </MultiActionForm>
//             <Transition fallback=move || view! {cx, <p>"Loading..."</p> }>
//                 {move || {
//                     let existing_todos = {
//                         move || {
//                             rates.read(cx)
//                                 .map(move |rates| match rates {
//                                     Err(e) => {
//                                         view! { cx, <pre class="error">"Server Error: " {e.to_string()}</pre>}.into_view(cx)
//                                     }
//                                     Ok(rates) => {
//                                         if rates.is_empty() {
//                                             view! { cx, <p>"No tasks were found."</p> }.into_view(cx)
//                                         } else {
//                                             rates
//                                                 .into_iter()
//                                                 .map(move |todo| {
//                                                     view! {
//                                                         cx,
//                                                         <li>
//                                                             {todo._id}
//                                                             <ActionForm action=delete_rate>
//                                                                 <input type="hidden" name="id" value={todo._id}/>
//                                                                 <input type="submit" value="X"/>
//                                                             </ActionForm>
//                                                         </li>
//                                                     }
//                                                 })
//                                                 .collect_view(cx)
//                                         }
//                                     }
//                                 })
//                                 .unwrap_or_default()
//                         }
//                     };
//
//                     let pending_todos = move || {
//                         submissions
//                         .get()
//                         .into_iter()
//                         .filter(|submission| submission.pending().get())
//                         .map(|submission| {
//                             view! {
//                                 cx,
//                                 <li class="pending">{move || submission.input.get().map(|data| data.title) }</li>
//                             }
//                         })
//                         .collect_view(cx)
//                     };
//
//                     view! {
//                         cx,
//                         <ul>
//                             {existing_todos}
//                             {pending_todos}
//                         </ul>
//                     }
//                 }
//             }
//             </Transition>
//         </div>
//     }
// }
