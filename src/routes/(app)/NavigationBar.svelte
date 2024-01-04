<script lang="ts">
	import { page } from '$app/stores';
	import { cn } from '$lib/utils/cn';
	import {
		// ImageIcon, PaintbrushIcon,
		PaletteIcon
		// SettingsIcon, TextIcon
	} from 'lucide-svelte';
</script>

<nav class="h-full max-h-full overflow-y-scroll border-r pt-4">
	<ul>
		<li>
			<a
				href="/"
				class={cn(
					$page.url.pathname === '/'
						? 'text-gray-900 dark:text-gray-50'
						: 'text-gray-500 hover:text-gray-900 dark:text-gray-400 dark:hover:text-gray-50',
					'flex items-center justify-center gap-3 rounded-lg px-3 py-2 transition-all md:justify-normal'
				)}
				title="GTK Themes"
			>
				<PaletteIcon class="h-5 w-5 md:h-4 md:w-4" />
				<h5 class="hidden md:block">GTK Theme</h5>
			</a>
		</li>
		<!-- <li> -->
		<!-- 	<a -->
		<!-- 		href="/qt-themes-config" -->
		<!-- 		class={cn( -->
		<!-- 			$page.url.pathname.startsWith('/qt-themes-config') -->
		<!-- 				? 'text-gray-900 dark:text-gray-50' -->
		<!-- 				: 'text-gray-500 hover:text-gray-900 dark:text-gray-400 dark:hover:text-gray-50', -->
		<!-- 			'flex items-center justify-center gap-3 rounded-lg px-3 py-2 transition-all md:justify-normal' -->
		<!-- 		)} -->
		<!-- 		title="QT Themes" -->
		<!-- 	> -->
		<!-- 		<ImageIcon class="h-5 w-5 md:h-4 md:w-4" /> -->
		<!-- 		<h5 class="hidden md:block">QT Themes</h5> -->
		<!-- 	</a> -->
		<!-- </li> -->
		<!-- <li> -->
		<!-- 	<a -->
		<!-- 		href="/font-config" -->
		<!-- 		class={cn( -->
		<!-- 			$page.url.pathname.startsWith('/font-config') -->
		<!-- 				? 'text-gray-900 dark:text-gray-50' -->
		<!-- 				: 'text-gray-500 hover:text-gray-900 dark:text-gray-400 dark:hover:text-gray-50', -->
		<!-- 			'flex items-center justify-center gap-3 rounded-lg px-3 py-2 transition-all md:justify-normal' -->
		<!-- 		)} -->
		<!-- 		title="Fonts" -->
		<!-- 	> -->
		<!-- 		<TextIcon class="h-5 w-5 md:h-4 md:w-4" /> -->
		<!-- 		<h5 class="hidden md:block">Fonts</h5> -->
		<!-- 	</a> -->
		<!-- </li> -->
		<!-- <li> -->
		<!-- 	<a -->
		<!-- 		href="/icons-config" -->
		<!-- 		class={cn( -->
		<!-- 			$page.url.pathname.startsWith('/icons-config') -->
		<!-- 				? 'text-gray-900 dark:text-gray-50' -->
		<!-- 				: 'text-gray-500 hover:text-gray-900 dark:text-gray-400 dark:hover:text-gray-50', -->
		<!-- 			'flex items-center justify-center gap-3 rounded-lg px-3 py-2 transition-all md:justify-normal' -->
		<!-- 		)} -->
		<!-- 		title="Icons" -->
		<!-- 	> -->
		<!-- 		<PaintbrushIcon class="h-5 w-5 md:h-4 md:w-4" /> -->
		<!-- 		<h5 class="hidden md:block">Icons</h5> -->
		<!-- 	</a> -->
		<!-- </li> -->
		<!-- <li> -->
		<!-- 	<a -->
		<!-- 		href="/settings" -->
		<!-- 		class={cn( -->
		<!-- 			$page.url.pathname.startsWith('/settings') -->
		<!-- 				? 'text-gray-900 dark:text-gray-50' -->
		<!-- 				: 'text-gray-500 hover:text-gray-900 dark:text-gray-400 dark:hover:text-gray-50', -->
		<!-- 			'flex items-center justify-center gap-3 rounded-lg px-3 py-2 transition-all md:justify-normal' -->
		<!-- 		)} -->
		<!-- 		title="Settings" -->
		<!-- 	> -->
		<!-- 		<SettingsIcon class="h-5 w-5 md:h-4 md:w-4" /> -->
		<!-- 		<h5 class="hidden md:block">Settings</h5> -->
		<!-- 	</a> -->
		<!-- </li> -->
	</ul>
</nav>
