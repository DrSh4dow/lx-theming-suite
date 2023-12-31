<script>
	import { PaletteIcon } from 'lucide-svelte';
</script>

<main class="w-screen h-screen overflow-clip bg-background text-foreground border border-red-500">
	<header
		class="h-16 w-full border-b grid grid-cols-[40px_1fr] justify-center sm:justify-normal sm:grid-cols-[280px_1fr] px-4 sm:px-8"
	>
		<div class="border-r flex items-center justify-start gap-x-2">
			<PaletteIcon class="h-6 w-6" />
			<h5 class="select-none font-semibold hidden sm:block">Linux Theming Suite</h5>
		</div>
		<div class="flex items-center justify-between pl-4 sm:pl-8">
			<input
				type="text"
				placeholder="Search themes..."
				class="form-input bg-background border text-foreground rounded-md h-8"
			/>
			<div>
				<label class="relative inline-flex items-center cursor-pointer">
					<input type="checkbox" value="" class="sr-only peer" />
					<div
						class="h-6 w-11 bg-gray-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-blue-300 dark:peer-focus:ring-blue-800 rounded-full peer dark:bg-gray-700 peer-checked:after:translate-x-full rtl:peer-checked:after:-translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:start-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600 peer-checked:bg-blue-600"
					/>
					<span class="ms-3 text-sm font-medium text-gray-900 dark:text-gray-300">Toggle me</span>
				</label>
			</div>
		</div>
	</header>
	<slot />
</main>
