import type { StylesheetStyle } from 'cytoscape';

export const cytoscapeStylesheet: StylesheetStyle[] = [
	{
		selector: 'node',
		style: {
			'background-color': '#7B8FA0',
			label: 'data(label)',
			color: '#8899AA',
			'font-size': '8px',
			'font-weight': 'normal',
			'text-wrap': 'ellipsis',
			'text-max-width': '90px',
			'text-valign': 'bottom',
			'text-margin-y': 5,
			width: 20,
			height: 20,
			'border-width': 1.5,
			'border-color': '#5A6E80',
			'text-outline-color': '#000000',
			'text-outline-width': 2,
			'overlay-opacity': 0
		}
	},
	{
		selector: 'node:selected',
		style: {
			'border-width': 2.5,
			'border-color': '#C4D4E0',
			'background-color': '#8FA3B5',
			width: 24,
			height: 24
		}
	},
	{
		selector: 'node.highlighted',
		style: {
			'background-color': '#8FA3B5',
			'border-width': 1.5,
			'border-color': '#C4D4E0'
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
			'background-color': '#38bdf8',
			'border-width': 3,
			'border-color': '#7dd3fc',
			width: 28,
			height: 28,
			color: '#e0f2fe',
			'font-weight': 'bold',
			'text-outline-color': '#000000',
			'text-outline-width': 3
		}
	},
	{
		selector: 'node:grabbed',
		style: {
			'background-color': '#C4D4E0',
			'border-color': '#E2E8F0',
			'border-width': 2,
			'overlay-opacity': 0
		}
	},
	{
		selector: 'edge[relationType="RELATED_TO"]',
		style: {
			'line-color': '#57534e',
			'target-arrow-color': '#57534e',
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
			'line-color': '#4d7c0f',
			'target-arrow-color': '#4d7c0f',
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
			'line-color': '#b91c1c',
			'target-arrow-color': '#b91c1c',
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
			'line-color': '#C4D4E0',
			'target-arrow-color': '#C4D4E0',
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
