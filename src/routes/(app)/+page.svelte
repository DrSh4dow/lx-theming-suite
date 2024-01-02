<script lang="ts">
	import { searchBarInput } from '$lib/stores/searchBarStore';
	import { onMount } from 'svelte';
	import type { z } from 'zod';

	import { getGtkThemes } from '../../commands/getGtkThemes';
	import type { gtkThemesSchema } from '../../commands/getGtkThemes';
	import Preview from './Preview.svelte';

	let themes: z.infer<typeof gtkThemesSchema> = [];

	onMount(() => {
		getGtkThemes()
			.then((t) => (themes = t))
			.catch(console.error);
	});
</script>

<section class="w-full overflow-x-scroll rounded-lg border shadow-sm">
	<div
		class="grid w-full cursor-default grid-cols-[min-content_min-content_1fr_min-content] text-sm"
	>
		<h1 class="select-none px-4 py-3 text-left align-middle font-medium text-muted-foreground">
			Preview
		</h1>
		<h1 class="select-none px-4 py-3 text-left align-middle font-medium text-muted-foreground">
			Name
		</h1>
		<h1 class="select-none px-4 py-3 text-left align-middle font-medium text-muted-foreground">
			Description
		</h1>
		<h1 class="select-none px-4 py-3 text-left align-middle font-medium text-muted-foreground">
			Select
		</h1>
		<hr class="col-span-full" />
		{#each themes.filter((t) => t.name
				.toLowerCase()
				.trim()
				.includes($searchBarInput.toLowerCase().trim())) as theme}
			<div class="p-4">
				<Preview src={theme.preview} />
			</div>
			<div class="flex max-w-64 items-center p-4 md:min-w-32 lg:min-w-40">
				<h1 class="font-medium">{theme.name}</h1>
			</div>
			<div class="flex items-center p-4">
				<h1 class="line-clamp-4 whitespace-pre-wrap break-normal">{theme.description}</h1>
			</div>
			<div class="flex items-center justify-center p-4">
				<button
					type="button"
					class="rounded-md bg-primary px-10 py-1.5 text-primary-foreground transition-colors duration-75 hover:bg-primary/80 active:bg-primary lg:px-14 xl:px-20"
					>Apply</button
				>
			</div>
			<hr class="col-span-full" />
		{/each}
	</div>
</section>
