use firefly_rust::Language;

pub enum Message {
    AppID,
    AuthorName,
    AppName,
    Size,

    Launches,
    Installed,
    Updated,

    Manual,
    Achievements,
    Scoreboards,
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
            Language::Romanian => self.translate_romanian(),
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
            Self::AppID => "ID",
            Self::AuthorName => "author name",
            Self::AppName => "app name",
            Self::Size => "size",

            Self::Launches => "launches",
            Self::Installed => "installed",
            Self::Updated => "updated",

            Self::Manual => "manual",
            Self::Achievements => "achievements",
            Self::Scoreboards => "scoreboards",
            Self::Remove => "remove",
        }
    }

    const fn translate_dutch(&self) -> &'static str {
        match self {
            Self::AppID => "ID",
            Self::AuthorName => "auteurnaam",
            Self::AppName => "appnaam",
            Self::Size => "grootte",

            Self::Launches => "starts",
            Self::Installed => "geinstalleerd",
            Self::Updated => "bijgewerkt",

            Self::Manual => "manual", // TODO
            Self::Achievements => "prestaties",
            Self::Scoreboards => "scoreborden",
            Self::Remove => "verwijderen",
        }
    }

    const fn translate_french(&self) -> &'static str {
        match self {
            Self::AppID => "ID",
            Self::AuthorName => "nom de l’auteur",
            Self::AppName => "nom de l’application",
            Self::Size => "taille",

            Self::Launches => "lancements",
            Self::Installed => "installé",
            Self::Updated => "mis à jour",

            Self::Manual => "manual", // TODO
            Self::Achievements => "succès",
            Self::Scoreboards => "classements",
            Self::Remove => "supprimer",
        }
    }

    const fn translate_german(&self) -> &'static str {
        match self {
            Self::AppID => "ID",
            Self::AuthorName => "autorname",
            Self::AppName => "appname",
            Self::Size => "größe",

            Self::Launches => "startet",
            Self::Installed => "installiert",
            Self::Updated => "aktualisiert",

            Self::Manual => "manual", // TODO
            Self::Achievements => "errungenschaften",
            Self::Scoreboards => "punktetafel",
            Self::Remove => "entfernen",
        }
    }

    const fn translate_italian(&self) -> &'static str {
        // TODO: translate
        match self {
            Self::AppID => "ID",
            Self::AuthorName => "author name",
            Self::AppName => "app name",
            Self::Size => "size",

            Self::Launches => "launches",
            Self::Installed => "installed",
            Self::Updated => "updated",

            Self::Manual => "manual", // TODO
            Self::Achievements => "achievements",
            Self::Scoreboards => "scoreboards",
            Self::Remove => "remove",
        }
    }

    const fn translate_polish(&self) -> &'static str {
        match self {
            Self::AppID => "ID",
            Self::AuthorName => "nazwa autora",
            Self::AppName => "nazwa aplikacji",
            Self::Size => "rozmiar",

            Self::Launches => "uruchomienia",
            Self::Installed => "zainstalowano",
            Self::Updated => "zaktualizowano",

            Self::Manual => "manual", // TODO
            Self::Achievements => "osiągnięcia",
            Self::Scoreboards => "tabele wyników",
            Self::Remove => "usuń",
        }
    }

    const fn translate_romanian(&self) -> &'static str {
        match self {
            Self::AppID => "ID-ul",
            Self::AuthorName => "numele autorului",
            Self::AppName => "numele aplicației",
            Self::Size => "dimensiune",

            Self::Launches => "lansări",
            Self::Installed => "instalat",
            Self::Updated => "actualizat",

            Self::Manual => "manual", // TODO
            Self::Achievements => "realizări",
            Self::Scoreboards => "scoruri",
            Self::Remove => "elimină",
        }
    }

    const fn translate_russian(&self) -> &'static str {
        match self {
            Self::AppID => "ID",
            Self::AuthorName => "имя автора",
            Self::AppName => "имя приложения",
            Self::Size => "размер",

            Self::Launches => "запусков",
            Self::Installed => "установлен",
            Self::Updated => "обновлён",

            Self::Manual => "руководство пользователя",
            Self::Achievements => "достижения",
            Self::Scoreboards => "лучшие результаты",
            Self::Remove => "удалить",
        }
    }

    const fn translate_spanish(&self) -> &'static str {
        // TODO: translate
        match self {
            Self::AppID => "ID",
            Self::AuthorName => "author name",
            Self::AppName => "app name",
            Self::Size => "size",

            Self::Launches => "launches",
            Self::Installed => "installed",
            Self::Updated => "updated",

            Self::Manual => "manual", // TODO
            Self::Achievements => "achievements",
            Self::Scoreboards => "scoreboards",
            Self::Remove => "remove",
        }
    }

    const fn translate_swedish(&self) -> &'static str {
        match self {
            Self::AppID => "ID",
            Self::AuthorName => "skaparens namn",
            Self::AppName => "appnamn",
            Self::Size => "storlek",

            Self::Launches => "startar",
            Self::Installed => "installerad",
            Self::Updated => "uppdaterad",

            Self::Manual => "manual", // TODO
            Self::Achievements => "utmärkelser",
            Self::Scoreboards => "topplistor",
            Self::Remove => "ta bort",
        }
    }

    const fn translate_turkish(&self) -> &'static str {
        // TODO: translate
        match self {
            Self::AppID => "ID",
            Self::AuthorName => "author name",
            Self::AppName => "app name",
            Self::Size => "size",

            Self::Launches => "launches",
            Self::Installed => "installed",
            Self::Updated => "updated",

            Self::Manual => "manual", // TODO
            Self::Achievements => "achievements",
            Self::Scoreboards => "scoreboards",
            Self::Remove => "remove",
        }
    }

    const fn translate_ukrainian(&self) -> &'static str {
        match self {
            Self::AppID => "ID",
            Self::AuthorName => "ім'я автора",
            Self::AppName => "ім'я програми",
            Self::Size => "розмір",

            Self::Launches => "запусків",
            Self::Installed => "встановлено",
            Self::Updated => "оновлено",

            Self::Manual => "manual", // TODO
            Self::Achievements => "досягнення",
            Self::Scoreboards => "найкращі результати",
            Self::Remove => "видалити",
        }
    }

    const fn translate_toki_pona(&self) -> &'static str {
        match self {
            Self::AppID => "musi nimi",
            Self::AuthorName => "jan pali",
            Self::AppName => "musi",
            Self::Size => "suli",

            Self::Launches => "namba kepeken",
            Self::Installed => "sin",
            Self::Updated => "sin sike",

            Self::Manual => "lipu",
            Self::Achievements => "sitelen pali",
            Self::Scoreboards => "nanpa pali",
            Self::Remove => "weka",
        }
    }
}
