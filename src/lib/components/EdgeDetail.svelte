<script lang="ts">
	import { graphStore } from '$lib/stores/graph.svelte';
	import { uiStore } from '$lib/stores/ui.svelte';
	import { t } from '$lib/i18n/index.svelte';

	const relationBadge: Record<string, string> = {
		RELATED_TO: 'bg-[#57534e] text-[#d6d3d1]',
		SUPPORTS: 'bg-[#4d7c0f] text-[#d9f99d]',
		CONFLICTS_WITH: 'bg-[#b91c1c] text-[#fecaca]'
	};

	function handleDelete() {
		if (!graphStore.selectedEdge) return;
		const { sourceId, targetId } = graphStore.selectedEdge;
		uiStore.showConfirm(t('confirm.deleteEdge'), () => {
			graphStore.removeEdge(sourceId, targetId);
			graphStore.clearSelection();
			uiStore.dismissConfirm();
			uiStore.setSidePanelMode('filter');
		});
	}

	function nodeLabel(id: string) {
		return graphStore.nodes.find((n) => n.id === id)?.summary ?? id;
	}
</script>

{#if graphStore.selectedEdge}
	{@const edge = graphStore.selectedEdge}
	<div class="space-y-3">
		<div class="flex items-center justify-between">
			<span class="badge badge-sm border-0 {relationBadge[edge.relationType] ?? ''}">
				{edge.relationType}
			</span>
			<div class="flex gap-1">
				<button
					class="btn btn-ghost btn-xs"
					onclick={() => uiStore.setSidePanelMode('edit-edge')}
				>
					{t('edge.edit')}
				</button>
				<button class="btn btn-ghost btn-xs text-error" onclick={handleDelete}>
					{t('edge.delete')}
				</button>
			</div>
		</div>

		<div class="text-sm space-y-2">
			<div>
				<span class="text-xs text-base-content/60">{t('edge.from')}:</span>
				<div class="font-medium">{nodeLabel(edge.sourceId)}</div>
			</div>
			<div class="text-center text-base-content/60">&rarr;</div>
			<div>
				<span class="text-xs text-base-content/60">{t('edge.to')}:</span>
				<div class="font-medium">{nodeLabel(edge.targetId)}</div>
			</div>
		</div>

		<div class="space-y-1 text-xs text-base-content/65">
			{#if edge.source}
				<div>{t('edge.source')}: <span class="badge badge-xs">{edge.source}</span></div>
			{/if}
			{#if edge.confidence != null}
				<div>{t('edge.confidence')}: {(edge.confidence * 100).toFixed(0)}%</div>
			{/if}
		</div>
	</div>
{/if}
