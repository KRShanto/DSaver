// A file of quick js functions that takes time in  Rust to write.

// Focus on the `#create-tags` input element
export function focusTag(inputId) {
    const tag = document.getElementById(inputId);

    tag.focus();
}

// Call the callback `whatToDo` when the user don't click on the `elementId` element.
export function ifNotClicked(elementId, whatToDo) {
    function callback(event) {
        const element = document.getElementById(elementId);
        console.log("I am calling from Bangladesh");

        if (event.target != element) {
            whatToDo();
        }
    }

    document.addEventListener('click', callback);

    return () => document.removeEventListener('click', callback);
}

// Decrease the opacity of the given id
export function downOpacity(elementId) {
    const element = document.getElementById(elementId);

    element.style.opacity = 0.3;
}

// Increase the opacity of the given id
export function upOpacity(elementId) {
    const element = document.getElementById(elementId);

    element.style.opacity = 1.0;
}

export async function initial() { }