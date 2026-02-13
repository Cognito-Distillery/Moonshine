<script lang="ts">
	import { onMount } from 'svelte';
	import GraphCanvas from '$lib/components/GraphCanvas.svelte';
	import { graphStore } from '$lib/stores/graph.svelte';
	import { uiStore } from '$lib/stores/ui.svelte';
	import { showToast } from '$lib/stores/toast.svelte';
	import * as graphCmd from '$lib/commands/graph';
	import { t } from '$lib/i18n/index.svelte';

	let graphCanvas: ReturnType<typeof GraphCanvas> | undefined = $state();

	onMount(async () => {
		uiStore.setLoading(true);
		try {
			const data = await graphCmd.getGraph({});
			graphStore.setGraphData({
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
		} catch {
			showToast(t('error.loadGraph'));
		} finally {
			uiStore.setLoading(false);
		}
	});
</script>

{#if uiStore.isLoading}
	<div class="flex items-center justify-center h-full">
		<span class="loading loading-spinner loading-lg text-primary"></span>
	</div>
{:else}
	<GraphCanvas bind:this={graphCanvas} />
{/if}
