use rusqlite::{params, Connection};

use crate::db::now_ms;
use crate::models::{Edge, GraphData, GraphEdge, GraphFilters, GraphNode};

fn row_to_edge(row: &rusqlite::Row) -> rusqlite::Result<Edge> {
    Ok(Edge {
        id: row.get(0)?,
        source_id: row.get(1)?,
        target_id: row.get(2)?,
        relation_type: row.get(3)?,
        source: row.get(4)?,
        confidence: row.get(5)?,
        created_at: row.get(6)?,
        updated_at: row.get(7)?,
    })
}

fn row_to_graph_node(row: &rusqlite::Row) -> rusqlite::Result<GraphNode> {
    Ok(GraphNode {
        id: row.get(0)?,
        node_type: row.get(1)?,
        summary: row.get(2)?,
        context: row.get(3)?,
        memo: row.get(4)?,
        created_at: row.get(5)?,
        updated_at: row.get(6)?,
    })
}

fn row_to_graph_edge(row: &rusqlite::Row) -> rusqlite::Result<GraphEdge> {
    Ok(GraphEdge {
        id: row.get(0)?,
        source_id: row.get(1)?,
        target_id: row.get(2)?,
        relation_type: row.get(3)?,
        source: row.get(4)?,
        confidence: row.get(5)?,
    })
}

pub fn add_edge(
    conn: &Connection,
    source_id: &str,
    target_id: &str,
    relation_type: &str,
    source: &str,
    confidence: f64,
) -> Result<Edge, String> {
    let now = now_ms();
    conn.execute(
        "INSERT INTO edges (source_id, target_id, relation_type, source, confidence, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
         ON CONFLICT(source_id, target_id) DO UPDATE SET
            relation_type = excluded.relation_type,
            source = excluded.source,
            confidence = excluded.confidence,
            updated_at = excluded.updated_at",
        params![source_id, target_id, relation_type, source, confidence, now, now],
    )
    .map_err(|e| e.to_string())?;

    let mut stmt = conn
        .prepare(
            "SELECT id, source_id, target_id, relation_type, source, confidence, created_at, updated_at
             FROM edges WHERE source_id = ?1 AND target_id = ?2",
        )
        .map_err(|e| e.to_string())?;
    stmt.query_row(params![source_id, target_id], row_to_edge)
        .map_err(|e| e.to_string())
}

pub fn update_edge(
    conn: &Connection,
    id: i64,
    relation_type: Option<&str>,
    confidence: Option<f64>,
) -> Result<Edge, String> {
    let now = now_ms();

    if let Some(rt) = relation_type {
        conn.execute(
            "UPDATE edges SET relation_type = ?1, updated_at = ?2 WHERE id = ?3",
            params![rt, now, id],
        )
        .map_err(|e| e.to_string())?;
    }
    if let Some(c) = confidence {
        conn.execute(
            "UPDATE edges SET confidence = ?1, updated_at = ?2 WHERE id = ?3",
            params![c, now, id],
        )
        .map_err(|e| e.to_string())?;
    }

    let mut stmt = conn
        .prepare(
            "SELECT id, source_id, target_id, relation_type, source, confidence, created_at, updated_at
             FROM edges WHERE id = ?1",
        )
        .map_err(|e| e.to_string())?;
    stmt.query_row(params![id], row_to_edge)
        .map_err(|e| e.to_string())
}

pub fn delete_edge(conn: &Connection, id: i64) -> Result<(), String> {
    conn.execute("DELETE FROM edges WHERE id = ?1", params![id])
        .map_err(|e| e.to_string())?;
    Ok(())
}

pub fn get_graph(conn: &Connection, filters: &GraphFilters) -> Result<GraphData, String> {
    // Build dynamic WHERE clauses for nodes
    let mut node_conditions = vec!["m.status = 'JARRED'".to_string()];
    if let Some(ref types) = filters.mash_types {
        if !types.is_empty() {
            let placeholders: Vec<String> = types.iter().map(|t| format!("'{}'", t.replace('\'', "''"))).collect();
            node_conditions.push(format!("m.type IN ({})", placeholders.join(",")));
        }
    }
    let node_where = node_conditions.join(" AND ");

    let node_sql = format!(
        "SELECT m.id, m.type, m.summary, m.context, m.memo, m.created_at, m.updated_at
         FROM mashes m WHERE {}",
        node_where
    );

    let mut stmt = conn.prepare(&node_sql).map_err(|e| e.to_string())?;
    let nodes: Vec<GraphNode> = stmt
        .query_map([], row_to_graph_node)
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    let node_ids: Vec<&str> = nodes.iter().map(|n| n.id.as_str()).collect();
    if node_ids.is_empty() {
        return Ok(GraphData {
            nodes: vec![],
            edges: vec![],
        });
    }

    // Build edge query
    let mut edge_conditions = vec![
        "e.source_id IN (SELECT id FROM mashes WHERE status = 'JARRED')".to_string(),
        "e.target_id IN (SELECT id FROM mashes WHERE status = 'JARRED')".to_string(),
    ];
    if let Some(ref relation_types) = filters.relation_types {
        if !relation_types.is_empty() {
            let placeholders: Vec<String> = relation_types.iter().map(|t| format!("'{}'", t.replace('\'', "''"))).collect();
            edge_conditions.push(format!("e.relation_type IN ({})", placeholders.join(",")));
        }
    }
    if let Some(ref sources) = filters.sources {
        if !sources.is_empty() {
            let placeholders: Vec<String> = sources.iter().map(|s| format!("'{}'", s.replace('\'', "''"))).collect();
            edge_conditions.push(format!("e.source IN ({})", placeholders.join(",")));
        }
    }
    let edge_where = edge_conditions.join(" AND ");

    let edge_sql = format!(
        "SELECT e.id, e.source_id, e.target_id, e.relation_type, e.source, e.confidence
         FROM edges e WHERE {}",
        edge_where
    );

    let mut stmt = conn.prepare(&edge_sql).map_err(|e| e.to_string())?;
    let edges: Vec<GraphEdge> = stmt
        .query_map([], row_to_graph_edge)
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    Ok(GraphData { nodes, edges })
}

pub fn get_node_neighbors(conn: &Connection, id: &str) -> Result<GraphData, String> {
    let mut stmt = conn
        .prepare(
            "SELECT m.id, m.type, m.summary, m.context, m.memo, m.created_at, m.updated_at
             FROM mashes m
             WHERE m.status = 'JARRED' AND (
                 m.id = ?1
                 OR m.id IN (SELECT target_id FROM edges WHERE source_id = ?1)
                 OR m.id IN (SELECT source_id FROM edges WHERE target_id = ?1)
             )",
        )
        .map_err(|e| e.to_string())?;
    let nodes: Vec<GraphNode> = stmt
        .query_map(params![id], row_to_graph_node)
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    let mut stmt = conn
        .prepare(
            "SELECT e.id, e.source_id, e.target_id, e.relation_type, e.source, e.confidence
             FROM edges e
             WHERE e.source_id = ?1 OR e.target_id = ?1",
        )
        .map_err(|e| e.to_string())?;
    let edges: Vec<GraphEdge> = stmt
        .query_map(params![id], row_to_graph_edge)
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    Ok(GraphData { nodes, edges })
}

pub fn expand_node(conn: &Connection, id: &str, depth: u32) -> Result<GraphData, String> {
    let mut stmt = conn
        .prepare(
            "WITH RECURSIVE reachable(id, depth) AS (
                SELECT ?1, 0
                UNION
                SELECT CASE WHEN e.source_id = r.id THEN e.target_id ELSE e.source_id END, r.depth + 1
                FROM edges e JOIN reachable r ON (e.source_id = r.id OR e.target_id = r.id)
                WHERE r.depth < ?2
             )
             SELECT DISTINCT m.id, m.type, m.summary, m.context, m.memo, m.created_at, m.updated_at
             FROM mashes m JOIN reachable r ON m.id = r.id
             WHERE m.status = 'JARRED'",
        )
        .map_err(|e| e.to_string())?;
    let nodes: Vec<GraphNode> = stmt
        .query_map(params![id, depth], row_to_graph_node)
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    let node_ids: Vec<String> = nodes.iter().map(|n| n.id.clone()).collect();
    if node_ids.is_empty() {
        return Ok(GraphData {
            nodes: vec![],
            edges: vec![],
        });
    }

    let placeholders: Vec<String> = node_ids.iter().map(|id| format!("'{}'", id.replace('\'', "''"))).collect();
    let ids_str = placeholders.join(",");
    let edge_sql = format!(
        "SELECT e.id, e.source_id, e.target_id, e.relation_type, e.source, e.confidence
         FROM edges e
         WHERE e.source_id IN ({ids}) AND e.target_id IN ({ids})",
        ids = ids_str
    );

    let mut stmt = conn.prepare(&edge_sql).map_err(|e| e.to_string())?;
    let edges: Vec<GraphEdge> = stmt
        .query_map([], row_to_graph_edge)
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    Ok(GraphData { nodes, edges })
}
