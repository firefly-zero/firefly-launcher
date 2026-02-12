use firefly_rust::Language;

pub enum Message {
    AuthorID,
    AppID,
    AuthorName,
    AppName,
    RomSize,
    DataSize,

    Launches,
    Installed,
    Updated,

    Back,
    Exit,
    Stats,
    Achievements,
    Scoreboards,
    ViewInCatalog,
    Remove,
}

impl Message {
    pub const fn translate(&self, lang: Language) -> &'static str {
        match lang {
            Language::English => self.translate_english(),
            Language::Dutch => self.translate_dutch(),
            Language::French => self.translate_french(),
            Language::German => self.translate_german(),
            Language::Italian => self.translate_italian(),
            Language::Polish => self.translate_polish(),
            Language::Russian => self.translate_russian(),
            Language::Spanish => self.translate_spanish(),
            Language::Swedish => self.translate_swedish(),
            Language::Turkish => self.translate_turkish(),
            Language::Ukrainian => self.translate_ukrainian(),
            Language::TokiPona => self.translate_toki_pona(),
        }
    }

    const fn translate_english(&self) -> &'static str {
        match self {
            Self::AuthorID => "author ID",
            Self::AppID => "app ID",
            Self::AuthorName => "author name",
            Self::AppName => "app name",
            Self::RomSize => "ROM size",
            Self::DataSize => "data size",

            Self::Launches => "launches",
            Self::Installed => "installed",
            Self::Updated => "updated",

            Self::Back => "back",
            Self::Exit => "exit",
            Self::Stats => "stats",
            Self::Achievements => "achievements",
            Self::Scoreboards => "scoreboards",
            Self::ViewInCatalog => "view in catalog",
            Self::Remove => "remove",
        }
    }

    const fn translate_dutch(&self) -> &'static str {
        match self {
            Self::AuthorID => "auteur ID",
            Self::AppID => "app ID",
            Self::AuthorName => "auteurnaam",
            Self::AppName => "appnaam",
            Self::RomSize => "ROM-grootte",
            Self::DataSize => "gegevensgrootte",

            Self::Launches => "starts",
            Self::Installed => "geinstalleerd",
            Self::Updated => "bijgewerkt",

            Self::Back => "terug",
            Self::Exit => "afsluiten",
            Self::Stats => "statistieken",
            Self::Achievements => "prestaties",
            Self::Scoreboards => "scoreborden",
            Self::ViewInCatalog => "bekijken in catalogus",
            Self::Remove => "verwijderen",
        }
    }

    const fn translate_french(&self) -> &'static str {
        // TODO: translate
        self.translate_english()
    }

    const fn translate_german(&self) -> &'static str {
        // TODO: translate
        self.translate_english()
    }

    const fn translate_italian(&self) -> &'static str {
        // TODO: translate
        self.translate_english()
    }

    const fn translate_polish(&self) -> &'static str {
        // TODO: translate
        self.translate_english()
    }

    const fn translate_russian(&self) -> &'static str {
        match self {
            Self::AuthorID => "ID автора",
            Self::AppID => "ID приложения",
            Self::AuthorName => "имя автора",
            Self::AppName => "имя приложения",
            Self::RomSize => "размер ROMа",
            Self::DataSize => "размер данных",

            Self::Launches => "запусков",
            Self::Installed => "установлен",
            Self::Updated => "обновлён",

            Self::Back => "назад",
            Self::Exit => "выйти",
            Self::Stats => "статистика",
            Self::Achievements => "достижения",
            Self::Scoreboards => "лучшие результаты",
            Self::ViewInCatalog => "открыть в каталоге",
            Self::Remove => "удалить",
        }
    }

    const fn translate_spanish(&self) -> &'static str {
        // TODO: translate
        self.translate_english()
    }

    const fn translate_swedish(&self) -> &'static str {
        // TODO: translate
        self.translate_english()
    }

    const fn translate_turkish(&self) -> &'static str {
        // TODO: translate
        self.translate_english()
    }

    const fn translate_ukrainian(&self) -> &'static str {
        match self {
            Self::AuthorID => "ID автора",
            Self::AppID => "ID програми",
            Self::AuthorName => "ім'я автора",
            Self::AppName => "ім'я програми",
            Self::RomSize => "розмір ROMа",
            Self::DataSize => "розмір даних",

            Self::Launches => "запусків",
            Self::Installed => "встановлено",
            Self::Updated => "оновлено",

            Self::Back => "тому",
            Self::Exit => "вийти",
            Self::Stats => "статистика",
            Self::Achievements => "досягнення",
            Self::Scoreboards => "найкращі результати",
            Self::ViewInCatalog => "відкрити у каталозі",
            Self::Remove => "видалити",
        }
    }

    const fn translate_toki_pona(&self) -> &'static str {
        match self {
            Self::AuthorID => "jan pali nimi",
            Self::AppID => "musi nimi",
            Self::AuthorName => "jan pali",
            Self::AppName => "musi",
            Self::RomSize => "musi suli",
            Self::DataSize => "sona suli",

            Self::Launches => "namba kepeken",
            Self::Installed => "sin",
            Self::Updated => "sin sike",

            Self::Back => "monsi",
            Self::Exit => "tawa",
            Self::Stats => "sona",
            Self::Achievements => "sitelen pali",
            Self::Scoreboards => "nanpa pali",
            Self::ViewInCatalog => "lukin e lipu",
            Self::Remove => "weka",
        }
    }
}
