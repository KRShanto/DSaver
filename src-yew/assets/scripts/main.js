// The directory to store data from the app
const ROOT_DIR = ".link-saver";

// Returns is the app running in webview or in the browser
export function isWebview() {
    if (window.__TAURI__) {
        return true;
    }
    return false;
}

// Store data in the file system or localstorage
export async function storeData(data) {
    if (isWebview()) {
        // If the app runningin webview, store data in the file system
        const { writeTextFile, createDir, BaseDirectory } = window.__TAURI__.fs;

        // create the directory
        await createDir(ROOT_DIR, { dir: BaseDirectory.Home, recursive: true });
        // write the data into the file
        await writeTextFile(`${ROOT_DIR}/links.json`, data, { dir: BaseDirectory.Home, recursive: true });

    } else {
        // If the app running in the browser, store data in the localstorage
        localStorage.setItem("data", data);
    }
}

// Get data from the file system or localstorage
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