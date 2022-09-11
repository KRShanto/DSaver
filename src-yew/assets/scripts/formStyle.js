export function main() {
    const TIME_TO_REMOVE = 900; // milliseconds
    // Get the .lable-input element
    const labelInput = document.getElementsByClassName('label-input');
    // Get the input inside the .label-input element
    const input = document.querySelectorAll('.label-input input');

    Array.from(input).forEach((element, i) => {

        element.addEventListener('focus', event => {
            // move the current label to the right side (ending of the input) and remove it
            // create a new label and spawn it left side and then move it to the right side (beginning of the input)
            if (event.target.value === '') {
                // Get the label inside the .lable-input element
                const label = document.getElementById(`label-${element.id}`);

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
                labelUpper.style.position = 'absolute';
                labelUpper.style.marginLeft = '-50%';
                labelUpper.style.top = '0';
                labelUpper.style.transition = '1.2s';
                labelUpper.style.opacity = '0';

                // put the label at the top inside the labelInput
                labelInput[i].prepend(labelUpper);

                setTimeout(() => {
                    // remove the current label
                    labelInput[i].removeChild(label);
                }, TIME_TO_REMOVE);

                setTimeout(() => {
                    // move the new label to the beginning of the input field
                    labelUpper.style.marginLeft = '0';
                    labelUpper.style.opacity = '1';
                }, TIME_TO_REMOVE - 700);
            }

        });

        element.addEventListener('blur', event => {
            // move the label to the left side and remove it
            // create a new label and move it from the right side
            if (event.target.value === '') {
                // get the lableUpper
                const label = document.getElementById(`label-${element.id}`);

                label.style.marginLeft = '-50%';
                label.style.opacity = '0';

                // spawn the new label
                const labelDown = document.createElement('label');

                // labelDown.textContent = label.textContent;
                labelDown.innerHTML = label.innerHTML;
                labelDown.htmlFor = label.htmlFor;
                labelDown.id = label.id;
                labelDown.className = label.className;
                labelDown.style.transition = '1.2s';
                labelDown.style.width = '120%';
                labelDown.style.marginLeft = '100%';
                labelDown.style.opacity = '0';

                // put the label at the top inside the labelInput
                labelInput[i].prepend(labelDown);

                setTimeout(() => {
                    // remove the current label
                    labelInput[i].removeChild(label);
                }, TIME_TO_REMOVE);

                setTimeout(() => {
                    labelDown.style.marginLeft = '0';
                    labelDown.style.opacity = '1';
                }, TIME_TO_REMOVE - 700);
            }
        });
    })
}
