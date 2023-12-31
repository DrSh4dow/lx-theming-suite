import { currentTheme } from '$lib/stores/themeProvider';

export const prerender = true;
export const ssr = false;

// This is the magic that makes the theme change work
currentTheme.subscribe((theme) => {
	if (typeof window === 'undefined' || !window || !window.document) return;

	const root = window.document.documentElement;
	root.classList.remove('light', 'dark');

	if (theme === 'system') {
		const systemTheme = window.matchMedia('(prefers-color-scheme: dark)').matches
			? 'dark'
			: 'light';

		root.classList.add(systemTheme);
		return;
	}

	root.classList.add(theme);
});
