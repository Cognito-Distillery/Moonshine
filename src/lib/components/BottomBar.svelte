<script lang="ts">
	import { graphStore } from '$lib/stores/graph.svelte';
	import { t } from '$lib/i18n/index.svelte';
	import type { LayoutParams } from '$lib/graph/cytoscape-config';

	interface Props {
		layoutParams: LayoutParams;
		wheelSensitivity: number;
		onParamChange: <K extends keyof LayoutParams>(key: K, value: LayoutParams[K]) => void;
		onWheelSensitivityChange: (value: number) => void;
		onReset: () => void;
	}

	let { layoutParams, wheelSensitivity, onParamChange, onWheelSensitivityChange, onReset }: Props = $props();

	let showSliders = $state(false);

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

{#if showSliders}
	<div class="bg-black/70 backdrop-blur-sm px-4 py-3 rounded-t-lg border border-b-0 border-white/5 ml-auto w-80 space-y-2">
		<label class="flex items-center gap-2 text-xs text-base-content/60">
			<span class="w-16 shrink-0">{t('layout.nodeSpacing')}</span>
			<input
				type="range"
				min="5" max="100" step="5"
				value={layoutParams.nodeSpacing}
				oninput={(e) => onParamChange('nodeSpacing', Number(e.currentTarget.value))}
				class="range range-xs range-primary flex-1"
				style="--range-fill: {fillPercent(layoutParams.nodeSpacing, 5, 100)}%;"
			/>
			<span class="w-16 text-right text-base-content/80 tabular-nums shrink-0">{layoutParams.nodeSpacing}/{100}</span>
		</label>
		<label class="flex items-center gap-2 text-xs text-base-content/60">
			<span class="w-16 shrink-0">{t('layout.edgeLength')}</span>
			<input
				type="range"
				min="30" max="300" step="10"
				value={layoutParams.edgeLength}
				oninput={(e) => onParamChange('edgeLength', Number(e.currentTarget.value))}
				class="range range-xs range-primary flex-1"
				style="--range-fill: {fillPercent(layoutParams.edgeLength, 30, 300)}%;"
			/>
			<span class="w-16 text-right text-base-content/80 tabular-nums shrink-0">{layoutParams.edgeLength}/{300}</span>
		</label>
		<label class="flex items-center gap-2 text-xs text-base-content/60">
			<span class="w-16 shrink-0">{t('layout.convergence')}</span>
			<input
				type="range"
				min="0" max="3" step="1"
				value={convergenceIndex}
				oninput={(e) => handleConvergenceChange(Number(e.currentTarget.value))}
				class="range range-xs range-primary flex-1"
				style="--range-fill: {fillPercent(convergenceIndex, 0, 3)}%;"
			/>
			<span class="w-16 text-right text-base-content/80 tabular-nums shrink-0">{convergenceIndex + 1}/{4}</span>
		</label>
		<label class="flex items-center gap-2 text-xs text-base-content/60">
			<span class="w-16 shrink-0">{t('layout.spreadForce')}</span>
			<input
				type="range"
				min="1" max="30" step="1"
				value={layoutParams.edgeSymDiffLength}
				oninput={(e) => onParamChange('edgeSymDiffLength', Number(e.currentTarget.value))}
				class="range range-xs range-primary flex-1"
				style="--range-fill: {fillPercent(layoutParams.edgeSymDiffLength, 1, 30)}%;"
			/>
			<span class="w-16 text-right text-base-content/80 tabular-nums shrink-0">{layoutParams.edgeSymDiffLength}/{30}</span>
		</label>
		<label class="flex items-center gap-2 text-xs text-base-content/60">
			<span class="w-16 shrink-0">{t('layout.zoomSensitivity')}</span>
			<input
				type="range"
				min="0.05" max="1" step="0.05"
				value={wheelSensitivity}
				oninput={(e) => onWheelSensitivityChange(Number(e.currentTarget.value))}
				class="range range-xs range-primary flex-1"
				style="--range-fill: {fillPercent(wheelSensitivity, 0.05, 1)}%;"
			/>
			<span class="w-16 text-right text-base-content/80 tabular-nums shrink-0">{wheelSensitivity.toFixed(2)}/{(1).toFixed(2)}</span>
		</label>
		<div class="flex justify-end pt-1">
			<button
				class="btn btn-xs btn-ghost text-base-content/50 hover:text-base-content/80"
				onclick={onReset}
			>
				{t('layout.reset')}
			</button>
		</div>
	</div>
{/if}

<footer class="bg-black/60 backdrop-blur-sm px-5 py-2 flex items-center gap-4 text-sm text-base-content/60">
	<span>{t('bottom.nodes')}: <strong class="text-base-content/80">{graphStore.nodes.length}</strong></span>
	<span class="text-base-content/25">|</span>
	<span>{t('bottom.edges')}: <strong class="text-base-content/80">{graphStore.edges.length}</strong></span>
	<span class="text-base-content/25">|</span>
	<span class="flex items-center gap-1.5">
		<span class="w-2 h-2 rounded-full bg-[#78716c] inline-block"></span>
		{relCounts.RELATED_TO}
	</span>
	<span class="flex items-center gap-1.5">
		<span class="w-2 h-2 rounded-full bg-[#65a30d] inline-block"></span>
		{relCounts.SUPPORTS}
	</span>
	<span class="flex items-center gap-1.5">
		<span class="w-2 h-2 rounded-full bg-[#dc2626] inline-block"></span>
		{relCounts.CONFLICTS_WITH}
	</span>
	<span class="text-base-content/25">|</span>
	<span>AI {sourceCounts.ai} / Human {sourceCounts.human}</span>

	<span class="ml-auto">
		<button
			class="btn btn-xs btn-square btn-ghost hover:text-base-content/80 {showSliders ? 'text-base-content/80' : 'text-base-content/40'}"
			onclick={() => (showSliders = !showSliders)}
			title={t('layout.title')}
		>
			<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="size-3.5">
				<path fill-rule="evenodd" d="M7.84 1.804A1 1 0 0 1 8.82 1h2.36a1 1 0 0 1 .98.804l.331 1.652a6.993 6.993 0 0 1 1.929 1.115l1.598-.54a1 1 0 0 1 1.186.447l1.18 2.044a1 1 0 0 1-.205 1.251l-1.267 1.113a7.047 7.047 0 0 1 0 2.228l1.267 1.113a1 1 0 0 1 .206 1.25l-1.18 2.045a1 1 0 0 1-1.187.447l-1.598-.54a6.993 6.993 0 0 1-1.929 1.115l-.33 1.652a1 1 0 0 1-.98.804H8.82a1 1 0 0 1-.98-.804l-.331-1.652a6.993 6.993 0 0 1-1.929-1.115l-1.598.54a1 1 0 0 1-1.186-.447l-1.18-2.044a1 1 0 0 1 .205-1.251l1.267-1.114a7.05 7.05 0 0 1 0-2.227L1.821 7.773a1 1 0 0 1-.206-1.25l1.18-2.045a1 1 0 0 1 1.187-.447l1.598.54A6.992 6.992 0 0 1 7.51 3.456l.33-1.652ZM10 13a3 3 0 1 0 0-6 3 3 0 0 0 0 6Z" clip-rule="evenodd" />
			</svg>
		</button>
	</span>
</footer>
