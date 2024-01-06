<script lang="ts">
	import { getGtkThemes } from '$lib/commands/getGtkThemes';
	import type { gtkThemesSchema } from '$lib/commands/getGtkThemes';
	import { searchBarInput } from '$lib/stores/searchBarStore';
	import { cn } from '$lib/utils/cn';
	import { onMount } from 'svelte';
	import type { z } from 'zod';

	import Preview from './Preview.svelte';

	let themes: z.infer<typeof gtkThemesSchema> = [];

	onMount(() => {
		getGtkThemes()
			.then((t) => (themes = t))
			.catch(console.error);
	});
</script>

<section class="w-full overflow-x-scroll rounded-lg border shadow-sm @container">
	<table
		class="grid w-full cursor-default grid-cols-[min-content_max-content_1fr_min-content] text-sm @3xl:grid-cols-[min-content_max-content_1fr_min-content_min-content]"
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
		<h1
			class="hidden select-none px-4 py-3 text-left align-middle font-medium text-muted-foreground @3xl:block"
		>
			Compatibility
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
				<Preview src={theme.name} />
			</div>
			<div class="flex max-w-64 items-center p-4 md:min-w-32 lg:min-w-40">
				<h1 class="font-medium">{theme.name}</h1>
			</div>
			<div class="flex items-center p-4">
				<h1 class="line-clamp-4 whitespace-pre-wrap break-normal">{theme.description ?? ""}</h1>
			</div>
			<div
				class="hidden max-w-40 flex-col items-center justify-center gap-y-2 p-4 @md:min-w-24 @2xl:min-w-32 @3xl:flex @3xl:min-w-40 @4xl:min-w-44 @4xl:max-w-64 @7xl:min-w-64"
			>
				<figure
					class={cn(
						'w-full rounded px-2 py-1 text-sm ',
						theme.compatibility.gtk2
							? 'bg-green-200 text-green-800 dark:bg-green-600 dark:text-green-100'
							: 'bg-red-200 text-red-800 dark:bg-red-600 dark:text-red-100'
					)}
				>
					GTK-2
				</figure>
				<figure
					class={cn(
						'w-full rounded px-2 py-1 text-sm ',
						theme.compatibility.gtk3
							? 'bg-green-200 text-green-800 dark:bg-green-600 dark:text-green-100'
							: 'bg-red-200 text-red-800 dark:bg-red-600 dark:text-red-100'
					)}
				>
					GTK-3
				</figure>
				<figure
					class={cn(
						'w-full rounded px-2 py-1 text-sm ',
						theme.compatibility.gtk4
							? 'bg-green-200 text-green-800 dark:bg-green-600 dark:text-green-100'
							: 'bg-red-200 text-red-800 dark:bg-red-600 dark:text-red-100'
					)}
				>
					GTK-4
				</figure>
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
	</table>
</section>
