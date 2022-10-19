use crate::*;

#[function_component(DisplayError)]
pub fn dis() -> Html {
    let data_state = use_context::<DisplayErrorData>().unwrap();
    let data = (*data_state.0).clone().unwrap();

    let error_reporter = data.error_reporter.clone();
    let options_message = data.options_message.clone();
    let options_buttons = data.options_buttons;

    let class = match data.class {
        DisplayErrorClass::Error => "error",
        DisplayErrorClass::Warn => "warn",
    }
    .to_string();

    html! {
        <Popup
            id="display-error"
            title={format!("Error - {}", error_reporter.error_title())}
            classes={vec![class]}
        >
            <div class="display-error">
                // error title
                // <h2>{error_reporter.error_title()}</h2>

                <div class="inner">
                    // when error
                    <div class="when-error">
                        <p>{error_reporter.when_error()}</p>
                    </div>

                    // why error
                    <div class="why-error">
                        <h4>{"Some reasons why the error occured"}</h4>
                        <ul>
                        {
                            error_reporter.why_error().iter().map(|reason| {
                                html! {
                                    <li>{reason}</li>
                                }
                            }).collect::<Html>()
                        }
                        </ul>
                    </div>

                    // how to fix
                    <div class="how-to-fix">
                        <h4>{"How to fix the error"}</h4>
                        <ul>
                        {
                            error_reporter.how_to_fix().iter().map(|how| {
                                html! {
                                    <li>{how}</li>
                                }
                            }).collect::<Html>()
                        }
                        </ul>
                    </div>

                    // actual error
                    <div class="actual-error">
                        <h4>{"The actual error"}</h4>
                        <p>{error_reporter.actual_error()}</p>
                    </div>

                    // Display the options message
                    {
                        if let Some(options_message) = options_message {
                            html! {
                                <div class="options">
                                    <h4>{"Some options you take"}</h4>
                                    <p>{options_message}</p>
                                </div>
                            }
                        } else { html! {""} }
                    }

                    // Display the buttons
                    {
                        if let Some(options_buttons) = options_buttons {
                            options_buttons.into_iter().map(|button| {
                                html! {
                                    <div class="options-buttons">
                                        <button
                                            class={classes!(
                                                match button.button_type {
                                                    DisplayErrorButtonType::Danger => "red",
                                                    DisplayErrorButtonType::Safe => "blue"
                                                }
                                            )}
                                            onclick={
                                                move |_| {
                                                    button.callback.emit(());
                                                }
                                            }
                                        >{button.name}</button>
                                    </div>
                                }
                            }).collect::<Html>()
                        } else { html! {""} }
                    }
                </div>
            </div>
        </Popup>
    }
}
