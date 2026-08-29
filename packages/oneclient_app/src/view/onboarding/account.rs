use freya::prelude::*;
use oneclient_auth::MinecraftAccount;

use crate::components::{Avatar, Button, Icon, IconType, TextInput, use_microsoft_login};
use crate::hooks::{
    try_default_account, use_add_offline_account, use_current_account,
    AddOfflineAccountKeys,
};
use crate::routes::Route;
use crate::theme::colors;
use crate::view::onboarding::{
    onboarding_illustration, onboarding_nav, onboarding_page, step_heading,
};

#[derive(PartialEq)]
pub struct OnboardingAccount;

impl Component for OnboardingAccount {
    fn render(&self) -> impl IntoElement {
        let account_query = use_current_account();
        let msa = use_microsoft_login();
        let add_offline = use_add_offline_account();
        let show_offline = use_state(|| false);
        let username = use_state(|| String::new());
        let offline_error = use_state(|| None::<String>);

        let account = try_default_account(&account_query);
        let has_account = account.is_some();

        let content = rect()
            .vertical()
            .width(Size::fill())
            .spacing(24.)
            .child(step_heading(
                "Account",
                "Before you continue, we require you to own a copy of Minecraft: Java Edition.",
            ))
            .child(match &account {
                Some(account) => account_preview(account).into_element(),
                None => {
                    if *show_offline.read() {
                        offline_form(
                            username,
                            offline_error,
                            add_offline,
                            show_offline,
                        ).into_element()
                    } else {
                        let start = msa.clone();
                        let show_offline_clone = show_offline.clone();
                        sign_in_options(
                            msa.pending,
                            msa.error.clone(),
                            move |_| start.start(),
                            move |_| show_offline_clone.set(true),
                        ).into_element()
                    }
                }
            })
            .into_element();

        let page = onboarding_page(
            onboarding_illustration(IconType::OnboardingAccount),
            content,
            onboarding_nav(
                Some(Route::OnboardingLanguage {}),
                Route::OnboardingBundles {},
                has_account,
            ),
        );

        rect()
            .width(Size::fill())
            .height(Size::fill())
            .child(page)
            .maybe_child(msa.popup())
    }
}

fn account_preview(account: &MinecraftAccount) -> impl IntoElement {
    rect()
        .horizontal()
        .width(Size::fill())
        .spacing(24.)
        .child(
            rect()
                .horizontal()
                .spacing(12.)
                .cross_align(Alignment::Center)
                .child(
                    Avatar::new(account.id.to_string())
                        .width(Size::px(48.))
                        .height(Size::px(48.)),
                )
                .child(
                    rect()
                        .vertical()
                        .spacing(4.)
                        .child(
                            label()
                                .text(account.username.clone())
                                .font_size(16.)
                                .font_weight(FontWeight::SEMI_BOLD)
                                .color(colors::fg_primary()),
                        )
                        .child(
                            label()
                                .text(account.id.to_string())
                                .font_size(12.)
                                .color(colors::fg_secondary()),
                        ),
                ),
        )
        .into_element()
}

fn sign_in_options(
    pending: bool,
    error: Option<String>,
    on_microsoft: impl FnMut(Event<PressEventData>) + 'static,
    on_offline: impl FnMut(Event<PressEventData>) + 'static,
) -> impl IntoElement {
    rect()
        .vertical()
        .spacing(12.)
        .cross_align(Alignment::Start)
        .child(
            Button::new()
                .primary()
                .large()
                .enabled(!pending)
                .on_press(on_microsoft)
                .child(Icon::new(IconType::Globe01).size(16.))
                .text(if pending {
                    "Signing in..."
                } else {
                    "Sign in with Microsoft"
                }),
        )
        .child(
            Button::new()
                .secondary()
                .large()
                .enabled(!pending)
                .on_press(on_offline)
                .child(Icon::new(IconType::Users01).size(16.))
                .text("Use Offline Account"),
        )
        .maybe_child(error.map(|message| {
            rect()
                .horizontal()
                .cross_align(Alignment::Center)
                .spacing(6.)
                .child(
                    Icon::new(IconType::AlertTriangle)
                        .size(13.)
                        .color(colors::danger()),
                )
                .child(label().text(message).font_size(12.).color(colors::danger()))
                .into_element()
        }))
        .into_element()
}

fn offline_form(
    username: State<String>,
    error: State<Option<String>>,
    add_offline: UseMutation<AddOfflineAccountMutation>,
    show_offline: State<bool>,
) -> impl IntoElement {
    use oneclient_auth::validate_offline_username;

    let error_text = error.read().clone();
    let on_submit = move |_| {
        let name = username.read().trim().to_string();
        if let Err(err) = validate_offline_username(&name) {
            error.set(Some(err.to_string()));
            return;
        }
        error.set(None);
        add_offline.mutate(AddOfflineAccountKeys { username: name });
        show_offline.set(false);
    };

    let on_back = move |_| {
        show_offline.set(false);
    };

    rect()
        .vertical()
        .spacing(12.)
        .cross_align(Alignment::Start)
        .child(
            TextInput::new(username)
                .placeholder("Username")
                .font_size(16.)
                .padding(8.)
                .width(Size::px(200.))
                .on_submit(on_submit.clone())
        )
        .child(
            rect()
                .horizontal()
                .spacing(12.)
                .child(
                    Button::new()
                        .secondary()
                        .on_press(on_back)
                        .text("Back")
                )
                .child(
                    Button::new()
                        .primary()
                        .on_press(on_submit)
                        .text("Add Offline Account")
                )
        )
        .maybe_child(error_text.map(|message| {
            rect()
                .horizontal()
                .cross_align(Alignment::Center)
                .spacing(6.)
                .child(
                    Icon::new(IconType::AlertTriangle)
                        .size(13.)
                        .color(colors::danger()),
                )
                .child(label().text(message).font_size(12.).color(colors::danger()))
                .into_element()
        }))
        .into_element()
}