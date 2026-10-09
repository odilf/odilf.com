import { browser } from '$app/env';

const STORAGE_KEY = 'theme-preference';

export type Theme = 'dark' | 'system' | 'light';

function storedTheme(): Theme {
	if (!browser) return 'dark';

	const stored = localStorage.getItem(STORAGE_KEY);
	return stored === 'light' || stored === 'system' ? stored : 'dark';
}

class ThemePreference {
	current = $state<Theme>(storedTheme());

	select(theme: Theme): void {
		this.current = theme;
		localStorage.setItem(STORAGE_KEY, theme);
	}
}

export const theme = new ThemePreference();
