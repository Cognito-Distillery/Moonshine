<script lang="ts">
	import { uiStore } from '$lib/stores/ui.svelte';
	import { graphStore } from '$lib/stores/graph.svelte';
	import { t } from '$lib/i18n/index.svelte';
	import type { LayoutParams } from '$lib/graph/cytoscape-config';
	import FilterPanel from './FilterPanel.svelte';
	import NodeDetail from './NodeDetail.svelte';
	import NodeEditor from './NodeEditor.svelte';
	import EdgeDetail from './EdgeDetail.svelte';
	import EdgeEditor from './EdgeEditor.svelte';

	interface Props {
		layoutParams: LayoutParams;
		wheelSensitivity: number;
		onParamChange: <K extends keyof LayoutParams>(key: K, value: LayoutParams[K]) => void;
		onWheelSensitivityChange: (value: number) => void;
		onReset: () => void;
	}

	let { layoutParams, wheelSensitivity, onParamChange, onWheelSensitivityChange, onReset }: Props = $props();
</script>

<aside
	class="absolute top-12 left-3 z-30 w-72 flex flex-col overflow-hidden rounded-lg bg-black/60 backdrop-blur-md border border-white/[0.08] transition-all duration-300 ease-out origin-top max-h-[calc(100%-5rem)]"
	class:scale-y-100={uiStore.sidePanelOpen}
	class:opacity-100={uiStore.sidePanelOpen}
	class:scale-y-0={!uiStore.sidePanelOpen}
	class:opacity-0={!uiStore.sidePanelOpen}
	class:pointer-events-none={!uiStore.sidePanelOpen}
>
	<div class="px-3 py-2 border-b border-white/[0.06] flex items-center justify-between">
		<span class="text-[10px] font-semibold uppercase tracking-wider text-base-content/40">
			{#if uiStore.sidePanelMode === 'filter'}
				{t('panel.filters')}
			{:else if uiStore.sidePanelMode === 'detail'}
				{t('panel.details')}
			{:else if uiStore.sidePanelMode === 'edit-node'}
				{t('panel.editNode')}
			{:else if uiStore.sidePanelMode === 'edit-edge'}
				{t('panel.editEdge')}
			{:else if uiStore.sidePanelMode === 'add-edge'}
				{t('panel.addEdge')}
			{/if}
		</span>
		{#if uiStore.sidePanelMode !== 'filter'}
			<button
				class="btn btn-ghost btn-xs text-base-content/40"
				onclick={() => {
					graphStore.clearSelection();
					uiStore.setSidePanelMode('filter');
				}}
			>
				{t('panel.back')}
			</button>
		{/if}
	</div>

	<div class="flex-1 overflow-y-auto p-3">
		{#if uiStore.sidePanelMode === 'filter'}
			<FilterPanel
				{layoutParams}
				{wheelSensitivity}
				{onParamChange}
				{onWheelSensitivityChange}
				{onReset}
			/>
		{:else if uiStore.sidePanelMode === 'detail'}
			{#if graphStore.selectedNode}
				<NodeDetail />
			{:else if graphStore.selectedEdge}
				<EdgeDetail />
			{:else}
				<p class="text-xs text-base-content/30">{t('panel.clickToView')}</p>
			{/if}
		{:else if uiStore.sidePanelMode === 'edit-node'}
			<NodeEditor />
		{:else if uiStore.sidePanelMode === 'edit-edge' || uiStore.sidePanelMode === 'add-edge'}
			<EdgeEditor />
		{/if}
	</div>
</aside>
