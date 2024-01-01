import { page } from '$app/stores';
import { get, writable } from 'svelte/store';

export const searchBarInput = writable('');

const lastPathname = writable('');

page.subscribe((p) => {
	if (
		typeof window === 'undefined' ||
		!lastPathname ||
		!searchBarInput ||
		!p ||
		!p.url ||
		!p.url.pathname ||
		get(lastPathname) === p.url.pathname
	)
		return;

	try {
		lastPathname.set(p.url.pathname);
		searchBarInput.set('');
	} catch (e) {
		console.error(e);
	}
});
