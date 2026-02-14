<script lang="ts">
	import { onMount, untrack } from 'svelte';
	import { graphStore } from '$lib/stores/graph.svelte';
	import { getRecentSearches, replayCachedSearch, deleteCachedSearch } from '$lib/commands/search';
	import type { RecentSearch } from '$lib/commands/search';
	import { showToast } from '$lib/stores/toast.svelte';
	import { t } from '$lib/i18n/index.svelte';

	let searches = $state<RecentSearch[]>([]);
	let loading = $state(false);
	let selfTriggered = false;

	async function refresh() {
		try {
			searches = await getRecentSearches();
		} catch {
			// silent
		}
	}

	onMount(() => {
		refresh();
	});

	// Refresh the list whenever search matches change (new search performed)
	$effect(() => {
		graphStore.searchMatchIds;
		if (selfTriggered) {
			selfTriggered = false;
			return;
		}
		untrack(() => refresh());
	});

	async function replay(item: RecentSearch) {
		loading = true;
		try {
			const data = await replayCachedSearch(item.id);
			graphStore.mergeGraphData({
				nodes: data.nodes.map((n) => ({
					id: n.id,
					summary: n.summary,
					context: n.context,
					memo: n.memo,
					type: n.mashType,
					createdAt: n.createdAt,
					updatedAt: n.updatedAt
				})),
				edges: data.edges.map((e) => ({
					sourceId: e.sourceId,
					targetId: e.targetId,
					relationType: e.relationType as 'RELATED_TO' | 'SUPPORTS' | 'CONFLICTS_WITH',
					source: e.source as 'ai' | 'human',
					confidence: e.confidence
				}))
			});
			selfTriggered = true;
			graphStore.setSearchMatches(data.nodes.map((n) => n.id));
		} catch {
			showToast(t('error.searchFailed'));
		} finally {
			loading = false;
		}
	}

	async function remove(e: Event, item: RecentSearch) {
		e.stopPropagation();
		try {
			await deleteCachedSearch(item.id);
			searches = searches.filter((s) => s.id !== item.id);
		} catch {
			showToast(t('error.deleteFailed'));
		}
	}
</script>

<div>
	<h3 class="text-xs font-semibold uppercase tracking-wider text-base-content/60 mb-2">
		{t('search.recent')}
	</h3>
	{#if searches.length === 0}
		<p class="text-xs text-base-content/30">{t('search.noRecent')}</p>
	{:else}
		<div class="space-y-0.5 max-h-48 overflow-y-auto">
			{#each searches as item (item.id)}
				<div
					role="button"
					tabindex="0"
					class="w-full text-left text-xs px-2 py-1.5 rounded hover:bg-white/[0.08] flex items-center gap-1.5 group transition-colors cursor-pointer"
					class:opacity-50={loading}
					onclick={() => !loading && replay(item)}
					onkeydown={(e) => e.key === 'Enter' && !loading && replay(item)}
				>
					<span class="truncate flex-1 text-base-content/70">{item.query}</span>
					<button
						class="opacity-0 group-hover:opacity-100 text-base-content/30 hover:text-error transition-opacity text-[10px] shrink-0"
						onclick={(e) => remove(e, item)}
						title={t('common.delete')}
					>
						&#x2715;
					</button>
				</div>
			{/each}
		</div>
	{/if}
</div>
