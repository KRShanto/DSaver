const TIME_TO_REMOVE = 900; // milliseconds

export function handleBlurEvent(event) {
    // move the label to the left side and remove it
    // create a new label and move it from the right side
    if (event.target.value === '') {
        labelDown(event.target.id);
    }
}

export function handleFocusEvent(event) {
    // move the current label to the right side (ending of the input) and remove it
    // create a new label and spawn it left side and then move it to the right side (beginning of the input)
    if (event.target.value === '') {
        labelUp(event.target.id);


    }
}

export function labelDown(inputId) {
    const element = document.getElementById(inputId);
    const label = element.labels[0];
    const parent = element.parentElement;

    label.style.marginLeft = '-50%';
    label.style.opacity = '0';

    // spawn the new label
    const labelDown = document.createElement('label');

    // labelDown.textContent = label.textContent;
    labelDown.innerHTML = label.innerHTML;
    labelDown.htmlFor = label.htmlFor;
    labelDown.id = label.id;
    labelDown.className = label.className;
    labelDown.classList.remove('active');
    labelDown.style.transition = '1.2s';
    labelDown.style.width = '120%';
    labelDown.style.marginLeft = '100%';
    labelDown.style.opacity = '0';

    // put the label at the top inside the labelInput
    parent.prepend(labelDown);

    setTimeout(() => {
        // remove the current label
        parent.removeChild(label);
    }, TIME_TO_REMOVE);

    setTimeout(() => {
        labelDown.style.marginLeft = '0';
        labelDown.style.opacity = '1';
    }, TIME_TO_REMOVE - 700);
}

export function labelUp(inputId) {
    console.log("The id: " + inputId);
    const element = document.getElementById(inputId);
    const label = element.labels[0];
    const parent = element.parentElement;

    // move the label to the right side of the input element
    label.style.transition = '1.2s';
    label.style.width = '120%';
    label.style.marginLeft = '100%';
    label.style.opacity = '0';

    // create another label
    let labelUpper = document.createElement('label');

    // span the label to the left of the input element 
    // labelUpper.textContent = label.textContent;
    labelUpper.innerHTML = label.innerHTML;
    labelUpper.htmlFor = label.htmlFor;
    labelUpper.id = label.id;
    labelUpper.className = label.className;
    labelUpper.classList.add("active");
    labelUpper.style.position = 'absolute';
    labelUpper.style.marginLeft = '-50%';
    labelUpper.style.top = '0';
    labelUpper.style.transition = '1.2s';
    labelUpper.style.opacity = '0';

    // put the label at the top inside the labelInput
    parent.prepend(labelUpper);

    setTimeout(() => {
        // remove the current label
        parent.removeChild(label);
    }, TIME_TO_REMOVE);

    setTimeout(() => {
        // move the new label to the beginning of the input field
        labelUpper.style.marginLeft = '0';
        labelUpper.style.opacity = '1';
    }, TIME_TO_REMOVE - 700);
}