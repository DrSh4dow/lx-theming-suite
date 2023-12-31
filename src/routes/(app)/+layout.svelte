<script lang="ts">
	import Header from './Header.svelte';
</script>

<main
	class="grid h-screen w-screen grid-rows-[64px_1fr] overflow-clip border-red-500 bg-background text-foreground"
>
	<Header />
	<div class="grid grid-cols-[40px_1fr] sm:grid-cols-[280px_1fr]">
		<nav class="h-full border-r"></nav>
		<section class="overflow-y-scroll p-4 sm:p-8">
			<slot />
		</section>
	</div>
</main>
