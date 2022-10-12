use crate::*;
use rand::prelude::*;
use webpage::{Webpage, WebpageOptions};

/// Command to generate random [`Link`]s.
///
/// Note that this command will only generate the links and won't save them in the filesystem.
///
/// This command will only available in `debug` mode (not in `release`).
///
/// So callers of this command should use the `cfg` macro for check if the caller is on `debug` mode or not.
///
/// It will only generate 10 links. If you need more, then call this command again.
///
/// # Example
///
/// Create a javascript async function in `/assets/scripts/generate.js` directory to call this command. You can also export the function to the wasm environment to call this command from wasm.
///
/// ```js
/// export async function generateAndSaveLinks() {
///     try {
///         // importing tauri apis
///         const invoke = window.__TAURI__.invoke;
///         const { writeTextFile, createDir, BaseDirectory } = window.__TAURI__.fs;
///
///         // call the command `generate` and generate and get random links
///         const links = await invoke("generate");
///
///         // create the directory
///         await createDir(".dsaver", { dir: BaseDirectory.Home, recursive: true });
///         // write the data into the file
///         let savingResult = await writeTextFile(".dsaver/links.json", JSON.stringify(links), { dir: BaseDirectory.Home, recursive: true });
///
///         console.log(savingResult);
///     } catch (e) {
///         console.error(e);
///     }
/// }
/// ```
///
/// Import that js function with wasm_bindgen
///
/// ```ignore
/// use wasm_bindgen::prelude::*;
///
/// #[wasm_bindgen(module = "/assets/scripts/generate.js")]
/// extern "C" {
///     #[wasm_bindgen(js_name = generateAndSaveLinks, catch)]
///     pub async fn generate_and_save_links() -> Result<JsValue, JsValue>;
/// }
/// ```
///
/// Now call the function from wasm environment
///
/// ```ignore
/// use wasm_bindgen_futures::spawn_local;
///
/// if cfg!(debug_assertions) {
///    spawn_local(async move {
///        generate_and_save_links().await.unwrap();
///    });
/// }
/// ```
#[tauri::command]
pub async fn generate() -> Vec<Link> {
    // list of random links. After generating each link, all will be stored in this variable.
    let mut random_links: Vec<Link> = Vec::new();

    // hardcoded urls.
    let urls = urls();
    // hardcoded tags
    let tags = tags();

    //
    for _ in 0..10 {
        // random url
        let rand_url: String = urls.choose(&mut thread_rng()).unwrap().clone();

        // random tags
        let mut rand_tags: Vec<String> = Vec::new();

        // generate random numbers of random tags
        for _ in 0..thread_rng().gen_range(1..tags.len()) {
            rand_tags.push(tags[thread_rng().gen_range(0..tags.len())].clone());
        }

        // random browser
        let browser: Browser = rand::random();

        // random priority
        let priority = thread_rng().gen_range('A'..='Z');

        // get the information about the webpage whose url was generated randomly earlier
        let req_info = Webpage::from_url(&rand_url, WebpageOptions::default()).unwrap();

        random_links.push(
            Link::new(rand_url)
                .title(req_info.html.title.unwrap_or_default())
                .tags_vec(rand_tags)
                .browser(browser)
                .priority(priority)
                .date(String::from("September 15, 2022")),
        );
    }

    random_links
}

/// Get some hardcoded urls
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

/// Get some hardcoded tags
fn tags() -> [String; 20] {
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
        "NotFound",
    ]
    .map(|t| t.to_string())
}
