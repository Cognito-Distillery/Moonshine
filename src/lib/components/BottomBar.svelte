<script lang="ts">
	import { graphStore } from '$lib/stores/graph.svelte';
	import { t } from '$lib/i18n/index.svelte';
	import { edgeDotColors } from '$lib/graph/cytoscape-styles';
	import { getTheme } from '$lib/stores/settings.svelte';

	const relCounts = $derived({
		RELATED_TO: graphStore.edges.filter((e) => e.relationType === 'RELATED_TO').length,
		SUPPORTS: graphStore.edges.filter((e) => e.relationType === 'SUPPORTS').length,
		CONFLICTS_WITH: graphStore.edges.filter((e) => e.relationType === 'CONFLICTS_WITH').length
	});

	const sourceCounts = $derived({
		ai: graphStore.edges.filter((e) => e.source === 'ai').length,
		human: graphStore.edges.filter((e) => e.source === 'human').length
	});
</script>

<footer class="bg-base-100/60 backdrop-blur-sm px-5 py-2 flex items-center gap-4 text-sm text-base-content/60">
	<span>{t('bottom.nodes')}: <strong class="text-base-content/80">{graphStore.nodes.length}</strong></span>
	<span class="text-base-content/40">|</span>
	<span>{t('bottom.edges')}: <strong class="text-base-content/80">{graphStore.edges.length}</strong></span>
	<span class="text-base-content/40">|</span>
	<span class="flex items-center gap-1.5">
		<span class="w-2 h-2 rounded-full inline-block" style="background: {edgeDotColors[getTheme()].related}"></span>
		{relCounts.RELATED_TO}
	</span>
	<span class="flex items-center gap-1.5">
		<span class="w-2 h-2 rounded-full inline-block" style="background: {edgeDotColors[getTheme()].supports}"></span>
		{relCounts.SUPPORTS}
	</span>
	<span class="flex items-center gap-1.5">
		<span class="w-2 h-2 rounded-full inline-block" style="background: {edgeDotColors[getTheme()].conflicts}"></span>
		{relCounts.CONFLICTS_WITH}
	</span>
	<span class="text-base-content/40">|</span>
	<span>AI {sourceCounts.ai} / Human {sourceCounts.human}</span>
</footer>
