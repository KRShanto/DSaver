// NOTE: If you want to recive data from Rust, you need to serialize it to JSON and then in js deserialize it to JS object
// NOTE: If you want to send data to Rust, you need to serialize it to JSON and then in Rust deserialize it to Rust object
// NOTE: If you want to send data to backend, you need to serialize it to JSON string and then in backend deserialize it to Rust object via `serde_json::from_str()`

// The directory to store data from the app
const ROOT_DIR = ".link-saver";

// Returns is the app running in webview or in the browser
export function isWebview() {
    if (window.__TAURI__) {
        return true;
    }
    return false;
}

// Store data in the file system or localstorage. It will return Option<String>. If it returns None, it means success. Else it means error. You can use the error by parsing it from json to Rust structs.
export async function storeData(fullData) {
    if (isWebview()) {
        // If the app runningin webview, store data in the file system
        const { writeTextFile, createDir, BaseDirectory } = window.__TAURI__.fs;

        try {
            // create the directory
            await createDir(ROOT_DIR, { dir: BaseDirectory.Home, recursive: true });
            // write the data into the file
            await writeTextFile(`${ROOT_DIR}/links.json`, JSON.stringify(fullData), { dir: BaseDirectory.Home, recursive: true });

            return null;
        } catch (error) {
            console.log("ERROR: ", error);
            return JSON.stringify(error);
        }
    } else {
        // If the app running in the browser, store data in the localstorage
        return localStorage.setItem("data", data);
    }
}

// Add data in the file system. It will return Option<String>. It will always return String. The string can be used by by parsing it from json to Rust structs. If this function successfully adds the data, then it will return `Link`. If any error occur, it will probably return `LinkSavingError`.
export async function addData(fullDataArg, newData) {
    if (isWebview()) {
        // If the app runningin webview, store data in the file system
        const invoke = window.__TAURI__.invoke;

        // parse the json into a js array
        const fullData = JSON.parse(fullDataArg);

        try {
            // Validate the link and get new link from it
            const returnedData = await invoke("validate_link", { link: newData });
            console.log("New returned data ", returnedData);

            // push the new data to the full data
            fullData.push(returnedData);

            await storeData(fullData);

            return JSON.stringify(returnedData);
        } catch (error) {
            console.log("ERROR: ", error);
            return JSON.stringify(error);
        }
    } else {
        // If the app running in the browser, store data in the localstorage
        return localStorage.setItem("data", data);
    }
}

// Get data from the file system or localstorage. It returns Option<String>. It will be None if the file is not exits or can't read the file. The string can be used by parsing it from json to Rust struct. You can parse the String to Vec<Link> if the file contains valid data
export async function getData() {
    if (isWebview()) {
        // If the app runningin webview, get data from the file system
        const { readTextFile, BaseDirectory } = window.__TAURI__.fs;

        // The file may not exist, so we need to handle the error
        try {
            // if the file exists, read the data from the file
            return await readTextFile(`${ROOT_DIR}/links.json`, { dir: BaseDirectory.Home });
        } catch (error) {
            // else return null
            return null;
        }
    } else {
        // If the app running in the browser, get data from the localstorage
        return localStorage.getItem("data");
    }
}