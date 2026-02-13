import { invoke } from '@tauri-apps/api/core';
import type { Mash } from '$lib/types';

export interface GraphNode {
	id: string;
	mashType: string;
	summary: string;
	context: string;
	memo: string;
	status: string;
	createdAt: number;
	updatedAt: number;
}

export interface GraphEdge {
	id: number;
	sourceId: string;
	targetId: string;
	relationType: string;
	source: string;
	confidence: number;
	createdAt: number;
	updatedAt: number;
}

export interface GraphData {
	nodes: GraphNode[];
	edges: GraphEdge[];
}

export interface GraphFilters {
	mashTypes?: string[];
	relationTypes?: string[];
	sources?: string[];
}

export function getGraph(filters: GraphFilters): Promise<GraphData> {
	return invoke<GraphData>('get_graph', { filters });
}

export function getNodeDetail(id: string): Promise<GraphData> {
	return invoke<GraphData>('get_node_detail', { id });
}

export function expandNode(id: string, depth: number): Promise<GraphData> {
	return invoke<GraphData>('expand_node', { id, depth });
}

export function addEdge(
	sourceId: string,
	targetId: string,
	relationType: string,
	source: string,
	confidence: number
): Promise<GraphEdge> {
	return invoke<GraphEdge>('add_edge', { sourceId, targetId, relationType, source, confidence });
}

export function updateEdge(id: number, relationType?: string, confidence?: number): Promise<GraphEdge> {
	return invoke<GraphEdge>('update_edge', { id, relationType, confidence });
}

export function deleteEdge(id: number): Promise<void> {
	return invoke('delete_edge', { id });
}

export function updateNode(
	id: string,
	mashType?: string,
	summary?: string,
	context?: string,
	memo?: string
): Promise<Mash> {
	return invoke<Mash>('update_node', { id, mashType, summary, context, memo });
}
