use yew::prelude::*;
use stylist::{style, yew::styled_component};

#[styled_component(TipForm)]
pub fn tip_form() -> Html {
    let amount = use_state(|| String::from("5.00"));
    let currency = use_state(|| String::from("USD"));
    let recipient = use_state(|| String::new());
    let course = use_state(|| String::new());

    let on_amount_change = {
        let amount = amount.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            amount.set(input.value());
        })
    };

    let on_currency_change = {
        let currency = currency.clone();
        Callback::from(move |e: Event| {
            let select: web_sys::HtmlSelectElement = e.target_unchecked_into();
            currency.set(select.value());
        })
    };

    let on_recipient_change = {
        let recipient = recipient.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            recipient.set(input.value());
        })
    };

    let on_course_change = {
        let course = course.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            course.set(input.value());
        })
    };

    let on_submit = Callback::from(|e: SubmitEvent| {
        e.prevent_default();
        // In a real implementation, this would call the tipping service
        web_sys::console::log_1(&"Tip form submitted".into());
    });

    let form_style = style!(
        r#"
        display: flex;
        flex-direction: column;
        gap: 1.5rem;
    "#
    ).unwrap();

    let field_style = style!(
        r#"
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
    "#
    ).unwrap();

    let label_style = style!(
        r#"
        font-weight: bold;
        color: var(--text);
    "#
    ).unwrap();

    let input_style = style!(
        r#"
        padding: 0.75rem;
        border: 1px solid var(--border);
        border-radius: 4px;
        background: var(--background);
        color: var(--text);
        font-size: 1rem;

        &:focus {
            outline: none;
            border-color: var(--primary);
        }
    "#
    ).unwrap();

    let button_style = style!(
        r#"
        background: var(--primary);
        color: white;
        border: none;
        padding: 1rem;
        border-radius: 4px;
        cursor: pointer;
        font-weight: bold;
        font-size: 1rem;
        margin-top: 0.5rem;

        &:hover {
            background: var(--secondary);
        }

        &:disabled {
            background: var(--text-secondary);
            cursor: not-allowed;
        }
    "#
    ).unwrap();

    html! {
        <form class={form_style} onsubmit={on_submit}>
            <div class={field_style.clone()}>
                <label class={label_style.clone()}>{"Recipient (User ID)"}</label>
                <input
                    type="text"
                    class={input_style.clone()}
                    value={(*recipient).clone()}
                    oninput={on_recipient_change}
                    placeholder="Enter recipient user ID"
                />
            </div>

            <div class={field_style.clone()}>
                <label class={label_style.clone()}>{"Amount"}</label>
                <input
                    type="number"
                    step="0.01"
                    min="0.01"
                    class={input_style.clone()}
                    value={(*amount).clone()}
                    oninput={on_amount_change}
                />
            </div>

            <div class={field_style.clone()}>
                <label class={label_style.clone()}>{"Currency"}</label>
                <select
                    class={input_style.clone()}
                    value={(*currency).clone()}
                    onchange={on_currency_change}
                >
                    <option value="USD">{"USD - US Dollar"}</option>
                    <option value="EUR">{"EUR - Euro"}</option>
                    <option value="GBP">{"GBP - British Pound"}</option>
                    <option value="JPY">{"JPY - Japanese Yen"}</option>
                </select>
            </div>

            <div class={field_style.clone()}>
                <label class={label_style.clone()}>{"Course (Optional)"}</label>
                <input
                    type="text"
                    class={input_style.clone()}
                    value={(*course).clone()}
                    oninput={on_course_change}
                    placeholder="Enter course ID (optional)"
                />
            </div>

            <button type="submit" class={button_style}>
                {"Send Tip"}
            </button>
        </form>
    }
}