use crate::*;

/// Props for the [`Form`] component.
#[derive(Properties, PartialEq, Clone)]
pub struct FormProps {
    /// Id of the form.
    pub id: String,
    /// Submit button's text.
    pub button_text: String,
    /// Callback function when the form is submitted.
    ///
    /// The component will call this function when the user clicks on the submit button.
    pub onclick: Callback<MouseEvent>,
    /// Children of the form.
    pub children: ChildrenRenderer<FormPropsChildren>,
}

/// Children of the [`Form`] component.
#[derive(Clone, derive_more::From, PartialEq)]
pub enum FormPropsChildren {
    InputWrapper(VChild<InputWrapper>),
    Select(VChild<Select>),
}

#[allow(clippy::from_over_into)]
impl Into<Html> for FormPropsChildren {
    fn into(self) -> Html {
        match self {
            Self::InputWrapper(child) => child.into(),
            Self::Select(child) => child.into(),
        }
    }
}

/// A component for making beautiful forms
///
/// You can use input, select, and other components inside of this component
///
/// `props` - [`FormProps`]
///
/// # Example
///
/// ```
/// use crate::focus_tag;
/// use crate::hooks::{InputType, UseInputOptions};
/// use crate::lib_components::{
///     Box, Checkbox, Form, Input, InputDiv, InputTags, InputWrapper, Label, Popup, Select,
///     SelectLabel, TagsType,
/// };
/// use weblog::{console_log, console_warn};
/// use yew::prelude::*;
///
/// // Assume a developer wants a form to join a bootcamp.
/// #[function_component(JoinBootcampForm)]
/// pub fn join_bootcamp() -> Html {
///     let name = use_state(String::new);
///     let email = use_state(String::new);
///     let phone = use_state(String::new);
///     let age = use_state(String::new);
///     let prog_language = use_state(String::new);
///     let short_desc = use_state(String::new);
///     let agree_to_terms = use_state(|| false);
///
///     let prog_language_list = vec!["Rust", "Python", "JavaScript", "C++"]
///         .into_iter()
///         .map(|s| s.to_string())
///         .collect::<Vec<String>>();
///     let short_description_list = vec![
///         "I am a beginner.",
///         "I am intermediate.",
///         "I am advanced.",
///         "I am already in a job.",
///     ]
///     .into_iter()
///     .map(|s| s.to_string())
///     .collect::<Vec<String>>();
///
///     let onclick = {
///         let name = name.clone();
///         let email = email.clone();
///         let phone = phone.clone();
///         let age = age.clone();
///         let prog_language = prog_language.clone();
///         let short_desc = short_desc.clone();
///         let agree_to_terms = agree_to_terms.clone();
///         Callback::from(move |_| {
///             if *agree_to_terms {
///                 // Do something with the values
///                 console_log!("Name: {}", (*name).clone());
///                 console_log!("Email: {}", (*email).clone());
///                 console_log!("Phone: {}", (*phone).clone());
///                 console_log!("Age: {}", (*age).clone());
///                 console_log!(
///                     "Programming language to learn: {}",
///                     (*prog_language).clone()
///                 );
///                 console_log!("Short description: {}", (*short_desc).clone());
///             } else {
///                 console_warn!("You must agree to the terms and conditions");
///             }
///         })
///     };
///
///     html! {
///         <>
///             <h1>{"Join Bootcamp"}</h1>
///             <Form id="join-bootcamp" button_text="Join" {onclick}>
///                 <InputWrapper id="name">
///                     <InputDiv>
///                         <Label text="Name"></Label>
///                         <Input value_state={name} init_focus={true}/>
///                     </InputDiv>
///                 </InputWrapper>
///
///                 <InputWrapper id="email">
///                     <InputDiv>
///                         <Label text="Email"></Label>
///                         <Input value_state={email} />
///                     </InputDiv>
///                 </InputWrapper>
///
///                 <InputWrapper id="phone">
///                     <InputDiv>
///                         <Label text="Phone"></Label>
///                         <Input value_state={phone} options={
///                             UseInputOptions::input_type(InputType::Number)
///                         } />
///                     </InputDiv>
///                 </InputWrapper>
///
///                 <InputWrapper id="age">
///                     <InputDiv>
///                         <Label text="Age"></Label>
///                         <Input value_state={age} />
///                     </InputDiv>
///                 </InputWrapper>
///
///                 <InputWrapper id="short-description">
///                     <InputDiv>
///                         <Label text="Short description"></Label>
///                         <Input value_state={short_desc.clone()} />
///                     </InputDiv>
///
///                     <InputTags
///                         label_text="Short description"
///                         id="short-description-tags"
///                         tags_values={short_description_list}
///                         tag_type={
///                             TagsType::Button(Callback::from(
///                                 move |args: (MouseEvent, String)| {
///                                     focus_tag("input-short-description");
///                                     let (_, tag) = args;
///                                     let old_value = (*short_desc).clone();
///                                     short_desc.set(old_value + " " + &tag);
///                                 }
///                             ))
///                         }
///                     />
///                 </InputWrapper>
///
///                 <Select>
///                     <SelectLabel text="Which programming language do you want to learn?" />
///                     <Box
///                         list={prog_language_list}
///                         class="programming-language-div"
///                         id="programming-language-div"
///                         value_state={prog_language}
///                     />
///                 </Select>
///
///                 <InputWrapper id="agree-terms">
///                     <Checkbox
///                         label_text="I agree to the terms and conditions"
///                         input_value_is_empty={true}
///                         disabled={agree_to_terms}
///                     />
///                 </InputWrapper>
///             </Form>
///         </>
///     }
/// }
///
/// ```
#[function_component(Form)]
pub fn form(props: &FormProps) -> Html {
    let FormProps {
        id,
        button_text,
        onclick,
        children,
    } = (*props).clone();

    html! {
        <>
            <div class={format!("form {}-form ", id)} id={format!("{}-form", id)}>

                { for children.iter() }

                <div class="option-buttons">
                    <button class="submit" {onclick}>{button_text}</button>
                </div>
            </div>
        </>
    }
}
