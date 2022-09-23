// A file of quick js functions that takes time in  Rust to write.

// Focus on the `#create-tags` input element
export function focusTag(inputId) {
    const tag = document.getElementById(inputId);

    tag.focus();
}

export function ifNotClicked(elementId, whatToDo) {
    document.addEventListener('click', event => {
        const element = document.getElementById(elementId);

        if (event.target != element) {
            whatToDo();
        }
    });
}

export function downOpacity(elementId) {
    const element = document.getElementById(elementId);

    element.style.opacity = 0.3;
}

export function upOpacity(elementId) {
    const element = document.getElementById(elementId);

    element.style.opacity = 1.0;
}