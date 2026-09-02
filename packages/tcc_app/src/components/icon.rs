//! Icon component

use freya::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IconType {
    // Navigation
    ChevronLeft,
    ChevronRight,
    ChevronDown,
    ChevronUp,
    Close,
    Menu,
    Home,
    Settings,
    Folder,
    File,
    
    // Account
    User,
    Users01,
    UserPlus,
    CheckCircle,
    AlertTriangle,
    Globe01,
    LinkExternal01,
    Copy01,
    Loading02,
    OnboardingAccount,
    Brand,
    
    // Actions
    Plus,
    Minus,
    Edit,
    Delete,
    Download,
    Upload,
    Refresh,
    Search,
    Filter,
    Sort,
    Play,
    Pause,
    Stop,
    
    // Status
    Success,
    Warning,
    Error,
    Info,
}

#[derive(PartialEq)]
pub struct Icon {
    icon_type: IconType,
    size: f32,
    color: Color,
}

impl Icon {
    pub fn new(icon_type: IconType) -> Self {
        Self {
            icon_type,
            size: 16.,
            color: Color::BLACK,
        }
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
}

impl Component for Icon {
    fn render(&self) -> impl IntoElement {
        let svg = match self.icon_type {
            IconType::ChevronLeft => include_str!("../assets/icons/chevron_left.svg"),
            IconType::ChevronRight => include_str!("../assets/icons/chevron_right.svg"),
            IconType::ChevronDown => include_str!("../assets/icons/chevron_down.svg"),
            IconType::ChevronUp => include_str!("../assets/icons/chevron_up.svg"),
            IconType::Close => include_str!("../assets/icons/close.svg"),
            IconType::Menu => include_str!("../assets/icons/menu.svg"),
            IconType::Home => include_str!("../assets/icons/home.svg"),
            IconType::Settings => include_str!("../assets/icons/settings.svg"),
            IconType::Folder => include_str!("../assets/icons/folder.svg"),
            IconType::File => include_str!("../assets/icons/file.svg"),
            IconType::User => include_str!("../assets/icons/user.svg"),
            IconType::Users01 => include_str!("../assets/icons/users01.svg"),
            IconType::UserPlus => include_str!("../assets/icons/user_plus.svg"),
            IconType::CheckCircle => include_str!("../assets/icons/check_circle.svg"),
            IconType::AlertTriangle => include_str!("../assets/icons/alert_triangle.svg"),
            IconType::Globe01 => include_str!("../assets/icons/globe01.svg"),
            IconType::LinkExternal01 => include_str!("../assets/icons/link_external01.svg"),
            IconType::Copy01 => include_str!("../assets/icons/copy01.svg"),
            IconType::Loading02 => include_str!("../assets/icons/loading02.svg"),
            IconType::OnboardingAccount => include_str!("../assets/icons/onboarding_account.svg"),
            IconType::Brand => include_str!("../assets/icons/brand.svg"),
            IconType::Plus => include_str!("../assets/icons/plus.svg"),
            IconType::Minus => include_str!("../assets/icons/minus.svg"),
            IconType::Edit => include_str!("../assets/icons/edit.svg"),
            IconType::Delete => include_str!("../assets/icons/delete.svg"),
            IconType::Download => include_str!("../assets/icons/download.svg"),
            IconType::Upload => include_str!("../assets/icons/upload.svg"),
            IconType::Refresh => include_str!("../assets/icons/refresh.svg"),
            IconType::Search => include_str!("../assets/icons/search.svg"),
            IconType::Filter => include_str!("../assets/icons/filter.svg"),
            IconType::Sort => include_str!("../assets/icons/sort.svg"),
            IconType::Play => include_str!("../assets/icons/play.svg"),
            IconType::Pause => include_str!("../assets/icons/pause.svg"),
            IconType::Stop => include_str!("../assets/icons/stop.svg"),
            IconType::Success => include_str!("../assets/icons/success.svg"),
            IconType::Warning => include_str!("../assets/icons/warning.svg"),
            IconType::Error => include_str!("../assets/icons/error.svg"),
            IconType::Info => include_str!("../assets/icons/info.svg"),
        };

        rect()
            .width(Size::px(self.size))
            .height(Size::px(self.size))
            .child(
                SvgViewer::new(svg).color(self.color),
            )
            .into_element()
    }
}