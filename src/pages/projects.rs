use dioxus::prelude::*;

/// Projects page
#[component]
pub fn Projects() -> Element {
    rsx! {
        div { class: "min-h-screen py-20 px-4",

            // Decorative background elements
            div { class: "floating-orb w-80 h-80 top-10 right-10" }
            div { class: "floating-orb w-64 h-64 bottom-20 left-20" }
            div { class: "floating-orb w-72 h-72 top-1/2 left-1/3" }

            h1 { class: "text-5xl sm:text-6xl font-bold text-center mb-6 bg-gradient-to-r from-azure-600 to-ocean-deep bg-clip-text text-transparent",
                "Projects"
            }

            p { class: "text-lg text-azure-700 text-center mb-12 max-w-2xl mx-auto",
                "Here are some of the projects I've worked on. Each represents a unique challenge and learning experience."
            }

            div { class: "max-w-7xl mx-auto grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8",

                // Project 1
                div { class: "group relative",
                    div { class: "glass-morphism rounded-xl p-6 h-full hover:bg-white/20 transition-all duration-300 transform hover:-translate-y-2 hover:shadow-2xl",
                        
                        div { class: "h-2 bg-azure-gradient rounded-full mb-6" }

                        h3 { class: "text-2xl font-bold text-azure-800 mb-4",
                            "WebAssembly Game Engine"
                        }

                        p { class: "text-azure-700 mb-6",
                            "A high-performance 2D game engine built with Rust and compiled to WebAssembly. Features include physics simulation, sprite rendering, and audio support."
                        }

                        div { class: "flex flex-wrap gap-2 mb-6",
                            span { class: "px-3 py-1 bg-azure-100 text-azure-700 rounded-full text-sm font-medium", "Rust" }
                            span { class: "px-3 py-1 bg-azure-100 text-azure-700 rounded-full text-sm font-medium", "WebAssembly" }
                            span { class: "px-3 py-1 bg-azure-100 text-azure-700 rounded-full text-sm font-medium", "WebGL" }
                        }

                        a {
                            href: "https://github.com/example/wasm-game-engine",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            class: "inline-flex items-center text-azure-600 hover:text-azure-800 font-medium transition-colors",
                            "View on GitHub →"
                        }
                    }
                }

                // Project 2
                div { class: "group relative",
                    div { class: "glass-morphism rounded-xl p-6 h-full hover:bg-white/20 transition-all duration-300 transform hover:-translate-y-2 hover:shadow-2xl",
                        
                        div { class: "h-2 bg-ocean-gradient rounded-full mb-6" }

                        h3 { class: "text-2xl font-bold text-azure-800 mb-4",
                            "Distributed Task Queue"
                        }

                        p { class: "text-azure-700 mb-6",
                            "A scalable task queue system with Redis backend, supporting delayed tasks, retries, and real-time monitoring dashboard."
                        }

                        div { class: "flex flex-wrap gap-2 mb-6",
                            span { class: "px-3 py-1 bg-azure-100 text-azure-700 rounded-full text-sm font-medium", "Python" }
                            span { class: "px-3 py-1 bg-azure-100 text-azure-700 rounded-full text-sm font-medium", "Redis" }
                            span { class: "px-3 py-1 bg-azure-100 text-azure-700 rounded-full text-sm font-medium", "Docker" }
                        }

                        a {
                            href: "https://github.com/example/task-queue",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            class: "inline-flex items-center text-azure-600 hover:text-azure-800 font-medium transition-colors",
                            "View on GitHub →"
                        }
                    }
                }

                // Project 3
                div { class: "group relative",
                    div { class: "glass-morphism rounded-xl p-6 h-full hover:bg-white/20 transition-all duration-300 transform hover:-translate-y-2 hover:shadow-2xl",
                        
                        div { class: "h-2 bg-gradient-to-r from-accent-cyan to-azure-500 rounded-full mb-6" }

                        h3 { class: "text-2xl font-bold text-azure-800 mb-4",
                            "Real-time Analytics Platform"
                        }

                        p { class: "text-azure-700 mb-6",
                            "A streaming analytics platform processing millions of events per second with Apache Kafka and ClickHouse for storage."
                        }

                        div { class: "flex flex-wrap gap-2 mb-6",
                            span { class: "px-3 py-1 bg-azure-100 text-azure-700 rounded-full text-sm font-medium", "Java" }
                            span { class: "px-3 py-1 bg-azure-100 text-azure-700 rounded-full text-sm font-medium", "Kafka" }
                            span { class: "px-3 py-1 bg-azure-100 text-azure-700 rounded-full text-sm font-medium", "ClickHouse" }
                        }

                        a {
                            href: "https://github.com/example/analytics-platform",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            class: "inline-flex items-center text-azure-600 hover:text-azure-800 font-medium transition-colors",
                            "View on GitHub →"
                        }
                    }
                }

                // Project 4
                div { class: "group relative",
                    div { class: "glass-morphism rounded-xl p-6 h-full hover:bg-white/20 transition-all duration-300 transform hover:-translate-y-2 hover:shadow-2xl",
                        
                        div { class: "h-2 bg-azure-gradient rounded-full mb-6" }

                        h3 { class: "text-2xl font-bold text-azure-800 mb-4",
                            "CLI Development Tools"
                        }

                        p { class: "text-azure-700 mb-6",
                            "A collection of command-line tools for developers, including a smart git helper, project scaffolding, and automated deployment scripts."
                        }

                        div { class: "flex flex-wrap gap-2 mb-6",
                            span { class: "px-3 py-1 bg-azure-100 text-azure-700 rounded-full text-sm font-medium", "Rust" }
                            span { class: "px-3 py-1 bg-azure-100 text-azure-700 rounded-full text-sm font-medium", "Shell" }
                            span { class: "px-3 py-1 bg-azure-100 text-azure-700 rounded-full text-sm font-medium", "CLI" }
                        }

                        a {
                            href: "https://github.com/example/dev-tools",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            class: "inline-flex items-center text-azure-600 hover:text-azure-800 font-medium transition-colors",
                            "View on GitHub →"
                        }
                    }
                }

                // Project 5
                div { class: "group relative",
                    div { class: "glass-morphism rounded-xl p-6 h-full hover:bg-white/20 transition-all duration-300 transform hover:-translate-y-2 hover:shadow-2xl",
                        
                        div { class: "h-2 bg-gradient-to-r from-accent-indigo to-azure-600 rounded-full mb-6" }

                        h3 { class: "text-2xl font-bold text-azure-800 mb-4",
                            "ML Model Serving API"
                        }

                        p { class: "text-azure-700 mb-6",
                            "RESTful API for serving machine learning models with automatic scaling, A/B testing support, and comprehensive monitoring."
                        }

                        div { class: "flex flex-wrap gap-2 mb-6",
                            span { class: "px-3 py-1 bg-azure-100 text-azure-700 rounded-full text-sm font-medium", "Python" }
                            span { class: "px-3 py-1 bg-azure-100 text-azure-700 rounded-full text-sm font-medium", "FastAPI" }
                            span { class: "px-3 py-1 bg-azure-100 text-azure-700 rounded-full text-sm font-medium", "TensorFlow" }
                        }

                        a {
                            href: "https://github.com/example/ml-serving",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            class: "inline-flex items-center text-azure-600 hover:text-azure-800 font-medium transition-colors",
                            "View on GitHub →"
                        }
                    }
                }

                // Project 6
                div { class: "group relative",
                    div { class: "glass-morphism rounded-xl p-6 h-full hover:bg-white/20 transition-all duration-300 transform hover:-translate-y-2 hover:shadow-2xl",
                        
                        div { class: "h-2 bg-ocean-gradient rounded-full mb-6" }

                        h3 { class: "text-2xl font-bold text-azure-800 mb-4",
                            "Open Source Contributions"
                        }

                        p { class: "text-azure-700 mb-6",
                            "Regular contributor to various open source projects including Dioxus, Rust ecosystem crates, and developer tooling."
                        }

                        div { class: "flex flex-wrap gap-2 mb-6",
                            span { class: "px-3 py-1 bg-azure-100 text-azure-700 rounded-full text-sm font-medium", "Various" }
                            span { class: "px-3 py-1 bg-azure-100 text-azure-700 rounded-full text-sm font-medium", "OSS" }
                        }

                        a {
                            href: "https://github.com/example",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            class: "inline-flex items-center text-azure-600 hover:text-azure-800 font-medium transition-colors",
                            "View Profile →"
                        }
                    }
                }
            }
        }
    }
}