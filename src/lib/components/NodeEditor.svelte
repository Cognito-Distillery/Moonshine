<script lang="ts">
	import { graphStore } from '$lib/stores/graph.svelte';
	import { uiStore } from '$lib/stores/ui.svelte';
	import { showToast } from '$lib/stores/toast.svelte';
	import { t } from '$lib/i18n/index.svelte';

	let summary = $state(graphStore.selectedNode?.summary ?? '');
	let context = $state(graphStore.selectedNode?.context ?? '');
	let memo = $state(graphStore.selectedNode?.memo ?? '');
	let saving = $state(false);

	$effect(() => {
		if (graphStore.selectedNode) {
			summary = graphStore.selectedNode.summary;
			context = graphStore.selectedNode.context;
			memo = graphStore.selectedNode.memo;
		}
	});

	async function save() {
		if (!graphStore.selectedNodeId) return;
		saving = true;
		try {
			graphStore.updateNode(graphStore.selectedNodeId, { summary, context, memo });
			uiStore.setSidePanelMode('detail');
		} catch {
			showToast(t('error.saveNode'));
		} finally {
			saving = false;
		}
	}
</script>

{#if graphStore.selectedNode}
	<div class="space-y-3">
		<div class="form-control">
			<label class="label" for="node-summary">
				<span class="label-text text-xs">{t('node.summary')}</span>
			</label>
			<input
				id="node-summary"
				type="text"
				class="input input-sm w-full bg-white/[0.12] border-white/[0.18] focus:border-primary"
				bind:value={summary}
			/>
		</div>

		<div class="form-control">
			<label class="label" for="node-context">
				<span class="label-text text-xs">{t('node.context')}</span>
			</label>
			<textarea
				id="node-context"
				class="textarea textarea-sm w-full h-24 bg-white/[0.12] border-white/[0.18] focus:border-primary"
				bind:value={context}
			></textarea>
		</div>

		<div class="form-control">
			<label class="label" for="node-memo">
				<span class="label-text text-xs">{t('node.memo')}</span>
			</label>
			<textarea
				id="node-memo"
				class="textarea textarea-sm w-full h-16 bg-white/[0.12] border-white/[0.18] focus:border-primary"
				bind:value={memo}
			></textarea>
		</div>

		<div class="flex gap-2">
			<button class="btn btn-ghost btn-sm flex-1" onclick={() => uiStore.setSidePanelMode('detail')}>
				{t('node.cancel')}
			</button>
			<button class="btn btn-primary btn-sm flex-1" onclick={save} disabled={saving}>
				{#if saving}
					<span class="loading loading-spinner loading-xs"></span>
				{/if}
				{t('node.save')}
			</button>
		</div>
	</div>
{/if}
