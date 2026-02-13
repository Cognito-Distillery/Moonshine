import type {
	GraphNode,
	GraphEdge,
	CytoscapeElement,
	RelationType,
	EdgeSource
} from '$lib/types/graph';
import { buildEdgeId } from '$lib/utils/edge-id';

function createGraphStore() {
	let nodes = $state<GraphNode[]>([]);
	let edges = $state<GraphEdge[]>([]);
	let selectedNodeId = $state<string | null>(null);
	let selectedEdgeId = $state<string | null>(null);
	let activeRelationTypes = $state<Set<RelationType>>(
		new Set(['RELATED_TO', 'SUPPORTS', 'CONFLICTS_WITH'])
	);
	let activeSource = $state<EdgeSource | 'all'>('all');
	let searchMatchIds = $state<Set<string>>(new Set());

	const filteredEdges = $derived(
		edges.filter((e) => {
			const typeMatch = activeRelationTypes.has(e.relationType);
			const sourceMatch = activeSource === 'all' || e.source === activeSource;
			return typeMatch && sourceMatch;
		})
	);

	const connectedNodeIds = $derived(
		new Set(filteredEdges.flatMap((e) => [e.sourceId, e.targetId]))
	);

	const cytoscapeElements = $derived<CytoscapeElement[]>([
		...nodes
			.filter((n) => filteredEdges.length === 0 || connectedNodeIds.has(n.id))
			.map((n) => ({
				data: {
					id: n.id,
					label: n.summary.length > 40 ? n.summary.slice(0, 37) + '...' : n.summary,
					summary: n.summary,
					context: n.context,
					memo: n.memo,
					nodeType: n.type
				}
			})),
		...filteredEdges.map((e) => ({
			data: {
				id: buildEdgeId(e.sourceId, e.targetId),
				source: e.sourceId,
				target: e.targetId,
				relationType: e.relationType,
				edgeSource: e.source,
				confidence: e.confidence
			}
		}))
	]);

	const selectedNode = $derived(
		selectedNodeId ? (nodes.find((n) => n.id === selectedNodeId) ?? null) : null
	);

	const selectedEdge = $derived(
		selectedEdgeId
			? (edges.find((e) => buildEdgeId(e.sourceId, e.targetId) === selectedEdgeId) ?? null)
			: null
	);

	return {
		get nodes() { return nodes; },
		get edges() { return edges; },
		get selectedNodeId() { return selectedNodeId; },
		get selectedEdgeId() { return selectedEdgeId; },
		get selectedNode() { return selectedNode; },
		get selectedEdge() { return selectedEdge; },
		get activeRelationTypes() { return activeRelationTypes; },
		get activeSource() { return activeSource; },
		get filteredEdges() { return filteredEdges; },
		get cytoscapeElements() { return cytoscapeElements; },
		get searchMatchIds() { return searchMatchIds; },

		setGraphData(data: { nodes: GraphNode[]; edges: GraphEdge[] }) {
			nodes = data.nodes;
			edges = data.edges;
		},
		mergeGraphData(data: { nodes: GraphNode[]; edges: GraphEdge[] }) {
			const existingNodeIds = new Set(nodes.map((n) => n.id));
			const existingEdgeIds = new Set(
				edges.map((e) => buildEdgeId(e.sourceId, e.targetId))
			);
			nodes = [...nodes, ...data.nodes.filter((n) => !existingNodeIds.has(n.id))];
			edges = [
				...edges,
				...data.edges.filter(
					(e) => !existingEdgeIds.has(buildEdgeId(e.sourceId, e.targetId))
				)
			];
		},
		selectNode(id: string | null) {
			selectedNodeId = id;
			selectedEdgeId = null;
		},
		selectEdge(id: string | null) {
			selectedEdgeId = id;
			selectedNodeId = null;
		},
		clearSelection() {
			selectedNodeId = null;
			selectedEdgeId = null;
		},
		toggleRelationType(type: RelationType) {
			const next = new Set(activeRelationTypes);
			if (next.has(type)) next.delete(type);
			else next.add(type);
			activeRelationTypes = next;
		},
		setActiveSource(source: EdgeSource | 'all') {
			activeSource = source;
		},
		addEdge(edge: GraphEdge) {
			edges = [...edges, edge];
		},
		updateEdge(sourceId: string, targetId: string, relationType: RelationType) {
			edges = edges.map((e) =>
				e.sourceId === sourceId && e.targetId === targetId
					? { ...e, relationType, source: 'human' as EdgeSource }
					: e
			);
		},
		removeEdge(sourceId: string, targetId: string) {
			edges = edges.filter((e) => !(e.sourceId === sourceId && e.targetId === targetId));
		},
		updateNode(id: string, updates: Partial<GraphNode>) {
			nodes = nodes.map((n) => (n.id === id ? { ...n, ...updates } : n));
		},
		setSearchMatches(ids: string[]) {
			searchMatchIds = new Set(ids);
		},
		clearSearchMatches() {
			searchMatchIds = new Set();
		}
	};
}

export const graphStore = createGraphStore();
