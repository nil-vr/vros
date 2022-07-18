use std::{rc::Rc, sync::Arc};

use deno_core::ModuleSpecifier;
use deno_runtime::{
    deno_broadcast_channel::InMemoryBroadcastChannel,
    deno_core::{error::AnyError, FsModuleLoader},
    deno_web::BlobStore,
    permissions::Permissions,
    worker::{MainWorker, WorkerOptions},
    BootstrapOptions,
};
use futures::future;

mod paths;

fn get_error_class_name(e: &AnyError) -> &'static str {
    deno_runtime::errors::get_error_class_name(e).unwrap_or("Error")
}

#[tokio::main]
async fn main() -> Result<(), AnyError> {
    let broadcast_channel = InMemoryBroadcastChannel::default();

    // TODO: This should be configurable.
    let mut workers = [
        "vrchat-logs.js",
        "steamvr.js",
        "vrchat-camera-meta.js",
        "steamvr-app-demo.js",
    ]
    .into_iter()
    .map(|script| {
        let main_module = deno_runtime::deno_core::resolve_path(script).unwrap();
        let worker = create_worker(main_module.clone(), broadcast_channel.clone());
        (main_module, worker)
    })
    .collect::<Vec<_>>();

    future::try_join_all(workers.iter_mut().map(|(main_module, worker)| async move {
        worker.execute_main_module(&main_module).await
    }))
    .await?;

    println!("Initialization completed");

    future::try_join_all(workers.iter_mut().map(|(main_module, worker)| async move {
        worker.dispatch_load_event(main_module.as_str())?;
        worker.run_event_loop(false).await
    }))
    .await?;

    Ok(())
}

fn create_worker(
    main_module: ModuleSpecifier,
    broadcast_channel: InMemoryBroadcastChannel,
) -> MainWorker {
    let mut worker = MainWorker::bootstrap_from_options(
        main_module.clone(),
        Permissions::allow_all(),
        WorkerOptions {
            bootstrap: BootstrapOptions {
                args: vec![],
                cpu_count: 1,
                debug_flag: false,
                enable_testing_features: false,
                location: None,
                no_color: false,
                is_tty: false,
                runtime_version: "x".to_string(),
                ts_version: "x".to_string(),
                unstable: true,
                user_agent: "vros".to_string(),
            },
            extensions: vec![paths::extension()],
            unsafely_ignore_certificate_errors: None,
            root_cert_store: None,
            seed: None,
            source_map_getter: None,
            format_js_error_fn: None,
            web_worker_preload_module_cb: Arc::new(|_| todo!()),
            create_web_worker_cb: Arc::new(|_| todo!()),
            maybe_inspector_server: None,
            should_break_on_first_statement: false,
            module_loader: Rc::new(FsModuleLoader),
            get_error_class_fn: Some(&get_error_class_name),
            origin_storage_dir: None,
            blob_store: BlobStore::default(),
            broadcast_channel,
            shared_array_buffer_store: None,
            compiled_wasm_module_store: None,
            stdio: Default::default(),
        },
    );

    worker
        .execute_script("vros:ext/paths", include_str!("00_paths.js"))
        .unwrap();

    worker
}
