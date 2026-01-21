use leptos::*;

/// Counter Component - Demonstrasi reactive state dasar
#[component]
pub fn Counter() -> impl IntoView {
    // Signal: reaktif state primitif di Leptos
    // Ownership: signal di-own oleh component scope
    let (count, set_count) = create_signal(0);
    
    // Derived state: computed value yang auto-update
    let double_count = move || count.get() * 2;
    let is_even = move || count.get() % 2 == 0;
    
    // Event handlers: closure yang capture signals
    let increment = move |_| {
        set_count.update(|n| *n += 1);
    };
    
    let decrement = move |_| {
        set_count.update(|n| *n -= 1);
    };
    
    let reset = move |_| {
        set_count.set(0);
    };

    view! {
        <div class="space-y-4">
            // Display counter value
            <div class="text-center">
                <p class="text-6xl font-bold mb-2">
                    {count}
                </p>
                <p class="text-sm opacity-75">
                    "Double: " {double_count} " | "
                    {move || if is_even() { "Even" } else { "Odd" }}
                </p>
            </div>

            // Control buttons
            <div class="flex gap-2 justify-center">
                <button 
                    on:click=decrement
                    class="px-4 py-2 bg-red-500 text-white rounded hover:bg-red-600 transition"
                >
                    "−"
                </button>
                <button 
                    on:click=reset
                    class="px-4 py-2 bg-gray-500 text-white rounded hover:bg-gray-600 transition"
                >
                    "Reset"
                </button>
                <button 
                    on:click=increment
                    class="px-4 py-2 bg-green-500 text-white rounded hover:bg-green-600 transition"
                >
                    "+"
                </button>
            </div>

            // Conditional rendering
            <Show
                when=move || count.get() >= 10
                fallback=|| view! { <p class="text-center text-sm opacity-50">"Keep counting..."</p> }
            >
                <div class="p-4 bg-yellow-100 dark:bg-yellow-900 rounded text-center">
                    "🎉 You reached 10!"
                </div>
            </Show>
        </div>
    }
}

/// Todo Item struct - Rust data model
#[derive(Clone, Debug)]
struct TodoItem {
    id: usize,
    text: String,
    completed: bool,
}

/// TodoList Component - Demonstrasi list rendering & state management
#[component]
pub fn TodoList() -> impl IntoView {
    // Vec di dalam Signal untuk reactive list
    let (todos, set_todos) = create_signal(Vec::<TodoItem>::new());
    let (input_value, set_input_value) = create_signal(String::new());
    let (next_id, set_next_id) = create_signal(0usize);
    
    // Add todo handler
    let add_todo = move |_| {
        let text = input_value.get();
        if !text.trim().is_empty() {
            let new_todo = TodoItem {
                id: next_id.get(),
                text: text.clone(),
                completed: false,
            };
            
            set_todos.update(|todos| {
                todos.push(new_todo);
            });
            
            set_next_id.update(|id| *id += 1);
            set_input_value.set(String::new());
        }
    };
    
    // Toggle completion
    let toggle_todo = move |id: usize| {
        set_todos.update(|todos| {
            if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
                todo.completed = !todo.completed;
            }
        });
    };
    
    // Delete todo
    let delete_todo = move |id: usize| {
        set_todos.update(|todos| {
            todos.retain(|t| t.id != id);
        });
    };
    
    // Computed stats
    let total_count = move || todos.get().len();
    let completed_count = move || {
        todos.get().iter().filter(|t| t.completed).count()
    };

    view! {
        <div class="space-y-4">
            // Input form
            <div class="flex gap-2">
                <input 
                    type="text"
                    placeholder="Add a new task..."
                    class="flex-1 px-4 py-2 border rounded dark:bg-gray-700 dark:border-gray-600"
                    prop:value=input_value
                    on:input=move |ev| {
                        set_input_value.set(event_target_value(&ev));
                    }
                    on:keypress=move |ev| {
                        if ev.key() == "Enter" {
                            add_todo(ev);
                        }
                    }
                />
                <button 
                    on:click=add_todo
                    class="px-6 py-2 bg-blue-500 text-white rounded hover:bg-blue-600 transition"
                >
                    "Add"
                </button>
            </div>

            // Stats
            <div class="text-sm opacity-75">
                {completed_count} " / " {total_count} " completed"
            </div>

            // Todo list rendering
            <ul class="space-y-2">
                <For
                    each=move || todos.get()
                    key=|todo| todo.id
                    children=move |todo: TodoItem| {
                        let id = todo.id;
                        view! {
                            <li class="flex items-center gap-2 p-3 bg-gray-100 dark:bg-gray-700 rounded">
                                <input 
                                    type="checkbox"
                                    checked=todo.completed
                                    on:change=move |_| toggle_todo(id)
                                    class="w-5 h-5"
                                />
                                <span 
                                    class=move || {
                                        if todo.completed {
                                            "flex-1 line-through opacity-50"
                                        } else {
                                            "flex-1"
                                        }
                                    }
                                >
                                    {todo.text.clone()}
                                </span>
                                <button 
                                    on:click=move |_| delete_todo(id)
                                    class="px-3 py-1 bg-red-500 text-white text-sm rounded hover:bg-red-600 transition"
                                >
                                    "Delete"
                                </button>
                            </li>
                        }
                    }
                />
            </ul>

            // Empty state
            <Show
                when=move || todos.get().is_empty()
                fallback=|| view! {}
            >
                <div class="text-center py-8 opacity-50">
                    <p>"No tasks yet. Add one above!"</p>
                </div>
            </Show>
        </div>
    }
}

/// ThemeToggle Component - Demonstrasi props & signal sharing
#[component]
pub fn ThemeToggle(
    theme: ReadSignal<String>,
    set_theme: WriteSignal<String>,
) -> impl IntoView {
    let toggle = move |_| {
        set_theme.update(|t| {
            *t = if t == "light" { 
                "dark".to_string() 
            } else { 
                "light".to_string() 
            };
        });
    };

    view! {
        <button 
            on:click=toggle
            class="px-4 py-2 border rounded hover:bg-gray-100 dark:hover:bg-gray-800 transition"
        >
            {move || if theme.get() == "dark" { "☀️ Light" } else { "🌙 Dark" }}
        </button>
    }
}