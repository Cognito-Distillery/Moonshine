<script lang="ts">
	import { graphStore } from '$lib/stores/graph.svelte';
	import { t } from '$lib/i18n/index.svelte';
	import type { RelationType, EdgeSource } from '$lib/types/graph';
	import type { LayoutParams } from '$lib/graph/cytoscape-config';

	interface Props {
		layoutParams: LayoutParams;
		wheelSensitivity: number;
		onParamChange: <K extends keyof LayoutParams>(key: K, value: LayoutParams[K]) => void;
		onWheelSensitivityChange: (value: number) => void;
		onReset: () => void;
	}

	let { layoutParams, wheelSensitivity, onParamChange, onWheelSensitivityChange, onReset }: Props = $props();

	const relationTypes: { type: RelationType; key: 'filter.relatedTo' | 'filter.supports' | 'filter.conflictsWith'; bg: string; text: string }[] = [
		{ type: 'RELATED_TO', key: 'filter.relatedTo', bg: 'bg-[#57534e]', text: 'text-[#d6d3d1]' },
		{ type: 'SUPPORTS', key: 'filter.supports', bg: 'bg-[#4d7c0f]', text: 'text-[#d9f99d]' },
		{ type: 'CONFLICTS_WITH', key: 'filter.conflictsWith', bg: 'bg-[#b91c1c]', text: 'text-[#fecaca]' }
	];

	const convergenceSteps = [0.1, 0.01, 0.001, 0.0001] as const;

	const convergenceIndex = $derived(
		Math.max(0, convergenceSteps.indexOf(layoutParams.convergenceThreshold as (typeof convergenceSteps)[number]))
	);

	function handleConvergenceChange(idx: number) {
		onParamChange('convergenceThreshold', convergenceSteps[idx]);
	}

	function fillPercent(value: number, min: number, max: number) {
		return ((value - min) / (max - min)) * 100;
	}
</script>

<div class="space-y-4">
	<div>
		<h3 class="text-xs font-semibold uppercase tracking-wider text-base-content/60 mb-2">
			{t('filter.relationType')}
		</h3>
		<div class="space-y-1">
			{#each relationTypes as rt}
				<label class="flex items-center gap-2 cursor-pointer px-1 py-1 rounded hover:bg-base-content/[0.05]">
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

	<!-- Layout Settings -->
	<div>
		<h3 class="text-xs font-semibold uppercase tracking-wider text-base-content/60 mb-3">
			{t('layout.title')}
		</h3>
		<div class="space-y-2">
			<label class="flex items-center gap-2 text-xs text-base-content/60">
				<span class="w-24 shrink-0">{t('layout.nodeSpacing')}</span>
				<input
					type="range"
					min="5" max="100" step="5"
					value={layoutParams.nodeSpacing}
					oninput={(e) => onParamChange('nodeSpacing', Number(e.currentTarget.value))}
					class="range range-xs range-primary flex-1"
					style="--range-fill: {fillPercent(layoutParams.nodeSpacing, 5, 100)}%;"
				/>
				<span class="w-8 text-right text-base-content/80 tabular-nums shrink-0 text-[10px]">{layoutParams.nodeSpacing}</span>
			</label>
			<label class="flex items-center gap-2 text-xs text-base-content/60">
				<span class="w-24 shrink-0">{t('layout.edgeLength')}</span>
				<input
					type="range"
					min="30" max="300" step="10"
					value={layoutParams.edgeLength}
					oninput={(e) => onParamChange('edgeLength', Number(e.currentTarget.value))}
					class="range range-xs range-primary flex-1"
					style="--range-fill: {fillPercent(layoutParams.edgeLength, 30, 300)}%;"
				/>
				<span class="w-8 text-right text-base-content/80 tabular-nums shrink-0 text-[10px]">{layoutParams.edgeLength}</span>
			</label>
			<label class="flex items-center gap-2 text-xs text-base-content/60">
				<span class="w-24 shrink-0">{t('layout.convergence')}</span>
				<input
					type="range"
					min="0" max="3" step="1"
					value={convergenceIndex}
					oninput={(e) => handleConvergenceChange(Number(e.currentTarget.value))}
					class="range range-xs range-primary flex-1"
					style="--range-fill: {fillPercent(convergenceIndex, 0, 3)}%;"
				/>
				<span class="w-8 text-right text-base-content/80 tabular-nums shrink-0 text-[10px]">{convergenceIndex + 1}/4</span>
			</label>
			<label class="flex items-center gap-2 text-xs text-base-content/60">
				<span class="w-24 shrink-0">{t('layout.spreadForce')}</span>
				<input
					type="range"
					min="1" max="30" step="1"
					value={layoutParams.edgeSymDiffLength}
					oninput={(e) => onParamChange('edgeSymDiffLength', Number(e.currentTarget.value))}
					class="range range-xs range-primary flex-1"
					style="--range-fill: {fillPercent(layoutParams.edgeSymDiffLength, 1, 30)}%;"
				/>
				<span class="w-8 text-right text-base-content/80 tabular-nums shrink-0 text-[10px]">{layoutParams.edgeSymDiffLength}</span>
			</label>
			<label class="flex items-center gap-2 text-xs text-base-content/60">
				<span class="w-24 shrink-0">{t('layout.zoomSensitivity')}</span>
				<input
					type="range"
					min="0.05" max="1" step="0.05"
					value={wheelSensitivity}
					oninput={(e) => onWheelSensitivityChange(Number(e.currentTarget.value))}
					class="range range-xs range-primary flex-1"
					style="--range-fill: {fillPercent(wheelSensitivity, 0.05, 1)}%;"
				/>
				<span class="w-8 text-right text-base-content/80 tabular-nums shrink-0 text-[10px]">{wheelSensitivity.toFixed(2)}</span>
			</label>
			<div class="flex justify-end pt-1">
				<button
					class="btn btn-xs btn-ghost text-base-content/65 hover:text-base-content/80"
					onclick={onReset}
				>
					{t('layout.reset')}
				</button>
			</div>
		</div>
	</div>
</div>
