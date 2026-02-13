<script lang="ts">
	import { graphStore } from '$lib/stores/graph.svelte';
	import { uiStore } from '$lib/stores/ui.svelte';
	import { showToast } from '$lib/stores/toast.svelte';
	import { t } from '$lib/i18n/index.svelte';
	import type { RelationType } from '$lib/types/graph';

	const relationTypes: RelationType[] = ['RELATED_TO', 'SUPPORTS', 'CONFLICTS_WITH'];

	let isCreateMode = $derived(uiStore.sidePanelMode === 'add-edge');
	let selectedRelationType = $state<RelationType>('RELATED_TO');
	let selectedTargetId = $state<string | null>(null);
	let saving = $state(false);

	$effect(() => {
		if (!isCreateMode && graphStore.selectedEdge) {
			selectedRelationType = graphStore.selectedEdge.relationType;
		}
	});

	$effect(() => {
		if (isCreateMode && graphStore.selectedNodeId && graphStore.selectedNodeId !== uiStore.addEdgeSourceId) {
			selectedTargetId = graphStore.selectedNodeId;
		}
	});

	function nodeLabel(id: string) {
		return graphStore.nodes.find((n) => n.id === id)?.summary ?? id;
	}

	async function save() {
		saving = true;
		try {
			if (isCreateMode) {
				const sourceId = uiStore.addEdgeSourceId;
				const targetId = selectedTargetId;
				if (!sourceId || !targetId) return;
				graphStore.addEdge({
					sourceId,
					targetId,
					relationType: selectedRelationType,
					source: 'human',
					confidence: null
				});
				uiStore.cancelAddEdge();
			} else if (graphStore.selectedEdge) {
				const { sourceId, targetId } = graphStore.selectedEdge;
				graphStore.updateEdge(sourceId, targetId, selectedRelationType);
				uiStore.setSidePanelMode('detail');
			}
		} catch {
			showToast(t('error.saveEdge'));
		} finally {
			saving = false;
		}
	}

	function cancel() {
		if (isCreateMode) {
			uiStore.cancelAddEdge();
		} else {
			uiStore.setSidePanelMode('detail');
		}
	}
</script>

<div class="space-y-3">
	{#if isCreateMode}
		<div class="text-sm space-y-2">
			<div>
				<span class="text-xs text-base-content/60">{t('edge.from')}:</span>
				<div class="font-medium">
					{uiStore.addEdgeSourceId ? nodeLabel(uiStore.addEdgeSourceId) : '\u2014'}
				</div>
			</div>
			<div>
				<span class="text-xs text-base-content/60">{t('edge.to')}:</span>
				{#if selectedTargetId}
					<div class="font-medium">{nodeLabel(selectedTargetId)}</div>
				{:else}
					<div class="text-warning text-xs">{t('edge.selectTarget')}</div>
				{/if}
			</div>
		</div>
	{/if}

	<div class="form-control">
		<label class="label" for="edge-relation">
			<span class="label-text text-xs">{t('edge.relationType')}</span>
		</label>
		<select
			id="edge-relation"
			class="select select-sm w-full bg-white/[0.12] border-white/[0.18] focus:border-primary"
			bind:value={selectedRelationType}
		>
			{#each relationTypes as rt}
				<option value={rt}>{rt}</option>
			{/each}
		</select>
	</div>

	<div class="flex gap-2">
		<button class="btn btn-ghost btn-sm flex-1" onclick={cancel}>
			{t('node.cancel')}
		</button>
		<button
			class="btn btn-primary btn-sm flex-1"
			onclick={save}
			disabled={saving || (isCreateMode && !selectedTargetId)}
		>
			{#if saving}
				<span class="loading loading-spinner loading-xs"></span>
			{/if}
			{isCreateMode ? t('edge.create') : t('edge.update')}
		</button>
	</div>
</div>
