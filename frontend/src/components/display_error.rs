use crate::*;

/// Display an error popup to the user
///
/// Use this if you want to show any kind of error/failure/warning to the user
///
/// This component is a popup box and it will appear if you set the [`PopupBox`] to [`PopupBox::CreateLink`].
///
/// Before you can use this component you need to set its data in [`DisplayErrorData`].
///
/// Note that if you remove its data before this component is closed, it will be panic.
///
/// When this component goes out of scope, it will make the state [`DisplayErrorData`] to be [`None`].
#[function_component(DisplayError)]
pub fn dis() -> Html {
    let data_state = use_context::<DisplayErrorData>().unwrap();
    let data = (*data_state.0).clone().unwrap();

    let error_reporter = data.error_reporter.clone();
    let options_message = data.options_message.clone();
    let options_buttons = data.options_buttons;

    // clear all the data when the component is unmounted
    use_effect_with_deps(move |_| move || data_state.0.set(None), ());

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
