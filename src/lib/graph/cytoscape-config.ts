import { cytoscapeStylesheet } from './cytoscape-styles';

export interface LayoutParams {
	nodeSpacing: number;
	edgeLength: number;
	convergenceThreshold: number;
	edgeSymDiffLength: number;
}

export const defaultLayoutParams: LayoutParams = {
	nodeSpacing: 30,
	edgeLength: 120,
	convergenceThreshold: 0.001,
	edgeSymDiffLength: 6
};

export function createCytoscapeLayout(overrides?: Partial<LayoutParams>) {
	const params = { ...defaultLayoutParams, ...overrides };
	return {
		name: 'cola',
		animate: true,
		infinite: true,
		fit: false,
		nodeSpacing: params.nodeSpacing,
		edgeLength: params.edgeLength,
		convergenceThreshold: params.convergenceThreshold,
		maxSimulationTime: 0,
		handleDisconnected: true,
		avoidOverlap: true,
		edgeSymDiffLength: params.edgeSymDiffLength,
		edgeJaccardLength: 12,
		unconstrIter: 20,
		userConstIter: 10,
		allConstIter: 20
	};
}

export const DEFAULT_WHEEL_SENSITIVITY = 0.25;

export const defaultCytoscapeOptions = {
	style: cytoscapeStylesheet,
	layout: { name: 'preset' },
	minZoom: 0.1,
	maxZoom: 4,
	wheelSensitivity: 0.25,
	boxSelectionEnabled: false,
	pixelRatio: 'auto' as const
};
