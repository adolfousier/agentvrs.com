use leptos::prelude::*;
use wasm_bindgen_futures::spawn_local;

const GITHUB_REPO: &str = "adolfousier/agentvrs";

#[component]
pub fn App() -> impl IntoView {
    let (stars, set_stars) = signal(0u32);
    
    spawn_local(async move {
        if let Some(count) = fetch_star_count().await {
            set_stars.set(count);
        }
    });
    
    view! {
        <div class="agentverse">
            <Nav stars={stars} />
            <Hero />
            <QuickStart />
            <Features />
            <Integrations />
            <Testimonials />
            <Community />
            <Footer stars={stars} />
        </div>
    }
}

async fn fetch_star_count() -> Option<u32> {
    use gloo_net::http::Request;
    
    #[derive(serde::Deserialize)]
    struct GitHubRepo { stargazers_count: u32 }
    
    let resp = Request::get(&format!("https://api.github.com/repos/{}", GITHUB_REPO))
        .header("Accept", "application/vnd.github+json")
        .header("User-Agent", "agentvrscom")
        .send()
        .await
        .ok()?;
    
    resp.json::<GitHubRepo>().await.ok().map(|r| r.stargazers_count)
}

#[component]
fn Nav(stars: ReadSignal<u32>) -> impl IntoView {
    view! {
        <nav class="nav">
            <div class="nav-inner">
                <div class="nav-logo">
                    <span class="logo-icon">{"{V}"}</span>
                    <span class="logo-text">AgentVerse</span>
                </div>
                <div class="nav-links">
                    <a href="#features">Features</a>
                    <a href="#quickstart">Quick Start</a>
                    <a href="https://docs.agentvrs.com">Docs</a>
                    <a href="https://github.com/adolfousier/agentvrs">GitHub</a>
                </div>
                <div class="nav-star">
                    <a href="https://github.com/adolfousier/agentvrs" class="star-btn" target="_blank">
                        <span class="star-icon">{"*"}</span>
                        <span class="star-count">{stars}</span>
                    </a>
                </div>
            </div>
        </nav>
    }
}

#[component]
fn Hero() -> impl IntoView {
    view! {
        <section class="hero">
            <div class="hero-bg"></div>
            <div class="hero-content">
                <div class="hero-badge">Rust-powered - Bevy-rendered</div>
                <h1 class="hero-title">
                    Multi-Agent<br/>
                    <span class="gradient-text">Orchestration</span><br/>
                    Platform
                </h1>
                <p class="hero-subtitle">
                    Deploy AI agent swarms into a shared 3D world. 
                    REST-first. Open source. Built for developers who need more than chat.
                </p>
                <div class="hero-cta">
                    <a href="#quickstart" class="btn btn-primary">Get Started</a>
                    <a href="https://docs.agentvrs.com" class="btn btn-secondary">Read Docs</a>
                </div>
                <div class="hero-cube">
                    <div class="cube">
                        <div class="cube-face cube-front"></div>
                        <div class="cube-face cube-back"></div>
                        <div class="cube-face cube-left"></div>
                        <div class="cube-face cube-right"></div>
                        <div class="cube-face cube-top"></div>
                        <div class="cube-face cube-bottom"></div>
                    </div>
                </div>
            </div>
        </section>
    }
}

#[component]
fn QuickStart() -> impl IntoView {
    view! {
        <section class="quickstart" id="quickstart">
            <div class="container">
                <h2>Get Running in Seconds</h2>
                <div class="terminal">
                    <div class="terminal-header">
                        <span class="terminal-dot red"></span>
                        <span class="terminal-dot yellow"></span>
                        <span class="terminal-dot green"></span>
                        <span class="terminal-title">Terminal</span>
                    </div>
                    <div class="terminal-body">
                        <pre><code>{"$ cargo add agentverse\n\n$ agentverse run\n\nServer running at http://127.0.0.1:18800\nCtrl+C to stop\n\nINFO agentverse: Listening on 0.0.0.0:18800\nINFO agentverse: WebSocket ready\nINFO agentverse: SQLite connected: agentverse.db"}</code></pre>
                    </div>
                </div>
            </div>
        </section>
    }
}

#[component]
fn Features() -> impl IntoView {
    view! {
        <section class="features" id="features">
            <div class="container">
                <h2>Everything You Need to Build Agent Systems</h2>
                <div class="features-grid">
                    <FeatureCard 
                        icon={"<->"} 
                        title="HTTP-Native API" 
                        desc="Every agent action is a REST call. WebSocket for real-time events. No proprietary protocols."
                    />
                    <FeatureCard 
                        icon={"[V]"} 
                        title="3D Isometric World" 
                        desc="Visualize agent state in a shared Bevy-rendered world. See your swarm think."
                    />
                    <FeatureCard 
                        icon={"[+]"} 
                        title="Tool System" 
                        desc="Agents can call external tools via HTTP. Drop in your API keys, define the schema, go."
                    />
                    <FeatureCard 
                        icon={"[~]"} 
                        title="Agent Memory" 
                        desc="Per-agent SQLite storage. Agents persist across sessions. Build long-running workflows."
                    />
                    <FeatureCard 
                        icon={"[#]"} 
                        title="Character System" 
                        desc="Load agent characters from TOML. System prompt, tools, avatar - all configurable."
                    />
                    <FeatureCard 
                        icon={"[T]"} 
                        title="TUI Mode" 
                        desc="SSH into a running server. Chat with agents, inspect state, issue commands."
                    />
                    <FeatureCard 
                        icon={"[W]"} 
                        title="Web Dashboard" 
                        desc="Live view of all agents, their positions, conversation history, and system health."
                    />
                    <FeatureCard 
                        icon={"[S]"} 
                        title="Swarm Coordination" 
                        desc="Group agents into teams. Shared context windows. Inter-agent messaging."
                    />
                </div>
            </div>
        </section>
    }
}

#[component]
fn FeatureCard(icon: &'static str, title: &'static str, desc: &'static str) -> impl IntoView {
    view! {
        <div class="feature-card">
            <div class="feature-icon">{icon}</div>
            <h3>{title}</h3>
            <p>{desc}</p>
        </div>
    }
}

#[component]
fn Integrations() -> impl IntoView {
    view! {
        <section class="integrations" id="integrations">
            <div class="container">
                <h2>Connect Any LLM</h2>
                <div class="integration-grid">
                    <span class="integration-badge">OpenAI</span>
                    <span class="integration-badge">Anthropic</span>
                    <span class="integration-badge">Ollama</span>
                    <span class="integration-badge">LM Studio</span>
                    <span class="integration-badge">Groq</span>
                    <span class="integration-badge">OpenRouter</span>
                </div>
                <div class="api-preview">
                    <div class="terminal">
                        <div class="terminal-header">
                            <span class="terminal-dot red"></span>
                            <span class="terminal-dot yellow"></span>
                            <span class="terminal-dot green"></span>
                            <span class="terminal-title">API Example</span>
                        </div>
                        <div class="terminal-body">
                            <pre><code>{"# Create an agent\ncurl -X POST http://127.0.0.1:18800/agents \\\n  -H 'Content-Type: application/json' \\\n  -d '{\"name\": \"assistant\", \"model\": \"gpt-4o\"}'\n\n# Send a message\ncurl -X POST http://127.0.0.1:18800/agents/assistant/messages \\\n  -H 'Content-Type: application/json' \\\n  -d '{\"content\": \"Deploy to production\"}'"}</code></pre>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}

#[component]
fn Testimonials() -> impl IntoView {
    view! {
        <section class="testimonials">
            <div class="container">
                <h2>Built for Developers</h2>
                <div class="testimonials-grid">
                    <div class="testimonial-card">
                        <p>"Finally a multi-agent framework that doesn't require a PhD in distributed systems."</p>
                        <div class="testimonial-author">- Sarah K., Backend Engineer</div>
                    </div>
                    <div class="testimonial-card">
                        <p>"The TUI mode is chef's kiss. I can debug agent state in real-time without leaving my terminal."</p>
                        <div class="testimonial-author">- Marcus R., DevOps Lead</div>
                    </div>
                    <div class="testimonial-card">
                        <p>"Being able to visualize agents in 3D space while they work? Mind-blowing for debugging."</p>
                        <div class="testimonial-author">- Alex T., ML Engineer</div>
                    </div>
                </div>
            </div>
        </section>
    }
}

#[component]
fn Community() -> impl IntoView {
    view! {
        <section class="community" id="community">
            <div class="container">
                <h2>Join the Community</h2>
                <div class="community-grid">
                    <a href="https://github.com/adolfousier/agentvrs" class="community-card" target="_blank">
                        <div class="community-icon">{"[G]"}</div>
                        <h3>GitHub</h3>
                        <p>Star, fork, contribute. 100% open source MIT.</p>
                    </a>
                    <a href="https://github.com/adolfousier/agentvrs/issues" class="community-card" target="_blank">
                        <div class="community-icon">{"[!]"}</div>
                        <h3>Issues</h3>
                        <p>Bug reports, feature requests, roadmap discussion.</p>
                    </a>
                    <a href="https://docs.agentvrs.com" class="community-card" target="_blank">
                        <div class="community-icon">{"[D]"}</div>
                        <h3>Documentation</h3>
                        <p>Full API docs, guides, and examples.</p>
                    </a>
                    <a href="https://crates.io/crates/agentverse" class="community-card" target="_blank">
                        <div class="community-icon">{"[C]"}</div>
                        <h3>Crate</h3>
                        <p>Published on crates.io. One cargo add away.</p>
                    </a>
                </div>
            </div>
        </section>
    }
}

#[component]
fn Footer(stars: ReadSignal<u32>) -> impl IntoView {
    view! {
        <footer class="footer">
            <div class="container">
                <div class="footer-inner">
                    <div class="footer-brand">
                        <span class="logo-icon">{"{V}"}</span>
                        <span>AgentVerse</span>
                        <p>MIT Licensed - Built with Rust</p>
                    </div>
                    <div class="footer-cta">
                        <a href="https://github.com/adolfousier/agentvrs" class="btn btn-primary" target="_blank">
                            {"* "}{stars} Stars on GitHub
                        </a>
                    </div>
                </div>
                <div class="footer-bottom">
                    <p>{"© 2024 AgentVerse. All rights reserved."}</p>
                </div>
            </div>
        </footer>
    }
}
