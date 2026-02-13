export function buildEdgeId(sourceId: string, targetId: string): string {
	return `${sourceId}__${targetId}`;
}

export function parseEdgeId(edgeId: string): { sourceId: string; targetId: string } {
	const idx = edgeId.indexOf('__');
	return {
		sourceId: edgeId.slice(0, idx),
		targetId: edgeId.slice(idx + 2)
	};
}
