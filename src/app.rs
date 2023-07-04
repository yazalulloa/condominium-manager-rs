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

    let double_count = move || count() * 2;

    view! {
               cx,
              <div class="rates-view">
           <div class="header">
            <h1>"Welcome to Leptos!"</h1>
               <button
                class=("button-20", move || count() % 2 == 1)
                class:red=move || count() % 2 == 1
                on:click=on_click>"Click Me: " {count}

                </button>
                <br/>
                <ProgressBar progress=count/>
            <br/>
            <ProgressBar max=50 progress=count/>
            <ProgressBar max=100 progress=Signal::derive(cx, double_count)/>
        <p>"Count: " {count}</p>
        <p>
        "Double Count: "
        // and again here
        {double_count}
    </p>
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
                                                        <RateCard item=rate/>
                                                        }

                                                       })
                                                       .collect_view(cx)
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

#[component]
fn RateCard(cx: Scope, item: ViewItemRate) -> impl IntoView {
    view! {
          cx,
          <div class="card">
    <span>{item._id}</span>
    <span>{item.rate}</span>
    <span>{item.from_currency.to_string()}</span>
    <span>{item.to_currency.to_string()}</span>
    <span>{item.date_of_rate}</span>
    <span>{item.source.to_string()}</span>
    <span>{item.created_at}</span>

          </div>
      }
}

/// Shows progress toward a goal.
#[component]
fn ProgressBar(
    cx: Scope,
    /// The maximum value of the progress bar.
    #[prop(default = 100)]
    max: u16,
    /// How much progress should be displayed.
    #[prop(into)]
    progress: Signal<i32>,
) -> impl IntoView {
    view! { cx,
        <progress
        max=max
            // hmm... where will we get this from?
            value=progress
        />
    }
}

#[component]
fn RateList(cx: Scope, array: Vec<ViewItemRate>) -> impl IntoView {
    let initial_rates = array
        .into_iter()
        .map(|rate| (rate, create_signal(cx, rate)))
        .collect::<Vec<_>>();

    let (rates, set_rates) = create_signal(cx, initial_rates);

    view! { cx,
        <div>

                // The <For/> component is central here
                // This allows for efficient, key list rendering
                <For
                    // `each` takes any function that returns an iterator
                    // this should usually be a signal or derived signal
                    // if it's not reactive, just render a Vec<_> instead of <For/>
                    each=rates
                    // the key should be unique and stable for each row
                    // using an index is usually a bad idea, unless your list
                    // can only grow, because moving items around inside the list
                    // means their indices will change and they will all rerender
                    key=|item| item.0._id
                    // the view function receives each item from your `each` iterator
                    // and returns a view
                    view=move |cx, (id, (rate, set_rate))| {
                        view! { cx,
                            <RateCard item=rate.get()/>
                            // <li>
                            //     <button
                            //         on:click=move |_| {
                            //             set_counters.update(|counters| {
                            //                 counters.retain(|(counter_id, _)| counter_id._id != &id._id.clone())
                            //             });
                            //         }
                            //     >
                            //         "Remove"
                            //     </button>
                            // </li>
                        }
                    }
                />
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
