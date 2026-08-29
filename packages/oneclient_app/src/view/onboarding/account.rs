use freya::prelude::*;
use freya::query::{MutationCapability, MutationStateData, UseMutation};
use oneclient_auth::MinecraftAccount;

use crate::components::{Avatar, Button, Icon, IconType, TextInput, use_microsoft_login};
use crate::hooks::{
    try_default_account, use_add_offline_account, use_current_account, AddOfflineAccountKeys,
    AddOfflineAccountMutation,
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
        let mut show_offline = use_state(|| true);
        let mut username = use_state(String::new);
        let mut closing_offline = use_state(|| false);

        use_side_effect(move || {
            if !*closing_offline.read() {
                return;
            }
            match &*add_offline.read().state() {
                MutationStateData::Settled { res: Ok(_), .. } => {
                    closing_offline.set(false);
                    show_offline.set(false);
                    username.set(String::new());
                }
                MutationStateData::Settled { res: Err(_), .. } => {
                    closing_offline.set(false);
                }
                _ => {}
            }
        });

        let account = try_default_account(&account_query);
        let has_account = account.is_some();

        let content = rect()
            .vertical()
            .width(Size::fill())
            .spacing(24.)
            .child(step_heading(
                "Account",
                "Enter a username to get started. You can sign in with Microsoft later.",
            ))
            .child(match &account {
                Some(account) => account_preview(account).into_element(),
                None => {
                    if *show_offline.read() {
                        offline_form(
                            username,
                            add_offline,
                            show_offline,
                            closing_offline,
                        )
                        .into_element()
                    } else {
                        let start = msa.clone();
                        let mut show_offline_clone = show_offline.clone();
                        sign_in_options(
                            msa.pending,
                            msa.error.clone(),
                            move |_| start.start(),
                            move |_| show_offline_clone.set(true),
                        )
                        .into_element()
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
    mut username: State<String>,
    add_offline: UseMutation<AddOfflineAccountMutation>,
    mut show_offline: State<bool>,
    mut closing_offline: State<bool>,
) -> impl IntoElement {
    let on_submit_button = move |_| {
        let name = username.peek().trim().to_string();
        if name.is_empty() {
            return;
        }
        add_offline.mutate(AddOfflineAccountKeys { username: name });
        closing_offline.set(true);
    };

    let on_submit_input = {
        let mut username = username.clone();
        let add_offline = add_offline.clone();
        let mut closing_offline = closing_offline.clone();
        move |_: String| {
            let name = username.peek().trim().to_string();
            if name.is_empty() {
                return;
            }
            add_offline.mutate(AddOfflineAccountKeys { username: name });
            closing_offline.set(true);
        }
    };

    let on_back = move |_| {
        show_offline.set(false);
    };

    rect()
        .vertical()
        .spacing(12.)
        .cross_align(Alignment::Start)
        .child(
            label()
                .text("Enter a username for offline play:")
                .font_size(13.)
                .color(colors::fg_secondary()),
        )
        .child(
            TextInput::new(username)
                .placeholder("Username (3-16 characters)")
                .font_size(16.)
                .on_submit(on_submit_input),
        )
        .child(
            rect()
                .horizontal()
                .spacing(12.)
                .child(
                    Button::new()
                        .secondary()
                        .on_press(on_back)
                        .text("Back"),
                )
                .child(
                    Button::new()
                        .primary()
                        .on_press(on_submit_button)
                        .text("Add Offline Account"),
                ),
        )
        .into_element()
}