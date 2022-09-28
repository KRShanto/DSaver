use crate::*;
use rand::prelude::*;
use webpage::{Webpage, WebpageOptions};

#[cfg(debug_assertions)]
#[tauri::command]
pub async fn generate() -> Vec<Link> {
    let mut random_links: Vec<Link> = Vec::new();

    let urls = urls();
    let tags = tags();

    for _ in 0..10 {
        let rand_url: String = urls.choose(&mut thread_rng()).unwrap().clone();

        let mut rand_tags: Vec<String> = Vec::new();

        for _ in 0..thread_rng().gen_range(1..tags.len()) {
            rand_tags.push(tags[thread_rng().gen_range(0..tags.len())].clone());
        }

        let browser: Browser = rand::random();

        let priority = thread_rng().gen_range('A'..='Z');

        let req_info = Webpage::from_url(&rand_url, WebpageOptions::default()).unwrap();

        random_links.push(
            Link::new(rand_url)
                .title(Some(req_info.html.title.unwrap_or_default()))
                .tags(rand_tags)
                .browser(browser)
                .priority(priority)
                .date(String::from("September 15, 2022")),
        );
    }

    random_links
}

fn urls() -> [String; 26] {
    [
        "https://www.google.com",
        "https://www.github.com/KRShanto",
        "https://www.facebook.com/KRshanto2005/",
        "https://rust-random.github.io/book/guide-start.html",
        "https://www.google.com/search?client=firefox-b-e&q=How+to+get+donation+for+my+open+source+project%3F%3F",
        "https://www.youtube.com/watch?v=yzeVMecydCE&t=384s",
        "https://www.youtube.com/watch?v=GbqSvJs-6W4&t=666s", 
        "https://www.google.com/search?client=firefox-b-e&q=fast+loop+in+rust", 
        "https://opensource.guide/getting-paid/", 
        "https://github.com/customer-stories/hzoo", 
        "https://opensource.guide/best-practices/", 
        "https://opensource.guide/leadership-and-governance/", 
        "https://www.freecodecamp.org/news/ultimate-owners-guide-to-open-source/", 
        "https://plausible.io/blog/open-source-funding", 
        "https://itsfoss.com/open-source-funding-platforms/", 
        "https://betterprogramming.pub/how-to-turn-your-open-source-project-into-a-stable-income-stream-4c46b15ed960", 
        "https://www.youtube.com/results?search_query=git+branching+strategy", 
        "https://www.svgrepo.com/", 
        "https://github.com/KRShanto/link-saver", 
        "https://www.youtube.com/results?search_query=namp+tutorial", 
        "https://www.youtube.com/watch?v=Px4WV6bvR2Y", 
        "https://www.youtube.com/watch?v=lEFTV3sX_HA", 
        "https://www.youtube.com/watch?v=7QJpkHw_RTw", 
        "https://www.youtube.com/watch?v=326SUO5HjWI", 
        "https://nextjs.org/learn/foundations/about-nextjs?utm_source=next-site&utm_medium=nav-cta&utm_campaign=next-website", 
        "https://www.google.com/search?client=firefox-b-e&q=How+to+earn+money+as+open+source+developer",
    ].map(|s| s.to_string())
}

fn tags() -> [String; 23] {
    [
        "Google",
        "Tag",
        "Website",
        "Tutorial",
        "Github",
        "Twitter",
        "Youtube",
        "Video",
        "RandomLink",
        "RandomNess",
        "Code",
        "GenerateLink",
        "Coding",
        "Cow",
        "Programming",
        "Program",
        "Short",
        "Canada",
        "Bangladesh",
        "Porn",
        "Sex",
        "Nudes",
        "NotFound",
    ]
    .map(|t| t.to_string())
}
