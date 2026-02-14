<script lang="ts">
	import { onMount } from 'svelte';
	import { graphStore } from '$lib/stores/graph.svelte';
	import { t } from '$lib/i18n/index.svelte';
	import { getSetting, setSetting } from '$lib/commands/settings';
	import type { RelationType, EdgeSource } from '$lib/types/graph';
	import RecentSearches from './RecentSearches.svelte';

	const relationTypes: { type: RelationType; key: 'filter.relatedTo' | 'filter.supports' | 'filter.conflictsWith'; bg: string; text: string }[] = [
		{ type: 'RELATED_TO', key: 'filter.relatedTo', bg: 'bg-[#57534e]', text: 'text-[#d6d3d1]' },
		{ type: 'SUPPORTS', key: 'filter.supports', bg: 'bg-[#4d7c0f]', text: 'text-[#d9f99d]' },
		{ type: 'CONFLICTS_WITH', key: 'filter.conflictsWith', bg: 'bg-[#b91c1c]', text: 'text-[#fecaca]' }
	];

	let searchThreshold = $state(0.3);
	let searchTopK = $state(10);

	onMount(async () => {
		try {
			const [thresholdVal, topKVal] = await Promise.all([
				getSetting('search_threshold'),
				getSetting('search_top_k')
			]);
			if (thresholdVal) searchThreshold = parseFloat(thresholdVal);
			if (topKVal) searchTopK = parseInt(topKVal);
		} catch {
			// ignore
		}
	});

	function handleSaveThreshold() {
		setSetting('search_threshold', String(searchThreshold));
	}

	function handleSaveTopK() {
		setSetting('search_top_k', String(searchTopK));
	}
</script>

<div class="space-y-4">
	<div>
		<h3 class="text-xs font-semibold uppercase tracking-wider text-base-content/60 mb-2">
			{t('filter.relationType')}
		</h3>
		<div class="space-y-1">
			{#each relationTypes as rt}
				<label class="flex items-center gap-2 cursor-pointer px-1 py-1 rounded hover:bg-white/[0.05]">
					<input
						type="checkbox"
						class="checkbox checkbox-xs"
						checked={graphStore.activeRelationTypes.has(rt.type)}
						onchange={() => graphStore.toggleRelationType(rt.type)}
					/>
					<span class="badge badge-sm {rt.bg} {rt.text} border-0 px-2.5 py-2.5">{t(rt.key)}</span>
				</label>
			{/each}
		</div>
	</div>

	<div class="divider my-1"></div>

	<div>
		<h3 class="text-xs font-semibold uppercase tracking-wider text-base-content/60 mb-2">
			{t('filter.source')}
		</h3>
		<div class="flex gap-1">
			{#each ['all', 'ai', 'human'] as s}
				<button
					class="btn btn-xs {graphStore.activeSource === s ? 'btn-primary' : 'btn-ghost'}"
					onclick={() => graphStore.setActiveSource(s as EdgeSource | 'all')}
				>
					{s === 'all' ? t('filter.all') : s === 'ai' ? 'AI' : 'Human'}
				</button>
			{/each}
		</div>
	</div>

	<div class="divider my-1"></div>

	<div class="text-xs text-base-content/50 space-y-1">
		<div>{t('filter.nodes')}: {graphStore.nodes.length}</div>
		<div>{t('filter.edges')}: {graphStore.filteredEdges.length} / {graphStore.edges.length}</div>
	</div>

	<div class="divider my-1"></div>

	<!-- Search Settings -->
	<div>
		<h3 class="text-xs font-semibold uppercase tracking-wider text-base-content/60 mb-3">
			{t('filter.searchSettings')}
		</h3>
		<div class="space-y-3">
			<div>
				<div class="flex items-center justify-between mb-1">
					<span class="text-xs text-base-content/70">{t('settings.searchThreshold')}</span>
					<input
						type="number"
						class="input input-xs w-16 bg-white/[0.12] border-white/[0.18] text-right text-xs"
						step="0.05"
						min="0.1"
						max="0.9"
						bind:value={searchThreshold}
						onchange={handleSaveThreshold}
					/>
				</div>
				<input
					type="range"
					class="range range-xs range-primary w-full"
					min="0.1"
					max="0.9"
					step="0.05"
					style="--range-fill: {((searchThreshold - 0.1) / (0.9 - 0.1)) * 100}%;"
					bind:value={searchThreshold}
					onchange={handleSaveThreshold}
				/>
			</div>
			<div>
				<div class="flex items-center justify-between mb-1">
					<span class="text-xs text-base-content/70">{t('settings.searchTopK')}</span>
					<input
						type="number"
						class="input input-xs w-16 bg-white/[0.12] border-white/[0.18] text-right text-xs"
						step="1"
						min="1"
						max="50"
						bind:value={searchTopK}
						onchange={handleSaveTopK}
					/>
				</div>
				<input
					type="range"
					class="range range-xs range-primary w-full"
					min="1"
					max="50"
					step="1"
					style="--range-fill: {((searchTopK - 1) / (50 - 1)) * 100}%;"
					bind:value={searchTopK}
					onchange={handleSaveTopK}
				/>
			</div>
		</div>
	</div>

	<div class="divider my-1"></div>

	<RecentSearches />
</div>
