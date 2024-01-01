<script>
	import { currentTheme } from '$lib/stores/themeProvider';
	import { cn } from '$lib/utils/cn';
	import { MoonIcon, PaletteIcon, SearchIcon, SunIcon } from 'lucide-svelte';
</script>

<header
	class="grid w-full cursor-default grid-cols-[64px_1fr] justify-center border-b sm:justify-normal md:grid-cols-[280px_1fr] bg-card"
>
	<div class="flex items-center justify-center gap-x-2 border-r md:justify-start md:px-8">
		<PaletteIcon class="h-6 w-6" />
		<h5 class="hidden select-none font-semibold md:block">Linux Theming Suite</h5>
	</div>
	<div class="flex items-center justify-between px-4 sm:px-8">
		<label class="relative">
			<input
				type="text"
				placeholder="Search themes..."
				class="form-input h-8 w-full appearance-none rounded-md border bg-background pl-8 text-foreground shadow-none focus:border-muted-foreground focus:ring-muted-foreground"
			/>
			<div class="absolute left-2.5 top-1/2 -translate-y-1/2">
				<SearchIcon color={'hsl(var(--muted-foreground))'} size={16} />
			</div>
		</label>
		<div>
			<label class="flex select-none items-center justify-center gap-x-2">
				<MoonIcon class="h-4 w-4" />
				<div class="relative inline-flex items-center">
					<input
						type="checkbox"
						checked={$currentTheme === 'light'}
						on:change={(e) => {
							$currentTheme = e.currentTarget.checked ? 'light' : 'dark';
						}}
						class="peer sr-only"
					/>
					<div
						class={cn(
							'peer h-5 w-10 cursor-pointer rounded-full bg-muted transition-colors',
							"after:absolute after:start-[4px] after:top-[2px] after:h-4 after:w-4 after:rounded-full after:bg-background after:transition-all after:content-[''] ",
							'peer-checked:bg-foreground peer-checked:after:translate-x-full peer-focus:outline-none peer-focus:ring-2 peer-focus:ring-muted-foreground rtl:peer-checked:after:-translate-x-full'
						)}
					/>
				</div>
				<SunIcon class="h-4 w-4" />
			</label>
		</div>
	</div>
</header>
