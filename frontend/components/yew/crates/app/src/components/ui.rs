//! Reusable UI components.

use yew::prelude::*;

/// A row in a dialog with label and value.
#[derive(Properties, PartialEq)]
pub struct DialogRowProps {
    pub label: AttrValue,
    pub children: Children,
}

#[function_component(DialogRow)]
pub fn dialog_row(props: &DialogRowProps) -> Html {
    html! {
        <div class="dialog-row">
            <label>{&props.label}</label>
            {props.children.clone()}
        </div>
    }
}

/// A hint row in a dialog.
#[derive(Properties, PartialEq)]
pub struct HintRowProps {
    pub message: AttrValue,
}

#[function_component(HintRow)]
pub fn hint_row(props: &HintRowProps) -> Html {
    html! {
        <div class="dialog-row hint">{&props.message}</div>
    }
}

/// Number input with increment/decrement buttons.
#[derive(Properties, PartialEq)]
pub struct NumberInputProps {
    pub value: i32,
    pub min: i32,
    pub max: i32,
    pub on_change: Callback<i32>,
    #[prop_or(false)]
    pub disabled: bool,
}

#[function_component(NumberInput)]
pub fn number_input(props: &NumberInputProps) -> Html {
    let value = props.value;
    let min = props.min;
    let max = props.max;

    let on_dec = {
        let cb = props.on_change.clone();
        Callback::from(move |_| cb.emit(value - 1))
    };
    let on_inc = {
        let cb = props.on_change.clone();
        Callback::from(move |_| cb.emit(value + 1))
    };

    html! {
        <div class="number-input-group">
            <button class="number-btn" onclick={on_dec} disabled={props.disabled || value <= min}>{"-"}</button>
            <input type="number" min={min.to_string()} max={max.to_string()} value={value.to_string()} disabled={props.disabled} />
            <button class="number-btn" onclick={on_inc} disabled={props.disabled || value >= max}>{"+"}</button>
        </div>
    }
}

/// Dialog body wrapper with title.
#[derive(Properties, PartialEq)]
pub struct DialogBodyProps {
    pub title: AttrValue,
    #[prop_or_default]
    pub class: Classes,
    pub children: Children,
}

#[function_component(DialogBody)]
pub fn dialog_body(props: &DialogBodyProps) -> Html {
    let class = classes!("dialog-body", props.class.clone());
    html! {
        <div {class}>
            <h3>{&props.title}</h3>
            {props.children.clone()}
        </div>
    }
}
