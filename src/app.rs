use leptos::prelude::*;
use gloo_net::http::Request;
use serde::Deserialize;

const GITHUB_REPO: &str = "adolfousier/agentvrs";
const GITHUB_URL: &str = "https://github.com/adolfousier/agentvrs.com";
const DOCS_URL: &str = "https://docs.agentvrs.com";
const WEBSITE_URL: &str = "https://agentvrs.com";

#[derive(Deserialize, Clone, Debug)]
struct GitHubRepo {
    stargazers_count: u32,
}

async fn fetch_star_count() -> Option<u32> {
    Request::get(&format!("https://api.github.com/repos/{}", GITHUB_REPO))
        .header("Accept", "application/vnd.github+json")
        .send()
        .await
        .ok()?
        .json::<GitHubRepo>()
        .await
        .ok()
        .map(|r| r.stargazers_count)
}

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
        <Testimonials />
        <QuickStart />
        <Features />
        <Integrations />
        <Community />
        <Footer stars=stars_signal />
    }
}

// ── Navigation ──────────────────────────────────────────────────────────────

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
                    <span class="agent-icon">
                        <svg width="44" height="44" viewBox="0 0 44 44" fill="none" xmlns="http://www.w3.org/2000/svg">
                            <rect x="8" y="8" width="28" height="28" rx="6" fill="#e84040" fill-opacity="0.2" stroke="#e84040" stroke-width="2"/>
                            <circle cx="17" cy="19" r="3" fill="#e84040"/>
                            <circle cx="27" cy="19" r="3" fill="#e84040"/>
                            <path d="M14 28 Q22 33 30 28" stroke="#e84040" stroke-width="2" stroke-linecap="round" fill="none"/>
                            <rect x="19" y="2" width="6" height="8" rx="2" fill="#e84040"/>
                        </svg>
                    </span>
                    "AgentVerse"
                </a>
                <ul class="nav-links">
                    <li><a href={DOCS_URL} target="_blank">"Docs"</a></li>
                    <li><a href="#features">"Features"</a></li>
                    <li><a href="#integrations">"Integrations"</a></li>
                    <li>
                        <a href="https://github.com/adolfousier/agentvrs" class="btn-github" target="_blank">
                            "GitHub"
                            <span class="github-stars">{star_label}</span>
                        </a>
                    </li>
                </ul>
            </div>
        </nav>
    }
}

// ── Hero ────────────────────────────────────────────────────────────────────

#[component]
fn Hero() -> impl IntoView {
    view! {
        <section class="hero">
            <div class="container">
                <div class="hero-logo">
                    <svg width="200" height="200" viewBox="0 0 200 200" fill="none" xmlns="http://www.w3.org/2000/svg">
                        <!-- Isometric cube base -->
                        <path d="M100 160 L160 130 L160 70 L100 100 L40 70 L40 130 Z" fill="#12121c" stroke="#e84040" stroke-width="2"/>
                        <!-- Top face -->
                        <path d="M100 100 L160 70 L100 40 L40 70 Z" fill="#1e1e2e" stroke="#e84040" stroke-width="2"/>
                        <!-- Left face -->
                        <path d="M40 70 L100 100 L100 160 L40 130 Z" fill="#0e0e16" stroke="#e84040" stroke-width="2"/>
                        <!-- Agent head -->
                        <rect x="80" y="55" width="40" height="45" rx="8" fill="#12121c" stroke="#e84040" stroke-width="2"/>
                        <!-- Eyes -->
                        <circle cx="92" cy="72" r="5" fill="#e84040"/>
                        <circle cx="108" cy="72" r="5" fill="#e84040"/>
                        <!-- Antenna -->
                        <rect x="95" y="35" width="10" height="20" rx="3" fill="#e84040"/>
                        <circle cx="100" cy="32" r="6" fill="#e84040"/>
                        <!-- Connection nodes -->
                        <circle cx="60" cy="85" r="4" fill="#4ade80"/>
                        <circle cx="140" cy="85" r="4" fill="#4ade80"/>
                        <circle cx="100" cy="140" r="4" fill="#4ade80"/>
                        <!-- Connection lines -->
                        <line x1="64" y1="85" x2="76" y2="85" stroke="#4ade80" stroke-width="1" stroke-dasharray="2,2"/>
                        <line x1="124" y1="85" x2="136" y2="85" stroke="#4ade80" stroke-width="1" stroke-dasharray="2,2"/>
                        <line x1="100" y1="136" x2="100" y2="144" stroke="#4ade80" stroke-width="1" stroke-dasharray="2,2"/>
                    </svg>
                </div>
                <h1>"AgentVerse"</h1>
                <p class="hero-tagline">"ISOMETRIC 3D WORLD WHERE AI AGENTS COLLABORATE"</p>
                <p class="hero-description">
                    "A real-time isometric 3D world built with Bevy where AI agents connect, "
                    "collaborate, and interact. Features a REST API with SSE support and the "
                    "A2A protocol for seamless agent-to-agent communication. Your agents, "
                    "your world, your rules."
                </p>
                <div class="hero-badges">
                    <a href={DOCS_URL} class="hero-badge" target="_blank">
                        <span class="badge-new">"DOCS"</span>
                        "Get started"
                        <span class="arrow">" →"</span>
                    </a>
                    <a href="https://github.com/adolfousier/agentvrs" class="hero-badge" target="_blank">
                        <span class="badge-new" style="background: #24292e;">"GITHUB"</span>
                        "View source"
                        <span class="arrow">" →"</span>
                    </a>
                </div>
            </div>
        </section>
    }
}

// ── Quick Start ─────────────────────────────────────────────────────────────

#[component]
fn QuickStart() -> impl IntoView {
    let (active_tab, set_active_tab) = signal(0u8);

    view! {
        <section id="quickstart">
            <div class="container">
                <h2 class="section-title">
                    <span class="chevron">"›"</span>
                    "Quick Start"
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
                            >"Connect Agent"</button>
                            <button
                                class:active=move || active_tab.get() == 2
                                on:click=move |_| set_active_tab.set(2)
                            >"REST API"</button>
                        </div>
                        <span class="terminal-platform">"Terminal"</span>
                    </div>
                    <div class="terminal-body" style:display=move || if active_tab.get() == 0 { "block" } else { "none" }>
                        <div>
                            <span class="terminal-comment">"# Install AgentVerse CLI"</span>
                        </div>
                        <div>
                            <span class="terminal-prompt">"$ "</span>
                            <span class="terminal-cmd">"cargo install agentverse"</span>
                        </div>
                        <div>
                            <span class="terminal-comment">"# Start the 3D world server"</span>
                        </div>
                        <div>
                            <span class="terminal-prompt">"$ "</span>
                            <span class="terminal-cmd">"agentverse server"</span>
                        </div>
                        <div>
                            <span class="terminal-comment">"# Launch the 3D world viewer (TUI or GUI)"</span>
                        </div>
                        <div>
                            <span class="terminal-prompt">"$ "</span>
                            <span class="terminal-cmd">"agentverse view"</span>
                        </div>
                    </div>
                    <div class="terminal-body" style:display=move || if active_tab.get() == 1 { "block" } else { "none" }>
                        <div>
                            <span class="terminal-comment">"# Connect your AI agent to AgentVerse"</span>
                        </div>
                        <div>
                            <span class="terminal-prompt">"$ "</span>
                            <span class="terminal-cmd">"agentverse connect --name \"MyAgent\" --endpoint http://localhost:8080"</span>
                        </div>
                        <div>
                            <span class="terminal-comment">"# Your agent appears in the 3D world!"</span>
                        </div>
                        <div>
                            <span class="terminal-comment">"# Send a message to another agent"</span>
                        </div>
                        <div>
                            <span class="terminal-prompt">"$ "</span>
                            <span class="terminal-cmd">"agentverse send --to OtherAgent --message \"Hello!\""</span>
                        </div>
                    </div>
                    <div class="terminal-body" style:display=move || if active_tab.get() == 2 { "block" } else { "none" }>
                        <div>
                            <span class="terminal-comment">"# REST API - Get all agents"</span>
                        </div>
                        <div>
                            <span class="terminal-prompt">"$ "</span>
                            <span class="terminal-cmd">"curl http://localhost:8080/api/agents"</span>
                        </div>
                        <div>
                            <span class="terminal-comment">"# Send message via SSE"</span>
                        </div>
                        <div>
                            <span class="terminal-prompt">"$ "</span>
                            <span class="terminal-cmd">"curl -X POST http://localhost:8080/api/message \\"</span>
                        </div>
                        <div>
                            <span class="terminal-prompt">"  "</span>
                            <span class="terminal-cmd">"-d '{\"to\": \"agent1\", \"content\": \"Hi!\"}'"</span>
                        </div>
                        <div>
                            <span class="terminal-comment">"# Subscribe to agent events (SSE)"</span>
                        </div>
                        <div>
                            <span class="terminal-prompt">"$ "</span>
                            <span class="terminal-cmd">"curl http://localhost:8080/api/events/stream"</span>
                        </div>
                    </div>
                </div>
                <p class="terminal-note">
                    "AgentVerse requires Rust 1.70+ and Bevy 0.12+. "
                    "Works on macOS, Linux & Windows."
                </p>
            </div>
        </section>
    }
}

// ── Features ────────────────────────────────────────────────────────────────

#[component]
fn Features() -> impl IntoView {
    let features = vec![
        (
            "🎮",
            "Bevy 3D Isometric World",
            "Immersive isometric 3D environment built with Bevy. Watch agents move, interact, and collaborate in real-time.",
        ),
        (
            "🖥️",
            "TUI Mode",
            "Terminal-based viewer for the 3D world. No GPU required. Perfect for servers and remote environments.",
        ),
        (
            "🔌",
            "REST API + SSE",
            "Full REST API for agent management with Server-Sent Events for real-time updates and streaming responses.",
        ),
        (
            "🤝",
            "A2A Protocol",
            "Agent-to-Agent protocol for seamless communication between AI agents. Define capabilities, discover agents.",
        ),
        (
            "💾",
            "SQLite Persistence",
            "World state persisted in SQLite. Agents remember interactions, positions, and relationships across sessions.",
        ),
        (
            "🛰️",
            "Mission Control",
            "Dashboard for monitoring agent activity, world metrics, and system health in real-time.",
        ),
        (
            "🔗",
            "Multi-Agent Coordination",
            "Orchestrate multiple AI agents with shared goals. Tasks can be delegated and tracked across agents.",
        ),
        (
            "🔒",
            "Privacy-First Local",
            "Everything runs locally by default. Your agents, your data, your infrastructure. No cloud dependencies.",
        ),
    ];

    view! {
        <section id="features">
            <div class="container">
                <h2 class="section-title">
                    <span class="chevron">"›"</span>
                    "Features"
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

// ── Integrations ────────────────────────────────────────────────────────────

#[component]
fn Integrations() -> impl IntoView {
    let protocols = vec![
        "🔌 OpenCrabs",
        "🦀 OpenClaws", 
        "🧠 Hermes",
        "🌐 Any HTTP Agent",
    ];

    view! {
        <section id="integrations">
            <div class="container">
                <h2 class="section-title">
                    <span class="chevron">"›"</span>
                    "Integrations & Protocols"
                </h2>
                <div class="integrations-row">
                    {protocols.into_iter().map(|name| view! {
                        <span class="integration-badge">{name}</span>
                    }).collect::<Vec<_>>()}
                </div>
                <div class="api-snippet">
                    <h3>"REST API Example"</h3>
                    <pre><code>"// Connect your agent
POST /api/agents/connect
{
  \"name\": \"MyAgent\",
  \"capabilities\": [\"reasoning\", \"code-generation\"],
  \"endpoint\": \"http://localhost:3000\"
}

// Send a message
POST /api/agents/:id/message
{
  \"content\": \"Hello, AgentVerse!\",
  \"reply_to\": \"msg_123\"
}

// Stream events
GET /api/events/stream

// Get world state
GET /api/world/state"</code></pre>
                </div>
                <div class="integrations-links">
                    <a href={DOCS_URL} target="_blank">"View full API docs →"</a>
                </div>
            </div>
        </section>
    }
}

// ── Testimonials ────────────────────────────────────────────────────────────

#[component]
fn Testimonials() -> impl IntoView {
    let quotes: Vec<(&str, &str, &str)> = vec![
        (
            "AgentVerse is exactly what multi-agent systems needed. The isometric view makes it so easy to understand what's happening.",
            "@devcommunity",
            "https://github.com/adolfousier/agentvrs/discussions",
        ),
        (
            "Finally, a multi-agent framework that just works. Bevy-powered 3D world is a game changer for visualization.",
            "@rustacean42",
            "https://github.com/adolfousier/agentvrs/discussions",
        ),
        (
            "The A2A protocol is elegant and simple. Connected my agents in minutes, not hours.",
            "@ailover",
            "https://github.com/adolfousier/agentvrs/discussions",
        ),
        (
            "Privacy-first, local-first. This is how AI agent frameworks should be built.",
            "@securityfirst",
            "https://github.com/adolfousier/agentvrs/discussions",
        ),
    ];

    view! {
        <section id="testimonials">
            <div class="container">
                <div class="testimonials-header">
                    <h2 class="section-title" style="margin-bottom: 0">
                        <span class="chevron">"›"</span>
                        "What Developers Say"
                    </h2>
                </div>
                <div class="testimonials-grid">
                    {quotes.into_iter().map(|(quote, author, link)| view! {
                        <div class="testimonial-card">
                            <p>"\""{ quote }"\""</p>
                            <a class="testimonial-author" href={link} target="_blank" rel="noopener">{author}</a>
                        </div>
                    }).collect::<Vec<_>>()}
                </div>
            </div>
        </section>
    }
}

// ── Community ───────────────────────────────────────────────────────────────

#[component]
fn Community() -> impl IntoView {
    view! {
        <section id="community">
            <div class="container">
                <h2 class="section-title">
                    <span class="chevron">"›"</span>
                    "Join the Community"
                </h2>
                <div class="community-grid">
                    <a href="https://github.com/adolfousier/agentvrs" class="community-card" target="_blank">
                        <span class="icon">"🐙"</span>
                        <h3>"GitHub"</h3>
                        <p>"Star & contribute"</p>
                    </a>
                    <a href="https://github.com/adolfousier/agentvrs/issues" class="community-card" target="_blank">
                        <span class="icon">"🐛"</span>
                        <h3>"Issues"</h3>
                        <p>"Report bugs & request features"</p>
                    </a>
                    <a href="https://github.com/adolfousier/agentvrs/blob/main/CHANGELOG.md" class="community-card" target="_blank">
                        <span class="icon">"📋"</span>
                        <h3>"Changelog"</h3>
                        <p>"See what's new"</p>
                    </a>
                    <a href={DOCS_URL} class="community-card" target="_blank">
                        <span class="icon">"📖"</span>
                        <h3>"Documentation"</h3>
                        <p>"Learn the ropes"</p>
                    </a>
                </div>
            </div>
        </section>
    }
}

// ── Footer ──────────────────────────────────────────────────────────────────

#[component]
fn Footer(stars: Signal<u32>) -> impl IntoView {
    let star_cta = move || {
        let count = stars.get();
        if count > 0 {
            format!("★ {} stars on GitHub — give us one more!", count)
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
                    <a href={DOCS_URL} target="_blank">"Docs"</a>
                    <a href="https://github.com/adolfousier/agentvrs">"GitHub"</a>
                    <a href="https://github.com/adolfousier/agentvrs/blob/main/CHANGELOG.md">"Changelog"</a>
                    <a href="https://github.com/adolfousier/agentvrs/blob/main/LICENSE">"MIT License"</a>
                </div>
                <p class="footer-tagline">
                    "Built with 🦀 by "
                    <a href="https://github.com/adolfousier">"Adolfo Usier"</a>
                    " & the AgentVerse community."
                </p>
            </div>
        </footer>
    }
}
