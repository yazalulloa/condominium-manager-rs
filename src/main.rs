use cfg_if::cfg_if;
use leptos::*;
mod app;

// boilerplate to run in different modes
cfg_if! {
    // server-only stuff
    if #[cfg(feature = "ssr")] {
        use actix_files::{Files};
        use actix_web::*;
        use crate::app::*;
        use leptos_actix::{generate_route_list, LeptosRoutes};

        #[get("/style.css")]
        async fn css() -> impl Responder {
            actix_files::NamedFile::open_async("./style.css").await
        }

        #[actix_web::main]
        async fn main() -> std::io::Result<()> {
           /* let mut conn = db().await.expect("couldn't connect to DB");
            sqlx::migrate!()
                .run(&mut conn)
                .await
                .expect("could not run SQLx migrations");*/

            // Explicit server function registration is no longer required
            // on the main branch. On 0.3.0 and earlier, uncomment the lines
            // below to register the server functions.
            // _ = GetTodos::register();
            // _ = AddTodo::register();
            // _ = DeleteTodo::register();

            // Setting this to None means we'll be using cargo-leptos and its env vars.
            let conf = get_configuration(None).await.unwrap();

            let addr = conf.leptos_options.site_addr;

            // Generate the list of routes in your Leptos App
            let routes = generate_route_list(|cx| view! { cx, <App/> });

            HttpServer::new(move || {
                let leptos_options = &conf.leptos_options;
                let site_root = &leptos_options.site_root;
                let routes = &routes;

                App::new()
                    .service(css)
                    .route("/api/{tail:.*}", leptos_actix::handle_server_fns())
                    .leptos_routes(leptos_options.to_owned(), routes.to_owned(), |cx| view! { cx, <App/> })
                    .service(Files::new("/", site_root))
                    //.wrap(middleware::Compress::default())
            })
            .bind(addr)?
            .run()
            .await
        }
    } else {
        fn main() {
            // no client-side main function
        }
    }
}









// use dotenv::dotenv;
//
// pub mod app;
// mod rates;
// mod mongo;
// mod rate_repository;
//
//
// #[cfg(feature = "ssr")]
// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     use actix_files::Files;
//     use actix_web::*;
//     use leptos::*;
//     use leptos_actix::{generate_route_list, LeptosRoutes};
//     use condominum_manager_rs::app::*;
//
//     dotenv().ok();
//
//     let conf = get_configuration(None).await.unwrap();
//     let addr = conf.leptos_options.site_addr;
//     // Generate the list of routes in your Leptos App
//     let routes = generate_route_list(|cx| view! { cx, <App/> });
//
//     HttpServer::new(move || {
//         let leptos_options = &conf.leptos_options;
//         let site_root = &leptos_options.site_root;
//
//         App::new()
//             .route("/api/{tail:.*}", leptos_actix::handle_server_fns())
//             .leptos_routes(
//                 leptos_options.to_owned(),
//                 routes.to_owned(),
//                 |cx| view! { cx, <App/> },
//             )
//             .service(Files::new("/", site_root))
//         //.wrap(middleware::Compress::default())
//     })
//         .bind(&addr)?
//         .run()
//         .await
// }
//
// #[cfg(not(any(feature = "ssr", feature = "csr")))]
// pub fn main() {
//     // no client-side main function
//     // unless we want this to work with e.g., Trunk for pure client-side testing
//     // see lib.rs for hydration function instead
//     // see optional feature `ssg` instead
// }
//
// #[cfg(all(not(feature = "ssr"), feature = "csr"))]
// pub fn main() {
//     // a client-side main function is required for using `trunk serve`
//     // prefer using `cargo leptos serve` instead
//     // to run: `trunk serve --open --features ssg`
//     use leptos::*;
//     use condominum_manager_rs::app::*;
//     use wasm_bindgen::prelude::wasm_bindgen;
//
//     console_error_panic_hook::set_once();
//
//     leptos::mount_to_body(move |cx| {
//         // note: for testing it may be preferrable to replace this with a
//         // more specific component, although leptos_router should still work
//         view! {cx, <App/> }
//     });
// }
