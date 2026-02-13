import { cytoscapeStylesheet } from './cytoscape-styles';

export const cytoscapeLayout = {
	name: 'cola',
	animate: true,
	infinite: true,
	fit: false,
	nodeSpacing: 30,
	edgeLength: 120,
	convergenceThreshold: 0.001,
	maxSimulationTime: 0,
	handleDisconnected: true,
	avoidOverlap: true,
	edgeSymDiffLength: 6,
	edgeJaccardLength: 12,
	unconstrIter: 20,
	userConstIter: 10,
	allConstIter: 20
};

export const defaultCytoscapeOptions = {
	style: cytoscapeStylesheet,
	layout: { name: 'preset' },
	minZoom: 0.1,
	maxZoom: 4,
	wheelSensitivity: 0.25,
	boxSelectionEnabled: false,
	pixelRatio: 'auto' as const
};
