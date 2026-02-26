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
        div id="theme-toggle" ."flex gap-1 transition-all duration-200" {
            ."text-bold" { "Theme:" }
            button."px-2 py-1 text-xs rounded-xs"
                data-theme="dark" title="Dark mode" { "Dark" }

            button."px-2 py-1 text-xs rounded-xs"
                data-theme="system" title="System mode" { "System" }

            button."px-2 py-1 text-xs rounded-xs"
                data-theme="light" title="Light mode" { "Light" }
        }

        script {
            (PreEscaped(r#"
            (function() {
                const STORAGE_KEY = 'theme-preference';
                const LIGHT = 'light';
                const DARK = 'dark';
                const SYSTEM = 'system';

                function getSystemTheme() {
                    return window.matchMedia('(prefers-color-scheme: dark)').matches ? DARK : LIGHT;
                }

                function getStoredTheme() {
                    return localStorage.getItem(STORAGE_KEY) ?? SYSTEM;
                }

                function setStoredTheme(theme) {
                    localStorage.setItem(STORAGE_KEY, theme);
                }

                function applyTheme(theme) {
                    const html = document.documentElement;
                    const effectiveTheme = theme === SYSTEM ? getSystemTheme() : theme;
                    
                    // Set data attribute for CSS to use
                    html.setAttribute('data-theme', effectiveTheme);

                    // Update button states
                    document.querySelectorAll('#theme-toggle button').forEach(btn => {
                        if (btn.dataset.theme === theme) {
                            btn.disabled = true;
                        } else {
                            btn.disabled = false;
                        }
                    });
                }

                applyTheme(getStoredTheme());

                // Setup event listeners
                document.querySelectorAll('#theme-toggle button').forEach(btn => {
                    btn.addEventListener('click', function() {
                        const theme = this.dataset.theme;
                        setStoredTheme(theme);
                        applyTheme(theme);
                    });
                });

                // Listen for system theme changes
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
