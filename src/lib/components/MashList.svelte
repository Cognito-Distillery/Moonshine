<script lang="ts">
	import { MashStatus } from '$lib/types';
	import MashCard from './MashCard.svelte';
	import { getMashes, getSearchQuery, setSearchQuery, loadMashes } from '$lib/stores/mashes.svelte';
	import { getViewMode, setViewMode } from '$lib/stores/settings.svelte';
	import { t } from '$lib/i18n/index.svelte';

	let { mode = 'mashes' }: { mode?: 'mashes' | 'still' } = $props();

	const statusMap = {
		mashes: MashStatus.MASH_TUN,
		still: MashStatus.ON_STILL
	} as const;

	let displayMashes = $derived(getMashes());

	let debounceTimer: ReturnType<typeof setTimeout>;

	$effect(() => {
		const status = statusMap[mode];
		const query = getSearchQuery();
		clearTimeout(debounceTimer);
		debounceTimer = setTimeout(() => {
			loadMashes(status, query);
		}, query ? 200 : 0);
		return () => clearTimeout(debounceTimer);
	});
</script>

<div class="flex flex-col gap-4">
	<div class="flex items-center gap-2">
		<label class="input bg-white/[0.12] border-white/[0.18] flex items-center gap-2 focus-within:border-primary flex-1">
			<svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="text-base-content/25">
				<circle cx="11" cy="11" r="8"/>
				<line x1="21" y1="21" x2="16.65" y2="16.65"/>
			</svg>
			<input
				type="text"
				class="grow bg-transparent placeholder:text-base-content/25"
				placeholder={t('common.search')}
				value={getSearchQuery()}
				oninput={(e) => setSearchQuery(e.currentTarget.value)}
			/>
		</label>

		<div class="flex">
			<button
				class="w-9 h-9 flex items-center justify-center rounded-lg transition-colors {getViewMode() === 'list' ? 'text-primary bg-primary/15' : 'text-base-content/40 hover:text-base-content/70 hover:bg-white/[0.05]'}"
				title={t('list.listView')}
				onclick={() => setViewMode('list')}
			>
				<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
					<line x1="8" y1="6" x2="21" y2="6"/>
					<line x1="8" y1="12" x2="21" y2="12"/>
					<line x1="8" y1="18" x2="21" y2="18"/>
					<line x1="3" y1="6" x2="3.01" y2="6"/>
					<line x1="3" y1="12" x2="3.01" y2="12"/>
					<line x1="3" y1="18" x2="3.01" y2="18"/>
				</svg>
			</button>
			<button
				class="w-9 h-9 flex items-center justify-center rounded-lg transition-colors {getViewMode() === 'card' ? 'text-primary bg-primary/15' : 'text-base-content/40 hover:text-base-content/70 hover:bg-white/[0.05]'}"
				title={t('list.cardView')}
				onclick={() => setViewMode('card')}
			>
				<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
					<rect x="3" y="3" width="7" height="7"/>
					<rect x="14" y="3" width="7" height="7"/>
					<rect x="3" y="14" width="7" height="7"/>
					<rect x="14" y="14" width="7" height="7"/>
				</svg>
			</button>
		</div>
	</div>

	{#if getViewMode() === 'card'}
		<div class="grid grid-cols-2 gap-3">
			{#each displayMashes as mash (mash.id)}
				<MashCard {mash} view="card" {mode} currentStatus={statusMap[mode]} />
			{/each}
		</div>
	{:else}
		<div class="flex flex-col gap-2">
			{#each displayMashes as mash (mash.id)}
				<MashCard {mash} view="list" {mode} currentStatus={statusMap[mode]} />
			{/each}
		</div>
	{/if}

	{#if displayMashes.length === 0}
		<div class="flex flex-col items-center gap-3 py-16">
			{#if getSearchQuery().trim()}
				<p class="text-base-content/25 text-sm">{t('list.emptySearch')}</p>
			{:else if mode === 'still'}
				<p class="text-base-content/25 text-sm">{t('list.emptyStill')}</p>
				<a href="/mashes" class="btn btn-ghost btn-sm text-base-content/40">{t('list.ctaStill')}</a>
			{:else}
				<p class="text-base-content/25 text-sm">{t('list.emptyMashes')}</p>
				<a href="/" class="btn btn-ghost btn-sm text-base-content/40">{t('list.ctaMashes')}</a>
			{/if}
		</div>
	{/if}
</div>
