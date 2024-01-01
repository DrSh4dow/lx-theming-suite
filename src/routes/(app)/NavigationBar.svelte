<script lang="ts">
	import { page } from '$app/stores';
	import { cn } from '$lib/utils/cn';
	import { ImageIcon, PaintbrushIcon, PaletteIcon, SettingsIcon, TextIcon } from 'lucide-svelte';
</script>

<nav class="h-full border-r pt-4">
	<ul>
		<li>
			<a
				href="/"
				class={cn(
					$page.url.pathname === '/'
						? 'text-gray-900 dark:text-gray-50'
						: 'text-gray-500 hover:text-gray-900 dark:text-gray-400 dark:hover:text-gray-50',
					'flex items-center gap-3 rounded-lg px-3 py-2 transition-all '
				)}
			>
				<PaletteIcon class="h-4 w-4" />
				GTK Theme
			</a>
		</li>
		<li>
			<a
				href="/qt-themes-config"
				class={cn(
					$page.url.pathname.startsWith('/qt-themes-config')
						? 'text-gray-900 dark:text-gray-50'
						: 'text-gray-500 hover:text-gray-900 dark:text-gray-400 dark:hover:text-gray-50',
					'flex items-center gap-3 rounded-lg px-3 py-2 transition-all '
				)}
			>
				<ImageIcon class="h-4 w-4" />
				QT Themes
			</a>
		</li>
		<li>
			<a
				href="/font-config"
				class={cn(
					$page.url.pathname.startsWith('/font-config')
						? 'text-gray-900 dark:text-gray-50'
						: 'text-gray-500 hover:text-gray-900 dark:text-gray-400 dark:hover:text-gray-50',
					'flex items-center gap-3 rounded-lg px-3 py-2 transition-all '
				)}
			>
				<TextIcon class="h-4 w-4" />
				Fonts
			</a>
		</li>
		<li>
			<a
				href="/icons-config"
				class={cn(
					$page.url.pathname.startsWith('/icons-config')
						? 'text-gray-900 dark:text-gray-50'
						: 'text-gray-500 hover:text-gray-900 dark:text-gray-400 dark:hover:text-gray-50',
					'flex items-center gap-3 rounded-lg px-3 py-2 transition-all '
				)}
			>
				<PaintbrushIcon class="h-4 w-4" />
				Icons
			</a>
		</li>
		<li>
			<a
				href="/settings"
				class={cn(
					$page.url.pathname.startsWith('/settings')
						? 'text-gray-900 dark:text-gray-50'
						: 'text-gray-500 hover:text-gray-900 dark:text-gray-400 dark:hover:text-gray-50',
					'flex items-center gap-3 rounded-lg px-3 py-2 transition-all '
				)}
			>
				<SettingsIcon class="h-4 w-4" />
				Settings
			</a>
		</li>
	</ul>
</nav>
