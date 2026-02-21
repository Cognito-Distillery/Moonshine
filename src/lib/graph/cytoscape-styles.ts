import type { StylesheetStyle } from 'cytoscape';
import type { ThemeId } from '$lib/stores/settings.svelte';

interface GraphColors {
	nodeBg: string;
	nodeLabel: string;
	nodeBorder: string;
	textOutline: string;
	selectedBorder: string;
	selectedBg: string;
	highlightedBg: string;
	highlightedBorder: string;
	searchMatchBg: string;
	searchMatchBorder: string;
	searchMatchLabel: string;
	searchMatchOutline: string;
	grabbedBg: string;
	grabbedBorder: string;
	edgeRelated: string;
	edgeSupports: string;
	edgeConflicts: string;
	edgeSelected: string;
}

const darkColors: GraphColors = {
	nodeBg: '#8e8e8e',
	nodeLabel: '#aaaaaa',
	nodeBorder: '#717171',
	textOutline: '#000000',
	selectedBorder: '#e3e3e3',
	selectedBg: '#aaaaaa',
	highlightedBg: '#aaaaaa',
	highlightedBorder: '#e3e3e3',
	searchMatchBg: '#ffffff',
	searchMatchBorder: '#c6c6c6',
	searchMatchLabel: '#e3e3e3',
	searchMatchOutline: '#000000',
	grabbedBg: '#c6c6c6',
	grabbedBorder: '#e3e3e3',
	edgeRelated: '#555555',
	edgeSupports: '#4d7c0f',
	edgeConflicts: '#b91c1c',
	edgeSelected: '#e3e3e3'
};

const lightColors: GraphColors = {
	nodeBg: '#555555',
	nodeLabel: '#393939',
	nodeBorder: '#717171',
	textOutline: '#ffffff',
	selectedBorder: '#1c1c1c',
	selectedBg: '#717171',
	highlightedBg: '#717171',
	highlightedBorder: '#1c1c1c',
	searchMatchBg: '#393939',
	searchMatchBorder: '#1c1c1c',
	searchMatchLabel: '#393939',
	searchMatchOutline: '#ffffff',
	grabbedBg: '#e3e3e3',
	grabbedBorder: '#aaaaaa',
	edgeRelated: '#aaaaaa',
	edgeSupports: '#3f6212',
	edgeConflicts: '#991b1b',
	edgeSelected: '#1c1c1c'
};

const warmEarthColors: GraphColors = {
	nodeBg: '#815839',
	nodeLabel: '#b76935',
	nodeBorder: '#6f523b',
	textOutline: '#143642',
	selectedBorder: '#b76935',
	selectedBg: '#935e38',
	highlightedBg: '#935e38',
	highlightedBorder: '#b76935',
	searchMatchBg: '#b76935',
	searchMatchBorder: '#a56336',
	searchMatchLabel: '#f0d5b8',
	searchMatchOutline: '#143642',
	grabbedBg: '#a56336',
	grabbedBorder: '#b76935',
	edgeRelated: '#5c4d3c',
	edgeSupports: '#5a8a3c',
	edgeConflicts: '#a84832',
	edgeSelected: '#b76935'
};

const warmEarthLightColors: GraphColors = {
	nodeBg: '#815839',
	nodeLabel: '#5c4d3c',
	nodeBorder: '#6f523b',
	textOutline: '#f0e4d7',
	selectedBorder: '#b76935',
	selectedBg: '#935e38',
	highlightedBg: '#935e38',
	highlightedBorder: '#b76935',
	searchMatchBg: '#b76935',
	searchMatchBorder: '#a56336',
	searchMatchLabel: '#4a473e',
	searchMatchOutline: '#f0e4d7',
	grabbedBg: '#d4c0aa',
	grabbedBorder: '#b76935',
	edgeRelated: '#c4b5a3',
	edgeSupports: '#4d7a2e',
	edgeConflicts: '#a84832',
	edgeSelected: '#5c4d3c'
};

const peachColors: GraphColors = {
	nodeBg: '#fec89a',
	nodeLabel: '#8b5e55',
	nodeBorder: '#ffd7ba',
	textOutline: '#f8edeb',
	selectedBorder: '#8b5e55',
	selectedBg: '#fec5bb',
	highlightedBg: '#fec5bb',
	highlightedBorder: '#8b5e55',
	searchMatchBg: '#ffe5d9',
	searchMatchBorder: '#fec89a',
	searchMatchLabel: '#5c3a30',
	searchMatchOutline: '#f8edeb',
	grabbedBg: '#fcd5ce',
	grabbedBorder: '#fec89a',
	edgeRelated: '#ece4db',
	edgeSupports: '#7db88a',
	edgeConflicts: '#c96060',
	edgeSelected: '#8b5e55'
};

const forestColors: GraphColors = {
	nodeBg: '#2a9d8f',
	nodeLabel: '#8ab17d',
	nodeBorder: '#287271',
	textOutline: '#264653',
	selectedBorder: '#e9c46a',
	selectedBg: '#2a9d8f',
	highlightedBg: '#2a9d8f',
	highlightedBorder: '#e9c46a',
	searchMatchBg: '#f4a261',
	searchMatchBorder: '#ee8959',
	searchMatchLabel: '#e9c46a',
	searchMatchOutline: '#264653',
	grabbedBg: '#babb74',
	grabbedBorder: '#efb366',
	edgeRelated: '#287271',
	edgeSupports: '#8ab17d',
	edgeConflicts: '#e76f51',
	edgeSelected: '#e9c46a'
};

const oceanicColors: GraphColors = {
	nodeBg: '#0a9396',
	nodeLabel: '#94d2bd',
	nodeBorder: '#005f73',
	textOutline: '#001219',
	selectedBorder: '#ee9b00',
	selectedBg: '#0a9396',
	highlightedBg: '#0a9396',
	highlightedBorder: '#ee9b00',
	searchMatchBg: '#ca6702',
	searchMatchBorder: '#bb3e03',
	searchMatchLabel: '#e9d8a6',
	searchMatchOutline: '#001219',
	grabbedBg: '#94d2bd',
	grabbedBorder: '#0a9396',
	edgeRelated: '#005f73',
	edgeSupports: '#94d2bd',
	edgeConflicts: '#ae2012',
	edgeSelected: '#ee9b00'
};

const themeGraphColors: Record<ThemeId, GraphColors> = {
	'monochrome-black': darkColors,
	'monochrome-white': lightColors,
	'warm-earth': warmEarthColors,
	'warm-earth-light': warmEarthLightColors,
	peach: peachColors,
	forest: forestColors,
	oceanic: oceanicColors
};

export function getCytoscapeStylesheet(themeId: ThemeId): StylesheetStyle[] {
	const c = themeGraphColors[themeId];

	return [
		{
			selector: 'node',
			style: {
				'background-color': c.nodeBg,
				label: 'data(label)',
				color: c.nodeLabel,
				'font-size': '8px',
				'font-weight': 'normal',
				'text-wrap': 'ellipsis',
				'text-max-width': '90px',
				'text-valign': 'bottom',
				'text-margin-y': 5,
				width: 20,
				height: 20,
				'border-width': 1.5,
				'border-color': c.nodeBorder,
				'text-outline-color': c.textOutline,
				'text-outline-width': 2,
				'overlay-opacity': 0
			}
		},
		{
			selector: 'node:selected',
			style: {
				'border-width': 2.5,
				'border-color': c.selectedBorder,
				'background-color': c.selectedBg,
				width: 24,
				height: 24
			}
		},
		{
			selector: 'node.highlighted',
			style: {
				'background-color': c.highlightedBg,
				'border-width': 1.5,
				'border-color': c.highlightedBorder
			}
		},
		{
			selector: 'node.dimmed',
			style: {
				opacity: 0.15
			}
		},
		{
			selector: 'node.search-match',
			style: {
				'background-color': c.searchMatchBg,
				'border-width': 3,
				'border-color': c.searchMatchBorder,
				width: 28,
				height: 28,
				color: c.searchMatchLabel,
				'font-weight': 'bold',
				'text-outline-color': c.searchMatchOutline,
				'text-outline-width': 3
			}
		},
		{
			selector: 'node:grabbed',
			style: {
				'background-color': c.grabbedBg,
				'border-color': c.grabbedBorder,
				'border-width': 2,
				'overlay-opacity': 0
			}
		},
		{
			selector: 'edge[relationType="RELATED_TO"]',
			style: {
				'line-color': c.edgeRelated,
				'target-arrow-color': c.edgeRelated,
				'line-style': 'solid',
				width: 1.2,
				'target-arrow-shape': 'triangle',
				'arrow-scale': 0.8,
				'curve-style': 'bezier',
				opacity: 0.6
			}
		},
		{
			selector: 'edge[relationType="SUPPORTS"]',
			style: {
				'line-color': c.edgeSupports,
				'target-arrow-color': c.edgeSupports,
				'line-style': 'solid',
				width: 1.2,
				'target-arrow-shape': 'triangle',
				'arrow-scale': 0.8,
				'curve-style': 'bezier',
				opacity: 0.7
			}
		},
		{
			selector: 'edge[relationType="CONFLICTS_WITH"]',
			style: {
				'line-color': c.edgeConflicts,
				'target-arrow-color': c.edgeConflicts,
				'line-style': 'dashed',
				width: 1.2,
				'target-arrow-shape': 'triangle',
				'arrow-scale': 0.8,
				'curve-style': 'bezier',
				opacity: 0.7
			}
		},
		{
			selector: 'edge[edgeSource="human"]',
			style: {
				width: 2.5,
				opacity: 0.85
			}
		},
		{
			selector: 'edge:selected',
			style: {
				width: 3,
				'line-color': c.edgeSelected,
				'target-arrow-color': c.edgeSelected,
				opacity: 1
			}
		},
		{
			selector: 'edge.dimmed',
			style: {
				opacity: 0.05
			}
		},
		{
			selector: 'edge.highlighted',
			style: {
				opacity: 0.9
			}
		}
	];
}

export const graphBackgrounds: Record<ThemeId, string> = {
	'monochrome-black': '#000000',
	'monochrome-white': '#ffffff',
	'warm-earth': '#143642',
	'warm-earth-light': '#f0e4d7',
	peach: '#f8edeb',
	forest: '#264653',
	oceanic: '#001219'
};

export const edgeDotColors: Record<ThemeId, { related: string; supports: string; conflicts: string }> = {
	'monochrome-black': { related: '#555555', supports: '#65a30d', conflicts: '#dc2626' },
	'monochrome-white': { related: '#aaaaaa', supports: '#3f6212', conflicts: '#991b1b' },
	'warm-earth': { related: '#815839', supports: '#5a8a3c', conflicts: '#a84832' },
	'warm-earth-light': { related: '#c4b5a3', supports: '#4d7a2e', conflicts: '#a84832' },
	peach: { related: '#ece4db', supports: '#7db88a', conflicts: '#c96060' },
	forest: { related: '#287271', supports: '#8ab17d', conflicts: '#e76f51' },
	oceanic: { related: '#005f73', supports: '#94d2bd', conflicts: '#ae2012' }
};
