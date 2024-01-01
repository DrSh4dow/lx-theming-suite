<script lang="ts">
	import { cn } from '$lib/utils/cn';

	import Header from './Header.svelte';
	import NavigationBar from './NavigationBar.svelte';
</script>

<main
	class="fixed left-0 top-0 grid h-screen max-h-screen min-h-screen w-screen grid-rows-[64px_1fr] overflow-clip bg-background text-foreground"
>
	<Header />
	<div
		class="grid grid-cols-[64px_1fr] overflow-clip md:grid-cols-[160px_1fr] lg:grid-cols-[280px_1fr]"
	>
		<NavigationBar />
		<section class={cn('h-full max-h-full overflow-y-scroll p-4 sm:p-8', 'pb-16')}>
			<slot />
		</section>
	</div>
</main>
