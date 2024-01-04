import { invoke } from '@tauri-apps/api';
import { z } from 'zod';

export const gtkThemesSchema = z
	.object({
		name: z.string(),
		description: z.string(),
		preview: z.string(),
		path: z.string()
	})
	.array();

/**
 * @description Get the list of GTK themes available on the system
 */
export async function getGtkThemes() {
	const test = await invoke('get_gtk_themes');

	const themes = gtkThemesSchema.safeParse(test);

	if (!themes.success) {
		console.error(themes.error);
		return [];
	}

	return themes.data;
}
