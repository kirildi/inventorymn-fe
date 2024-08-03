#![allow(non_snake_case, unused)]

use dioxus::prelude::*;

pub fn LoginForm() -> Element {
    let login_label_common = "w-full h-12";
    let login_input_common = "w-full rounded-full h-12 my-4 py-2 px-4 text-stone-800 bg-white ";

    let mut input_value = use_signal(|| String::from(""));
    let mut input_field_type = use_signal(|| String::from("password"));
    let mut eye_closed = use_signal(|| String::from(""));
    let mut eye_open = use_signal(|| String::from("hidden"));

    let mut toggle_password = move |_| {
        let temp_type = input_field_type.to_string();

        if temp_type != String::from("text") {
            input_field_type.set(String::from("text"));
            eye_open.set(String::from(""));
            eye_closed.set(String::from("hidden"));
            return;
        }
        input_field_type.set(String::from("password"));
        eye_open.set(String::from("hidden"));
        eye_closed.set(String::from(""));
    };
    rsx!(
        div {
            class: "px-8 h-full ",
            h1 {
                class: "w-full h-12 mb-4 text-left text-2xl font-semibold",
                "Sign in"
            },
            form {
                class: "flex flex-auto flex-wrap flex-col h-80",
                div {
                    label {
                        class: "{login_label_common}",
                        r#for: "username",
                        "USERNAME"
                    },
                    input {
                        class: "{login_input_common}",
                        id: "username",
                        r#type: "text",
                        placeholder: "Username",
                        autocomplete: "on"
                    }
                },
                div {
                    label {
                        class: "{login_label_common}",
                        r#for: "password",
                        "PASSWORD"
                    },
                    div {
                        class: "relative",
                        input {
                            class: "{login_input_common} appearance-none",
                            id: "password",
                            value: "{input_value}",
                            "type": "{input_field_type}",
                            placeholder: "Password",
                            onchange: move |event| input_value.set(event.value()),
                            autocomplete: "on"
                        },
                        button {
                            r#type: "button",
                            class: "absolute inset-y-0 end-1 flex items-center z-20 px-3 cursor-pointer text-stone-500 rounded-full focus:outline-none focus:text-blue-600 ",
                            onclick: toggle_password,
                            svg {
                                class: "shrink-0 fill-white stroke-stone-900",
                                width: "1.5rem",
                                height: "1.5rem",
                                view_box: "0 0 24 24",
                                path {
                                    class: "{eye_closed}",
                                    d: "M9.88 9.88a3 3 0 1 0 4.24 4.24",
                                },
                                path {
                                    class: "{eye_closed}",
                                    d: "M10.73 5.08A10.43 10.43 0 0 1 12 5c7 0 10 7 10 7a13.16 13.16 0 0 1-1.67 2.68",
                                },
                                path {
                                    class: "{eye_closed}",
                                    d: "M6.61 6.61A13.526 13.526 0 0 0 2 12s3 7 10 7a9.74 9.74 0 0 0 5.39-1.61",
                                }
                                line {
                                    class: "{eye_closed}",
                                    x1: "2",
                                    x2: "22",
                                    y1: "2",
                                    y2 : "22"
                                }
                                path{
                                    class: "{eye_open}",
                                    d: "M2 12s3-7 10-7 10 7 10 7-3 7-10 7-10-7-10-7Z",
                                }
                                circle{
                                    class: "{eye_open}",
                                    cx: "12",
                                    cy: "12",
                                    r: "3",
                                }
                            }
                        }
                    }
                },
                div {
                    button {
                        class: "w-full h-12 mt-10 rounded-full bg-violet-800",
                        "Sign In"
                    }
                }
            }
        }
    )
}
