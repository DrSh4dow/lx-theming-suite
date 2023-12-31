import { persisted } from './persistent';

export const currentTheme = persisted<'dark' | 'light'>('lx-theming-suite-theme', 'dark');

// This is the magic that makes the theme change work
currentTheme.subscribe((theme) => {
	if (typeof window === 'undefined' || !window || !window.document) return;

	const root = window.document.documentElement;
	root.classList.remove('light', 'dark');

	root.classList.add(theme);
});
