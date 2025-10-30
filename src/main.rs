use axum::{response::Html, routing::get, Router};
use rand::seq::SliceRandom;
use std::{ffi::OsStr, net::SocketAddr};
use tokio::net::TcpListener;
use tower_http::{services::ServeDir, trace::TraceLayer};

const IMAGE_DIR: &str = "static/wallpapers";

#[tokio::main]
async fn main() {
    // Static images under /wallpapers
    let static_service = ServeDir::new(IMAGE_DIR);

    let app = Router::new()
        .route("/", get(gallery))
        .route("/random", get(random_wallpaper))
        .nest_service("/wallpapers", static_service)
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Wallpapers Gallery running at http://{}/", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn gallery() -> Html<String> {
    let images = match list_images().await {
        Ok(v) if !v.is_empty() => v,
        _ => {
            return Html(minimal_page(
                "Wallpapers Gallery",
                r#"<p>No images found. Add files to <code>static/wallpapers</code>.</p>"#,
            ))
        }
    };

    // Derive simple tags by filename keywords; adjust to your naming scheme

    let mut grid = String::new();
    for img in &images {
        let src = format!("/wallpapers/{}", html_escape(img));
        let tag = if img.to_ascii_lowercase().contains("bocchi") {
           
            "Bocchi The Rock"


        } else if img.to_ascii_lowercase().contains("rentagirlfriend") {

            "Rent-a-Girlfriend"

        } else if img.to_ascii_lowercase().contains("kon") {

            "K-ON"


        } else if img.to_ascii_lowercase().contains("lovelive") {

            "Love Live"
        
        } else if img.to_ascii_lowercase().contains("madoka") {

          "Madoka Magica"

        } else if img.to_ascii_lowercase().contains("frieren") {

           "Frieren"


        } else if img.to_ascii_lowercase().contains("kobayashi") {


            "Kobayashi's Dragon Maid"

        } else if img.to_ascii_lowercase().contains("lycoris") {

            "Lycoris Recoil"

        } // else if img.to_ascii_lowercase().contains("gits") {

           // "Ghost in the Shell"  

       // }
         else if img.to_ascii_lowercase().contains("overlord") {
            
            "Overlord"

        } else {
           
            "Various"
        };

        grid.push_str(&format!(
            r#"<a class="card" href="{src}" data-tag="{tag}">
                   <img src="{src}" alt="Wallpaper">
               </a>"#
        ));
    }

    let body = format!(
        r#"
        <header>
            <h1>Wallpaper Gallery</h1>
            <nav>
                <a class="btn" href="/">Gallery</a>
                <a class="btn" href="/random">🎲 Random Wallpaper</a>
            </nav>
        </header>
    
         <div class="filters">
            <button class="filter-btn" data-filter="all">All</button>
            <button class="filter-btn" data-filter="Bocchi The Rock">Bocchi The Rock!!!</button>
            <button class="filter-btn" data-filter="K-ON">K-ON</button>
            <button class="filter-btn" data-filter="Rent-a-Girlfriend">Rent-a-Girlfriend</button>
            <button class="filter-btn" data-filter="Love Live">Love Live</button>
            <button class="filter-btn" data-filter="Frieren">Frieren</button>
            <button class="filter-btn" data-filter="Kobayashi's Dragon Maid">Kobayashi's Dragon Maid</button>
            <button class="filter-btn" data-filter="Madoka Magica">Madoka Magica</button>
            <button class="filter-btn" data-filter="Lycoris Recoil">Lycoris Recoil</button>
            <button class="filter-btn" data-filter="Various">Various</button>

        </div> 

        <section class="grid">{grid}</section>

        <div id="lightbox" class="lightbox">
            <span class="close">&times;</span>
            <img class="lightbox-img" src="" alt="Wallpaper full view">
        </div>

        <script>
        document.addEventListener('DOMContentLoaded', () => {{
            const lightbox = document.getElementById('lightbox');
            const lightboxImg = document.querySelector('.lightbox-img');
            const closeBtn = document.querySelector('.close');

            // Lightbox for gallery cards
            document.querySelectorAll('.card img').forEach(img => {{
                img.addEventListener('click', e => {{
                    e.preventDefault();
                    if (lightbox && lightboxImg) {{
                        lightbox.style.display = 'flex';
                        lightboxImg.src = img.src;
                    }}
                }});
            }});

            if (closeBtn && lightbox) {{
                closeBtn.addEventListener('click', () => lightbox.style.display = 'none');
            }}
            if (lightbox) {{
                lightbox.addEventListener('click', e => {{
                    if (e.target === lightbox) lightbox.style.display = 'none';
                }});
            }}
            document.addEventListener('keydown', e => {{
                if (e.key === 'Escape' && lightbox) lightbox.style.display = 'none';
            }});

            // Tag filters
            document.querySelectorAll('.filter-btn').forEach(btn => {{
                btn.addEventListener('click', () => {{
                    const filter = btn.dataset.filter;
                    document.querySelectorAll('.card').forEach(card => {{
                        const el = card;
                        if (filter === 'all' || el.dataset.tag === filter) {{
                            el.style.display = 'block';
                        }} else {{
                            el.style.display = 'none';
                        }}
                    }});
                }});
            }});
        }});
        </script>
        "#
    );

    Html(styled_page("Wallpapers Gallery", &body))
}

async fn random_wallpaper() -> Html<String> {
    let quotes = [
        "Love is the Law, Love under Will."
    ];

    let images = match list_images().await {
        Ok(v) if !v.is_empty() => v,
        _ => {
            return Html(minimal_page(
                "Random Wallpaper",
                r#"<p>No images found. Add files to <code>static/wallpapers</code>.</p>"#,
            ))
        }
    };

    let choice = images.choose(&mut rand::thread_rng()).unwrap();
    let src = format!("/wallpapers/{}", html_escape(choice));
    let quote = quotes.choose(&mut rand::thread_rng()).unwrap();

    let body = format!(
        r#"
        <header>
            <h1>Random Wallpaper</h1>
            <nav>
                <a class="btn" href="/">← Back to Gallery</a>
                <a class="btn" href="/random">🔁 Another</a>
            </nav>
        </header>

        <section class="random">
            <img class="hero" src="{src}" alt="Wallpaper">
            <p class="quote">“{quote}”</p>
        </section>

        <div id="lightbox" class="lightbox">
            <span class="close">&times;</span>
            <img class="lightbox-img" src="" alt="Wallpaper full view">
        </div>

        <script>
        document.addEventListener('DOMContentLoaded', () => {{
            const lightbox = document.getElementById('lightbox');
            const lightboxImg = document.querySelector('.lightbox-img');
            const closeBtn = document.querySelector('.close');

            // Lightbox for hero image
            const hero = document.querySelector('.hero');
            if (hero) {{
                hero.addEventListener('click', e => {{
                    e.preventDefault();
                    if (lightbox && lightboxImg) {{
                        lightbox.style.display = 'flex';
                        lightboxImg.src = hero.getAttribute('src');
                    }}
                }});
            }}

            if (closeBtn && lightbox) {{
                closeBtn.addEventListener('click', () => lightbox.style.display = 'none');
            }}
            if (lightbox) {{
                lightbox.addEventListener('click', e => {{
                    if (e.target === lightbox) lightbox.style.display = 'none';
                }});
            }}
            document.addEventListener('keydown', e => {{
                if (e.key === 'Escape' && lightbox) lightbox.style.display = 'none';
            }});
        }});
        </script>
        "#
    );

    Html(styled_page("Random Wallpaper", &body))
}

async fn list_images() -> Result<Vec<String>, std::io::Error> {
    let mut images = Vec::new();
    let mut rd = tokio::fs::read_dir(IMAGE_DIR).await?;
    while let Some(entry) = rd.next_entry().await? {
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension().and_then(OsStr::to_str) {
                let ext = ext.to_ascii_lowercase();
                if matches!(ext.as_str(), "jpg" | "jpeg" | "png" | "gif" | "webp") {
                    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                        images.push(name.to_string());
                    }
                }
            }
        }
    }
    images.sort();
    Ok(images)
}

fn styled_page(title: &str, body: &str) -> String {
    format!(
        r#"<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<title>{title}</title>
<meta name="viewport" content="width=device-width, initial-scale=1" />
<style>
:root {{
  --bg: #0d1117; --fg: #e6edf3; --muted: #8b949e; --card: #161b22; --accent: #ffb3c7;
}}
* {{ box-sizing: border-box; }}
body {{ margin: 0; font-family: system-ui, -apple-system, Segoe UI, Roboto, Helvetica, Arial, sans-serif; background: var(--bg); color: var(--fg); }}
header {{ padding: 16px 20px; display: flex; align-items: center; justify-content: space-between; border-bottom: 1px solid #222; }}
h1 {{ margin: 0; font-size: 1.25rem; }}
nav {{ display: flex; gap: 10px; }}
.btn {{ color: var(--bg); background: var(--accent); padding: 8px 12px; border-radius: 8px; text-decoration: none; font-weight: 600; }}

.filters {{ padding: 10px; display: flex; gap: 8px; justify-content: center; }}
.filter-btn {{ background: var(--card); color: var(--fg); border: 1px solid #333; padding: 6px 12px; border-radius: 6px; cursor: pointer; }}
.filter-btn:hover {{ background: var(--accent); color: var(--bg); }}

.grid {{ display: grid; grid-template-columns: repeat(auto-fill, minmax(180px, 1fr)); gap: 12px; padding: 16px; }}
.card {{ display: block; background: var(--card); border-radius: 10px; overflow: hidden; border: 1px solid #222; }}
.card img {{ display: block; width: 100%; height: 180px; object-fit: cover; }}

.random {{ padding: 18px; display: grid; place-items: center; gap: 12px; }}
.hero {{ width: min(720px, 90vw); height: auto; border-radius: 12px; border: 1px solid #222; box-shadow: 0 8px 24px rgba(0,0,0,0.3); cursor: zoom-in; }}
.quote {{ font-size: 1.1rem; color: var(--muted); text-align: center; }}

.lightbox {{ display: none; position: fixed; top: 0; left: 0; width: 100%; height: 100%; background: rgba(0,0,0,0.85); justify-content: center; align-items: center; z-index: 1000; }}
.lightbox-img {{ max-width: 90%; max-height: 90%; border-radius: 8px; }}
.close {{ position: absolute; top: 20px; right: 30px; font-size: 2rem; color: white; cursor: pointer; }}
</style>
</head>
<body>
{body}
</body>
</html>
"#,
        title = html_escape(title),
        body = body
    )
}

fn minimal_page(title: &str, inner: &str) -> String {
    format!(
        r#"<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>{}</title></head>
<body>{}</body></html>"#,
        html_escape(title),
        inner
    )
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}


