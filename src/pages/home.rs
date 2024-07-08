use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn Home() -> impl IntoView {
    println!("Home page rendered");

    view! {
        <div class="container"> 
        <section class="hero is-primary">
            <div class="hero-body">
                <p class="title">
                    Software Engineer & Passionate Coder
                </p>
                <p class="subtitle">
                    Welcome to my digital space! I write about code, tech, and the occasional life hack.
                </p>
            </div>
        </section>

        <section class="section">
            <h2 class="title is-2">About Me</h2>
            <p>
                Introduce yourself, your expertise, and what makes your blog unique. Share your passion for software engineering and what readers can expect to find here.
            </p>
        </section>

        <section class="section">
            <h2 class="title is-2">Latest Blog Posts</h2>
            <div class="columns">
                // ... your blog post previews will go here ...
            </div>
        </section>
        <div class="box">
  <article class="media">
    <div class="media-left">
      <figure class="image is-64x64">
        <img src="https://bulma.io/assets/images/placeholders/128x128.png" alt="Image" />
      </figure>
    </div>
    <div class="media-content">
      <div class="content">
        <p>
          <strong>John Smith</strong> <small>@johnsmith</small>
          <small>31m</small>
          <br />
          Lorem ipsum dolor sit amet, consectetur adipiscing elit. Aenean
          efficitur sit amet massa fringilla egestas. Nullam condimentum luctus
          turpis.
        </p>
      </div>
      <nav class="level is-mobile">
        <div class="level-left">
          <a class="level-item" aria-label="reply">
            <span class="icon is-small">
              <i class="fas fa-reply" aria-hidden="true"></i>
            </span>
          </a>
          <a class="level-item" aria-label="retweet">
            <span class="icon is-small">
              <i class="fas fa-retweet" aria-hidden="true"></i>
            </span>
          </a>
          <a class="level-item" aria-label="like">
            <span class="icon is-small">
              <i class="fas fa-heart" aria-hidden="true"></i>
            </span>
          </a>
        </div>
      </nav>
    </div>
  </article>
</div>

        <section class="section">
            <h2 class="title is-2">Featured Projects</h2>
            <div class="columns">
                // ... your project showcases will go here ...
            </div>
        </section>
    </div>
    }
    
}
