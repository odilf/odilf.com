use maud::{Markup, PreEscaped, html};

pub fn back() -> Markup {
    html! {
        ."faint sticky absolute top-0 left-0 right-0 z-10 bg-neutral" {
            a href=".." { "<-- (back)" }
        }
    }
}

pub fn tag(topic: impl AsRef<str>) -> Markup {
    html! {
        ."content-center text-center whitespace-nowrap px-1 text-xs rounded-xs
        opacity-80 w-fit h-fit outline-1 outline-primary/50 text-primary py-[1px]" {
            (topic.as_ref())
        }
    }
}

pub fn theme_toggle() -> Markup {
    html! {
        ."flex flex-col opacity-80" {
            ."text-bold text-center" { "Theme" }
            #theme-toggle."flex gap-1 transition-all duration-200" {
                button."w-[8ch] px-0 text-center py-1 text-xs rounded-xs outline-1 outline-white" data-theme="dark" title="Dark mode"
                { "Dark" }

                button."w-[8ch] px-0 text-center py-1 text-xs rounded-xs outline-1 outline-white" data-theme="system" title="System mode"
                { "System" }

                button."w-[8ch] px-0 text-center py-1 text-xs rounded-xs outline-1 outline-white" data-theme="light" title="Light mode"
                { "Light" }
            }
        }

        script {
            (PreEscaped(r#"
            (function() {
                const STORAGE_KEY = 'theme-preference';
                const LIGHT = 'light';
                const DARK = 'dark';
                const SYSTEM = 'system';

                const getSystemTheme = () => window.matchMedia('(prefers-color-scheme: dark)').matches ? DARK : LIGHT;
                const getStoredTheme = () => localStorage.getItem(STORAGE_KEY) ?? DARK;

                function applyTheme(theme) {
                    const html = document.documentElement;
                    const effectiveTheme = theme === SYSTEM ? getSystemTheme() : theme;

                    html.setAttribute('data-theme', effectiveTheme);

                    document.querySelectorAll('#theme-toggle button').forEach(btn => {
                        btn.disabled = btn.dataset.theme === theme;
                    });
                }

                applyTheme(getStoredTheme());

                document.querySelectorAll('#theme-toggle button').forEach(btn => {
                    btn.addEventListener('click', function() {
                        const theme = this.dataset.theme;
                        localStorage.setItem(STORAGE_KEY, theme);
                        applyTheme(theme);
                    });
                });

                window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', function() {
                    const stored = getStoredTheme();
                    if (stored === SYSTEM) {
                        applyTheme(SYSTEM);
                    }
                });
            })();
            "#))
        }
    }
}
