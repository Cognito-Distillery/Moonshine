<script lang="ts">
	import { onMount } from 'svelte';
	import cytoscape from 'cytoscape';
	import cola from 'cytoscape-cola';
	import { graphStore } from '$lib/stores/graph.svelte';
	import { uiStore } from '$lib/stores/ui.svelte';
	import { showToast } from '$lib/stores/toast.svelte';
	import { t } from '$lib/i18n/index.svelte';
	import { defaultCytoscapeOptions, cytoscapeLayout } from '$lib/graph/cytoscape-config';
	import {
		setupNodeEvents,
		setupHoverEvents,
		setupDoubleClickEvents,
		createContextMenuHandler,
		handleContextMenuAction,
		type ContextMenuState
	} from '$lib/graph/cytoscape-events';
	import ContextMenu from './ContextMenu.svelte';
	import SearchInput from './SearchInput.svelte';
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

	let prevElementIds = new Set<string>();

	function startLayout() {
		if (!cy) return;
		layout?.stop();
		layout = cy.layout(cytoscapeLayout as unknown as cytoscape.LayoutOptions);
		layout.run();
	}

	onMount(() => {
		if (!container) return;

		try {
			cy = cytoscape({
				container,
				elements: graphStore.cytoscapeElements,
				...defaultCytoscapeOptions
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

<div class="relative w-full h-full" style="background: #000000;">
	<div bind:this={container} class="w-full h-full"></div>

	<!-- Floating hamburger button - top left -->
	<button
		class="absolute top-3 left-3 z-20 btn btn-sm btn-square bg-black/60 border-white/10 hover:bg-black/80 backdrop-blur-sm"
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
	<div class="absolute top-3 right-3 z-20 flex items-center gap-1.5">
		{#if graphStore.searchMatchIds.size > 0}
			<button
				class="p-1.5 rounded-lg bg-black/60 backdrop-blur-sm border border-white/10 text-base-content/40 hover:text-base-content/80 hover:border-white/20 transition-colors"
				onclick={() => graphStore.clearSearchMatches()}
				title={t('graph.clearHighlights')}
			>
				<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" fill="currentColor" class="size-3.5">
					<path d="M5.28 4.22a.75.75 0 0 0-1.06 1.06L6.94 8l-2.72 2.72a.75.75 0 1 0 1.06 1.06L8 9.06l2.72 2.72a.75.75 0 1 0 1.06-1.06L9.06 8l2.72-2.72a.75.75 0 0 0-1.06-1.06L8 6.94 5.28 4.22Z" />
				</svg>
			</button>
		{/if}
		<SearchInput onfocus={focusNode} />
	</div>

	<!-- Side panel -->
	<SidePanel />

	<ContextMenu
		state={contextMenu}
		onaction={onContextAction}
		onclose={() => (contextMenu.visible = false)}
	/>

	<ConfirmDialog />

	<!-- Bottom bar -->
	<div class="absolute bottom-0 left-0 right-0 z-10">
		<BottomBar />
	</div>
</div>
