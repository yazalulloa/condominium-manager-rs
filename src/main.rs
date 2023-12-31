pub mod app;
pub mod domain;
#[cfg(feature = "ssr")]
pub mod persistence;
pub mod views;

#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_files::Files;
    use actix_web::*;
    use condominum_manager_rs::app::*;
    use condominum_manager_rs::persistence::db::print_collection_names;
    use dotenv::dotenv;
    use leptos::*;
    use leptos_actix::{generate_route_list, LeptosRoutes};
    use tokio::runtime::Handle;

    dotenv().ok();

    let handle = Handle::current();
    handle.spawn(async {
        print_collection_names().await;
    });
    leptos::log!("{}", "STARTING");

    let conf = get_configuration(None).await.unwrap();
    let addr = conf.leptos_options.site_addr;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(|cx| view! { cx, <App/> });

    let http_server_res = HttpServer::new(move || {
        let leptos_options = &conf.leptos_options;
        let site_root = &leptos_options.site_root;

        App::new()
            .route("/api/{tail:.*}", leptos_actix::handle_server_fns())
            .leptos_routes(
                leptos_options.to_owned(),
                routes.to_owned(),
                |cx| view! { cx, <App/> },
            )
            .service(Files::new("/", site_root))
        //.wrap(middleware::Compress::default())
    })
        .bind(&addr)
        .unwrap()
        .run()
        .await;
    leptos::log!("{}", "STARTED");

    http_server_res
}

#[cfg(not(any(feature = "ssr", feature = "csr")))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
    // see optional feature `ssg` instead
}

#[cfg(all(not(feature = "ssr"), feature = "csr"))]
pub fn main() {
    // a client-side main function is required for using `trunk serve`
    // prefer using `cargo leptos serve` instead
    // to run: `trunk serve --open --features ssg`
    use leptos::*;
    use leptos_start::app::*;
    use wasm_bindgen::prelude::wasm_bindgen;

    console_error_panic_hook::set_once();

    leptos::mount_to_body(move |cx| {
        // note: for testing it may be preferrable to replace this with a
        // more specific component, although leptos_router should still work
        view! {cx, <App/> }
    });
}
