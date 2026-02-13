<script lang="ts">
	import { graphStore } from '$lib/stores/graph.svelte';
	import { uiStore } from '$lib/stores/ui.svelte';
	import { t } from '$lib/i18n/index.svelte';
</script>

{#if graphStore.selectedNode}
	{@const node = graphStore.selectedNode}
	<div class="space-y-3">
		<div class="flex items-center justify-between">
			<span class="badge badge-sm badge-outline">{node.type}</span>
			<button
				class="btn btn-ghost btn-xs"
				onclick={() => uiStore.setSidePanelMode('edit-node')}
			>
				{t('node.edit')}
			</button>
		</div>

		<div>
			<h3 class="font-semibold text-sm">{node.summary}</h3>
		</div>

		{#if node.context}
			<div>
				<div class="text-xs font-medium text-base-content/60 mb-1">{t('node.context')}</div>
				<p class="text-sm">{node.context}</p>
			</div>
		{/if}

		{#if node.memo}
			<div>
				<div class="text-xs font-medium text-base-content/60 mb-1">{t('node.memo')}</div>
				<p class="text-sm italic">{node.memo}</p>
			</div>
		{/if}

		<div class="text-xs text-base-content/40">ID: {node.id}</div>
	</div>
{/if}
