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

          const toggle = document.querySelector('#theme-toggle');
          if (toggle) {
              toggle.querySelectorAll('button').forEach(btn => {
                  btn.disabled = btn.dataset.theme === theme;
              });
          }
      }

      applyTheme(getStoredTheme());

      const toggle = document.querySelector('#theme-toggle');
      if (toggle) {
          toggle.querySelectorAll('button').forEach(btn => {
              btn.addEventListener('click', function() {
                  const theme = this.dataset.theme;
                  localStorage.setItem(STORAGE_KEY, theme);
                  applyTheme(theme);
              });
          });
      }

      window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', function() {
          const stored = getStoredTheme();
          if (stored === SYSTEM) {
              applyTheme(SYSTEM);
          }
      });
  })();
