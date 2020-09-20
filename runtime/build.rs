use wasm_builder_runner::WasmBuilder;

fn main() {
    WasmBuilder::new()
        .with_current_project()
        .with_wasm_builder_from_crates("")
        .export_heap_base()
        .import_memory()
        .build()
}
