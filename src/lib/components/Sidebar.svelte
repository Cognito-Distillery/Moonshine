<script lang="ts">
	import { page } from '$app/state';
	import { isHorizontal } from '$lib/stores/settings.svelte';
	import { t } from '$lib/i18n/index.svelte';

	const links = [
		{ href: '/', titleKey: 'nav.mashing' as const, icon: 'plus' },
		{ href: '/mashes', titleKey: 'nav.mashTun' as const, icon: 'doc' },
		{ href: '/still', titleKey: 'nav.still' as const, icon: 'flame' },
		{ href: '/jar-shelf', titleKey: 'nav.jarShelf' as const, icon: 'graph' }
	];

	function isActive(href: string) {
		return page.url.pathname === href;
	}

	const activeClass = 'bg-primary/15 text-primary';
	const inactiveClass = 'text-base-content/60 hover:text-base-content/70 hover:bg-base-content/[0.05]';
</script>

{#if isHorizontal()}
	<nav class="h-12 border-b border-base-300 flex items-center justify-between px-5 shrink-0">
		<div class="flex items-center gap-1">
			{#each links as link}
				<a
					href={link.href}
					class="w-9 h-9 flex items-center justify-center rounded-lg transition-colors {isActive(link.href) ? activeClass : inactiveClass}"
					title={t(link.titleKey)}
				>
					{#if link.icon === 'plus'}
						<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
							<line x1="12" y1="5" x2="12" y2="19"/><line x1="5" y1="12" x2="19" y2="12"/>
						</svg>
					{:else if link.icon === 'doc'}
						<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
							<path d="M14 2H6a2 2 0 00-2 2v16a2 2 0 002 2h12a2 2 0 002-2V8z"/><polyline points="14 2 14 8 20 8"/><line x1="16" y1="13" x2="8" y2="13"/><line x1="16" y1="17" x2="8" y2="17"/>
						</svg>
					{:else if link.icon === 'flame'}
						<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
							<path d="M10 2v7.527a2 2 0 01-.211.896L4.72 20.55a1 1 0 00.9 1.45h12.76a1 1 0 00.9-1.45l-5.069-10.127A2 2 0 0114 9.527V2"/><path d="M8.5 2h7"/><path d="M7 16.5h10"/>
						</svg>
					{:else if link.icon === 'graph'}
						<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
							<circle cx="6" cy="6" r="3"/><circle cx="18" cy="6" r="3"/><circle cx="12" cy="18" r="3"/><line x1="8.5" y1="7.5" x2="10.5" y2="16"/><line x1="15.5" y1="7.5" x2="13.5" y2="16"/>
						</svg>
					{/if}
				</a>
			{/each}
		</div>

		<div class="flex items-center gap-1">
			<a
				href="/help"
				class="w-9 h-9 flex items-center justify-center rounded-lg transition-colors {isActive('/help') ? activeClass : inactiveClass}"
				title={t('nav.help')}
			>
				<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
					<circle cx="12" cy="12" r="10"/><path d="M9.09 9a3 3 0 015.83 1c0 2-3 3-3 3"/><line x1="12" y1="17" x2="12.01" y2="17"/>
				</svg>
			</a>
			<a
				href="/settings"
				class="w-9 h-9 flex items-center justify-center rounded-lg transition-colors {isActive('/settings') ? activeClass : inactiveClass}"
				title={t('nav.settings')}
			>
				<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
					<circle cx="12" cy="12" r="3"/>
					<path d="M19.4 15a1.65 1.65 0 00.33 1.82l.06.06a2 2 0 010 2.83 2 2 0 01-2.83 0l-.06-.06a1.65 1.65 0 00-1.82-.33 1.65 1.65 0 00-1 1.51V21a2 2 0 01-4 0v-.09A1.65 1.65 0 009 19.4a1.65 1.65 0 00-1.82.33l-.06.06a2 2 0 01-2.83 0 2 2 0 010-2.83l.06-.06A1.65 1.65 0 004.68 15a1.65 1.65 0 00-1.51-1H3a2 2 0 010-4h.09A1.65 1.65 0 004.6 9a1.65 1.65 0 00-.33-1.82l-.06-.06a2 2 0 010-2.83 2 2 0 012.83 0l.06.06A1.65 1.65 0 009 4.68a1.65 1.65 0 001-1.51V3a2 2 0 014 0v.09a1.65 1.65 0 001 1.51 1.65 1.65 0 001.82-.33l.06-.06a2 2 0 012.83 0 2 2 0 010 2.83l-.06.06A1.65 1.65 0 0019.4 9a1.65 1.65 0 001.51 1H21a2 2 0 010 4h-.09a1.65 1.65 0 00-1.51 1z"/>
				</svg>
			</a>
		</div>
	</nav>
{:else}
	<nav class="w-14 h-screen border-r border-base-300 flex flex-col items-center justify-between py-5 shrink-0">
		<div class="flex flex-col items-center gap-1">
			{#each links as link}
				<a
					href={link.href}
					class="w-9 h-9 flex items-center justify-center rounded-lg transition-colors {isActive(link.href) ? activeClass : inactiveClass}"
					title={t(link.titleKey)}
				>
					{#if link.icon === 'plus'}
						<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
							<line x1="12" y1="5" x2="12" y2="19"/><line x1="5" y1="12" x2="19" y2="12"/>
						</svg>
					{:else if link.icon === 'doc'}
						<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
							<path d="M14 2H6a2 2 0 00-2 2v16a2 2 0 002 2h12a2 2 0 002-2V8z"/><polyline points="14 2 14 8 20 8"/><line x1="16" y1="13" x2="8" y2="13"/><line x1="16" y1="17" x2="8" y2="17"/>
						</svg>
					{:else if link.icon === 'flame'}
						<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
							<path d="M10 2v7.527a2 2 0 01-.211.896L4.72 20.55a1 1 0 00.9 1.45h12.76a1 1 0 00.9-1.45l-5.069-10.127A2 2 0 0114 9.527V2"/><path d="M8.5 2h7"/><path d="M7 16.5h10"/>
						</svg>
					{:else if link.icon === 'graph'}
						<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
							<circle cx="6" cy="6" r="3"/><circle cx="18" cy="6" r="3"/><circle cx="12" cy="18" r="3"/><line x1="8.5" y1="7.5" x2="10.5" y2="16"/><line x1="15.5" y1="7.5" x2="13.5" y2="16"/>
						</svg>
					{/if}
				</a>
			{/each}
		</div>

		<div class="flex flex-col items-center gap-1">
			<a
				href="/help"
				class="w-9 h-9 flex items-center justify-center rounded-lg transition-colors {isActive('/help') ? activeClass : inactiveClass}"
				title={t('nav.help')}
			>
				<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
					<circle cx="12" cy="12" r="10"/><path d="M9.09 9a3 3 0 015.83 1c0 2-3 3-3 3"/><line x1="12" y1="17" x2="12.01" y2="17"/>
				</svg>
			</a>
			<a
				href="/settings"
				class="w-9 h-9 flex items-center justify-center rounded-lg transition-colors {isActive('/settings') ? activeClass : inactiveClass}"
				title={t('nav.settings')}
			>
				<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
					<circle cx="12" cy="12" r="3"/>
					<path d="M19.4 15a1.65 1.65 0 00.33 1.82l.06.06a2 2 0 010 2.83 2 2 0 01-2.83 0l-.06-.06a1.65 1.65 0 00-1.82-.33 1.65 1.65 0 00-1 1.51V21a2 2 0 01-4 0v-.09A1.65 1.65 0 009 19.4a1.65 1.65 0 00-1.82.33l-.06.06a2 2 0 01-2.83 0 2 2 0 010-2.83l.06-.06A1.65 1.65 0 004.68 15a1.65 1.65 0 00-1.51-1H3a2 2 0 010-4h.09A1.65 1.65 0 004.6 9a1.65 1.65 0 00-.33-1.82l-.06-.06a2 2 0 010-2.83 2 2 0 012.83 0l.06.06A1.65 1.65 0 009 4.68a1.65 1.65 0 001-1.51V3a2 2 0 014 0v.09a1.65 1.65 0 001 1.51 1.65 1.65 0 001.82-.33l.06-.06a2 2 0 012.83 0 2 2 0 010 2.83l-.06.06A1.65 1.65 0 0019.4 9a1.65 1.65 0 001.51 1H21a2 2 0 010 4h-.09a1.65 1.65 0 00-1.51 1z"/>
				</svg>
			</a>
		</div>
	</nav>
{/if}
