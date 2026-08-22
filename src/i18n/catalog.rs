use super::Lang;

#[derive(Debug, Clone, Copy)]
pub struct Catalog {
    pub en: &'static str,
    pub hu: &'static str,
    pub de: &'static str,
    pub es: &'static str,
    pub it: &'static str,
    pub zh: &'static str,
    pub ru: &'static str,
    pub uk: &'static str,
}

impl Catalog {
    pub const fn get(self, lang: Lang) -> &'static str {
        match lang {
            Lang::En => self.en,
            Lang::Hu => self.hu,
            Lang::De => self.de,
            Lang::Es => self.es,
            Lang::It => self.it,
            Lang::Zh => self.zh,
            Lang::Ru => self.ru,
            Lang::Uk => self.uk,
        }
    }
}

macro_rules! cat {
    (
        en: $en:expr,
        hu: $hu:expr,
        de: $de:expr,
        es: $es:expr,
        it: $it:expr,
        zh: $zh:expr,
        ru: $ru:expr,
        uk: $uk:expr $(,)?
    ) => {
        Catalog {
            en: $en,
            hu: $hu,
            de: $de,
            es: $es,
            it: $it,
            zh: $zh,
            ru: $ru,
            uk: $uk,
        }
    };
}

// —— Help / CLI ————————————————————————————————————————————————

pub const APP_ABOUT: Catalog = cat! {
    en: "d CLI — by Dezső Benedek Péter",
    hu: "d CLI — Dezső Benedek Péter",
    de: "d CLI — von Dezső Benedek Péter",
    es: "d CLI — por Dezső Benedek Péter",
    it: "d CLI — di Dezső Benedek Péter",
    zh: "d CLI — 作者 Dezső Benedek Péter",
    ru: "d CLI — Dezső Benedek Péter",
    uk: "d CLI — Dezső Benedek Péter",
};

pub const FLAG_AUTHORS: Catalog = cat! {
    en: "Print the author name",
    hu: "A szerző nevének kiírása",
    de: "Autorennamen anzeigen",
    es: "Mostrar el nombre del autor",
    it: "Mostra il nome dell'autore",
    zh: "显示作者姓名",
    ru: "Показать имя автора",
    uk: "Показати ім'я автора",
};

pub const FLAG_DOC: Catalog = cat! {
    en: "Print the documentation URL",
    hu: "A dokumentáció linkjének kiírása",
    de: "Dokumentations-URL anzeigen",
    es: "Mostrar la URL de documentación",
    it: "Mostra l'URL della documentazione",
    zh: "显示文档链接",
    ru: "Показать ссылку на документацию",
    uk: "Показати посилання на документацію",
};

pub const FLAG_LANG: Catalog = cat! {
    en: "UI language (en, hu, de, es, it, zh, ru, uk)",
    hu: "Felület nyelve (en, hu, de, es, it, zh, ru, uk)",
    de: "UI-Sprache (en, hu, de, es, it, zh, ru, uk)",
    es: "Idioma de la interfaz (en, hu, de, es, it, zh, ru, uk)",
    it: "Lingua dell'interfaccia (en, hu, de, es, it, zh, ru, uk)",
    zh: "界面语言 (en, hu, de, es, it, zh, ru, uk)",
    ru: "Язык интерфейса (en, hu, de, es, it, zh, ru, uk)",
    uk: "Мова інтерфейсу (en, hu, de, es, it, zh, ru, uk)",
};

pub const CMD_IP: Catalog = cat! {
    en: "Print the machine's local (LAN) IP address",
    hu: "A gép helyi (LAN) IP-címének kiírása",
    de: "Lokale (LAN) IP-Adresse ausgeben",
    es: "Mostrar la IP local (LAN) de la máquina",
    it: "Mostra l'indirizzo IP locale (LAN)",
    zh: "显示本机局域网 (LAN) IP 地址",
    ru: "Показать локальный (LAN) IP-адрес",
    uk: "Показати локальну (LAN) IP-адресу",
};

pub const CMD_VERSION: Catalog = cat! {
    en: "Print the CLI version",
    hu: "A CLI verziószámának kiírása",
    de: "CLI-Version anzeigen",
    es: "Mostrar la versión de la CLI",
    it: "Mostra la versione della CLI",
    zh: "显示 CLI 版本号",
    ru: "Показать версию CLI",
    uk: "Показати версію CLI",
};

pub const CMD_UPDATE: Catalog = cat! {
    en: "Download and install the latest version (github.com/DezBenedek/d)",
    hu: "A legújabb verzió letöltése és telepítése (github.com/DezBenedek/d)",
    de: "Neueste Version herunterladen und installieren (github.com/DezBenedek/d)",
    es: "Descargar e instalar la última versión (github.com/DezBenedek/d)",
    it: "Scarica e installa l'ultima versione (github.com/DezBenedek/d)",
    zh: "下载并安装最新版本 (github.com/DezBenedek/d)",
    ru: "Скачать и установить последнюю версию (github.com/DezBenedek/d)",
    uk: "Завантажити й встановити найновішу версію (github.com/DezBenedek/d)",
};

pub const CMD_PUSH: Catalog = cat! {
    en: "git add -A + commit + push to the current branch",
    hu: "git add -A + commit + push az aktuális branch-re",
    de: "git add -A + commit + push auf den aktuellen Branch",
    es: "git add -A + commit + push a la rama actual",
    it: "git add -A + commit + push sul branch corrente",
    zh: "git add -A + commit + push 到当前分支",
    ru: "git add -A + commit + push в текущую ветку",
    uk: "git add -A + commit + push у поточну гілку",
};

pub const CMD_PUSH_MSG: Catalog = cat! {
    en: "Commit message (multiple words are joined automatically)",
    hu: "A commit üzenet (több szó esetén automatikusan összefűzve)",
    de: "Commit-Nachricht (mehrere Wörter werden automatisch verbunden)",
    es: "Mensaje del commit (varias palabras se unen automáticamente)",
    it: "Messaggio di commit (più parole vengono unite automaticamente)",
    zh: "提交说明（多个词会自动拼接）",
    ru: "Сообщение коммита (несколько слов объединяются автоматически)",
    uk: "Повідомлення коміту (кілька слів об'єднуються автоматично)",
};

pub const CMD_GIT: Catalog = cat! {
    en: "Git helpers (fix, setup, update)",
    hu: "Git-hez kapcsolódó segédparancsok (fix, setup, update)",
    de: "Git-Hilfsbefehle (fix, setup, update)",
    es: "Utilidades de Git (fix, setup, update)",
    it: "Utility Git (fix, setup, update)",
    zh: "Git 辅助命令 (fix, setup, update)",
    ru: "Вспомогательные команды Git (fix, setup, update)",
    uk: "Допоміжні команди Git (fix, setup, update)",
};

pub const CMD_GIT_FIX: Catalog = cat! {
    en: "Untrack files that are ignored by .gitignore but still tracked",
    hu: "A .gitignore által tiltott, de már trackelt fájlok eltávolítása a git indexből",
    de: "Von .gitignore ignorierte, aber noch getrackte Dateien aus dem Index entfernen",
    es: "Quitar del índice archivos ignorados por .gitignore pero aún rastreados",
    it: "Rimuovere dall'indice i file ignorati da .gitignore ma ancora tracciati",
    zh: "从索引中移除已被 .gitignore 忽略但仍被跟踪的文件",
    ru: "Убрать из индекса файлы, игнорируемые .gitignore, но всё ещё отслеживаемые",
    uk: "Прибрати з індексу файли, ігноровані .gitignore, але все ще відстежувані",
};

pub const CMD_GIT_SETUP: Catalog = cat! {
    en: "Interactive git + GitHub repo setup (via gh CLI)",
    hu: "Interaktív git + GitHub repo setup (gh CLI-vel)",
    de: "Interaktives Git- + GitHub-Repo-Setup (über gh CLI)",
    es: "Configuración interactiva de git + repo de GitHub (vía gh CLI)",
    it: "Setup interattivo git + repo GitHub (tramite gh CLI)",
    zh: "交互式 git + GitHub 仓库设置（通过 gh CLI）",
    ru: "Интерактивная настройка git + репозитория GitHub (через gh CLI)",
    uk: "Інтерактивне налаштування git + репозиторію GitHub (через gh CLI)",
};

pub const CMD_GIT_UPDATE: Catalog = cat! {
    en: "Pull the latest changes for the current branch",
    hu: "Az aktuális branch legfrissebb változásainak letöltése",
    de: "Neueste Änderungen für den aktuellen Branch holen",
    es: "Traer los últimos cambios de la rama actual",
    it: "Scarica gli ultimi cambiamenti del branch corrente",
    zh: "拉取当前分支的最新更改",
    ru: "Получить последние изменения текущей ветки",
    uk: "Отримати останні зміни поточної гілки",
};

pub const CMD_MACOS: Catalog = cat! {
    en: "macOS-specific settings (start, dock, flushdns, reset)",
    hu: "macOS-specifikus beállítások (start, dock, flushdns, reset)",
    de: "macOS-spezifische Einstellungen (start, dock, flushdns, reset)",
    es: "Ajustes específicos de macOS (start, dock, flushdns, reset)",
    it: "Impostazioni specifiche di macOS (start, dock, flushdns, reset)",
    zh: "macOS 专用设置（start、dock、flushdns、reset）",
    ru: "Настройки для macOS (start, dock, flushdns, reset)",
    uk: "Налаштування для macOS (start, dock, flushdns, reset)",
};

pub const CMD_MACOS_START: Catalog = cat! {
    en: "Initial macOS tweaks: battery %, Finder path/status bar, hidden files",
    hu: "Kezdeti macOS-beállítások: akku százalék, Finder path/status bar, rejtett fájlok",
    de: "Erste macOS-Einstellungen: Akku-%, Finder-Pfad/Statusleiste, versteckte Dateien",
    es: "Ajustes iniciales de macOS: % de batería, barra de ruta/estado del Finder, ocultos",
    it: "Impostazioni iniziali macOS: % batteria, path/status bar Finder, file nascosti",
    zh: "初始 macOS 设置：电池百分比、Finder 路径/状态栏、显示隐藏文件",
    ru: "Начальные настройки macOS: % батареи, путь/статус Finder, скрытые файлы",
    uk: "Початкові налаштування macOS: % батареї, шлях/статус Finder, приховані файли",
};

pub const CMD_MACOS_DOCK: Catalog = cat! {
    en: "Auto-hide the Dock",
    hu: "Dock automatikus elrejtése",
    de: "Dock automatisch ausblenden",
    es: "Ocultar automáticamente el Dock",
    it: "Nascondi automaticamente il Dock",
    zh: "自动隐藏程序坞",
    ru: "Автоскрытие Dock",
    uk: "Автоприховування Dock",
};

pub const CMD_MACOS_FLUSHDNS: Catalog = cat! {
    en: "Flush the DNS cache",
    hu: "DNS-gyorsítótár ürítése",
    de: "DNS-Cache leeren",
    es: "Vaciar la caché DNS",
    it: "Svuota la cache DNS",
    zh: "刷新 DNS 缓存",
    ru: "Очистить DNS-кэш",
    uk: "Очистити DNS-кеш",
};

pub const CMD_MACOS_RESET: Catalog = cat! {
    en: "Undo the `d macos start` tweaks",
    hu: "A `d macos start` beállításainak visszavonása",
    de: "Die `d macos start`-Änderungen rückgängig machen",
    es: "Deshacer los ajustes de `d macos start`",
    it: "Annulla le modifiche di `d macos start`",
    zh: "撤销 `d macos start` 的设置",
    ru: "Отменить настройки `d macos start`",
    uk: "Скасувати налаштування `d macos start`",
};

pub const CMD_GEN: Catalog = cat! {
    en: "Generators (hex, base64, uuid, password)",
    hu: "Generáló segédparancsok (hex, base64, uuid, password)",
    de: "Generatoren (hex, base64, uuid, password)",
    es: "Generadores (hex, base64, uuid, password)",
    it: "Generatori (hex, base64, uuid, password)",
    zh: "生成器（hex、base64、uuid、password）",
    ru: "Генераторы (hex, base64, uuid, password)",
    uk: "Генератори (hex, base64, uuid, password)",
};

pub const CMD_GEN_HEX: Catalog = cat! {
    en: "Generate random hex (`openssl rand -hex`)",
    hu: "Véletlenszerű hex generálása (`openssl rand -hex`)",
    de: "Zufälliges Hex erzeugen (`openssl rand -hex`)",
    es: "Generar hex aleatorio (`openssl rand -hex`)",
    it: "Genera hex casuale (`openssl rand -hex`)",
    zh: "生成随机十六进制 (`openssl rand -hex`)",
    ru: "Сгенерировать случайный hex (`openssl rand -hex`)",
    uk: "Згенерувати випадковий hex (`openssl rand -hex`)",
};

pub const CMD_GEN_BASE64: Catalog = cat! {
    en: "Generate random base64 (`openssl rand -base64`)",
    hu: "Véletlenszerű base64 generálása (`openssl rand -base64`)",
    de: "Zufälliges Base64 erzeugen (`openssl rand -base64`)",
    es: "Generar base64 aleatorio (`openssl rand -base64`)",
    it: "Genera base64 casuale (`openssl rand -base64`)",
    zh: "生成随机 base64 (`openssl rand -base64`)",
    ru: "Сгенерировать случайный base64 (`openssl rand -base64`)",
    uk: "Згенерувати випадковий base64 (`openssl rand -base64`)",
};

pub const CMD_GEN_BYTES: Catalog = cat! {
    en: "Number of bytes (default: 32)",
    hu: "A byte-ok száma (alapértelmezett: 32)",
    de: "Anzahl der Bytes (Standard: 32)",
    es: "Número de bytes (predeterminado: 32)",
    it: "Numero di byte (predefinito: 32)",
    zh: "字节数（默认：32）",
    ru: "Количество байт (по умолчанию: 32)",
    uk: "Кількість байтів (за замовчуванням: 32)",
};

pub const CMD_GEN_UUID: Catalog = cat! {
    en: "Generate a random UUID v4",
    hu: "Véletlenszerű UUID v4 generálása",
    de: "Zufällige UUID v4 erzeugen",
    es: "Generar un UUID v4 aleatorio",
    it: "Genera un UUID v4 casuale",
    zh: "生成随机 UUID v4",
    ru: "Сгенерировать случайный UUID v4",
    uk: "Згенерувати випадковий UUID v4",
};

pub const CMD_GEN_PASSWORD: Catalog = cat! {
    en: "Generate a random password",
    hu: "Véletlenszerű jelszó generálása",
    de: "Zufälliges Passwort erzeugen",
    es: "Generar una contraseña aleatoria",
    it: "Genera una password casuale",
    zh: "生成随机密码",
    ru: "Сгенерировать случайный пароль",
    uk: "Згенерувати випадковий пароль",
};

pub const CMD_GEN_PASSWORD_LEN: Catalog = cat! {
    en: "Password length (default: 24)",
    hu: "Jelszó hossza (alapértelmezett: 24)",
    de: "Passwortlänge (Standard: 24)",
    es: "Longitud de la contraseña (predeterminado: 24)",
    it: "Lunghezza della password (predefinito: 24)",
    zh: "密码长度（默认：24）",
    ru: "Длина пароля (по умолчанию: 24)",
    uk: "Довжина пароля (за замовчуванням: 24)",
};

pub const CMD_DOWNLOAD: Catalog = cat! {
    en: "Download helpers (youtube, music) — For educational purposes only.",
    hu: "Letöltő parancsok (youtube, music) — For educational purposes only.",
    de: "Download-Befehle (youtube, music) — For educational purposes only.",
    es: "Comandos de descarga (youtube, music) — For educational purposes only.",
    it: "Comandi di download (youtube, music) — For educational purposes only.",
    zh: "下载命令（youtube、music）— For educational purposes only.",
    ru: "Команды загрузки (youtube, music) — For educational purposes only.",
    uk: "Команди завантаження (youtube, music) — For educational purposes only.",
};

pub const CMD_DOWNLOAD_YOUTUBE: Catalog = cat! {
    en: "Download a YouTube video. For educational purposes only.",
    hu: "YouTube-videó letöltése. For educational purposes only.",
    de: "YouTube-Video herunterladen. For educational purposes only.",
    es: "Descargar un vídeo de YouTube. For educational purposes only.",
    it: "Scarica un video di YouTube. For educational purposes only.",
    zh: "下载 YouTube 视频。For educational purposes only.",
    ru: "Скачать видео с YouTube. For educational purposes only.",
    uk: "Завантажити відео з YouTube. For educational purposes only.",
};

pub const CMD_DOWNLOAD_YOUTUBE_URL: Catalog = cat! {
    en: "YouTube video URL",
    hu: "YouTube-videó URL-je",
    de: "YouTube-Video-URL",
    es: "URL del vídeo de YouTube",
    it: "URL del video di YouTube",
    zh: "YouTube 视频链接",
    ru: "URL видео YouTube",
    uk: "URL відео YouTube",
};

pub const CMD_DOWNLOAD_VIDEO_QUALITY: Catalog = cat! {
    en: "Video quality: 270p, 480p, 720p, 1080p, 1440p",
    hu: "Videóminőség: 270p, 480p, 720p, 1080p, 1440p",
    de: "Videoqualität: 270p, 480p, 720p, 1080p, 1440p",
    es: "Calidad de vídeo: 270p, 480p, 720p, 1080p, 1440p",
    it: "Qualità video: 270p, 480p, 720p, 1080p, 1440p",
    zh: "视频质量：270p、480p、720p、1080p、1440p",
    ru: "Качество видео: 270p, 480p, 720p, 1080p, 1440p",
    uk: "Якість відео: 270p, 480p, 720p, 1080p, 1440p",
};

pub const CMD_DOWNLOAD_MUSIC: Catalog = cat! {
    en: "Download audio from YouTube, YouTube Music, or Spotify. For educational purposes only.",
    hu: "Hang letöltése YouTube, YouTube Music vagy Spotify linkről. For educational purposes only.",
    de: "Audio von YouTube, YouTube Music oder Spotify herunterladen. For educational purposes only.",
    es: "Descargar audio de YouTube, YouTube Music o Spotify. For educational purposes only.",
    it: "Scarica audio da YouTube, YouTube Music o Spotify. For educational purposes only.",
    zh: "从 YouTube、YouTube Music 或 Spotify 下载音频。For educational purposes only.",
    ru: "Скачать аудио с YouTube, YouTube Music или Spotify. For educational purposes only.",
    uk: "Завантажити аудіо з YouTube, YouTube Music або Spotify. For educational purposes only.",
};

pub const CMD_DOWNLOAD_MUSIC_URL: Catalog = cat! {
    en: "YouTube, YouTube Music, or Spotify URL",
    hu: "YouTube, YouTube Music vagy Spotify URL",
    de: "YouTube-, YouTube-Music- oder Spotify-URL",
    es: "URL de YouTube, YouTube Music o Spotify",
    it: "URL di YouTube, YouTube Music o Spotify",
    zh: "YouTube、YouTube Music 或 Spotify 链接",
    ru: "URL YouTube, YouTube Music или Spotify",
    uk: "URL YouTube, YouTube Music або Spotify",
};

pub const CMD_DOWNLOAD_AUDIO_QUALITY: Catalog = cat! {
    en: "Audio quality: 128k, 192k, 256k, 320k",
    hu: "Hangminőség: 128k, 192k, 256k, 320k",
    de: "Audioqualität: 128k, 192k, 256k, 320k",
    es: "Calidad de audio: 128k, 192k, 256k, 320k",
    it: "Qualità audio: 128k, 192k, 256k, 320k",
    zh: "音频质量：128k、192k、256k、320k",
    ru: "Качество аудио: 128k, 192k, 256k, 320k",
    uk: "Якість аудіо: 128k, 192k, 256k, 320k",
};

pub const CMD_INSTALL: Catalog = cat! {
    en: "Install tools (yt-dlp)",
    hu: "Eszközök telepítése (yt-dlp)",
    de: "Tools installieren (yt-dlp)",
    es: "Instalar herramientas (yt-dlp)",
    it: "Installa strumenti (yt-dlp)",
    zh: "安装工具（yt-dlp）",
    ru: "Установка инструментов (yt-dlp)",
    uk: "Встановлення інструментів (yt-dlp)",
};

pub const CMD_INSTALL_YT_DLP: Catalog = cat! {
    en: "Install yt-dlp and ffmpeg via Homebrew",
    hu: "yt-dlp és ffmpeg telepítése Homebrew-val",
    de: "yt-dlp und ffmpeg per Homebrew installieren",
    es: "Instalar yt-dlp y ffmpeg con Homebrew",
    it: "Installa yt-dlp e ffmpeg con Homebrew",
    zh: "通过 Homebrew 安装 yt-dlp 和 ffmpeg",
    ru: "Установить yt-dlp и ffmpeg через Homebrew",
    uk: "Встановити yt-dlp і ffmpeg через Homebrew",
};

// —— Common ————————————————————————————————————————————————

pub const ERR_EXIT_CODE: Catalog = cat! {
    en: "exit code: {code}",
    hu: "kilépési kód: {code}",
    de: "Exit-Code: {code}",
    es: "código de salida: {code}",
    it: "codice di uscita: {code}",
    zh: "退出码：{code}",
    ru: "код выхода: {code}",
    uk: "код виходу: {code}",
};

pub const ERR_START_PROGRAM: Catalog = cat! {
    en: "failed to start {program}: {error}",
    hu: "nem sikerült elindítani a(z) {program}-t: {error}",
    de: "{program} konnte nicht gestartet werden: {error}",
    es: "no se pudo iniciar {program}: {error}",
    it: "impossibile avviare {program}: {error}",
    zh: "无法启动 {program}：{error}",
    ru: "не удалось запустить {program}: {error}",
    uk: "не вдалося запустити {program}: {error}",
};

pub const ERR_COMMAND_FAILED: Catalog = cat! {
    en: "{program} {args} failed (code: {code})",
    hu: "{program} {args} sikertelen (kód: {code})",
    de: "{program} {args} fehlgeschlagen (Code: {code})",
    es: "{program} {args} falló (código: {code})",
    it: "{program} {args} non riuscito (codice: {code})",
    zh: "{program} {args} 失败（代码：{code}）",
    ru: "{program} {args} не удалось (код: {code})",
    uk: "{program} {args} не вдалося (код: {code})",
};

pub const ERR_EMPTY_BRANCH: Catalog = cat! {
    en: "empty branch name (detached HEAD?)",
    hu: "üres branch név (detached HEAD?)",
    de: "leerer Branch-Name (detached HEAD?)",
    es: "nombre de rama vacío (¿HEAD detached?)",
    it: "nome del branch vuoto (HEAD detached?)",
    zh: "分支名为空（是否处于 detached HEAD？）",
    ru: "пустое имя ветки (detached HEAD?)",
    uk: "порожня назва гілки (detached HEAD?)",
};

pub const FIELD_REQUIRED: Catalog = cat! {
    en: "This field is required.",
    hu: "Ez a mező kötelező.",
    de: "Dieses Feld ist erforderlich.",
    es: "Este campo es obligatorio.",
    it: "Questo campo è obbligatorio.",
    zh: "此字段为必填项。",
    ru: "Это поле обязательно.",
    uk: "Це поле обов'язкове.",
};

pub const ENTER_YN: Catalog = cat! {
    en: "Enter y or n",
    hu: "Írd be: y vagy n",
    de: "Gib y oder n ein",
    es: "Escribe y o n",
    it: "Digita y o n",
    zh: "请输入 y 或 n",
    ru: "Введите y или n",
    uk: "Введіть y або n",
};

pub const ENTER_VISIBILITY: Catalog = cat! {
    en: "Enter: public or private",
    hu: "Írd be: public vagy private",
    de: "Eingabe: public oder private",
    es: "Escribe: public o private",
    it: "Digita: public o private",
    zh: "请输入：public 或 private",
    ru: "Введите: public или private",
    uk: "Введіть: public або private",
};

pub const CANCELLED: Catalog = cat! {
    en: "Cancelled.",
    hu: "Megszakítva.",
    de: "Abgebrochen.",
    es: "Cancelado.",
    it: "Annullato.",
    zh: "已取消。",
    ru: "Отменено.",
    uk: "Скасовано.",
};

pub const DONE: Catalog = cat! {
    en: "Done.",
    hu: "Kész.",
    de: "Fertig.",
    es: "Listo.",
    it: "Fatto.",
    zh: "完成。",
    ru: "Готово.",
    uk: "Готово.",
};

pub const ERR_STDOUT_FLUSH: Catalog = cat! {
    en: "stdout flush error: {error}",
    hu: "stdout flush hiba: {error}",
    de: "stdout-Flush-Fehler: {error}",
    es: "error al vaciar stdout: {error}",
    it: "errore di flush di stdout: {error}",
    zh: "stdout 刷新错误：{error}",
    ru: "ошибка сброса stdout: {error}",
    uk: "помилка скидання stdout: {error}",
};

pub const ERR_STDIN_READ: Catalog = cat! {
    en: "stdin read error: {error}",
    hu: "stdin olvasási hiba: {error}",
    de: "stdin-Lesefehler: {error}",
    es: "error de lectura de stdin: {error}",
    it: "errore di lettura di stdin: {error}",
    zh: "stdin 读取错误：{error}",
    ru: "ошибка чтения stdin: {error}",
    uk: "помилка читання stdin: {error}",
};

// —— IP ————————————————————————————————————————————————

pub const IP_OK: Catalog = cat! {
    en: "Local IP: {addr}",
    hu: "Helyi IP: {addr}",
    de: "Lokale IP: {addr}",
    es: "IP local: {addr}",
    it: "IP locale: {addr}",
    zh: "本地 IP：{addr}",
    ru: "Локальный IP: {addr}",
    uk: "Локальний IP: {addr}",
};

pub const IP_ERR: Catalog = cat! {
    en: "Failed to resolve local IP: {error}",
    hu: "Nem sikerült lekérdezni a helyi IP-t: {error}",
    de: "Lokale IP konnte nicht ermittelt werden: {error}",
    es: "No se pudo obtener la IP local: {error}",
    it: "Impossibile ottenere l'IP locale: {error}",
    zh: "无法获取本地 IP：{error}",
    ru: "Не удалось получить локальный IP: {error}",
    uk: "Не вдалося отримати локальний IP: {error}",
};

// —— Push ————————————————————————————————————————————————

pub const PUSH_NEED_MSG: Catalog = cat! {
    en: "Provide a commit message! Example: d push fix ready",
    hu: "Adj meg egy commit üzenetet! Pl: d push javítás kész",
    de: "Commit-Nachricht angeben! Beispiel: d push fix fertig",
    es: "¡Indica un mensaje de commit! Ej.: d push arreglo listo",
    it: "Fornisci un messaggio di commit! Es.: d push fix pronto",
    zh: "请提供提交说明！例如：d push fix ready",
    ru: "Укажите сообщение коммита! Пример: d push fix ready",
    uk: "Вкажіть повідомлення коміту! Приклад: d push fix ready",
};

pub const PUSH_ADD_FAIL: Catalog = cat! {
    en: "git add failed: {error}",
    hu: "git add sikertelen: {error}",
    de: "git add fehlgeschlagen: {error}",
    es: "git add falló: {error}",
    it: "git add non riuscito: {error}",
    zh: "git add 失败：{error}",
    ru: "git add не удалось: {error}",
    uk: "git add не вдалося: {error}",
};

pub const PUSH_COMMIT_WARN: Catalog = cat! {
    en: "git commit notice (e.g. nothing to commit): {error}",
    hu: "git commit figyelmeztetés (pl. nincs mit commitolni): {error}",
    de: "git commit Hinweis (z. B. nichts zu committen): {error}",
    es: "aviso de git commit (p. ej. nada que confirmar): {error}",
    it: "avviso git commit (es. niente da committare): {error}",
    zh: "git commit 提示（例如没有可提交的更改）：{error}",
    ru: "уведомление git commit (например, нечего коммитить): {error}",
    uk: "повідомлення git commit (наприклад, немає що комітити): {error}",
};

pub const PUSH_BRANCH_ERR: Catalog = cat! {
    en: "failed to get current branch: {error}",
    hu: "nem sikerült lekérdezni az aktuális branch-et: {error}",
    de: "aktueller Branch konnte nicht ermittelt werden: {error}",
    es: "no se pudo obtener la rama actual: {error}",
    it: "impossibile ottenere il branch corrente: {error}",
    zh: "无法获取当前分支：{error}",
    ru: "не удалось получить текущую ветку: {error}",
    uk: "не вдалося отримати поточну гілку: {error}",
};

pub const PUSH_FAIL: Catalog = cat! {
    en: "git push failed: {error}",
    hu: "git push sikertelen: {error}",
    de: "git push fehlgeschlagen: {error}",
    es: "git push falló: {error}",
    it: "git push non riuscito: {error}",
    zh: "git push 失败：{error}",
    ru: "git push не удалось: {error}",
    uk: "git push не вдалося: {error}",
};

// —— Update ————————————————————————————————————————————————

pub const UPDATE_FAIL: Catalog = cat! {
    en: "Update failed: {error}",
    hu: "Frissítés sikertelen: {error}",
    de: "Update fehlgeschlagen: {error}",
    es: "Actualización fallida: {error}",
    it: "Aggiornamento non riuscito: {error}",
    zh: "更新失败：{error}",
    ru: "Обновление не удалось: {error}",
    uk: "Оновлення не вдалося: {error}",
};

pub const UPDATE_DOWNLOADING: Catalog = cat! {
    en: "Downloading latest version from: {url}",
    hu: "Legújabb verzió letöltése innen: {url}",
    de: "Neueste Version wird heruntergeladen von: {url}",
    es: "Descargando la última versión desde: {url}",
    it: "Download dell'ultima versione da: {url}",
    zh: "正在从以下地址下载最新版本：{url}",
    ru: "Загрузка последней версии из: {url}",
    uk: "Завантаження найновішої версії з: {url}",
};

pub const UPDATE_OK: Catalog = cat! {
    en: "Update successful.",
    hu: "Sikeres frissítés.",
    de: "Update erfolgreich.",
    es: "Actualización correcta.",
    it: "Aggiornamento riuscito.",
    zh: "更新成功。",
    ru: "Обновление выполнено.",
    uk: "Оновлення виконано.",
};

pub const UPDATE_API_REACH: Catalog = cat! {
    en: "failed to reach GitHub API: {error}",
    hu: "nem sikerült elérni a GitHub API-t: {error}",
    de: "GitHub-API nicht erreichbar: {error}",
    es: "no se pudo contactar la API de GitHub: {error}",
    it: "impossibile raggiungere l'API di GitHub: {error}",
    zh: "无法访问 GitHub API：{error}",
    ru: "не удалось связаться с GitHub API: {error}",
    uk: "не вдалося зв'язатися з GitHub API: {error}",
};

pub const UPDATE_API_STATUS: Catalog = cat! {
    en: "GitHub API returned an error (code: {code}) — does a release exist yet?",
    hu: "a GitHub API lekérdezése hibával tért vissza (kód: {code}) — létezik már kiadás a repóban?",
    de: "GitHub-API antwortete mit Fehler (Code: {code}) — existiert schon ein Release?",
    es: "la API de GitHub devolvió un error (código: {code}) — ¿existe ya una release?",
    it: "l'API di GitHub ha restituito un errore (codice: {code}) — esiste già una release?",
    zh: "GitHub API 返回错误（代码：{code}）— 仓库里是否已有发行版？",
    ru: "GitHub API вернул ошибку (код: {code}) — есть ли уже релиз?",
    uk: "GitHub API повернув помилку (код: {code}) — чи вже є реліз?",
};

pub const UPDATE_API_PARSE: Catalog = cat! {
    en: "failed to parse GitHub response: {error}",
    hu: "nem sikerült értelmezni a GitHub válaszát: {error}",
    de: "GitHub-Antwort konnte nicht gelesen werden: {error}",
    es: "no se pudo interpretar la respuesta de GitHub: {error}",
    it: "impossibile interpretare la risposta di GitHub: {error}",
    zh: "无法解析 GitHub 响应：{error}",
    ru: "не удалось разобрать ответ GitHub: {error}",
    uk: "не вдалося розібрати відповідь GitHub: {error}",
};

pub const UPDATE_NO_ASSETS: Catalog = cat! {
    en: "the release has no attached files (assets)",
    hu: "a kiadásnak nincsenek csatolt fájljai (assets)",
    de: "dem Release sind keine Dateien (Assets) angehängt",
    es: "la release no tiene archivos adjuntos (assets)",
    it: "la release non ha file allegati (assets)",
    zh: "该发行版没有附加文件（assets）",
    ru: "у релиза нет вложенных файлов (assets)",
    uk: "у релізу немає вкладених файлів (assets)",
};

pub const UPDATE_NO_ASSET_NAME: Catalog = cat! {
    en: "no asset named '{name}' found in the latest release",
    hu: "nem található '{name}' nevű csatolt fájl a legújabb kiadásban",
    de: "kein Asset namens '{name}' in der neuesten Version gefunden",
    es: "no se encontró un asset llamado '{name}' en la última release",
    it: "nessun asset chiamato '{name}' nell'ultima release",
    zh: "最新发行版中找不到名为 '{name}' 的附件",
    ru: "в последнем релизе нет файла с именем '{name}'",
    uk: "у найновішому релізі немає файлу з назвою '{name}'",
};

pub const UPDATE_NO_URL: Catalog = cat! {
    en: "the matched asset has no download URL",
    hu: "a talált csatolt fájlnak nincs letöltési URL-je",
    de: "das gefundene Asset hat keine Download-URL",
    es: "el asset encontrado no tiene URL de descarga",
    it: "l'asset trovato non ha URL di download",
    zh: "找到的附件没有下载 URL",
    ru: "у найденного файла нет URL для скачивания",
    uk: "у знайденого файлу немає URL для завантаження",
};

pub const UPDATE_EXE_PATH: Catalog = cat! {
    en: "cannot find path of the running executable: {error}",
    hu: "nem található a saját futtatható fájl helye: {error}",
    de: "Pfad der laufenden Binary nicht gefunden: {error}",
    es: "no se encuentra la ruta del ejecutable en ejecución: {error}",
    it: "impossibile trovare il percorso dell'eseguibile in esecuzione: {error}",
    zh: "找不到当前可执行文件路径：{error}",
    ru: "не найден путь к запущенному исполняемому файлу: {error}",
    uk: "не знайдено шлях до запущеного виконуваного файлу: {error}",
};

pub const UPDATE_CURL_START: Catalog = cat! {
    en: "failed to start curl: {error}",
    hu: "nem sikerült elindítani a curl-t: {error}",
    de: "curl konnte nicht gestartet werden: {error}",
    es: "no se pudo iniciar curl: {error}",
    it: "impossibile avviare curl: {error}",
    zh: "无法启动 curl：{error}",
    ru: "не удалось запустить curl: {error}",
    uk: "не вдалося запустити curl: {error}",
};

pub const UPDATE_DOWNLOAD_FAIL: Catalog = cat! {
    en: "binary download failed",
    hu: "a bináris letöltése sikertelen volt",
    de: "Download der Binary fehlgeschlagen",
    es: "falló la descarga del binario",
    it: "download del binario non riuscito",
    zh: "二进制文件下载失败",
    ru: "не удалось скачать бинарный файл",
    uk: "не вдалося завантажити бінарний файл",
};

pub const UPDATE_CHMOD_START: Catalog = cat! {
    en: "failed to make downloaded file executable: {error}",
    hu: "nem sikerült futtathatóvá tenni a letöltött fájlt: {error}",
    de: "heruntergeladene Datei konnte nicht ausführbar gemacht werden: {error}",
    es: "no se pudo hacer ejecutable el archivo descargado: {error}",
    it: "impossibile rendere eseguibile il file scaricato: {error}",
    zh: "无法将下载的文件设为可执行：{error}",
    ru: "не удалось сделать скачанный файл исполняемым: {error}",
    uk: "не вдалося зробити завантажений файл виконуваним: {error}",
};

pub const UPDATE_CHMOD_FAIL: Catalog = cat! {
    en: "chmod +x failed",
    hu: "a chmod +x sikertelen volt",
    de: "chmod +x fehlgeschlagen",
    es: "chmod +x falló",
    it: "chmod +x non riuscito",
    zh: "chmod +x 失败",
    ru: "chmod +x не удалось",
    uk: "chmod +x не вдалося",
};

pub const UPDATE_REPLACE_FAIL: Catalog = cat! {
    en: "failed to replace the running binary (permission issue?): {error}",
    hu: "nem sikerült lecserélni a futó binárist (jogosultság hiánya?): {error}",
    de: "laufende Binary konnte nicht ersetzt werden (Berechtigung?): {error}",
    es: "no se pudo reemplazar el binario en ejecución (¿permisos?): {error}",
    it: "impossibile sostituire il binario in esecuzione (permessi?): {error}",
    zh: "无法替换正在运行的二进制文件（权限问题？）：{error}",
    ru: "не удалось заменить запущенный бинарник (права доступа?): {error}",
    uk: "не вдалося замінити запущений бінарник (права доступу?): {error}",
};

pub const UPDATE_UP_TO_DATE: Catalog = cat! {
    en: "Already up to date (d {version}).",
    hu: "Már a legfrissebb verzió van telepítve (d {version}).",
    de: "Bereits aktuell (d {version}).",
    es: "Ya está actualizado (d {version}).",
    it: "Già aggiornato (d {version}).",
    zh: "已是最新版本（d {version}）。",
    ru: "Уже установлена последняя версия (d {version}).",
    uk: "Уже встановлено найновішу версію (d {version}).",
};

pub const UPDATE_NEW_VERSION: Catalog = cat! {
    en: "Update available: {current} → {latest}",
    hu: "Elérhető frissítés: {current} → {latest}",
    de: "Update verfügbar: {current} → {latest}",
    es: "Actualización disponible: {current} → {latest}",
    it: "Aggiornamento disponibile: {current} → {latest}",
    zh: "有可用更新：{current} → {latest}",
    ru: "Доступно обновление: {current} → {latest}",
    uk: "Доступне оновлення: {current} → {latest}",
};

pub const UPDATE_NO_TAG: Catalog = cat! {
    en: "the latest release has no tag name",
    hu: "a legújabb kiadásnak nincs tag neve",
    de: "die neueste Version hat keinen Tag-Namen",
    es: "la última release no tiene nombre de etiqueta",
    it: "l'ultima release non ha un nome di tag",
    zh: "最新发行版没有标签名",
    ru: "у последнего релиза нет имени тега",
    uk: "у найновішого релізу немає назви тега",
};

pub const UPDATE_CHECKSUM_MISSING: Catalog = cat! {
    en: "the latest release has no SHA-256 checksum file (d.sha256)",
    hu: "a legújabb kiadáshoz nincs SHA-256 ellenőrzőösszeg (d.sha256)",
    de: "der neuesten Version fehlt die SHA-256-Prüfsummendatei (d.sha256)",
    es: "la última release no tiene archivo de checksum SHA-256 (d.sha256)",
    it: "l'ultima release non ha un file checksum SHA-256 (d.sha256)",
    zh: "最新发行版没有 SHA-256 校验文件（d.sha256）",
    ru: "в последнем релизе нет файла контрольной суммы SHA-256 (d.sha256)",
    uk: "у найновішому релізі немає файлу контрольної суми SHA-256 (d.sha256)",
};

pub const UPDATE_CHECKSUM_PARSE: Catalog = cat! {
    en: "failed to read checksum file: {error}",
    hu: "nem sikerült beolvasni az ellenőrzőösszeg-fájlt: {error}",
    de: "Prüfsummendatei konnte nicht gelesen werden: {error}",
    es: "no se pudo leer el archivo de checksum: {error}",
    it: "impossibile leggere il file checksum: {error}",
    zh: "无法读取校验文件：{error}",
    ru: "не удалось прочитать файл контрольной суммы: {error}",
    uk: "не вдалося прочитати файл контрольної суми: {error}",
};

pub const UPDATE_CHECKSUM_FORMAT: Catalog = cat! {
    en: "checksum file has no SHA-256 hash for '{name}'",
    hu: "az ellenőrzőösszeg-fájlban nincs SHA-256 hash a(z) '{name}' fájlhoz",
    de: "Prüfsummendatei enthält keinen SHA-256-Hash für '{name}'",
    es: "el archivo de checksum no tiene hash SHA-256 para '{name}'",
    it: "il file checksum non ha un hash SHA-256 per '{name}'",
    zh: "校验文件中没有 '{name}' 的 SHA-256 哈希",
    ru: "в файле контрольной суммы нет SHA-256 для '{name}'",
    uk: "у файлі контрольної суми немає SHA-256 для '{name}'",
};

pub const UPDATE_CHECKSUM_MISMATCH: Catalog = cat! {
    en: "checksum mismatch (expected {expected}, got {actual})",
    hu: "az ellenőrzőösszeg nem egyezik (várt: {expected}, kapott: {actual})",
    de: "Prüfsumme stimmt nicht (erwartet {expected}, erhalten {actual})",
    es: "el checksum no coincide (esperado {expected}, obtenido {actual})",
    it: "checksum non corrispondente (atteso {expected}, ottenuto {actual})",
    zh: "校验和不匹配（期望 {expected}，实际 {actual}）",
    ru: "контрольная сумма не совпадает (ожидалось {expected}, получено {actual})",
    uk: "контрольна сума не збігається (очікувалось {expected}, отримано {actual})",
};

pub const UPDATE_SHA256_START: Catalog = cat! {
    en: "failed to start shasum: {error}",
    hu: "nem sikerült elindítani a shasum-ot: {error}",
    de: "shasum konnte nicht gestartet werden: {error}",
    es: "no se pudo iniciar shasum: {error}",
    it: "impossibile avviare shasum: {error}",
    zh: "无法启动 shasum：{error}",
    ru: "не удалось запустить shasum: {error}",
    uk: "не вдалося запустити shasum: {error}",
};

pub const UPDATE_SHA256_FAIL: Catalog = cat! {
    en: "failed to compute SHA-256 of the downloaded binary",
    hu: "nem sikerült kiszámolni a letöltött bináris SHA-256 hash-ét",
    de: "SHA-256 der heruntergeladenen Binary konnte nicht berechnet werden",
    es: "no se pudo calcular el SHA-256 del binario descargado",
    it: "impossibile calcolare lo SHA-256 del binario scaricato",
    zh: "无法计算已下载二进制文件的 SHA-256",
    ru: "не удалось вычислить SHA-256 скачанного бинарника",
    uk: "не вдалося обчислити SHA-256 завантаженого бінарника",
};

pub const UPDATE_SUDO: Catalog = cat! {
    en: "Permission denied — retrying with sudo...",
    hu: "Nincs jogosultság — újrapróbálás sudo-val...",
    de: "Keine Berechtigung — erneuter Versuch mit sudo...",
    es: "Permiso denegado — reintentando con sudo...",
    it: "Permesso negato — nuovo tentativo con sudo...",
    zh: "权限不足 — 正在使用 sudo 重试...",
    ru: "Нет прав — повтор с sudo...",
    uk: "Немає прав — повтор із sudo...",
};

pub const UPDATE_SUDO_FAIL: Catalog = cat! {
    en: "sudo replace failed: {error}",
    hu: "a sudo-s csere sikertelen: {error}",
    de: "sudo-Ersetzung fehlgeschlagen: {error}",
    es: "el reemplazo con sudo falló: {error}",
    it: "sostituzione con sudo non riuscita: {error}",
    zh: "sudo 替换失败：{error}",
    ru: "замена через sudo не удалась: {error}",
    uk: "заміна через sudo не вдалася: {error}",
};

// —— Git fix / update ————————————————————————————————————————————————

pub const GIT_FIX_LIST_ERR: Catalog = cat! {
    en: "Failed to list ignored-but-tracked files: {error}",
    hu: "Nem sikerült listázni a git-ignore-olt, de trackelt fájlokat: {error}",
    de: "Ignorierte, aber getrackte Dateien konnten nicht gelistet werden: {error}",
    es: "No se pudieron listar archivos ignorados pero rastreados: {error}",
    it: "Impossibile elencare i file ignorati ma tracciati: {error}",
    zh: "无法列出被忽略但仍被跟踪的文件：{error}",
    ru: "Не удалось перечислить игнорируемые, но отслеживаемые файлы: {error}",
    uk: "Не вдалося перелічити ігноровані, але відстежувані файли: {error}",
};

pub const GIT_FIX_EMPTY: Catalog = cat! {
    en: "No files are ignored by .gitignore but still tracked by git.",
    hu: "Nincs olyan fájl, amit a .gitignore tiltana, de a git mégis trackelne.",
    de: "Keine Dateien, die von .gitignore ignoriert, aber noch von Git getrackt werden.",
    es: "No hay archivos ignorados por .gitignore pero aún rastreados por git.",
    it: "Nessun file ignorato da .gitignore ma ancora tracciato da git.",
    zh: "没有被 .gitignore 忽略却仍被 git 跟踪的文件。",
    ru: "Нет файлов, игнорируемых .gitignore, но всё ещё отслеживаемых git.",
    uk: "Немає файлів, ігнорованих .gitignore, але все ще відстежуваних git.",
};

pub const GIT_FIX_REMOVING: Catalog = cat! {
    en: "Removing from git tracking ({count} file(s)):",
    hu: "Eltávolítás a git trackingből ({count} fájl):",
    de: "Entfernen aus dem Git-Tracking ({count} Datei(en)):",
    es: "Quitando del seguimiento de git ({count} archivo(s)):",
    it: "Rimozione dal tracking di git ({count} file):",
    zh: "正在从 git 跟踪中移除（{count} 个文件）：",
    ru: "Удаление из отслеживания git ({count} файл(ов)):",
    uk: "Видалення з відстеження git ({count} файл(ів)):",
};

pub const GIT_FIX_UNTRACK_ERR: Catalog = cat! {
    en: "Failed to untrack files: {error}",
    hu: "Nem sikerült eltávolítani a fájlokat: {error}",
    de: "Dateien konnten nicht aus dem Tracking entfernt werden: {error}",
    es: "No se pudieron dejar de rastrear los archivos: {error}",
    it: "Impossibile smettere di tracciare i file: {error}",
    zh: "无法取消跟踪文件：{error}",
    ru: "Не удалось убрать файлы из отслеживания: {error}",
    uk: "Не вдалося прибрати файли з відстеження: {error}",
};

pub const GIT_FIX_DONE: Catalog = cat! {
    en: "Done. Commit the change, e.g.: d push untrack gitignored files",
    hu: "Kész. A változást még commitolnod kell, pl.: d push gitignore-olt fajlok eltavolitasa",
    de: "Fertig. Noch committen, z. B.: d push gitignore Dateien entfernt",
    es: "Listo. Confirma el cambio, p. ej.: d push quitar archivos gitignore",
    it: "Fatto. Fai commit, es.: d push rimuovi file gitignore",
    zh: "完成。请提交更改，例如：d push untrack gitignored files",
    ru: "Готово. Закоммитьте изменение, напр.: d push untrack gitignored files",
    uk: "Готово. Зробіть коміт, напр.: d push untrack gitignored files",
};

pub const GIT_PULL_BRANCH_ERR: Catalog = cat! {
    en: "Failed to get current branch: {error}",
    hu: "Nem sikerült lekérdezni az aktuális branch-et: {error}",
    de: "Aktueller Branch konnte nicht ermittelt werden: {error}",
    es: "No se pudo obtener la rama actual: {error}",
    it: "Impossibile ottenere il branch corrente: {error}",
    zh: "无法获取当前分支：{error}",
    ru: "Не удалось получить текущую ветку: {error}",
    uk: "Не вдалося отримати поточну гілку: {error}",
};

pub const GIT_PULL_FETCHING: Catalog = cat! {
    en: "Pulling latest changes: origin/{branch}",
    hu: "Legfrissebb változások letöltése: origin/{branch}",
    de: "Neueste Änderungen werden geholt: origin/{branch}",
    es: "Obteniendo los últimos cambios: origin/{branch}",
    it: "Scaricamento degli ultimi cambiamenti: origin/{branch}",
    zh: "正在拉取最新更改：origin/{branch}",
    ru: "Получение последних изменений: origin/{branch}",
    uk: "Отримання останніх змін: origin/{branch}",
};

pub const GIT_PULL_FAIL: Catalog = cat! {
    en: "git pull failed: {error}",
    hu: "git pull sikertelen: {error}",
    de: "git pull fehlgeschlagen: {error}",
    es: "git pull falló: {error}",
    it: "git pull non riuscito: {error}",
    zh: "git pull 失败：{error}",
    ru: "git pull не удалось: {error}",
    uk: "git pull не вдалося: {error}",
};

pub const GIT_PULL_OK: Catalog = cat! {
    en: "Update successful.",
    hu: "Sikeres frissítés.",
    de: "Aktualisierung erfolgreich.",
    es: "Actualización correcta.",
    it: "Aggiornamento riuscito.",
    zh: "更新成功。",
    ru: "Обновление выполнено.",
    uk: "Оновлення виконано.",
};

// —— macOS ————————————————————————————————————————————————

pub const MACOS_BATTERY: Catalog = cat! {
    en: "battery percentage in the menu bar",
    hu: "akkumulátor százalék a menüsorban",
    de: "Akkustand in der Menüleiste",
    es: "porcentaje de batería en la barra de menú",
    it: "percentuale batteria nella barra dei menu",
    zh: "菜单栏电池百分比",
    ru: "процент батареи в строке меню",
    uk: "відсоток батареї в рядку меню",
};

pub const MACOS_PATHBAR: Catalog = cat! {
    en: "path bar (where am I)",
    hu: "elérési út sáv (hol vagyok)",
    de: "Pfadleiste (wo bin ich)",
    es: "barra de ruta (dónde estoy)",
    it: "barra del percorso (dove sono)",
    zh: "路径栏（当前位置）",
    ru: "строка пути (где я)",
    uk: "рядок шляху (де я)",
};

pub const MACOS_STATUSBAR: Catalog = cat! {
    en: "status bar (file sizes)",
    hu: "állapotsor (fájlméretek)",
    de: "Statusleiste (Dateigrößen)",
    es: "barra de estado (tamaños de archivo)",
    it: "barra di stato (dimensioni file)",
    zh: "状态栏（文件大小）",
    ru: "строка состояния (размеры файлов)",
    uk: "рядок стану (розміри файлів)",
};

pub const MACOS_HIDDEN: Catalog = cat! {
    en: "hidden files",
    hu: "rejtett fájlok",
    de: "versteckte Dateien",
    es: "archivos ocultos",
    it: "file nascosti",
    zh: "隐藏文件",
    ru: "скрытые файлы",
    uk: "приховані файли",
};

pub const MACOS_SET_ERR: Catalog = cat! {
    en: "Failed to set ({label}): {error}",
    hu: "Nem sikerült beállítani ({label}): {error}",
    de: "Einstellen fehlgeschlagen ({label}): {error}",
    es: "No se pudo configurar ({label}): {error}",
    it: "Impostazione non riuscita ({label}): {error}",
    zh: "无法设置（{label}）：{error}",
    ru: "Не удалось настроить ({label}): {error}",
    uk: "Не вдалося налаштувати ({label}): {error}",
};

pub const MACOS_SET_OK: Catalog = cat! {
    en: "Set: {label}",
    hu: "Beállítva: {label}",
    de: "Gesetzt: {label}",
    es: "Configurado: {label}",
    it: "Impostato: {label}",
    zh: "已设置：{label}",
    ru: "Настроено: {label}",
    uk: "Налаштовано: {label}",
};

pub const MACOS_DOCK: Catalog = cat! {
    en: "Dock auto-hide",
    hu: "Dock automatikus elrejtése",
    de: "Dock automatisch ausblenden",
    es: "Ocultar automáticamente el Dock",
    it: "Nascondi automaticamente il Dock",
    zh: "程序坞自动隐藏",
    ru: "Автоскрытие Dock",
    uk: "Автоприховування Dock",
};

pub const MACOS_FLUSHDNS_OK: Catalog = cat! {
    en: "DNS cache flushed.",
    hu: "DNS-gyorsítótár ürítve.",
    de: "DNS-Cache geleert.",
    es: "Caché DNS vaciada.",
    it: "Cache DNS svuotata.",
    zh: "DNS 缓存已刷新。",
    ru: "DNS-кэш очищен.",
    uk: "DNS-кеш очищено.",
};

pub const MACOS_HINT: Catalog = cat! {
    en: "Full folder size in Finder: open a folder, Cmd+J, enable 'Calculate all sizes', then 'Use as Defaults'.",
    hu: "Teljes mappaméret Finderben: nyiss meg egy mappát, Cmd+J, pipáld be a 'Calculate all sizes'-t, majd 'Use as Defaults'.",
    de: "Ordnergröße im Finder: Ordner öffnen, Cmd+J, 'Calculate all sizes' aktivieren, dann 'Use as Defaults'.",
    es: "Tamaño total en Finder: abre una carpeta, Cmd+J, activa 'Calculate all sizes' y luego 'Use as Defaults'.",
    it: "Dimensione cartella in Finder: apri una cartella, Cmd+J, attiva 'Calculate all sizes', poi 'Use as Defaults'.",
    zh: "在 Finder 中显示完整文件夹大小：打开文件夹，按 Cmd+J，勾选 “Calculate all sizes”，再选 “Use as Defaults”。",
    ru: "Полный размер папки в Finder: откройте папку, Cmd+J, включите 'Calculate all sizes', затем 'Use as Defaults'.",
    uk: "Повний розмір теки у Finder: відкрийте теку, Cmd+J, увімкніть 'Calculate all sizes', потім 'Use as Defaults'.",
};

// —— Gen ————————————————————————————————————————————————

pub const GEN_BYTES_ZERO: Catalog = cat! {
    en: "Byte count must be greater than zero.",
    hu: "A byte-számnak nagyobbnak kell lennie nullánál.",
    de: "Die Byte-Anzahl muss größer als null sein.",
    es: "El número de bytes debe ser mayor que cero.",
    it: "Il numero di byte deve essere maggiore di zero.",
    zh: "字节数必须大于零。",
    ru: "Количество байт должно быть больше нуля.",
    uk: "Кількість байтів має бути більшою за нуль.",
};

pub const GEN_OPENSSL_FAIL: Catalog = cat! {
    en: "openssl rand {flag} {bytes} failed (code: {code}): {stderr}",
    hu: "Az openssl rand {flag} {bytes} sikertelen volt (kód: {code}): {stderr}",
    de: "openssl rand {flag} {bytes} fehlgeschlagen (Code: {code}): {stderr}",
    es: "openssl rand {flag} {bytes} falló (código: {code}): {stderr}",
    it: "openssl rand {flag} {bytes} non riuscito (codice: {code}): {stderr}",
    zh: "openssl rand {flag} {bytes} 失败（代码：{code}）：{stderr}",
    ru: "openssl rand {flag} {bytes} не удалось (код: {code}): {stderr}",
    uk: "openssl rand {flag} {bytes} не вдалося (код: {code}): {stderr}",
};

pub const GEN_OPENSSL_START: Catalog = cat! {
    en: "Failed to start openssl: {error}",
    hu: "Nem sikerült elindítani az openssl-t: {error}",
    de: "openssl konnte nicht gestartet werden: {error}",
    es: "No se pudo iniciar openssl: {error}",
    it: "Impossibile avviare openssl: {error}",
    zh: "无法启动 openssl：{error}",
    ru: "Не удалось запустить openssl: {error}",
    uk: "Не вдалося запустити openssl: {error}",
};

pub const GEN_RAND_SHORT: Catalog = cat! {
    en: "openssl rand returned too few bytes",
    hu: "az openssl rand túl kevés byte-ot adott vissza",
    de: "openssl rand hat zu wenige Bytes zurückgegeben",
    es: "openssl rand devolvió demasiados pocos bytes",
    it: "openssl rand ha restituito troppi pochi byte",
    zh: "openssl rand 返回的字节太少",
    ru: "openssl rand вернул слишком мало байт",
    uk: "openssl rand повернув замало байтів",
};

// —— Git setup ————————————————————————————————————————————————

pub const SETUP_FAIL: Catalog = cat! {
    en: "Git setup failed: {error}",
    hu: "Git setup sikertelen: {error}",
    de: "Git-Setup fehlgeschlagen: {error}",
    es: "Configuración de git fallida: {error}",
    it: "Setup git non riuscito: {error}",
    zh: "Git 设置失败：{error}",
    ru: "Настройка git не удалась: {error}",
    uk: "Налаштування git не вдалося: {error}",
};

pub const SETUP_REPO_NAME: Catalog = cat! {
    en: "Repo name",
    hu: "Repo név",
    de: "Repo-Name",
    es: "Nombre del repo",
    it: "Nome del repo",
    zh: "仓库名称",
    ru: "Имя репозитория",
    uk: "Назва репозиторію",
};

pub const SETUP_VISIBILITY: Catalog = cat! {
    en: "Visibility (public/private)",
    hu: "Láthatóság (public/private)",
    de: "Sichtbarkeit (public/private)",
    es: "Visibilidad (public/private)",
    it: "Visibilità (public/private)",
    zh: "可见性（public/private）",
    ru: "Видимость (public/private)",
    uk: "Видимість (public/private)",
};

pub const SETUP_ORG: Catalog = cat! {
    en: "Organization (empty = your user)",
    hu: "Organization (üres = saját felhasználó)",
    de: "Organisation (leer = dein Benutzer)",
    es: "Organización (vacío = tu usuario)",
    it: "Organizzazione (vuoto = il tuo utente)",
    zh: "组织（留空 = 你的用户）",
    ru: "Организация (пусто = ваш пользователь)",
    uk: "Організація (порожньо = ваш користувач)",
};

pub const SETUP_NEED_IDENTITY: Catalog = cat! {
    en: "git user.name and user.email are required",
    hu: "a git user.name és user.email megadása kötelező",
    de: "git user.name und user.email sind erforderlich",
    es: "git user.name y user.email son obligatorios",
    it: "git user.name e user.email sono obbligatori",
    zh: "必须填写 git user.name 和 user.email",
    ru: "нужно указать git user.name и user.email",
    uk: "потрібно вказати git user.name і user.email",
};

pub const SETUP_REMOTE_EXISTS: Catalog = cat! {
    en: "Remote repo already exists: {name}",
    hu: "A remote repo már létezik: {name}",
    de: "Remote-Repo existiert bereits: {name}",
    es: "El repo remoto ya existe: {name}",
    it: "Il repo remoto esiste già: {name}",
    zh: "远程仓库已存在：{name}",
    ru: "Удалённый репозиторий уже существует: {name}",
    uk: "Віддалений репозиторій уже існує: {name}",
};

pub const SETUP_OVERWRITE: Catalog = cat! {
    en: "Overwrite? Old code will be moved to a backup branch",
    hu: "Felülírod? A régi kód egy backup branchre kerül",
    de: "Überschreiben? Alter Code wandert auf einen Backup-Branch",
    es: "¿Sobrescribir? El código antiguo irá a una rama de respaldo",
    it: "Sovrascrivere? Il vecchio codice andrà su un branch di backup",
    zh: "要覆盖吗？旧代码会移到备份分支",
    ru: "Перезаписать? Старый код будет перенесён в backup-ветку",
    uk: "Перезаписати? Старий код буде перенесено в backup-гілку",
};

pub const SETUP_DONE_REPO: Catalog = cat! {
    en: "Done. Repo: https://github.com/{name}",
    hu: "Kész. Repo: https://github.com/{name}",
    de: "Fertig. Repo: https://github.com/{name}",
    es: "Listo. Repo: https://github.com/{name}",
    it: "Fatto. Repo: https://github.com/{name}",
    zh: "完成。仓库：https://github.com/{name}",
    ru: "Готово. Репозиторий: https://github.com/{name}",
    uk: "Готово. Репозиторій: https://github.com/{name}",
};

pub const SETUP_GH_OK: Catalog = cat! {
    en: "GitHub CLI is available.",
    hu: "GitHub CLI megvan.",
    de: "GitHub CLI ist vorhanden.",
    es: "GitHub CLI está disponible.",
    it: "GitHub CLI è disponibile.",
    zh: "已安装 GitHub CLI。",
    ru: "GitHub CLI доступен.",
    uk: "GitHub CLI доступний.",
};

pub const SETUP_GH_MISSING: Catalog = cat! {
    en: "GitHub CLI (gh) is not installed.",
    hu: "A GitHub CLI (gh) nincs telepítve.",
    de: "GitHub CLI (gh) ist nicht installiert.",
    es: "GitHub CLI (gh) no está instalado.",
    it: "GitHub CLI (gh) non è installato.",
    zh: "未安装 GitHub CLI (gh)。",
    ru: "GitHub CLI (gh) не установлен.",
    uk: "GitHub CLI (gh) не встановлено.",
};

pub const SETUP_BREW_MISSING: Catalog = cat! {
    en: "Homebrew is not installed either. Install gh: https://cli.github.com/",
    hu: "a Homebrew sincs telepítve. Telepítsd a gh-t: https://cli.github.com/",
    de: "Homebrew fehlt ebenfalls. Installiere gh: https://cli.github.com/",
    es: "Homebrew tampoco está instalado. Instala gh: https://cli.github.com/",
    it: "Manca anche Homebrew. Installa gh: https://cli.github.com/",
    zh: "也未安装 Homebrew。请安装 gh：https://cli.github.com/",
    ru: "Homebrew тоже не установлен. Установите gh: https://cli.github.com/",
    uk: "Homebrew також не встановлено. Встановіть gh: https://cli.github.com/",
};

pub const SETUP_INSTALL_GH: Catalog = cat! {
    en: "Install via Homebrew (`brew install gh`)",
    hu: "Telepítsem Homebrew-val (`brew install gh`)",
    de: "Per Homebrew installieren (`brew install gh`)",
    es: "Instalar con Homebrew (`brew install gh`)",
    it: "Installare con Homebrew (`brew install gh`)",
    zh: "通过 Homebrew 安装（`brew install gh`）",
    ru: "Установить через Homebrew (`brew install gh`)",
    uk: "Встановити через Homebrew (`brew install gh`)",
};

pub const SETUP_GH_REQUIRED: Catalog = cat! {
    en: "setup cannot continue without gh",
    hu: "gh nélkül a setup nem folytatható",
    de: "Setup ohne gh nicht möglich",
    es: "el setup no puede continuar sin gh",
    it: "il setup non può continuare senza gh",
    zh: "没有 gh 无法继续设置",
    ru: "без gh настройка невозможна",
    uk: "без gh налаштування неможливе",
};

pub const SETUP_GH_INSTALLING: Catalog = cat! {
    en: "Installing gh...",
    hu: "gh telepítése...",
    de: "gh wird installiert...",
    es: "Instalando gh...",
    it: "Installazione di gh...",
    zh: "正在安装 gh...",
    ru: "Установка gh...",
    uk: "Встановлення gh...",
};

pub const SETUP_GH_PATH: Catalog = cat! {
    en: "gh is still not on PATH after install",
    hu: "a gh telepítése után sem elérhető a PATH-ban",
    de: "gh ist nach der Installation nicht im PATH",
    es: "gh sigue sin estar en el PATH tras instalarse",
    it: "gh non è ancora nel PATH dopo l'installazione",
    zh: "安装后仍无法在 PATH 中找到 gh",
    ru: "после установки gh всё ещё нет в PATH",
    uk: "після встановлення gh досі немає в PATH",
};

pub const SETUP_GH_INSTALLED: Catalog = cat! {
    en: "gh installed.",
    hu: "gh telepítve.",
    de: "gh installiert.",
    es: "gh instalado.",
    it: "gh installato.",
    zh: "gh 已安装。",
    ru: "gh установлен.",
    uk: "gh встановлено.",
};

pub const SETUP_GH_LOGIN: Catalog = cat! {
    en: "Not logged in to GitHub CLI. Starting `gh auth login`...",
    hu: "Nincs bejelentkezve a GitHub CLI-be. Indítom a `gh auth login`-t...",
    de: "Nicht bei GitHub CLI angemeldet. Starte `gh auth login`...",
    es: "No has iniciado sesión en GitHub CLI. Iniciando `gh auth login`...",
    it: "Non sei autenticato in GitHub CLI. Avvio `gh auth login`...",
    zh: "尚未登录 GitHub CLI。正在启动 `gh auth login`...",
    ru: "Нет входа в GitHub CLI. Запускаю `gh auth login`...",
    uk: "Немає входу в GitHub CLI. Запускаю `gh auth login`...",
};

pub const SETUP_GH_AUTH_FAIL: Catalog = cat! {
    en: "still not logged in after gh auth login",
    hu: "a gh auth login után sem vagy bejelentkezve",
    de: "nach gh auth login immer noch nicht angemeldet",
    es: "sigues sin sesión tras gh auth login",
    it: "ancora non autenticato dopo gh auth login",
    zh: "gh auth login 之后仍未登录",
    ru: "после gh auth login вход всё ещё не выполнен",
    uk: "після gh auth login вхід досі не виконано",
};

pub const SETUP_GIT_INIT: Catalog = cat! {
    en: "No git repo — running `git init`...",
    hu: "Nincs git repo — `git init`...",
    de: "Kein Git-Repo — starte `git init`...",
    es: "No hay repo git — ejecutando `git init`...",
    it: "Nessun repo git — eseguo `git init`...",
    zh: "没有 git 仓库 — 正在执行 `git init`...",
    ru: "Нет git-репозитория — запускаю `git init`...",
    uk: "Немає git-репозиторію — запускаю `git init`...",
};

pub const SETUP_CREATE_REPO: Catalog = cat! {
    en: "Creating new {visibility} repo: {name}",
    hu: "Új {visibility} repo létrehozása: {name}",
    de: "Neues {visibility}-Repo wird erstellt: {name}",
    es: "Creando nuevo repo {visibility}: {name}",
    it: "Creazione nuovo repo {visibility}: {name}",
    zh: "正在创建新的 {visibility} 仓库：{name}",
    ru: "Создание нового {visibility}-репозитория: {name}",
    uk: "Створення нового {visibility}-репозиторію: {name}",
};

pub const SETUP_FETCH_BACKUP: Catalog = cat! {
    en: "Fetching remote for backup...",
    hu: "Remote letöltése backuphoz...",
    de: "Remote wird für Backup geholt...",
    es: "Obteniendo remoto para respaldo...",
    it: "Fetch del remoto per il backup...",
    zh: "正在获取远程内容以便备份...",
    ru: "Загрузка remote для резервной копии...",
    uk: "Отримання remote для резервної копії...",
};

pub const SETUP_BACKUP_BRANCH: Catalog = cat! {
    en: "Saving old code to branch: {branch}",
    hu: "Régi kód mentése branchre: {branch}",
    de: "Alter Code wird auf Branch gesichert: {branch}",
    es: "Guardando código antiguo en la rama: {branch}",
    it: "Salvataggio del vecchio codice sul branch: {branch}",
    zh: "正在将旧代码保存到分支：{branch}",
    ru: "Сохранение старого кода в ветку: {branch}",
    uk: "Збереження старого коду в гілку: {branch}",
};

pub const SETUP_BACKUP_PUSHED: Catalog = cat! {
    en: "Backup branch pushed: {branch}",
    hu: "Backup branch pusholva: {branch}",
    de: "Backup-Branch gepusht: {branch}",
    es: "Rama de respaldo enviada: {branch}",
    it: "Branch di backup inviato: {branch}",
    zh: "备份分支已推送：{branch}",
    ru: "Backup-ветка отправлена: {branch}",
    uk: "Backup-гілку надіслано: {branch}",
};

pub const SETUP_NO_BACKUP: Catalog = cat! {
    en: "No existing remote content on `{branch}` — nothing to back up.",
    hu: "Nincs meglévő remote tartalom a(z) `{branch}` branchen — nincs mit backupolni.",
    de: "Kein Remote-Inhalt auf `{branch}` — nichts zu sichern.",
    es: "No hay contenido remoto en `{branch}` — nada que respaldar.",
    it: "Nessun contenuto remoto su `{branch}` — niente da salvare.",
    zh: "`{branch}` 分支上没有远程内容 — 无需备份。",
    ru: "Нет удалённого содержимого на `{branch}` — нечего сохранять.",
    uk: "Немає віддаленого вмісту на `{branch}` — немає що зберігати.",
};

pub const SETUP_FORCE_PUSH: Catalog = cat! {
    en: "Force-pushing local code: {local} → origin/{remote}",
    hu: "Helyi kód felülírása force push-sal: {local} → origin/{remote}",
    de: "Lokaler Code wird per Force-Push überschrieben: {local} → origin/{remote}",
    es: "Sobrescribiendo con force push: {local} → origin/{remote}",
    it: "Sovrascrittura con force push: {local} → origin/{remote}",
    zh: "正在强制推送本地代码：{local} → origin/{remote}",
    ru: "Принудительная отправка локального кода: {local} → origin/{remote}",
    uk: "Примусове надсилання локального коду: {local} → origin/{remote}",
};

pub const SETUP_DEFAULT_BRANCH_ERR: Catalog = cat! {
    en: "failed to query default branch",
    hu: "nem sikerült lekérdezni a default branchet",
    de: "Standard-Branch konnte nicht abgefragt werden",
    es: "no se pudo consultar la rama predeterminada",
    it: "impossibile interrogare il branch predefinito",
    zh: "无法查询默认分支",
    ru: "не удалось запросить ветку по умолчанию",
    uk: "не вдалося запитати гілку за замовчуванням",
};

pub const SETUP_GH_USER_ERR: Catalog = cat! {
    en: "failed to query GitHub username",
    hu: "nem sikerült lekérdezni a GitHub felhasználónevet",
    de: "GitHub-Benutzername konnte nicht abgefragt werden",
    es: "no se pudo consultar el usuario de GitHub",
    it: "impossibile interrogare il nome utente GitHub",
    zh: "无法查询 GitHub 用户名",
    ru: "не удалось запросить имя пользователя GitHub",
    uk: "не вдалося запитати ім'я користувача GitHub",
};

pub const SETUP_GH_USER_EMPTY: Catalog = cat! {
    en: "empty GitHub username",
    hu: "üres GitHub felhasználónév",
    de: "leerer GitHub-Benutzername",
    es: "nombre de usuario de GitHub vacío",
    it: "nome utente GitHub vuoto",
    zh: "GitHub 用户名为空",
    ru: "пустое имя пользователя GitHub",
    uk: "порожнє ім'я користувача GitHub",
};

// —— Download / YouTube ————————————————————————————————————————————————

pub const YT_DISCLAIMER: Catalog = cat! {
    en: "For educational purposes only.",
    hu: "For educational purposes only.",
    de: "For educational purposes only.",
    es: "For educational purposes only.",
    it: "For educational purposes only.",
    zh: "For educational purposes only.",
    ru: "For educational purposes only.",
    uk: "For educational purposes only.",
};

pub const YT_URL_PROMPT: Catalog = cat! {
    en: "YouTube URL",
    hu: "YouTube URL",
    de: "YouTube-URL",
    es: "URL de YouTube",
    it: "URL di YouTube",
    zh: "YouTube 链接",
    ru: "URL YouTube",
    uk: "URL YouTube",
};

pub const YT_INVALID_URL: Catalog = cat! {
    en: "Not a YouTube URL. Use youtube.com or youtu.be.",
    hu: "Ez nem YouTube-URL. Használj youtube.com-ot vagy youtu.be-t.",
    de: "Keine YouTube-URL. Verwende youtube.com oder youtu.be.",
    es: "No es una URL de YouTube. Usa youtube.com o youtu.be.",
    it: "Non è un URL di YouTube. Usa youtube.com o youtu.be.",
    zh: "不是 YouTube 链接。请使用 youtube.com 或 youtu.be。",
    ru: "Это не URL YouTube. Используйте youtube.com или youtu.be.",
    uk: "Це не URL YouTube. Використовуйте youtube.com або youtu.be.",
};

pub const YT_QUALITY_PROMPT: Catalog = cat! {
    en: "Quality ({options})",
    hu: "Minőség ({options})",
    de: "Qualität ({options})",
    es: "Calidad ({options})",
    it: "Qualità ({options})",
    zh: "质量（{options}）",
    ru: "Качество ({options})",
    uk: "Якість ({options})",
};

pub const YT_QUALITY_INVALID: Catalog = cat! {
    en: "Choose 270p, 480p, 720p, 1080p, or 1440p.",
    hu: "Válassz: 270p, 480p, 720p, 1080p vagy 1440p.",
    de: "Wähle 270p, 480p, 720p, 1080p oder 1440p.",
    es: "Elige 270p, 480p, 720p, 1080p o 1440p.",
    it: "Scegli 270p, 480p, 720p, 1080p o 1440p.",
    zh: "请选择 270p、480p、720p、1080p 或 1440p。",
    ru: "Выберите 270p, 480p, 720p, 1080p или 1440p.",
    uk: "Оберіть 270p, 480p, 720p, 1080p або 1440p.",
};

pub const MUSIC_URL_PROMPT: Catalog = cat! {
    en: "YouTube / YouTube Music / Spotify URL",
    hu: "YouTube / YouTube Music / Spotify URL",
    de: "YouTube- / YouTube-Music- / Spotify-URL",
    es: "URL de YouTube / YouTube Music / Spotify",
    it: "URL di YouTube / YouTube Music / Spotify",
    zh: "YouTube / YouTube Music / Spotify 链接",
    ru: "URL YouTube / YouTube Music / Spotify",
    uk: "URL YouTube / YouTube Music / Spotify",
};

pub const MUSIC_INVALID_URL: Catalog = cat! {
    en: "Not a supported URL. Use YouTube, YouTube Music, or Spotify.",
    hu: "Nem támogatott URL. Használj YouTube, YouTube Music vagy Spotify linket.",
    de: "Keine unterstützte URL. Verwende YouTube, YouTube Music oder Spotify.",
    es: "URL no compatible. Usa YouTube, YouTube Music o Spotify.",
    it: "URL non supportato. Usa YouTube, YouTube Music o Spotify.",
    zh: "不支持的链接。请使用 YouTube、YouTube Music 或 Spotify。",
    ru: "Неподдерживаемый URL. Используйте YouTube, YouTube Music или Spotify.",
    uk: "Непідтримуваний URL. Використовуйте YouTube, YouTube Music або Spotify.",
};

pub const MUSIC_QUALITY_PROMPT: Catalog = cat! {
    en: "Audio quality ({options})",
    hu: "Hangminőség ({options})",
    de: "Audioqualität ({options})",
    es: "Calidad de audio ({options})",
    it: "Qualità audio ({options})",
    zh: "音频质量（{options}）",
    ru: "Качество аудио ({options})",
    uk: "Якість аудіо ({options})",
};

pub const MUSIC_QUALITY_INVALID: Catalog = cat! {
    en: "Choose 128k, 192k, 256k, or 320k.",
    hu: "Válassz: 128k, 192k, 256k vagy 320k.",
    de: "Wähle 128k, 192k, 256k oder 320k.",
    es: "Elige 128k, 192k, 256k o 320k.",
    it: "Scegli 128k, 192k, 256k o 320k.",
    zh: "请选择 128k、192k、256k 或 320k。",
    ru: "Выберите 128k, 192k, 256k или 320k.",
    uk: "Оберіть 128k, 192k, 256k або 320k.",
};

pub const MUSIC_FAIL: Catalog = cat! {
    en: "Music download failed: {error}",
    hu: "Zeneletöltés sikertelen: {error}",
    de: "Musik-Download fehlgeschlagen: {error}",
    es: "La descarga de música falló: {error}",
    it: "Download musica non riuscito: {error}",
    zh: "音乐下载失败：{error}",
    ru: "Не удалось скачать музыку: {error}",
    uk: "Не вдалося завантажити музику: {error}",
};

pub const YT_FOLDER_PROMPT: Catalog = cat! {
    en: "Choose a download folder",
    hu: "Válaszd ki a letöltési mappát",
    de: "Download-Ordner wählen",
    es: "Elige la carpeta de descarga",
    it: "Scegli la cartella di download",
    zh: "选择下载文件夹",
    ru: "Выберите папку для загрузки",
    uk: "Оберіть теку для завантаження",
};

pub const YT_FOLDER_PATH_PROMPT: Catalog = cat! {
    en: "Download folder path",
    hu: "Letöltési mappa elérési útja",
    de: "Pfad des Download-Ordners",
    es: "Ruta de la carpeta de descarga",
    it: "Percorso della cartella di download",
    zh: "下载文件夹路径",
    ru: "Путь к папке загрузки",
    uk: "Шлях до теки завантаження",
};

pub const YT_DOWNLOADING: Catalog = cat! {
    en: "Downloading to {path}...",
    hu: "Letöltés ide: {path}...",
    de: "Download nach {path}...",
    es: "Descargando en {path}...",
    it: "Download in {path}...",
    zh: "正在下载到 {path}...",
    ru: "Загрузка в {path}...",
    uk: "Завантаження до {path}...",
};

pub const YT_DONE: Catalog = cat! {
    en: "Download complete.",
    hu: "Letöltés kész.",
    de: "Download abgeschlossen.",
    es: "Descarga completada.",
    it: "Download completato.",
    zh: "下载完成。",
    ru: "Загрузка завершена.",
    uk: "Завантаження завершено.",
};

pub const YT_FAIL: Catalog = cat! {
    en: "YouTube download failed: {error}",
    hu: "YouTube-letöltés sikertelen: {error}",
    de: "YouTube-Download fehlgeschlagen: {error}",
    es: "La descarga de YouTube falló: {error}",
    it: "Download da YouTube non riuscito: {error}",
    zh: "YouTube 下载失败：{error}",
    ru: "Не удалось скачать с YouTube: {error}",
    uk: "Не вдалося завантажити з YouTube: {error}",
};

pub const YT_TOOL_OK: Catalog = cat! {
    en: "{tool} is available.",
    hu: "{tool} megvan.",
    de: "{tool} ist vorhanden.",
    es: "{tool} está disponible.",
    it: "{tool} è disponibile.",
    zh: "已安装 {tool}。",
    ru: "{tool} доступен.",
    uk: "{tool} доступний.",
};

pub const YT_TOOL_MISSING: Catalog = cat! {
    en: "{tool} is not installed.",
    hu: "A(z) {tool} nincs telepítve.",
    de: "{tool} ist nicht installiert.",
    es: "{tool} no está instalado.",
    it: "{tool} non è installato.",
    zh: "未安装 {tool}。",
    ru: "{tool} не установлен.",
    uk: "{tool} не встановлено.",
};

pub const YT_BREW_MISSING: Catalog = cat! {
    en: "Homebrew is not installed either. Install it from https://brew.sh/",
    hu: "a Homebrew sincs telepítve. Telepítsd: https://brew.sh/",
    de: "Homebrew fehlt ebenfalls. Installiere es unter https://brew.sh/",
    es: "Homebrew tampoco está instalado. Instálalo desde https://brew.sh/",
    it: "Manca anche Homebrew. Installalo da https://brew.sh/",
    zh: "也未安装 Homebrew。请从 https://brew.sh/ 安装。",
    ru: "Homebrew тоже не установлен. Установите его с https://brew.sh/",
    uk: "Homebrew також не встановлено. Встановіть його з https://brew.sh/",
};

pub const YT_INSTALLING: Catalog = cat! {
    en: "Installing {tool}...",
    hu: "{tool} telepítése...",
    de: "{tool} wird installiert...",
    es: "Instalando {tool}...",
    it: "Installazione di {tool}...",
    zh: "正在安装 {tool}...",
    ru: "Установка {tool}...",
    uk: "Встановлення {tool}...",
};

pub const YT_PATH: Catalog = cat! {
    en: "{tool} is still not on PATH after install",
    hu: "a(z) {tool} telepítése után sem elérhető a PATH-ban",
    de: "{tool} ist nach der Installation nicht im PATH",
    es: "{tool} sigue sin estar en el PATH tras instalarse",
    it: "{tool} non è ancora nel PATH dopo l'installazione",
    zh: "安装后仍无法在 PATH 中找到 {tool}",
    ru: "после установки {tool} всё ещё нет в PATH",
    uk: "після встановлення {tool} досі немає в PATH",
};

pub const YT_TOOL_INSTALLED: Catalog = cat! {
    en: "{tool} installed.",
    hu: "{tool} telepítve.",
    de: "{tool} installiert.",
    es: "{tool} instalado.",
    it: "{tool} installato.",
    zh: "{tool} 已安装。",
    ru: "{tool} установлен.",
    uk: "{tool} встановлено.",
};

pub const YT_NOT_A_DIR: Catalog = cat! {
    en: "not a directory: {path}",
    hu: "nem mappa: {path}",
    de: "kein Ordner: {path}",
    es: "no es una carpeta: {path}",
    it: "non è una cartella: {path}",
    zh: "不是文件夹：{path}",
    ru: "это не папка: {path}",
    uk: "це не тека: {path}",
};

pub const INSTALL_FAIL: Catalog = cat! {
    en: "Install failed: {error}",
    hu: "Telepítés sikertelen: {error}",
    de: "Installation fehlgeschlagen: {error}",
    es: "La instalación falló: {error}",
    it: "Installazione non riuscita: {error}",
    zh: "安装失败：{error}",
    ru: "Установка не удалась: {error}",
    uk: "Встановлення не вдалося: {error}",
};
