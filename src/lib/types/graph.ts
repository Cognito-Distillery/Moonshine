export type RelationType = 'RELATED_TO' | 'SUPPORTS' | 'CONFLICTS_WITH';
export type EdgeSource = 'ai' | 'human';

export interface GraphNode {
	id: string;
	summary: string;
	context: string;
	memo: string;
	type: string;
	createdAt?: number;
	updatedAt?: number;
}

export interface GraphEdge {
	sourceId: string;
	targetId: string;
	relationType: RelationType;
	source: EdgeSource;
	confidence: number | null;
	createdAt?: number;
	updatedAt?: number;
}

export interface GraphData {
	nodes: GraphNode[];
	edges: GraphEdge[];
}

export interface CytoscapeNodeData {
	id: string;
	label: string;
	summary: string;
	context: string;
	memo: string;
	nodeType: string;
}

export interface CytoscapeEdgeData {
	id: string;
	source: string;
	target: string;
	relationType: RelationType;
	edgeSource?: EdgeSource;
	confidence?: number | null;
}

export type CytoscapeElement =
	| { data: CytoscapeNodeData }
	| { data: CytoscapeEdgeData };
