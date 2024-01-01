<script lang="ts">
	import { AlignHorizontalDistributeCenterIcon } from 'lucide-svelte';

	export let src: string;
</script>

<div class="w-fit max-w-fit">
	<figure
		class="flex h-32 w-32 min-w-32 cursor-pointer items-center justify-center rounded-md bg-muted"
	>
		<h5 class="sr-only">{src}</h5>
		<div class="flex h-6 w-6 items-center justify-center rounded-full bg-muted-foreground/40">
			<AlignHorizontalDistributeCenterIcon class="h-3 w-3" />
		</div>
	</figure>
</div>
