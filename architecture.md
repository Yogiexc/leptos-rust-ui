graph TB
    subgraph DEV["Development Phase"]
        A[Rust Source Code] -->|view! macro| B[Leptos Macro Expansion]
        B -->|AST transforms| C[Expanded Rust Code]
        C -->|rustc| D[LLVM IR]
        D -->|wasm-backend| E[WebAssembly Binary]
    end
    
    subgraph BUILD["Build Artifacts"]
        E --> F[.wasm file 180KB]
        F --> G[JS Glue Code 20KB]
        G --> H[index.html]
    end
    
    subgraph BROWSER["Browser Runtime"]
        H --> I[HTML Parser]
        I --> J[Load WASM Module]
        J --> K[WASM Instantiation]
        K --> L[Leptos Runtime Init]
        L --> M[Signal System Setup]
        M --> N[Initial DOM Render]
    end
    
    subgraph REACTIVE["Reactive Updates"]
        O[User Event] --> P[WASM Function Call]
        P --> Q[Signal Update]
        Q --> R[Reactivity Graph Traverse]
        R --> S[Only Changed Nodes]
        S --> T[Direct DOM Mutation]
        T -.->|No VDOM diff| N
    end
    
    N --> O
    
    style A fill:#e3f2fd
    style E fill:#fff3e0
    style J fill:#f3e5f5
    style Q fill:#e8f5e9
    style T fill:#ffebee