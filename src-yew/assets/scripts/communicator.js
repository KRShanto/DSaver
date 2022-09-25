// NOTE: If you want to recive data from Rust, you need to serialize it to JSON and then in js deserialize it to JS object
// NOTE: If you want to send data to Rust, you need to serialize it to JSON and then in Rust deserialize it to Rust object
// NOTE: If you want to send data to backend, you need to serialize it to JSON string and then in backend deserialize it to Rust object via `serde_json::from_str()`

// The directory to store data from the app
const ROOT_DIR = ".link-saver";


// Store data in the file system. It will return Option<String>. If it returns None, it means success. Else it means error. 
export async function storeData(fullData) {
    const { writeTextFile, createDir, BaseDirectory } = window.__TAURI__.fs;

    try {
        // create the directory
        await createDir(ROOT_DIR, { dir: BaseDirectory.Home, recursive: true });
        // write the data into the file
        await writeTextFile(`${ROOT_DIR}/links.json`, fullData, { dir: BaseDirectory.Home, recursive: true });

        return null;
    } catch (error) {
        console.log("ERROR: ", error);
        return JSON.stringify(error);
    }

}

// Add data in the file system. It will return Option<String>. It will always return String. The string can be used by by parsing it from json to Rust structs. If this function successfully adds the data, then it will return `Link`. If any error occur, it will probably return `ErrorReporter`.
export async function addData(fullDataArg, newData) {
    const invoke = window.__TAURI__.invoke;
    // parse the json into a js array
    const fullData = JSON.parse(fullDataArg);

    try {
        // Validate the link and get new link from it
        const returnedData = await invoke("validate_link", { link: newData });
        console.log("New returned data ", returnedData);

        // push the new data to the full data
        fullData.push(returnedData);

        await storeData(JSON.stringify(fullData));

        return JSON.stringify(returnedData);
    } catch (error) {
        console.log("ERROR: ", error);
        return JSON.stringify(error);
    }

}

// Get data from the file system. It returns Option<String>. It will be None if the file is not exits or can't read the file. The string can be used by parsing it from json to Rust struct. You can parse the String to Vec<Link> if the file contains valid data
export async function getData() {
    const { readTextFile, BaseDirectory } = window.__TAURI__.fs;

    // The file may not exist, so we need to handle the error
    try {
        // if the file exists, read the data from the file
        return await readTextFile(`${ROOT_DIR}/links.json`, { dir: BaseDirectory.Home });
    } catch (error) {
        // else return null
        return null;
    }

}

// Open the browser corresponding to the the user's OS
export async function openBrowser(path, browser) {
    const invoke = window.__TAURI__.invoke;
    const { platform } = window.__TAURI__.os;

    const platformName = await platform();
    let functionName;

    if (platformName === "win32") {
        functionName = "open_browser_windows";
    } else if (platformName === "linux") {
        functionName = "open_browser_linux";
    } else {
        functionName = "open_browser_macos";
    }

    const result = await invoke(functionName, { path, browser });

    return JSON.stringify(result);
}

// Generate some random links
export async function generateLink() {
    const invoke = window.__TAURI__.invoke;

    const links = await invoke("generate");

    let result = await storeData(JSON.stringify(links));

    if (result === null) {
        console.log("Random links are created");
    }

}