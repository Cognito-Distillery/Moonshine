<script lang="ts">
	import { onMount } from 'svelte';
	import cytoscape from 'cytoscape';
	import cola from 'cytoscape-cola';
	import { graphStore } from '$lib/stores/graph.svelte';
	import { uiStore } from '$lib/stores/ui.svelte';
	import { showToast } from '$lib/stores/toast.svelte';
	import { t } from '$lib/i18n/index.svelte';
	import {
		createCytoscapeOptions,
		createCytoscapeLayout,
		defaultLayoutParams,
		DEFAULT_WHEEL_SENSITIVITY,
		type LayoutParams
	} from '$lib/graph/cytoscape-config';
	import { getCytoscapeStylesheet, graphBackgrounds } from '$lib/graph/cytoscape-styles';
	import { getTheme } from '$lib/stores/settings.svelte';
	import {
		setupNodeEvents,
		setupHoverEvents,
		setupDoubleClickEvents,
		createContextMenuHandler,
		handleContextMenuAction,
		type ContextMenuState
	} from '$lib/graph/cytoscape-events';
	import { getSetting, setSetting } from '$lib/commands/settings';
	import ContextMenu from './ContextMenu.svelte';
	import SearchInput from './SearchInput.svelte';
	import RecentSearches from './RecentSearches.svelte';
	import SidePanel from './SidePanel.svelte';
	import ConfirmDialog from './ConfirmDialog.svelte';
	import BottomBar from './BottomBar.svelte';

	cytoscape.use(cola);

	let container: HTMLDivElement | undefined = $state();
	let cy: cytoscape.Core | undefined = $state();
	let layout: cytoscape.Layouts | undefined = $state();
	let contextMenu = $state<ContextMenuState>({
		visible: false,
		x: 0,
		y: 0,
		items: [],
		targetId: null,
		targetType: 'canvas'
	});

	let layoutParams = $state<LayoutParams>({ ...defaultLayoutParams });
	let wheelSensitivity = $state(DEFAULT_WHEEL_SENSITIVITY);
	let showHistory = $state(false);
	let showSearchSettings = $state(false);
	let searchThreshold = $state(0.3);
	let searchTopK = $state(10);
	let prevElementIds = new Set<string>();

	onMount(async () => {
		try {
			const [thresholdVal, topKVal] = await Promise.all([
				getSetting('search_threshold'),
				getSetting('search_top_k')
			]);
			if (thresholdVal) searchThreshold = parseFloat(thresholdVal);
			if (topKVal) searchTopK = parseInt(topKVal);
		} catch {
			// ignore
		}
	});

	function startLayout() {
		if (!cy) return;
		layout?.stop();
		layout = cy.layout(createCytoscapeLayout(layoutParams) as unknown as cytoscape.LayoutOptions);
		layout.run();
	}

	function updateLayoutParam<K extends keyof LayoutParams>(key: K, value: LayoutParams[K]) {
		layoutParams = { ...layoutParams, [key]: value };
	}

	function updateWheelSensitivity(value: number) {
		wheelSensitivity = value;
		if (cy) {
			(cy as unknown as { _private: { options: { wheelSensitivity: number } } })._private.options.wheelSensitivity = value;
		}
	}

	function resetLayoutParams() {
		layoutParams = { ...defaultLayoutParams };
		updateWheelSensitivity(DEFAULT_WHEEL_SENSITIVITY);
	}

	// Re-run layout when params change (debounced, skip initial)
	let layoutParamsInitialized = false;
	let debounceTimer: ReturnType<typeof setTimeout> | undefined;
	$effect(() => {
		const _params = layoutParams;
		if (!layoutParamsInitialized) {
			layoutParamsInitialized = true;
			return;
		}
		clearTimeout(debounceTimer);
		debounceTimer = setTimeout(() => {
			if (cy) startLayout();
		}, 300);
	});

	onMount(() => {
		if (!container) return;

		try {
			cy = cytoscape({
				container,
				elements: graphStore.cytoscapeElements,
				...createCytoscapeOptions(getTheme())
			});

			setupNodeEvents(cy);
			setupHoverEvents(cy);
			setupDoubleClickEvents(cy);
			createContextMenuHandler(cy, (state) => {
				contextMenu = state;
			});

			prevElementIds = new Set(graphStore.cytoscapeElements.map((el) => el.data.id));

			startLayout();
			setTimeout(() => cy?.fit(undefined, 60), 800);
		} catch {
			showToast(t('error.renderGraph'));
		}

		return () => {
			clearTimeout(debounceTimer);
			layout?.stop();
			cy?.destroy();
		};
	});

	// Sync store elements → Cytoscape
	$effect(() => {
		if (!cy) return;
		try {
			const elements = graphStore.cytoscapeElements;
			const newIds = new Set(elements.map((el) => el.data.id));

			let changed = false;

			cy.batch(() => {
				for (const id of prevElementIds) {
					if (!newIds.has(id)) {
						const ele = cy!.getElementById(id);
						if (ele.length) cy!.remove(ele);
						changed = true;
					}
				}

				for (const el of elements) {
					const existing = cy!.getElementById(el.data.id);
					if (existing.length) {
						existing.data(el.data);
					} else {
						cy!.add(el as cytoscape.ElementDefinition);
						changed = true;
					}
				}
			});

			if (changed) {
				startLayout();
			}

			prevElementIds = newIds;
		} catch {
			showToast(t('error.renderGraph'));
		}
	});

	// Pan to selected node
	$effect(() => {
		if (!cy || !graphStore.selectedNodeId) return;
		const node = cy.getElementById(graphStore.selectedNodeId);
		if (node.length) {
			cy.animate({
				center: { eles: node },
				duration: 300
			});
		}
	});

	// Highlight search matches
	$effect(() => {
		if (!cy) return;
		const matchIds = graphStore.searchMatchIds;
		cy.nodes().forEach((node) => {
			if (matchIds.size > 0) {
				if (matchIds.has(node.id())) {
					node.addClass('search-match');
					node.removeClass('dimmed');
				} else {
					node.removeClass('search-match');
					node.addClass('dimmed');
				}
			} else {
				node.removeClass('search-match');
				node.removeClass('dimmed');
			}
		});
		cy.edges().forEach((edge) => {
			if (matchIds.size > 0) {
				const src = edge.source().id();
				const tgt = edge.target().id();
				if (matchIds.has(src) || matchIds.has(tgt)) {
					edge.removeClass('dimmed');
				} else {
					edge.addClass('dimmed');
				}
			} else {
				edge.removeClass('dimmed');
			}
		});
	});

	// Re-apply Cytoscape styles when theme changes
	$effect(() => {
		if (!cy) return;
		const theme = getTheme();
		const s = cy.style() as unknown as {
			clear(): typeof s;
			fromJson(json: unknown): typeof s;
			update(): void;
		};
		s.clear().fromJson(getCytoscapeStylesheet(theme)).update();
	});

	function onContextAction(action: string) {
		if (!cy) return;
		handleContextMenuAction(action, contextMenu.targetId, contextMenu.targetType, cy);
	}

	export function focusNode(nodeId: string) {
		if (!cy) return;
		const node = cy.getElementById(nodeId);
		if (node.length) {
			cy.animate({
				center: { eles: node },
				zoom: 2,
				duration: 400
			});
		}
	}

	export function fitGraph() {
		cy?.fit(undefined, 60);
	}
</script>

<div class="relative w-full h-full" style="background: {graphBackgrounds[getTheme()]}">
	<div bind:this={container} class="w-full h-full"></div>

	<!-- Floating hamburger button - top left -->
	<button
		class="absolute top-3 left-3 z-20 btn btn-sm btn-square bg-base-100/60 border-base-content/10 hover:bg-base-100/80 backdrop-blur-sm"
		onclick={() => uiStore.toggleSidePanel()}
		title={t('graph.togglePanel')}
	>
		<svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
			<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
		</svg>
	</button>

	<!-- Floating logo - top center -->
	<div class="absolute top-3 left-1/2 -translate-x-1/2 z-20 flex items-center gap-2 opacity-40 pointer-events-none select-none">
		<span class="text-xs font-medium tracking-widest uppercase text-base-content/60">{t('topbar.title')}</span>
	</div>

	<!-- Floating search - top right -->
	<div class="absolute top-3 right-3 z-20">
		<div class="flex items-center gap-1.5">
			{#if graphStore.searchMatchIds.size > 0}
				<button
					class="p-1.5 rounded-lg bg-base-100/60 backdrop-blur-sm border border-base-content/10 text-base-content/60 hover:text-base-content/80 hover:border-base-content/20 transition-colors"
					onclick={() => graphStore.clearSearchMatches()}
					title={t('graph.clearHighlights')}
				>
					<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" fill="currentColor" class="size-3.5">
						<path d="M5.28 4.22a.75.75 0 0 0-1.06 1.06L6.94 8l-2.72 2.72a.75.75 0 1 0 1.06 1.06L8 9.06l2.72 2.72a.75.75 0 1 0 1.06-1.06L9.06 8l2.72-2.72a.75.75 0 0 0-1.06-1.06L8 6.94 5.28 4.22Z" />
					</svg>
				</button>
			{/if}
			<SearchInput onfocus={focusNode} />
			<button
				class="p-1.5 rounded-lg bg-base-100/60 backdrop-blur-sm border border-base-content/10 transition-colors {showHistory ? 'text-primary border-primary/30' : 'text-base-content/60 hover:text-base-content/80 hover:border-base-content/20'}"
				onclick={() => { showHistory = !showHistory; showSearchSettings = false; }}
				title={t('search.recent')}
			>
				<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" fill="currentColor" class="size-3.5">
					<path fill-rule="evenodd" d="M1 8a7 7 0 1 1 14 0A7 7 0 0 1 1 8Zm7.75-4.25a.75.75 0 0 0-1.5 0V8c0 .414.336.75.75.75h3.25a.75.75 0 0 0 0-1.5h-2.5v-3.5Z" clip-rule="evenodd" />
				</svg>
			</button>
			<button
				class="p-1.5 rounded-lg bg-base-100/60 backdrop-blur-sm border border-base-content/10 transition-colors {showSearchSettings ? 'text-primary border-primary/30' : 'text-base-content/60 hover:text-base-content/80 hover:border-base-content/20'}"
				onclick={() => { showSearchSettings = !showSearchSettings; showHistory = false; }}
				title={t('filter.searchSettings')}
			>
				<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" fill="currentColor" class="size-3.5">
					<path d="M6.455 1.45A.5.5 0 0 1 6.952 1h2.096a.5.5 0 0 1 .497.45l.186 1.858a4.996 4.996 0 0 1 1.466.848l1.703-.769a.5.5 0 0 1 .639.206l1.048 1.814a.5.5 0 0 1-.142.656l-1.517 1.09a5.026 5.026 0 0 1 0 1.694l1.517 1.09a.5.5 0 0 1 .142.656l-1.048 1.814a.5.5 0 0 1-.639.206l-1.703-.769c-.433.36-.928.649-1.466.848l-.186 1.858a.5.5 0 0 1-.497.45H6.952a.5.5 0 0 1-.497-.45l-.186-1.858a4.993 4.993 0 0 1-1.466-.848l-1.703.769a.5.5 0 0 1-.639-.206L1.413 9.814a.5.5 0 0 1 .142-.656l1.517-1.09a5.026 5.026 0 0 1 0-1.694l-1.517-1.09a.5.5 0 0 1-.142-.656L2.461 2.814a.5.5 0 0 1 .639-.206l1.703.769c.433-.36.928-.649 1.466-.848l.186-1.858ZM8 10a2 2 0 1 0 0-4 2 2 0 0 0 0 4Z" />
				</svg>
			</button>
		</div>
		{#if showSearchSettings}
			<div class="absolute top-full right-0 mt-1.5 w-64 bg-base-100/80 backdrop-blur-md rounded-lg border border-base-content/[0.08] p-3 shadow-xl space-y-2">
				<div>
					<div class="flex items-center justify-between mb-1">
						<span class="text-xs text-base-content/70">{t('settings.searchThreshold')}</span>
						<span class="text-xs text-base-content/65 tabular-nums">{searchThreshold}</span>
					</div>
					<input
						type="range"
						class="range range-xs range-primary w-full"
						min="0.1"
						max="0.9"
						step="0.05"
						style="--range-fill: {((searchThreshold - 0.1) / (0.9 - 0.1)) * 100}%;"
						bind:value={searchThreshold}
						onchange={() => setSetting('search_threshold', String(searchThreshold))}
					/>
				</div>
				<div>
					<div class="flex items-center justify-between mb-1">
						<span class="text-xs text-base-content/70">{t('settings.searchTopK')}</span>
						<span class="text-xs text-base-content/65 tabular-nums">{searchTopK}</span>
					</div>
					<input
						type="range"
						class="range range-xs range-primary w-full"
						min="1"
						max="50"
						step="1"
						style="--range-fill: {((searchTopK - 1) / (50 - 1)) * 100}%;"
						bind:value={searchTopK}
						onchange={() => setSetting('search_top_k', String(searchTopK))}
					/>
				</div>
			</div>
		{/if}
		{#if showHistory}
			<div class="absolute top-full right-0 mt-1.5 w-64 bg-base-100/80 backdrop-blur-md rounded-lg border border-base-content/[0.08] p-3 shadow-xl">
				<RecentSearches />
			</div>
		{/if}
	</div>

	<!-- Side panel -->
	<SidePanel
		{layoutParams}
		{wheelSensitivity}
		onParamChange={updateLayoutParam}
		onWheelSensitivityChange={updateWheelSensitivity}
		onReset={resetLayoutParams}
	/>

	<ContextMenu
		state={contextMenu}
		onaction={onContextAction}
		onclose={() => (contextMenu.visible = false)}
	/>

	<ConfirmDialog />

	<!-- Bottom bar -->
	<div class="absolute bottom-0 left-0 right-0 z-40">
		<BottomBar />
	</div>
</div>
