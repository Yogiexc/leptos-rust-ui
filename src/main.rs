use leptos::*;
use crate::components::{Counter, TodoList, ThemeToggle};

#[component]
fn App() -> impl IntoView {
    // Reactive state: Signal adalah primitif reaktif Leptos
    // create_signal mengembalikan (ReadSignal, WriteSignal)
    let (theme, set_theme) = create_signal("light".to_string());
    
    // Derived signal - computed value yang otomatis update
    let theme_class = move || {
        if theme.get() == "dark" {
            "bg-gray-900 text-white min-h-screen"
        } else {
            "bg-gray-50 text-gray-900 min-h-screen"
        }
    };

    view! {
        <div class=theme_class>
            <div class="container mx-auto px-4 py-8 max-w-4xl">
                // Header Section
                <header class="mb-12">
                    <div class="flex justify-between items-center mb-4">
                        <h1 class="text-4xl font-bold">
                            "Leptos + WebAssembly"
                        </h1>
                        <ThemeToggle theme=theme set_theme=set_theme />
                    </div>
                    <p class="text-lg opacity-75">
                        "Exploring Rust-based frontend development using Leptos to build reactive web interfaces compiled to WebAssembly."
                    </p>
                </header>

                // Main Content
                <main class="space-y-8">
                    // Counter Component
                    <section class="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-6">
                        <h2 class="text-2xl font-semibold mb-4">
                            "Interactive Counter"
                        </h2>
                        <Counter />
                    </section>

                    // Todo List Component
                    <section class="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-6">
                        <h2 class="text-2xl font-semibold mb-4">
                            "Reactive Todo List"
                        </h2>
                        <TodoList />
                    </section>

                    // Info Section
                    <section class="bg-blue-50 dark:bg-blue-900 rounded-lg p-6">
                        <h2 class="text-2xl font-semibold mb-4">
                            "Why Leptos?"
                        </h2>
                        <ul class="space-y-2 list-disc list-inside">
                            <li>"Fine-grained reactivity - hanya update yang berubah"</li>
                            <li>"Zero-cost abstractions dari Rust"</li>
                            <li>"Type safety end-to-end"</li>
                            <li>"WASM performance benefits"</li>
                            <li>"No Virtual DOM overhead"</li>
                        </ul>
                    </section>
                </main>

                // Footer
                <footer class="mt-12 text-center opacity-60">
                    <p>"Built with Rust 🦀 | Compiled to WebAssembly"</p>
                </footer>
            </div>
        </div>
    }
}

fn main() {
    // Mount aplikasi ke DOM
    // Leptos akan mengkompilasi ini ke WASM dan inject ke HTML
    mount_to_body(|| view! { <App/> })
}

// Module declaration
mod components;