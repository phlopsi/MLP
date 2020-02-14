#![no_implicit_prelude]

use ::std::boxed::Box;
use ::std::iter::Iterator;
use ::std::option::Option;

pub fn feed_forward(vertices: &mut [Box<[f32]>], edges: &[Box<[f32]>]) {
    let mut vertices_iter = vertices.iter_mut();

    if let Option::Some(mut layer) = vertices_iter.next() {
        for (next_layer, edge_matrix) in vertices_iter.zip(edges.iter()) {
            let (carry_target_vertices, target_vertices) = next_layer.split_at_mut(layer.len());

            for (source_vertex, target_vertex) in layer.iter().zip(carry_target_vertices) {
                *target_vertex = *source_vertex;
            }

            for (source_vertex, edge_row) in layer
                .iter()
                .zip(edge_matrix.chunks_exact(target_vertices.len()))
            {
                for (edge, target_vertex) in edge_row.iter().zip(target_vertices.iter_mut()) {
                    *target_vertex += *source_vertex * edge;
                }
            }

            for target_vertex in target_vertices.iter_mut() {
                *target_vertex = lgc(*target_vertex);
            }

            layer = next_layer;
        }
    }
}

#[inline]
fn lgc(x: f32) -> f32 {
    1.0 / (1.0 + (-x).exp())
}
