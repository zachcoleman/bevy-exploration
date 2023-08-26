use bevy::prelude::*;
use std::collections::{HashSet, VecDeque};

#[derive(Clone, Debug, Resource)]
pub struct QuadTree {
    pub bounds: [Vec2; 2],
    pub min_size: f32,
    pub children: Option<[Box<QuadTree>; 4]>,
    pub objects: Option<HashSet<Entity>>,
}

impl QuadTree {
    pub fn new(bounds: [Vec2; 2], min_size: f32) -> Self {
        let mut root = QuadTree {
            bounds,
            min_size,
            children: None,
            objects: None,
        };
        let curr_size = bounds[1].x - bounds[0].x;
        if curr_size > min_size {
            let midpoint = Vec2::new(
                (bounds[0].x + bounds[1].x) / 2.,
                (bounds[0].y + bounds[1].y) / 2.,
            );
            let children_bounds = [
                // (0, 0)
                [root.bounds[0], midpoint],
                // (1, 0)
                [
                    Vec2::new(midpoint.x, bounds[0].y),
                    Vec2::new(bounds[1].x, midpoint.y),
                ],
                // (0, 1)
                [
                    Vec2::new(bounds[0].x, midpoint.y),
                    Vec2::new(midpoint.x, bounds[1].y),
                ],
                // (1, 1)
                [midpoint, root.bounds[1]],
            ];
            root.children = Some([
                Box::new(QuadTree::new(children_bounds[0], min_size)),
                Box::new(QuadTree::new(children_bounds[1], min_size)),
                Box::new(QuadTree::new(children_bounds[2], min_size)),
                Box::new(QuadTree::new(children_bounds[3], min_size)),
            ]);
        }
        root
    }

    pub fn contains(&self, position: Vec2) -> bool {
        self.bounds[0].x < position.x
            && position.x <= self.bounds[1].x
            && self.bounds[0].y < position.y
            && position.y <= self.bounds[1].y
    }

    pub fn get_leaf_nodes(&self) -> Vec<QuadTree> {
        let mut leaf_nodes = Vec::new();
        let mut node_queue = VecDeque::new();
        if let Some(children) = &self.children {
            for child in children {
                node_queue.push_back(child);
            }
        }

        while let Some(node) = node_queue.pop_front() {
            if let Some(children) = &node.children {
                for child in children {
                    node_queue.push_back(child);
                }
            } else {
                leaf_nodes.push((**node).clone());
            }
        }
        leaf_nodes
    }

    pub fn insert(&mut self, entity: Entity, position: Vec2) -> Result<(), ()> {
        while let Some(children) = &mut self.children {
            for child in children {
                if child.contains(position) {
                    return child.insert(entity, position);
                }
            }
            return Err(());
        }
        if self.objects.is_none() {
            self.objects = Some(HashSet::new());
        }
        self.objects.as_mut().unwrap().insert(entity);
        Ok(())
    }

    pub fn clear_objects(&mut self) {
        if self.objects.is_some() {
            self.objects = None;
        }
        if let Some(children) = &mut self.children {
            for child in children {
                child.clear_objects();
            }
        }
    }
}


#[derive(Clone, Component, Default, Reflect)]
#[reflect(Component)]
pub struct LeafNode {
    ix: usize,
}

pub fn add_quad_tree(mut commands: Commands) {
    commands.insert_resource(QuadTree::new(
        [Vec2::new(-64., -64.), Vec2::new(64., 64.)],
        4.,
    ));
}

pub fn clear_quad_tree(mut quad_tree: ResMut<QuadTree>) {
    quad_tree.clear_objects();
}

pub fn visualize_quad_tree_leaves(
    quad_tree: Res<QuadTree>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (ix, node) in quad_tree.get_leaf_nodes().iter().enumerate() {
        let bounds = node.bounds;
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube {
                    size: quad_tree.min_size,
                })),
                material: materials.add(StandardMaterial {
                    base_color: Color::rgba(1., 0., 0., 0.1),
                    alpha_mode: AlphaMode::Blend,
                    ..Default::default()
                }),
                transform: Transform::from_translation(Vec3::new(
                    (bounds[0].x + bounds[1].x) / 2.,
                    0.,
                    (bounds[0].y + bounds[1].y) / 2.,
                )),
                ..Default::default()
            },
            LeafNode { ix },
        ));
    }
}

pub fn update_leaf_node_color(
    quad_tree: ResMut<QuadTree>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    leaf_node_query: Query<(&LeafNode, &Handle<StandardMaterial>)>,
) {
    let leaf_node_vec: Vec<_> = quad_tree.get_leaf_nodes().into();
    for (leaf_node, material_handle) in leaf_node_query.iter() {
        let material = materials.get_mut(material_handle).unwrap();
        let orig_color = Color::rgba(1., 0., 0., 0.1);
        let node = &leaf_node_vec[leaf_node.ix];
        if node.objects.is_some() {
            material.base_color = Color::rgba(0., 0., 1., 0.1);
        } else if node.objects.is_none() {
            material.base_color = orig_color;
        }
    }
}