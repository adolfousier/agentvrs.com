use leptos::prelude::*;
use gloo_net::http::Request;
use serde::Deserialize;

fn main() {
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let stars = LocalResource::new(|| fetch_star_count());
    let stars_signal = Signal::derive(move || {
        stars.get().flatten().unwrap_or(0)
    });

    view! {
        <Nav stars=stars_signal />
        <Hero />
        <QuickStart />
        <Features />
        <Integrations />
        <Testimonials />
        <Community />
        <Footer stars=stars_signal />
    }
}

// ── GitHub API ───────────────────────────────────────────────────────────

#[derive(Deserialize, Clone, Debug)]
struct GitHubRepo {
    stargazers_count: u32,
}

async fn fetch_star_count() -> Option<u32> {
    Request::get("https://api.github.com/repos/adolfousier/agentvrs")
        .header("Accept", "application/vnd.github+json")
        .send()
        .await
        .ok()?
        .json::<GitHubRepo>()
        .await
        .ok()
        .map(|r| r.stargazers_count)
}

// ── Navigation ───────────────────────────────────────────────────────────

#[component]
fn Nav(stars: Signal<u32>) -> impl IntoView {
    let star_label = move || {
        let count = stars.get();
        if count > 0 {
            format!(" ★ {}", count)
        } else {
            String::new()
        }
    };

    view! {
        <nav>
            <div class="container">
                <a href="/" class="nav-logo">
                    <span class="logo-icon">"◈"</span>
                    "AgentVerse"
                </a>
                <ul class="nav-links">
                    <li><a href="https://docs.agentvrs.com" target="_blank">"Docs"</a></li>
                    <li><a href="#features">"Features"</a></li>
                    <li><a href="#integrations">"Integrations"</a></li>
                    <li><a href="https://github.com/adolfousier/agentvrs" class="btn-github" target="_blank">
                        "GitHub"
                        <span class="github-stars">{star_label}</span>
                    </a></li>
                </ul>
            </div>
        </nav>
    }
}

// ── Hero ───────────────────────────────────────────────────────────────

#[component]
fn Hero() -> impl IntoView {
    view! {
        <section class="hero">
            <div class="container">
                <div class="hero-isometric">
                    <div class="iso-cube">
                        <div class="iso-face iso-top"></div>
                        <div class="iso-face iso-left"></div>
                        <div class="iso-face iso-right"></div>
                    </div>
                </div>
                <h1>"AgentVerse"</h1>
                <p class="hero-tagline">"ISOMETRIC 3D WORLD WHERE AI AGENTS COLLABORATE."</p>
                <p class="hero-description">
                    "Built with Bevy. Runs on your machine. Agents written in any language connect "
                    "over HTTP — OpenCrabs, OpenClaws, Hermes, anything. See them work together in real-time "
                    "or run it headless on a VPS. Privacy-first. No telemetry. All local."
                </p>
                <div class="hero-badges">
                    <a href="https://github.com/adolfousier/agentvrs" class="hero-badge" target="_blank">
                        <span>"★ Star on GitHub"</span>
                    </a>
                    <a href="https://docs.agentvrs.com" class="hero-badge secondary" target="_blank">
                        <span>"Read the Docs →"</span>
                    </a>
                </div>
            </div>
        </section>
    }
}

// ── Quick Start ─────────────────────────────────────────────────────────

#[component]
fn QuickStart() -> impl IntoView {
    let (active_tab, set_active_tab) = signal(0u8);

    view! {
        <section id="quickstart">
            <div class="container">
                <h2 class="section-title">
                    <span class="chevron">"›"</span>
                    "Get Started"
                </h2>
                <div class="terminal">
                    <div class="terminal-header">
                        <div class="terminal-dots">
                            <span class="red"></span>
                            <span class="yellow"></span>
                            <span class="green"></span>
                        </div>
                        <div class="terminal-tabs">
                            <button
                                class:active=move || active_tab.get() == 0
                                on:click=move |_| set_active_tab.set(0)
                            >"Install"</button>
                            <button
                                class:active=move || active_tab.get() == 1
                                on:click=move |_| set_active_tab.set(1)
                            >"Connect"</button>
                        </div>
                        <span class="terminal-platform">"macOS / Linux / Windows"</span>
                    </div>
                    <div class="terminal-body" style:display=move || if active_tab.get() == 0 { "block" } else { "none" }>
                        <div>
                            <span class="terminal-comment">"# Install via cargo (Rust required)"</span>
                        </div>
                        <div>
                            <span class="terminal-prompt">"$ "</span>
                            <span class="terminal-cmd">"cargo install agentverse"</span>
                        </div>
                        <div>
                            <span class="terminal-prompt">"$ "</span>
                            <span class="terminal-cmd">"agentverse"</span>
                        </div>
                        <div>
                            <span class="terminal-comment">"# Or build from source"</span>
                        </div>
                        <div>
                            <span class="terminal-prompt">"$ "</span>
                            <span class="terminal-cmd">"git clone https://github.com/adolfousier/agentvrs && cd agentvrs"</span>
                        </div>
                        <div>
                            <span class="terminal-prompt">"$ "</span>
                            <span class="terminal-cmd">"cargo build --release && ./target/release/agentverse"</span>
                        </div>
                    </div>
                    <div class="terminal-body" style:display=move || if active_tab.get() == 1 { "block" } else { "none" }>
                        <div>
                            <span class="terminal-comment">"# Connect your first agent (any language)"</span>
                        </div>
                        <div>
                            <span class="terminal-prompt">"$ "</span>
                            <span class="terminal-cmd">"curl -X POST http://127.0.0.1:18800/agents/connect \\"</span>
                        </div>
                        <div>
                            <span class="terminal-prompt">"  "</span>
                            <span class="terminal-cmd">"-H 'Content-Type: application/json' \\"</span>
                        </div>
                        <div>
                            <span class="terminal-prompt">"  "</span>
                            <span class="terminal-cmd">"-d '{\"name\":\"my-agent\"}'"</span>
                        </div>
                        <div>
                            <span class="terminal-comment">"# OpenCrabs connects natively via A2A or HTTP"</span>
                        </div>
                        <div>
                            <span class="terminal-prompt">"$ "</span>
                            <span class="terminal-cmd">"opencrabs a2a connect http://127.0.0.1:18800"</span>
                        </div>
                    </div>
                </div>
                <p class="terminal-note">
                    "Works 3D on desktop (GPU) or headless on any server (TUI mode: <code>agentverse --tui</code>)."
                </p>
            </div>
        </section>
    }
}

// ── Features ─────────────────────────────────────────────────────────────

#[component]
fn Features() -> impl IntoView {
    let features = vec![
        (
            "◈",
            "Bevy 3D Isometric World",
            "Isometric office world with desks, break room, gym, arcade, server room. Voxel agents with walking animations and speech bubbles.",
        ),
        (
            "▣",
            "TUI Mode",
            "Full terminal UI on any VPS or headless server. ASCII world, agent labels, and full Mission Control dashboard.",
        ),
        (
            "⚡",
            "REST API + SSE",
            "Full REST API on 127.0.0.1:18800. Real-time events via SSE. JSON error responses. API key auth.",
        ),
        (
            "🔗",
            "A2A Protocol",
            "Wire-compatible A2A client for OpenCrabs agents. Connect natively or over HTTP from any language.",
        ),
        (
            "🗄️",
            "SQLite Persistence",
            "Agents, messages, activity, tasks, heartbeats — all survive restarts.",
        ),
        (
            "🛰️",
            "Mission Control",
            "Press M for full dashboard. See all agents, activity feeds, task lists, and connection health in one view.",
        ),
        (
            "🤖",
            "Multi-Agent Coordination",
            "Move agents, set goals, send messages, track tasks. Control everything from the API or the dashboard.",
        ),
        (
            "🔒",
            "Privacy-First Local",
            "Runs entirely on your machine. No telemetry. No cloud. Your agents, your world.",
        ),
    ];

    view! {
        <section id="features">
            <div class="container">
                <h2 class="section-title">
                    <span class="chevron">"›"</span>
                    "What It Does"
                </h2>
                <div class="features-grid">
                    {features.into_iter().map(|(icon, title, desc)| view! {
                        <div class="feature-card">
                            <span class="feature-icon">{icon}</span>
                            <h3>{title}</h3>
                            <p>{desc}</p>
                        </div>
                    }).collect::<Vec<_>>()}
                </div>
            </div>
        </section>
    }
}

// ── Integrations ────────────────────────────────────────────────────────

#[component]
fn Integrations() -> impl IntoView {
    let agents = vec!["OpenCrabs", "OpenClaws", "Hermes", "Any HTTP Agent"];
    let apis = vec!["REST API", "SSE Events", "A2A Protocol", "Agent Inbox", "Mission Control"];

    view! {
        <section id="integrations">
            <div class="container">
                <h2 class="section-title">
                    <span class="chevron">"›"</span>
                    "Any Agent. Any Language."
                </h2>
                <div class="integrations-row">
                    {agents.into_iter().map(|name| view! {
                        <span class="integration-badge">{name}</span>
                    }).collect::<Vec<_>>()}
                </div>
                <div class="integrations-row">
                    {apis.into_iter().map(|name| view! {
                        <span class="integration-badge secondary">{name}</span>
                    }).collect::<Vec<_>>()}
                </div>
                <div class="api-preview">
                    <div class="terminal" style="margin: 0">
                        <div class="terminal-header">
                            <div class="terminal-dots">
                                <span class="red"></span>
                                <span class="yellow"></span>
                                <span class="green"></span>
                            </div>
                            <span class="terminal-platform">"REST API — 127.0.0.1:18800"</span>
                        </div>
                        <div class="terminal-body" style="display: block">
                            <div>
                                <span class="terminal-comment">"# Connect an agent"</span>
                            </div>
                            <div>
                                <span class="terminal-prompt">"POST "</span>
                                <span class="terminal-cmd">"/agents/connect"</span>
                            </div>
                            <div>
                                <span class="terminal-comment">"# Move agent, send message, set goal"</span>
                            </div>
                            <div>
                                <span class="terminal-prompt">"POST "</span>
                                <span class="terminal-cmd">"/agents/{id}/move  /message  /goal"</span>
                            </div>
                            <div>
                                <span class="terminal-comment">"# Monitor from Mission Control"</span>
                            </div>
                            <div>
                                <span class="terminal-prompt">"GET "</span>
                                <span class="terminal-cmd">"/agents/{id}/dashboard"</span>
                            </div>
                        </div>
                    </div>
                </div>
                <div class="integrations-links">
                    <a href="https://docs.agentvrs.com" target="_blank">"Full API reference →"</a>
                </div>
            </div>
        </section>
    }
}

// ── Testimonials ───────────────────────────────────────────────────────

#[component]
fn Testimonials() -> impl IntoView {
    let quotes: Vec<(&str, &str)> = vec![
        (
            "Finally a way to actually see what my agents are doing. AgentVerse gives me the observability I've been missing — it's like having a mission control for my whole agent swarm.",
            "A dev building autonomous coding agents",
        ),
        (
            "The TUI mode on my VPS is a game changer. I can run the full world on a $6 server and SSH in to check on my agents. No GPU needed.",
            "A researcher running multi-agent simulations",
        ),
        (
            "OpenCrabs + AgentVerse = the dream team. My crabs live in the same world now. They can see each other work, message each other, coordinate. It's like building a tiny digital office.",
            "An AI hobbyist",
        ),
    ];

    view! {
        <section id="testimonials">
            <div class="container">
                <h2 class="section-title">
                    <span class="chevron">"›"</span>
                    "What People Build With It"
                </h2>
                <div class="testimonials-grid">
                    {quotes.into_iter().map(|(quote, author)| view! {
                        <div class="testimonial-card">
                            <p>"\""{ quote }"\""</p>
                            <span class="testimonial-author">{"— "}{author}</span>
                        </div>
                    }).collect::<Vec<_>>()}
                </div>
            </div>
        </section>
    }
}

// ── Community ───────────────────────────────────────────────────────────

#[component]
fn Community() -> impl IntoView {
    view! {
        <section id="community">
            <div class="container">
                <h2 class="section-title">
                    <span class="chevron">"›"</span>
                    "Join the Verse"
                </h2>
                <div class="community-grid">
                    <a href="https://github.com/adolfousier/agentvrs" class="community-card" target="_blank">
                        <span class="icon">"◈"</span>
                        <h3>"GitHub"</h3>
                        <p>"Star, fork, contribute"</p>
                    </a>
                    <a href="https://github.com/adolfousier/agentvrs/issues" class="community-card" target="_blank">
                        <span class="icon">"◉"</span>
                        <h3>"Issues"</h3>
                        <p>"Bug reports & feature requests"</p>
                    </a>
                    <a href="https://docs.agentvrs.com" class="community-card" target="_blank">
                        <span class="icon">"▤"</span>
                        <h3>"Documentation"</h3>
                        <p>"Full API reference & guides"</p>
                    </a>
                    <a href="https://crates.io/crates/agentverse" class="community-card" target="_blank">
                        <span class="icon">"◆"</span>
                        <h3>"Crate"</h3>
                        <p>"cargo install agentverse"</p>
                    </a>
                </div>
            </div>
        </section>
    }
}

// ── Footer ───────────────────────────────────────────────────────────────

#[component]
fn Footer(stars: Signal<u32>) -> impl IntoView {
    let star_cta = move || {
        let count = stars.get();
        if count > 0 {
            format!("★ {} stars on GitHub", count)
        } else {
            "★ Star us on GitHub!".to_string()
        }
    };

    view! {
        <footer>
            <div class="container">
                <a href="https://github.com/adolfousier/agentvrs" class="footer-star-cta" target="_blank">
                    {star_cta}
                </a>
                <div class="footer-links">
                    <a href="https://docs.agentvrs.com" target="_blank">"Docs"</a>
                    <a href="https://github.com/adolfousier/agentvrs">"GitHub"</a>
                    <a href="https://crates.io/crates/agentverse">"Crate"</a>
                    <a href="https://github.com/adolfousier/agentvrs/blob/main/LICENSE">"MIT License"</a>
                </div>
                <p class="footer-tagline">
                    "Built with Rust and Bevy by "
                    <a href="https://github.com/adolfousier">"Adolfo Usier"</a>
                    "."
                </p>
            </div>
        </footer>
    }
}
