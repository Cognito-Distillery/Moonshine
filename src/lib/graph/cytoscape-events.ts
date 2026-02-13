import type cytoscape from 'cytoscape';
import { graphStore } from '$lib/stores/graph.svelte';
import { uiStore } from '$lib/stores/ui.svelte';
import { showToast } from '$lib/stores/toast.svelte';
import { parseEdgeId } from '$lib/utils/edge-id';
import * as graphCmd from '$lib/commands/graph';
import { nodeMenuItems, edgeMenuItems, canvasMenuItems, type ContextMenuItem } from './context-menu';
import { t } from '$lib/i18n/index.svelte';

export interface ContextMenuState {
	visible: boolean;
	x: number;
	y: number;
	items: ContextMenuItem[];
	targetId: string | null;
	targetType: 'node' | 'edge' | 'canvas';
}

export function setupNodeEvents(cy: cytoscape.Core) {
	cy.on('tap', 'node', (evt) => {
		const nodeId = evt.target.id();

		if (uiStore.sidePanelMode === 'add-edge' && uiStore.addEdgeSourceId) {
			if (uiStore.addEdgeSourceId !== nodeId) {
				graphStore.selectNode(nodeId);
			}
			return;
		}

		graphStore.selectNode(nodeId);
		uiStore.setSidePanelMode('detail');
	});

	cy.on('tap', 'edge', (evt) => {
		const edgeId = evt.target.id();
		graphStore.selectEdge(edgeId);
		uiStore.setSidePanelMode('detail');
	});

	cy.on('tap', (evt) => {
		if (evt.target === cy) {
			graphStore.clearSelection();
			if (uiStore.sidePanelMode === 'detail') {
				uiStore.setSidePanelMode('filter');
			}
		}
	});
}

export function setupHoverEvents(cy: cytoscape.Core) {
	cy.on('mouseover', 'node', (evt) => {
		const node = evt.target;
		const neighborhood = node.neighborhood().add(node);
		cy.elements().not(neighborhood).addClass('dimmed');
		neighborhood.addClass('highlighted');
	});

	cy.on('mouseout', 'node', () => {
		cy.elements().removeClass('dimmed highlighted');
	});

	cy.on('mouseover', 'edge', (evt) => {
		const edge = evt.target;
		const connected = edge.connectedNodes().add(edge);
		cy.elements().not(connected).addClass('dimmed');
		connected.addClass('highlighted');
	});

	cy.on('mouseout', 'edge', () => {
		cy.elements().removeClass('dimmed highlighted');
	});
}

export function setupDoubleClickEvents(cy: cytoscape.Core) {
	cy.on('dbltap', 'node', async (evt) => {
		const nodeId = evt.target.id();
		try {
			const data = await graphCmd.expandNode(nodeId, 1);
			graphStore.mergeGraphData({
				nodes: data.nodes.map((n) => ({
					id: n.id,
					summary: n.summary,
					context: n.context,
					memo: n.memo,
					type: n.mashType
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
			showToast(t('error.expandNode'));
		}
	});
}

export function createContextMenuHandler(
	cy: cytoscape.Core,
	setContextMenu: (state: ContextMenuState) => void
) {
	cy.on('cxttap', 'node', (evt) => {
		evt.originalEvent.preventDefault();
		const pos = evt.renderedPosition || evt.position;
		setContextMenu({
			visible: true,
			x: pos.x,
			y: pos.y,
			items: nodeMenuItems,
			targetId: evt.target.id(),
			targetType: 'node'
		});
	});

	cy.on('cxttap', 'edge', (evt) => {
		evt.originalEvent.preventDefault();
		const pos = evt.renderedPosition || evt.position;
		setContextMenu({
			visible: true,
			x: pos.x,
			y: pos.y,
			items: edgeMenuItems,
			targetId: evt.target.id(),
			targetType: 'edge'
		});
	});

	cy.on('cxttap', (evt) => {
		if (evt.target === cy) {
			evt.originalEvent.preventDefault();
			const pos = { x: evt.renderedPosition.x, y: evt.renderedPosition.y };
			setContextMenu({
				visible: true,
				x: pos.x,
				y: pos.y,
				items: canvasMenuItems,
				targetId: null,
				targetType: 'canvas'
			});
		}
	});
}

export async function handleContextMenuAction(
	action: string,
	targetId: string | null,
	targetType: 'node' | 'edge' | 'canvas',
	cy: cytoscape.Core
) {
	switch (action) {
		case 'view-detail':
			if (targetId && targetType === 'node') {
				graphStore.selectNode(targetId);
				uiStore.setSidePanelMode('detail');
			}
			break;
		case 'edit-node':
			if (targetId && targetType === 'node') {
				graphStore.selectNode(targetId);
				uiStore.setSidePanelMode('edit-node');
			}
			break;
		case 'add-edge':
			if (targetId && targetType === 'node') {
				uiStore.startAddEdge(targetId);
			}
			break;
		case 'expand':
			if (targetId && targetType === 'node') {
				try {
					const data = await graphCmd.expandNode(targetId, 1);
					graphStore.mergeGraphData({
						nodes: data.nodes.map((n) => ({
							id: n.id,
							summary: n.summary,
							context: n.context,
							memo: n.memo,
							type: n.mashType
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
					showToast(t('error.expandNode'));
				}
			}
			break;
		case 'edit-edge':
			if (targetId && targetType === 'edge') {
				graphStore.selectEdge(targetId);
				uiStore.setSidePanelMode('edit-edge');
			}
			break;
		case 'delete-edge':
			if (targetId && targetType === 'edge') {
				const { sourceId, targetId: tId } = parseEdgeId(targetId);
				uiStore.showConfirm(t('confirm.deleteEdge'), () => {
					graphStore.removeEdge(sourceId, tId);
					uiStore.dismissConfirm();
				});
			}
			break;
		case 'fit':
			cy.fit(undefined, 40);
			break;
	}
}
