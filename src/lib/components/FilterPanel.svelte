<script lang="ts">
	import { graphStore } from '$lib/stores/graph.svelte';
	import { t } from '$lib/i18n/index.svelte';
	import type { RelationType, EdgeSource } from '$lib/types/graph';
	import RecentSearches from './RecentSearches.svelte';

	const relationTypes: { type: RelationType; key: 'filter.relatedTo' | 'filter.supports' | 'filter.conflictsWith'; bg: string; text: string }[] = [
		{ type: 'RELATED_TO', key: 'filter.relatedTo', bg: 'bg-[#57534e]', text: 'text-[#d6d3d1]' },
		{ type: 'SUPPORTS', key: 'filter.supports', bg: 'bg-[#4d7c0f]', text: 'text-[#d9f99d]' },
		{ type: 'CONFLICTS_WITH', key: 'filter.conflictsWith', bg: 'bg-[#b91c1c]', text: 'text-[#fecaca]' }
	];
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

	<RecentSearches />
</div>
