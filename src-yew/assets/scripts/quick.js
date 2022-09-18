// A file of quick js functions that takes time in  Rust to write.

// Focus on the `#create-tags` input element
export function focusTag(inputId) {
    const tag = document.getElementById(inputId);

    tag.focus();
}

export function ifNotClicked(elementId, whatToDo) {
    document.addEventListener('click', event => {
        // console.log('Target: ', event.target);

        const element = document.getElementById(elementId);

        if (event.target != element) {
            whatToDo();
        }
    });
}