use crate::*;

#[function_component(DisplayError)]
pub fn dis() -> Html {
    let display_error_state = use_context::<DisplayErrorState>().unwrap();

    let data_state = use_context::<DisplayErrorData>().unwrap();
    let data = (*data_state.0).clone().unwrap();

    let error_reporter = data.error_reporter.clone();
    let options_message = data.options_message.clone();
    let options_buttons = data.options_buttons;

    html! {
        <>
        // component title
        <h1>{"Error occured"}</h1>

        // error title
        <h2>{error_reporter.error_title()}</h2>

        <br />

        // when error
        <p>{error_reporter.when_error()}</p>

        <br />

        // why error
        <h2>{"Some reasons why the error occured"}</h2>
        <ul>
        {
            error_reporter.why_error().iter().map(|reason| {
                html! {
                    <li>{reason}</li>
                }
            }).collect::<Html>()
        }
        </ul>

        <br />

        // how to fix
        <h2>{"How to fix the error"}</h2>
        <ul>
        {
            error_reporter.how_to_fix().iter().map(|how| {
                html! {
                    <li>{how}</li>
                }
            }).collect::<Html>()
        }
        </ul>

        <br />

        // actual error
        <h2>{"The actual error"}</h2>
        <p>{error_reporter.actual_error()}</p>

        <br />

        // Display the options message
        {
            if let Some(options_message) = options_message {
                html! {
                    <div class="options">
                        <h2>{"Some options you take"}</h2>
                        {options_message}
                    </div>
                }
            } else { html! {""} }
        }

        <br />

        // Display the buttons
        {
            if let Some(options_buttons) = options_buttons {
                options_buttons.into_iter().map(|button| {
                    html! {
                        <button onclick={
                            move |_| {
                                button.1.emit(());
                            }
                        }>{button.0}</button>
                    }
                }).collect::<Html>()
            } else { html! {""} }
        }

        <br />

        // hide button
        <button onclick={
            move |_| {
                // hide the component
                display_error_state.0.set(false);

                // remove all the data
                data_state.0.set(None);
            }
        }>{"X Cancel X"}</button>
        </>
    }
}
